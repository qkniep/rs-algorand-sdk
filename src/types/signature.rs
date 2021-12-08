// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};

use crate::util::is_default;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Signature(ed25519::Signature);

/// Contains a single public key and, optionally, a signature.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigSubsig {
    #[serde(rename = "pk", default, skip_serializing_if = "is_default")]
    pub key: PublicKey,
    #[serde(rename = "s", default, skip_serializing_if = "is_default")]
    pub sig: Option<Signature>,
}

/// Holds multiple Subsigs, as well as threshold and version info.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultisigSignature {
    #[serde(rename = "v", default, skip_serializing_if = "is_default")]
    pub version: u8,
    #[serde(rename = "thr", default, skip_serializing_if = "is_default")]
    pub threshold: u8,
    #[serde(rename = "subsig", default, skip_serializing_if = "is_default")]
    pub subsigs: Vec<MultisigSubsig>,
}

/// LogicSig contains logic for validating a transaction.
/// LogicSig is signed by an account, allowing delegation of operations.
/// OR
/// LogicSig defines a contract account.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogicSig {
    /// Logic signed by Sig or Msig
    /// OR hashed to be the Address of an account.
    #[serde(rename = "l", default, skip_serializing_if = "is_default")]
    pub logic: Vec<u8>,

    /// The signature of the account that has delegated to this LogicSig, if any
    #[serde(rename = "sig", default, skip_serializing_if = "is_default")]
    pub sig: Signature,

    /// The signature of the multisig account that has delegated to this LogicSig, if any
    #[serde(rename = "sig", default, skip_serializing_if = "is_default")]
    pub msig: MultisigSignature,

    /// Args are not signed, but checked by Logic
    #[serde(rename = "arg", default, skip_serializing_if = "is_default")]
    pub args: Vec<Vec<u8>>,
}

impl Default for Signature {
    fn default() -> Self {
        Self(ed25519::Signature::from_bytes(&[0; ed25519::Signature::BYTE_SIZE]).unwrap())
    }
}
