// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

pub fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}

fn is_zero(s: &ed25519::Signature) -> bool {
    s.to_bytes() == [0; ed25519_dalek::SIGNATURE_LENGTH]
}
