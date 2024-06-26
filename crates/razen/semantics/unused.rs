use crate::ns::*;

pub struct Unused<'a>(pub &'a SemanticHost);

impl<'a> Unused<'a> {
    pub fn all(&self) -> std::cell::Ref<Vec<Thingy>> {
        self.0.unused_things()
    }

    pub fn is_unused(&self, thingy: &Thingy) -> bool {
        self.0.is_unused(thingy)
    }

    pub(crate) fn add(&self, thing: &Thingy) {
        self.0.add_unused_thing(thing);
    }

    pub(crate) fn add_named_entity(&self, thing: &Thingy) {
        let name = thing.name();
        if name.in_public_or_protected_ns() || name.local_name().starts_with('_') {
            return;
        }
        self.add(thing);
    }

    pub fn mark_used(&self, property: &Thingy) {
        if property.is::<InvalidationThingy>() {
            return;
        }
        let qn = property.name();
        if !qn.in_public_or_protected_ns() {
            if property.is_entity_after_substitution() {
                self.mark_used(&property.origin());
            } else {
                self.0.remove_unused_thing(property);
            }
        }
    }
}