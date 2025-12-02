#![allow(dead_code)]

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Cipher {
    fn transform(&self, secret: &str, key: &str) -> String;
}
