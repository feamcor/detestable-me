//! Module for Super Villains and their related stuff

use rand::Rng;
use std::time::Duration;
use thiserror::Error;

#[cfg(not(test))]
use crate::Sidekick;
#[cfg(test)]
use tests::doubles::Sidekick;

use crate::Henchman;
use crate::{Cipher, Gadget};

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
    use crate::Henchman;
    use crate::test_common;
    use assertables::{assert_matches, assert_none, assert_some, assert_some_eq_x};
    use std::cell::Cell;
    use std::panic;
    use test_context::AsyncTestContext;
    use test_context::test_context;

    fn at_least(minimum: u32) -> impl Fn(u32) -> bool {
        move |times: u32| times >= minimum
    }

    fn once() -> impl Fn(u32) -> bool {
        move |times: u32| times == 1
    }

    #[test_context(Context)]
    #[test]
    fn full_name_returns_first_name_space_last_name(context: &mut Context) {
        // Arrange
        // Act
        let full_name = context.supervillain.full_name();
        // Assert
        assert_eq!(
            full_name,
            test_common::PRIMARY_FULL_NAME,
            "Unexpected full name"
        );
    }

    #[test_context(Context)]
    #[test]
    fn set_full_name_sets_first_and_last_names(context: &mut Context) {
        // Arrange
        // Act
        context
            .supervillain
            .set_full_name(test_common::SECONDARY_FULL_NAME);
        // Assert
        assert2::check!(context.supervillain.first_name == test_common::SECONDARY_FIRST_NAME);
        assert2::assert!(context.supervillain.last_name == test_common::SECONDARY_LAST_NAME);
    }

    #[test_context(Context)]
    #[test]
    #[should_panic(expected = "Name must have first and last name, separated by a space")]
    fn set_full_name_panics_with_empty_name(context: &mut Context) {
        // Arrange
        // Act
        context.supervillain.set_full_name("");
        // Assert
    }

    #[test]
    fn try_from_str_slice_produces_supervillain_full_with_first_and_last_name()
    -> Result<(), EvilError> {
        // Arrange
        // Act
        let supervillain = SuperVillain::try_from(test_common::SECONDARY_FULL_NAME)?;
        // Assert
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

    struct WeaponDouble {
        pub times_shot: Cell<u32>,
    }

    impl WeaponDouble {
        fn new() -> WeaponDouble {
            WeaponDouble {
                times_shot: Cell::default(),
            }
        }

        fn verify<T: Fn(u32) -> bool>(&self, check: T) {
            assert!(check(self.times_shot.get()));
        }
    }

    impl MegaWeapon for WeaponDouble {
        fn shoot(&self) {
            self.times_shot.set(self.times_shot.get() + 1);
        }
    }

    impl Drop for WeaponDouble {
        fn drop(&mut self) {
            if self.times_shot.get() == 0 {
                panic!("Failed to call shoot()");
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn non_intense_attack_shoots_weapon_once(context: &mut Context) {
        // Arrange
        let weapon = WeaponDouble::new();
        // Act
        context.supervillain.attack(&weapon, false);
        // Assert
        weapon.verify(once());
    }

    #[test_context(Context)]
    #[test]
    fn intensive_attack_shoots_weapon_twice_or_more(context: &mut Context) {
        let weapon = WeaponDouble::new();
        context.supervillain.attack(&weapon, true);
        weapon.verify(at_least(2));
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

    #[tokio::test]
    #[test_context(Context)]
    async fn plan_is_sadly_expected(context: &mut Context<'_>) {
        assert_eq!(
            context.supervillain.come_up_with_plan().await,
            "Take over the world!"
        );
    }

    #[test_context(Context)]
    #[test]
    fn keep_sidekick_if_agrees_with_conspiracy(context: &mut Context<'_>) {
        // Arrange
        let mut sidekick_double = doubles::Sidekick::new();
        sidekick_double.agree_answer = true;
        context.supervillain.sidekick = Some(sidekick_double);
        // Act
        context.supervillain.conspire();
        // Assert
        assert_some!(
            &context.supervillain.sidekick,
            "Sidekick fired unexpectedly"
        );
    }

    #[test_context(Context)]
    #[test]
    fn fire_sidekick_if_doesnt_agree_with_conspiracy(context: &mut Context<'_>) {
        // Arrange
        let mut sidekick_double = doubles::Sidekick::new();
        sidekick_double.agree_answer = false;
        context.supervillain.sidekick = Some(sidekick_double);
        // Act
        context.supervillain.conspire();
        // Assert
        assert_none!(
            &context.supervillain.sidekick,
            "Sidekick isn't fired unexpectedly"
        );
    }

    #[test_context(Context)]
    #[test]
    fn conspiracy_without_sidekick_doesnt_fail(context: &mut Context<'_>) {
        // Arrange
        // Act
        context.supervillain.conspire();
        // Assert
        assert_none!(&context.supervillain.sidekick, "Unexpected sidekick");
    }

    pub(crate) mod doubles {
        use crate::Gadget;
        use std::cell::RefCell;
        use std::fmt;
        use std::marker::PhantomData;

        pub struct Sidekick<'a> {
            phantom: PhantomData<&'a ()>,
            pub agree_answer: bool,
            pub targets: Vec<String>,
            pub received_message: RefCell<String>,
            pub assertions: Vec<Box<dyn Fn(&Sidekick) + Send>>,
        }

        impl<'a> Default for Sidekick<'a> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'a> Sidekick<'a> {
            pub fn new() -> Sidekick<'a> {
                Sidekick {
                    phantom: PhantomData,
                    agree_answer: false,
                    targets: vec![],
                    received_message: RefCell::new(String::new()),
                    assertions: vec![],
                }
            }

            pub fn agree(&self) -> bool {
                self.agree_answer
            }

            pub fn get_weak_targets<G: Gadget>(&self, _gadget: &G) -> Vec<String> {
                self.targets.clone()
            }

            pub fn tell(&self, ciphered_msg: &str) {
                *self.received_message.borrow_mut() = ciphered_msg.into();
            }

            pub fn verify_received_message(&self, expected_message: &str) {
                assert_eq!(*self.received_message.borrow(), expected_message);
            }
        }

        impl<'a> Drop for Sidekick<'a> {
            fn drop(&mut self) {
                for assertion in &self.assertions {
                    assertion(self);
                }
            }
        }

        impl fmt::Debug for Sidekick<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Sidekick")
                    .field("agree_answer", &self.agree_answer)
                    .field("targets", &self.targets)
                    .field("received_msg", &self.received_message)
                    .finish()
            }
        }
    }

    struct GadgetDummy;

    impl Gadget for GadgetDummy {
        fn do_stuff(&self) {}
    }

    #[derive(Default)]
    struct HenchmanDouble {
        hq_location: Option<String>,
        current_invocation: Cell<u32>,
        done_hard_things: Cell<u32>,
        fought_enemies: Cell<u32>,
        assertions: Vec<Box<dyn Fn(&HenchmanDouble) + Send>>,
    }

    impl HenchmanDouble {
        fn verify_two_things_done(&self) {
            assert!(self.done_hard_things.get() == 2 && self.fought_enemies.get() == 1);
        }
    }

    impl Henchman for HenchmanDouble {
        fn build_secret_hq(&mut self, location: String) {
            self.hq_location = Some(location);
        }

        fn do_hard_things(&self) {
            self.current_invocation
                .set(self.current_invocation.get() + 1);
            self.done_hard_things.set(self.current_invocation.get());
        }

        fn fight_enemies(&self) {
            self.current_invocation
                .set(self.current_invocation.get() + 1);
            self.fought_enemies.set(self.current_invocation.get());
        }
    }

    impl Drop for HenchmanDouble {
        fn drop(&mut self) {
            for assertion in &self.assertions {
                assertion(self);
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn world_domination_stage1_builds_hq_in_first_weak_target(context: &mut Context) {
        // Arrange
        let gadget_dummy = GadgetDummy {};
        let mut henchman_spy = HenchmanDouble::default();
        let mut sidekick_double = doubles::Sidekick::new();
        sidekick_double.targets = test_common::TARGETS.map(String::from).to_vec();
        context.supervillain.sidekick = Some(sidekick_double);
        // Act
        context
            .supervillain
            .start_world_domination_stage1(&mut henchman_spy, &gadget_dummy);
        // Assert
        assert_some_eq_x!(&henchman_spy.hq_location, test_common::FIRST_TARGET);
    }

    #[test_context(Context)]
    #[test]
    fn world_domination_stage2_tells_henchman_to_do_hard_things_and_fight_with_enemies(
        context: &mut Context,
    ) {
        let mut henchman = HenchmanDouble::default();
        henchman.assertions = vec![Box::new(move |h| h.verify_two_things_done())];
        context.supervillain.start_world_domination_stage2(henchman);
    }

    struct CipherDouble;

    impl Cipher for CipherDouble {
        fn transform(&self, secret: &str, _key: &str) -> String {
            format!("+{secret}+")
        }
    }

    #[test_context(Context)]
    #[test]
    fn tell_plans_sends_ciphered_message(context: &mut Context) {
        // Arrange
        let mut sidekick_double = doubles::Sidekick::new();
        sidekick_double.assertions = vec![Box::new(move |s| {
            s.verify_received_message(test_common::MAIN_CIPHERED_MESSAGE)
        })];
        context.supervillain.sidekick = Some(sidekick_double);
        let fake_cipher = CipherDouble {};
        // Act
        context
            .supervillain
            .tell_plans(test_common::MAIN_SECRET_MESSAGE, &fake_cipher);
        // Assert
    }
}
