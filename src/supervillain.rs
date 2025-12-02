//! Module for Super Villains and their related stuff

use rand::Rng;
use std::io::Read;
use std::time::Duration;
use thiserror::Error;

#[cfg(not(test))]
use std::fs::File;
#[cfg(test)]
use tests::doubles::File;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, double)]
use crate::sidekick::Sidekick;
#[cfg(test)]
use mockall_double::double;

use crate::Henchman;
use crate::{Cipher, Gadget};

const LISTING_PATH: &str = "tmp/listings.csv";

/// Type that represents supervillains
#[derive(Default)]
pub struct SuperVillain<'a> {
    pub first_name: String,
    pub last_name: String,
    pub sidekick: Option<Sidekick<'a>>,
    pub shared_key: String,
}

#[derive(Error, Debug)]
pub enum EvilError {
    #[error("Parse error: purpose='{}', reason='{}'", .purpose, .reason)]
    ParseError { purpose: String, reason: String },
}

#[cfg_attr(test, automock)]
pub trait MegaWeapon {
    fn shoot(&self);
}

impl SuperVillain<'_> {
    /// Returns the Super Villain's full name as a single string.
    ///
    /// A Full Name is produced by concatenating the first and last names with a space.
    ///
    /// # Examples
    /// ```
    ///# use evil::SuperVillain;
    /// let lex = SuperVillain {
    ///     first_name: "Lex".into(),
    ///     last_name: "Luthor".into(),
    ///     ..Default::default()
    /// };
    /// assert_eq!(lex.full_name(), "Lex Luthor");
    /// ```
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn set_full_name(&mut self, name: &str) {
        let components = name.split_whitespace().collect::<Vec<_>>();
        if components.len() != 2 {
            panic!("Name must have first and last name, separated by a space");
        }
        self.first_name = components[0].into();
        self.last_name = components[1].into();
    }

    pub fn attack(&self, weapon: &impl MegaWeapon, intense: bool) {
        weapon.shoot();
        if intense {
            let mut rng = rand::rng();
            let times = rng.random_range(1..3);
            for _ in 0..times {
                weapon.shoot();
            }
        }
    }

    pub async fn come_up_with_plan(&self) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        String::from("Take over the world!")
    }

    pub fn conspire(&mut self) {
        if let Some(ref sidekick) = self.sidekick {
            if !sidekick.agree() {
                self.sidekick = None;
            }
        }
    }

    pub fn start_world_domination_stage1<H: Henchman, G: Gadget>(
        &self,
        henchman: &mut H,
        gadget: &G,
    ) {
        if let Some(ref sidekick) = self.sidekick {
            let targets = sidekick.get_weak_targets(gadget);
            if !targets.is_empty() {
                henchman.build_secret_hq(targets[0].clone());
            }
        }
    }

    pub fn start_world_domination_stage2<H: Henchman>(&self, henchman: H) {
        henchman.fight_enemies();
        henchman.do_hard_things();
    }

    pub fn tell_plans<C: Cipher>(&self, secret: &str, cipher: &C) {
        if let Some(ref sidekick) = self.sidekick {
            let ciphered_message = cipher.transform(secret, &self.shared_key);
            sidekick.tell(&ciphered_message);
        }
    }

    pub fn are_there_vulnerable_locations(&self) -> Option<bool> {
        let mut listing = String::new();

        let Ok(mut file_listing) = File::open(LISTING_PATH) else {
            return None;
        };

        let Ok(_) = file_listing.read_to_string(&mut listing) else {
            return None;
        };

        for line in listing.lines() {
            if line.ends_with("weak") {
                return Some(true);
            }
        }

        Some(false)
    }
}

