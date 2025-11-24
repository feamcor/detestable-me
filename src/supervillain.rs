use std::time::Duration;
use thiserror::Error;

pub struct Supervillain {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Error, Debug)]
pub enum EvilError {
    #[error("Parse error: purpose='{}', reason='{}'", .purpose, .reason)]
    ParseError { purpose: String, reason: String },
}

pub trait MegaWeapon {
    fn shoot(&self);
}

impl Supervillain {
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

    pub fn attack(&self, weapon: &impl MegaWeapon) {
        weapon.shoot();
    }

    pub async fn come_up_with_plan(&self) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        String::from("Take over the world!")
    }
}

impl TryFrom<&str> for Supervillain {
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
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_common;
    use std::cell::RefCell;
    use std::panic;
    use test_context::{AsyncTestContext, test_context};

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
        assert_eq!(
            context.supervillain.first_name,
            test_common::SECONDARY_FIRST_NAME
        );
        assert_eq!(
            context.supervillain.last_name,
            test_common::SECONDARY_LAST_NAME
        );
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
        let supervillain = Supervillain::try_from(test_common::SECONDARY_FULL_NAME)?;
        // Assert
        assert_eq!(supervillain.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(supervillain.last_name, test_common::SECONDARY_LAST_NAME);
        Ok(())
    }

    #[test]
    fn try_from_str_slice_produces_error_with_less_than_two_substrings() {
        let result = Supervillain::try_from("");
        let Err(error) = result else {
            panic!("Unexpected value returned by try_from");
        };
        assert!(matches!(error,
            EvilError::ParseError { purpose, reason }
            if purpose == "full_name" && reason == "Too few arguments"));
    }

    struct WeaponDouble {
        pub is_shot: RefCell<bool>,
    }

    impl WeaponDouble {
        fn new() -> Self {
            Self {
                is_shot: RefCell::new(false),
            }
        }
    }

    impl MegaWeapon for WeaponDouble {
        fn shoot(&self) {
            *self.is_shot.borrow_mut() = true;
        }
    }

    impl Drop for WeaponDouble {
        fn drop(&mut self) {
            if *self.is_shot.borrow() != true {
                panic!("Failed to call shoot()");
            }
        }
    }

    #[test_context(Context)]
    #[test]
    fn attack_shoots_weapon(context: &mut Context) {
        // Arrange
        let weapon = WeaponDouble::new();
        // Act
        context.supervillain.attack(&weapon);
        // Assert
        assert!(*weapon.is_shot.borrow());
    }

    struct Context {
        supervillain: Supervillain,
    }

    impl AsyncTestContext for Context {
        async fn setup() -> Self {
            Self {
                supervillain: Supervillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.into(),
                    last_name: test_common::PRIMARY_LAST_NAME.into(),
                },
            }
        }

        async fn teardown(self) {}
    }

    #[tokio::test]
    #[test_context(Context)]
    async fn plan_is_sadly_expected(context: &mut Context) {
        assert_eq!(
            context.supervillain.come_up_with_plan().await,
            "Take over the world!"
        );
    }
}
