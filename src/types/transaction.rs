// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use serde::{Deserialize, Serialize};

use super::*;
use crate::util::is_default;

/// Describes a transaction that can appear in a block.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(flatten)]
    pub header: Header,

    #[serde(flatten)]
    pub fields: TxFields,
}

/// Captures the fields common to every transaction type.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "snd", default, skip_serializing_if = "is_default")]
    pub sender: Address,
    #[serde(rename = "fee", default, skip_serializing_if = "is_default")]
    pub fee: MicroAlgos,
    #[serde(rename = "fv", default, skip_serializing_if = "is_default")]
    pub first_valid: Round,
    #[serde(rename = "lv", default, skip_serializing_if = "is_default")]
    pub last_valid: Round,
    #[serde(default, skip_serializing_if = "is_default")]
    pub note: Vec<u8>,
    #[serde(rename = "gen", default, skip_serializing_if = "is_default")]
    pub genesis_id: String,
    #[serde(rename = "gh", default, skip_serializing_if = "is_default")]
    pub genesis_hash: Digest,

    /// Specifies that this transaction is part of a transaction group
    /// (and, if so, specifies the hash of the transaction group).
    #[serde(rename = "grp", default, skip_serializing_if = "is_default")]
    pub group: Digest,

    /// Enforces mutual exclusion of transactions.
    /// If this field is nonzero, then once the transaction is confirmed, it acquires the
    /// lease identified by the pair (sender, lease) until the last_valid round passes.
    /// While this transaction possesses the lease, no other transaction with this lease can be confirmed.
    #[serde(rename = "lx", default, skip_serializing_if = "is_default")]
    pub lease: [u8; 32],

    /// If nonzero, sets the sender's `auth_addr` to the given address.
    /// If the `rekey_to` address is the sender's actual address, the `auth_addr` is set to zero.
    /// This allows "re-keying" a long-lived account -- rotating the signing key,
    /// changing membership of a multisig account, etc.
    #[serde(rename = "rekey", default, skip_serializing_if = "is_default")]
    pub rekey_to: Address,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TxFields {
    #[serde(rename = "keyreg")]
    Keyreg(KeyregFields),
    #[serde(rename = "pay")]
    Payment(PaymentFields),
    #[serde(rename = "acfg")]
    AssetConfig(AssetConfigFields),
    #[serde(rename = "axfer")]
    AssetTransfer(AssetTransferFields),
    #[serde(rename = "afrz")]
    AssetFreeze(AssetFreezeFields),
    #[serde(rename = "appl")]
    AppCall(AppCallFields),
    //#[serde(rename = "cert")]
    //CompactCert(CompactCertFields),
}

/// Wraps a transaction and a signature.
/// It exposes a `verify()` method that verifies the signature
/// and checks that the underlying transaction is well-formed.
// TODO: update this documentation now that there's multisig
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTx {
    #[serde(rename = "sig", default, skip_serializing_if = "is_default")]
    pub sig: Signature,
    #[serde(rename = "msig", default, skip_serializing_if = "is_default")]
    pub msig: Option<MultisigSignature>,
    #[serde(rename = "lsig", default, skip_serializing_if = "is_default")]
    pub lsig: Option<LogicSig>,
    #[serde(rename = "txn")]
    pub tx: Transaction,
    #[serde(rename = "sgnr", default, skip_serializing_if = "is_default")]
    pub auth_addr: Address,
}

/// Captures the fields used for key registration transactions.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyregFields {
    #[serde(rename = "votekey", default, skip_serializing_if = "is_default")]
    pub vote_pk: VotePK,
    #[serde(rename = "selkey", default, skip_serializing_if = "is_default")]
    pub selection_pk: VrfPK,
    #[serde(rename = "votefst", default, skip_serializing_if = "is_default")]
    pub vote_first: basics::Round,
    #[serde(rename = "votelst", default, skip_serializing_if = "is_default")]
    pub vote_last: basics::Round,
    #[serde(rename = "votekd", default, skip_serializing_if = "is_default")]
    pub vote_key_dilution: u64,
    #[serde(rename = "nonpart", default, skip_serializing_if = "is_default")]
    pub nonparticipation: bool,
}

/// The fields used by payment transactions.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentFields {
    #[serde(rename = "rcv", default, skip_serializing_if = "is_default")]
    pub receiver: Address,
    #[serde(rename = "amt", default, skip_serializing_if = "is_default")]
    pub amount: basics::MicroAlgos,

    /// When `close_remainder_to` is set, the transaction is requesting that the account should be closed,
    /// and all remaining funds be transferred to this address.
    #[serde(rename = "close", default, skip_serializing_if = "is_default")]
    pub close_remainder_to: Option<Address>,
}

/// Fields used for asset allocation, re-configuration, and destruction.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetConfigFields {
    /// ConfigAsset is the asset being configured or destroyed.
    /// A zero value means allocation.
    #[serde(rename = "caid", default, skip_serializing_if = "is_default")]
    pub config_asset: AssetIndex,

    /// Parameters for the asset being created or re-configured.
    /// A zero value means destruction.
    #[serde(rename = "apar", default, skip_serializing_if = "is_default")]
    pub asset_params: AssetParams,
}

/// Fields used for asset transfers.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetTransferFields {
    #[serde(rename = "xaid", default, skip_serializing_if = "is_default")]
    pub transfer_asset: AssetIndex,

    /// The amount of asset to transfer.
    /// A zero amount transferred to self allocates that asset in the account's Assets map.
    #[serde(rename = "aamt", default, skip_serializing_if = "is_default")]
    pub asset_amount: u64,

    /// Sender of the transfer.
    /// If this is not a zero value, the real transaction sender must be the Clawback address from the AssetParams.
    /// If this is the zero value, the asset is sent from the transaction's Sender.
    #[serde(rename = "asnd", default, skip_serializing_if = "is_default")]
    pub asset_sender: Address,

    /// Recipient of the transfer.
    #[serde(rename = "arcv", default, skip_serializing_if = "is_default")]
    pub asset_receiver: Address,

    /// Indicates that the asset should be removed from the account's Assets map,
    /// and specifies where the remaining asset holdings should be transferred.
    /// It's always valid to transfer remaining asset holdings to the creator account.
    #[serde(rename = "aclose", default, skip_serializing_if = "is_default")]
    pub asset_close_to: Address,
}

/// Fields used for freezing asset slots.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetFreezeFields {
    /// Address of the account whose asset slot is being frozen or un-frozen.
    #[serde(rename = "fadd", default, skip_serializing_if = "is_default")]
    pub freeze_account: Address,

    /// Asset ID being frozen or un-frozen.
    #[serde(rename = "faid", default, skip_serializing_if = "is_default")]
    pub freeze_asset: AssetIndex,

    /// The new frozen value.
    #[serde(rename = "afrz", default, skip_serializing_if = "is_default")]
    pub asset_frozen: bool,
}

/// Describes a group of transactions that must appear together in a specific order in a block.
#[derive(Serialize, Deserialize)]
struct TxGroup {
    /// Specifies a list of hashes of transactions that must appear together,
    /// sequentially, in a block in order for the group to be valid.
    /// Each hash in the list is a hash of a transaction with the `group` field omitted.
    #[serde(rename = "txlist", default, skip_serializing_if = "is_default")]
    pub tx_group_hashes: Vec<Digest>,
}
