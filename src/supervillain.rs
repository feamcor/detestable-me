pub struct Supervillain {
    pub first_name: String,
    pub last_name: String,
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
        self.first_name = components[0].into();
        self.last_name = components[1].into();
    }

    pub fn attack(&self, weapon: &impl MegaWeapon) {
        weapon.shoot();
    }
}

impl From<&str> for Supervillain {
    fn from(name: &str) -> Self {
        let components = name.split_whitespace().collect::<Vec<_>>();
        Self {
            first_name: components[0].into(),
            last_name: components[1].into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::panic;
    use test_context::{test_context, TestContext};
    use super::*;

    const PRIMARY_FIRST_NAME: &str = "Lex";
    const PRIMARY_LAST_NAME: &str = "Luthor";
    const PRIMARY_FULL_NAME: &str = "Lex Luthor";
    const SECONDARY_FIRST_NAME: &str = "Darth";
    const SECONDARY_LAST_NAME: &str = "Vader";
    const SECONDARY_FULL_NAME: &str = "Darth Vader";

    #[test_context(Context)]
    #[test]
    fn full_name_returns_first_name_space_last_name(context: &mut Context) {
        // Arrange
        // Act
        let full_name = context.supervillain.full_name();
        // Assert
        assert_eq!(full_name, PRIMARY_FULL_NAME, "Unexpected full name");
    }

    #[test_context(Context)]
    #[test]
    fn set_full_name_sets_first_and_last_names(context: &mut Context) {
        // Arrange
        // Act
        context.supervillain.set_full_name(SECONDARY_FULL_NAME);
        // Assert
        assert_eq!(context.supervillain.first_name, SECONDARY_FIRST_NAME);
        assert_eq!(context.supervillain.last_name, SECONDARY_LAST_NAME);
    }

    #[test]
    fn from_str_slice_produces_supervillain_full_with_first_and_last_name() {
        // Arrange
        // Act
        let supervillain = Supervillain::from(PRIMARY_FULL_NAME);
        // Assert
        assert_eq!(supervillain.first_name, PRIMARY_FIRST_NAME);
        assert_eq!(supervillain.last_name, PRIMARY_LAST_NAME);
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

    impl TestContext for Context {
        fn setup() -> Self {
            Self {
                supervillain: Supervillain {
                    first_name: PRIMARY_FIRST_NAME.into(),
                    last_name: PRIMARY_LAST_NAME.into(),
                },
            }
        }

        fn teardown(self) {}
    }
}
