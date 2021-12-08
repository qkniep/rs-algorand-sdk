// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};

const MASTER_DERIVATION_KEY_LEN_BYTES: usize = 32;

/// Maximum number of transactions in a single group.
const MAX_TX_GROUP_SIZE: usize = 16;

/// Maximum TEAL program size (with args).
const LOGIC_SIG_MAX_SIZE: usize = 1000;

/// Maximum execution cost of a TEAL program.
const LOGIC_SIG_MAX_COST: usize = 20_000;

/// Base unit of currency in Algorand, which is 1e-6 Algos.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MicroAlgos(pub u64);

/// Represents a round of the Algorand consensus protocol.
pub type Round = u64;

/// Participation public key used in key registration transactions.
pub type VotePK = PublicKey;

/// VRF public key used in key registration transactions.
pub type VrfPK = PublicKey;

/// Secret key used to derive keys in wallets.
type MasterDerivationKey = [u8; MASTER_DERIVATION_KEY_LEN_BYTES];

/// A SHA512_256 hash value.
pub type Digest = [u8; 32];

const MICROALGO_CONVERSION_FACTOR: f64 = 1e6;

impl MicroAlgos {
    /// Converts currency amount in `MicroAlgos` to Algos.
    pub fn to_algos(&self) -> f64 {
        self.0 as f64 / MICROALGO_CONVERSION_FACTOR
    }

    /// Converts currency amount in Algos to `MicroAlgos`.
    pub fn from_algos(algos: f64) -> MicroAlgos {
        MicroAlgos((algos * MICROALGO_CONVERSION_FACTOR) as u64)
    }
}

/*func (signedTxn *SignedTxn) FromBase64String(b64string string) error {
    txnBytes, err := base64.StdEncoding.DecodeString(b64string)
    if err != nil {
        return err
    }
    err = msgpack.Decode(txnBytes, &signedTxn)
    if err != nil {
        return err
    }
    return nil
}

func (block *Block) FromBase64String(b64string string) error {
    txnBytes, err := base64.StdEncoding.DecodeString(b64string)
    if err != nil {
        return err
    }
    err = msgpack.Decode(txnBytes, &block)
    if err != nil {
        return err
    }
    return nil
}*/
