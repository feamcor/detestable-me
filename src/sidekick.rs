//! Module for Sidekicks and all the related functionality
#![allow(dead_code)]
use crate::gadget::Gadget;

/// Type that represents a sidekick.
pub struct Sidekick<'a> {
    gadget: Box<dyn Gadget + 'a>,
}

impl<'a> Sidekick<'a> {
    pub fn new<G: Gadget + 'a>(gadget: G) -> Sidekick<'a> {
        Self {
            gadget: Box::new(gadget),
        }
    }

    pub fn agree(&self) -> bool {
        true
    }

    pub fn get_weak_targets<G: Gadget>(&self, _gadget: &G) -> Vec<String> {
        vec![]
    }
}
