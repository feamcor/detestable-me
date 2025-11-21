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
    use super::*;

    #[test]
    fn full_name_is_first_name_space_last_name() {
        // Arrange
        let supervillain = Supervillain {
            first_name: "Lex".into(),
            last_name: "Luthor".into(),
        };
        // Act
        let full_name = supervillain.full_name();
        // Assert
        assert_eq!(full_name, "Lex Luthor", "Unexpected full name");
    }

    #[test]
    fn set_full_name_sets_first_and_last_names() {
        // Arrange
        let mut supervillain = Supervillain {
            first_name: "Lex".into(),
            last_name: "Luthor".into(),
        };
        // Act
        supervillain.set_full_name("Darth Vader");
        // Assert
        assert_eq!(supervillain.first_name, "Darth");
        assert_eq!(supervillain.last_name, "Vader");
    }

    #[test]
    fn from_str_slice_produces_supervillain_full_with_first_and_last_name() {
        // Arrange
        // Act
        let supervillain = Supervillain::from("Darth Vader");
        // Assert
        assert_eq!(supervillain.first_name, "Darth");
        assert_eq!(supervillain.last_name, "Vader");
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

    #[test]
    fn attack_shoots_weapon() {
        // Arrange
        let supervillain = Supervillain {
            first_name: "Lex".into(),
            last_name: "Luthor".into(),
        };
        let weapon = WeaponDouble::new();
        // Act
        supervillain.attack(&weapon);
        // Assert
        assert!(*weapon.is_shot.borrow());
    }
}
