#![allow(dead_code)]

pub trait Cipher {
    fn transform(&self, secret: &str, key: &str) -> String;
}