impl TryFrom<&str> for SuperVillain<'_> {
    type Error = EvilError;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let components = name.split_whitespace().collect::<Vec<_>>();
        if components.len() < 2 {
            Err(EvilError::ParseError {
                purpose: "full_name".into(),
                reason: "Too few arguments".into(),
            })
        } else {
            Ok(Self {
                first_name: components[0].into(),
                last_name: components[1].into(),
                ..Default::default()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cipher::MockCipher;
    use crate::gadget::MockGadget;
    use crate::henchman::MockHenchman;
    use crate::test_common;
    use assertables::{assert_matches, assert_some};
    use assertables::{assert_none, assert_some_eq_x};
    use mockall::Sequence;
    use mockall::predicate::eq;
    use std::cell::RefCell;
    use std::panic;
    use test_context::AsyncTestContext;
    use test_context::test_context;

    #[test_context(Context)]
    #[test]
    fn full_name_returns_first_name_space_last_name(context: &mut Context) {
        let full_name = context.supervillain.full_name();
        assert_eq!(
            full_name,
            test_common::PRIMARY_FULL_NAME,
            "Unexpected full name"
        );
    }

    #[test_context(Context)]
    #[test]
    fn set_full_name_sets_first_and_last_names(context: &mut Context) {
        context
            .supervillain
            .set_full_name(test_common::SECONDARY_FULL_NAME);
        assert2::check!(context.supervillain.first_name == test_common::SECONDARY_FIRST_NAME);
        assert2::assert!(context.supervillain.last_name == test_common::SECONDARY_LAST_NAME);
    }

    #[test_context(Context)]
    #[test]
    #[should_panic(expected = "Name must have first and last name, separated by a space")]
    fn set_full_name_panics_with_empty_name(context: &mut Context) {
        context.supervillain.set_full_name("");
    }

    #[test]
    fn try_from_str_slice_produces_supervillain_full_with_first_and_last_name()
    -> Result<(), EvilError> {
        let supervillain = SuperVillain::try_from(test_common::SECONDARY_FULL_NAME)?;
        assert_eq!(supervillain.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(supervillain.last_name, test_common::SECONDARY_LAST_NAME);
        Ok(())
    }

    #[test]
    fn try_from_str_slice_produces_error_with_less_than_two_substrings() {
        let result = SuperVillain::try_from("");
        let Err(error) = result else {
            panic!("Unexpected value returned by try_from");
        };
        assert_matches!(error, EvilError::ParseError { purpose, reason } if purpose == "full_name" && reason == "Too few arguments");
    }

    #[test_context(Context)]
    #[test]
    fn non_intense_attack_shoots_weapon_once(context: &mut Context) {
        let mut weapon = MockMegaWeapon::new();
        weapon.expect_shoot().once().return_const(());
        context.supervillain.attack(&weapon, false);
    }

    #[test_context(Context)]
    #[test]
    fn intensive_attack_shoots_weapon_twice_or_more(context: &mut Context) {
        let mut weapon = MockMegaWeapon::new();
        weapon.expect_shoot().times(2..=3).return_const(());
        context.supervillain.attack(&weapon, true);
    }

    #[test_context(Context)]
    #[tokio::test]
    async fn plan_is_sadly_expected(context: &mut Context<'_>) {
        assert_eq!(
            context.supervillain.come_up_with_plan().await,
            "Take over the world!"
        );
    }

    #[test_context(Context)]
    #[test]
    fn keep_sidekick_if_agrees_with_conspiracy(context: &mut Context<'_>) {
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick.expect_agree().once().return_const(true);
        context.supervillain.sidekick = Some(mock_sidekick);
        context.supervillain.conspire();
        assert_some!(&context.supervillain.sidekick, "Unexpected: Sidekick fired");
    }

    #[test_context(Context)]
    #[test]
    fn fire_sidekick_if_doesnt_agree_with_conspiracy(context: &mut Context<'_>) {
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick.expect_agree().once().return_const(false);
        context.supervillain.sidekick = Some(mock_sidekick);
        context.supervillain.conspire();
        assert_none!(
            &context.supervillain.sidekick,
            "Unexpected: Sidekick didn't fire"
        );
    }

    #[test_context(Context)]
    #[test]
    fn conspiracy_without_sidekick_doesnt_fail(context: &mut Context<'_>) {
        context.supervillain.conspire();
        assert_none!(&context.supervillain.sidekick, "Unexpected: no sidekick");
    }

    #[test_context(Context)]
    #[test]
    fn world_domination_stage1_builds_hq_in_first_weak_target(context: &mut Context) {
        let gadget_dummy = MockGadget::new();
        let mut mock_henchman = MockHenchman::new();
        mock_henchman
            .expect_build_secret_hq()
            .with(eq(String::from(test_common::FIRST_TARGET)))
            .return_const(());
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick
            .expect_get_weak_targets()
            .once()
            .returning(|_| test_common::TARGETS.map(String::from).to_vec());
        context.supervillain.sidekick = Some(mock_sidekick);
        context
            .supervillain
            .start_world_domination_stage1(&mut mock_henchman, &gadget_dummy);
    }

    #[test_context(Context)]
    #[test]
    fn world_domination_stage2_tells_henchman_to_do_hard_things_and_fight_with_enemies(
        context: &mut Context,
    ) {
        let mut mock_henchman = MockHenchman::new();
        let mut sequence = Sequence::new();

        mock_henchman
            .expect_fight_enemies()
            .once()
            .in_sequence(&mut sequence)
            .return_const(());

        mock_henchman
            .expect_do_hard_things()
            .once()
            .in_sequence(&mut sequence)
            .return_const(());

        context
            .supervillain
            .start_world_domination_stage2(mock_henchman);
    }

    #[test_context(Context)]
    #[test]
    fn tell_plans_sends_ciphered_message(context: &mut Context) {
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick
            .expect_tell()
            .with(eq(String::from(test_common::MAIN_CIPHERED_MESSAGE)))
            .once()
            .return_const(());
        context.supervillain.sidekick = Some(mock_sidekick);

        let mut mock_cipher = MockCipher::new();
        mock_cipher
            .expect_transform()
            .returning(|secret, _| String::from("+") + secret + "+");

        context
            .supervillain
            .tell_plans(test_common::MAIN_SECRET_MESSAGE, &mock_cipher);
    }

    #[test_context(Context)]
    #[test]
    fn vulnerable_locations_with_no_file_returns_none(context: &mut Context) {
        FILE_OPEN_OK.replace(None);
        assert_none!(context.supervillain.are_there_vulnerable_locations());
    }

    #[test_context(Context)]
    #[test]
    fn vulnerable_locations_with_file_reading_error_returns_none(context: &mut Context) {
        FILE_OPEN_OK.replace(Some(doubles::File::new(None)));
        assert_none!(context.supervillain.are_there_vulnerable_locations());
    }

    #[test_context(Context)]
    #[test]
    fn vulnerable_locations_with_weak_returns_true(context: &mut Context) {
        FILE_OPEN_OK.replace(Some(doubles::File::new(Some(String::from(
            r#"Madrid,strong
             Las Vegas,weak
             New York,strong"#,
        )))));
        assert_some_eq_x!(context.supervillain.are_there_vulnerable_locations(), true);
    }

    #[test_context(Context)]
    #[test]
    fn vulnerable_locations_without_weak_returns_false(context: &mut Context) {
        FILE_OPEN_OK.replace(Some(doubles::File::new(Some(String::from(
            r#"Madrid,strong
             Oregon,strong
             New York,strong"#,
        )))));
        assert_some_eq_x!(context.supervillain.are_there_vulnerable_locations(), false);
    }

    thread_local! {
        static FILE_OPEN_OK: RefCell<Option<doubles::File>> = const { RefCell::new(None) };
    }

    struct Context<'a> {
        supervillain: SuperVillain<'a>,
    }

    impl<'a> AsyncTestContext for Context<'a> {
        async fn setup() -> Context<'a> {
            Self {
                supervillain: SuperVillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.into(),
                    last_name: test_common::PRIMARY_LAST_NAME.into(),
                    ..Default::default()
                },
            }
        }

        async fn teardown(self) {}
    }

    pub(crate) mod doubles {
        use crate::supervillain::tests::FILE_OPEN_OK;
        use std::io;
        use std::io::Error;
        use std::io::ErrorKind;
        use std::path::Path;

        pub(crate) struct File {
            read_result: Option<String>,
        }

        impl File {
            pub(crate) fn new(read_result: Option<String>) -> File {
                File { read_result }
            }

            pub(crate) fn open<P: AsRef<Path>>(_path: P) -> io::Result<File> {
                if let Some(file) = FILE_OPEN_OK.take() {
                    Ok(file)
                } else {
                    Err(Error::from(ErrorKind::NotFound))
                }
            }

            pub(crate) fn read_to_string(&mut self, buffer: &mut String) -> io::Result<usize> {
                if let Some(ref content) = self.read_result {
                    *buffer = content.to_owned();
                    Ok(buffer.len())
                } else {
                    Err(Error::from(ErrorKind::Other))
                }
            }
        }
    }
}
