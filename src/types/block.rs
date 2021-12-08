// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::*;
use crate::util::is_default;

// TODO ConsensusVersion and String...
// TODO impl Borrow<Header> for Block?

/// A Block contains the Payset and metadata corresponding to a given Round.
#[derive(Clone, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub payset: Payset,
}

/// Represents the metadata and commitments to the state of a Block.
/// The Algorand Ledger may be defined minimally as a cryptographically authenticated series of `BlockHeader` objects.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct BlockHeader {
    pub round: basics::Round,

    /// The hash of the previous block
    pub branch: Digest,

    /// Sortition seed
    pub seed: [u8; 32],

    /// Root hash that authenticates the set of transactions appearing in the block.
    /// Computed based on the `PaysetCommitType` specified in the block's consensus protocol.
    pub tx_root: Digest,

    /// TimeStamp in seconds since epoch
    pub timestamp: u64,

    /// Genesis ID to which this block belongs.
    pub genesis_id: String,

    /// Genesis hash to which this block belongs.
    pub genesis_hash: Digest,

    /// Rewards.
    ///
    /// When a block is applied, some amount of rewards are accrued to
    /// every account with AccountData.Status=/=NotParticipating.  The
    /// amount is (thisBlock.RewardsLevel-prevBlock.RewardsLevel) of
    /// MicroAlgos for every whole config.Protocol.RewardUnit of MicroAlgos in
    /// that account's AccountData.MicroAlgos.
    ///
    /// Rewards are not compounded (i.e., not added to AccountData.MicroAlgos)
    /// until some other transaction is executed on that account.
    ///
    /// Not compounding rewards allows us to precisely know how many algos
    /// of rewards will be distributed without having to examine every
    /// account to determine if it should get one more algo of rewards
    /// because compounding formed another whole config.Protocol.RewardUnit
    /// of algos.
    pub rewards_state: RewardsState,

    /// Consensus protocol versioning.
    ///
    /// Each block is associated with a version of the consensus protocol,
    /// stored under UpgradeState.current_protocol.  The protocol version
    /// for a block can be determined without having to first decode the
    /// block and its CurrentProtocol field, and this field is present for
    /// convenience and explicitness.  Block.Valid() checks that this field
    /// correctly matches the expected protocol version.
    ///
    /// Each block is associated with at most one active upgrade proposal
    /// (a new version of the protocol).  An upgrade proposal can be made
    /// by a block proposer, as long as no other upgrade proposal is active.
    /// The upgrade proposal lasts for many rounds (UpgradeVoteRounds), and
    /// in each round, that round's block proposer votes to support (or not)
    /// the proposed upgrade.
    ///
    /// If enough votes are collected, the proposal is approved, and will
    /// definitely take effect.  The proposal lingers for some number of
    /// rounds to give clients a chance to notify users about an approved
    /// upgrade, if the client doesn't support it, so the user has a chance
    /// to download updated client software.
    ///
    /// Block proposers influence this upgrade machinery through two fields
    /// in UpgradeVote: UpgradePropose, which proposes an upgrade to a new
    /// protocol, and UpgradeApprove, which signals approval of the current
    /// proposal.
    ///
    /// Once a block proposer determines its UpgradeVote, then UpdateState
    /// is updated deterministically based on the previous UpdateState and
    /// the new block's UpgradeVote.
    #[serde(skip)]
    pub upgrade_state: UpgradeState,
    pub upgrade_vote: UpgradeVote,

    /// Counts the number of transactions committed in the ledger,
    /// from the time at which support for this feature was introduced.
    ///
    /// Specifically, `tx_counter` is the number of the next transaction that will be committed after this block.
    /// It is 0 when no transactions have ever been committed (since `tx_counter` started being supported).
    pub tx_counter: u64,
}

/// Represents a common, unforgeable, consistent, ordered set of `SignedTxInBlock` objects.
//msgp:allocbound Payset 100000
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Payset(pub Vec<SignedTxInBlock>);

/// RewardsState represents the global parameters controlling the rate at which accounts accrue rewards.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RewardsState {
    /// The fee sink accepts transaction fees.
    /// It can only spend to the incentive pool.
    pub fee_sink: Address,

    /// The rewards pool accepts periodic injections from the fee sink
    /// and continually redistributes them to adresses as rewards.
    pub rewards_pool: Address,

    /// Specifies how many rewards, in MicroAlgos, have been distributed to each
    /// config.protocol.reward_unit of MicroAlgos since genesis.
    pub rewards_level: u64,

    /// Number of new MicroAlgos added to the participation stake from rewards at the next round.
    pub rewards_rate: u64,

    /// Leftover MicroAlgos after the distribution of rewards_rate/reward_units
    /// MicroAlgos for every reward unit in the next round.
    pub rewards_residue: u64,

    /// The round at which the RewardsRate will be recalculated.
    pub rewards_recalculation_round: basics::Round,
}

/// Represents the vote of the block proposer with respect to protocol upgrades.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct UpgradeVote {
    /// UpgradePropose indicates a proposed upgrade
    pub upgrade_propose: String,

    /// UpgradeDelay indicates the time between acceptance and execution
    pub upgrade_delay: basics::Round,

    /// UpgradeApprove indicates a yes vote for the current proposal
    pub upgrade_approve: bool,
}

/// UpgradeState tracks the protocol upgrade state machine.  It is,
/// strictly speaking, computable from the history of all UpgradeVotes
/// but we keep it in the block for explicitness and convenience
/// (instead of materializing it separately, like balances).
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpgradeState {
    #[serde(rename = "proto")]
    pub current_protocol: String,
    #[serde(rename = "nextproto")]
    pub next_protocol: Option<String>,
    #[serde(rename = "nextyes")]
    pub next_protocol_approvals: u64,
    #[serde(rename = "nextbefore")]
    pub next_protocol_vote_before: basics::Round,
    #[serde(rename = "nextswitch")]
    pub next_protocol_switch_on: basics::Round,
}

/// How a signed transaction is encoded in a block.
#[derive(Clone, Serialize, Deserialize)]
pub struct SignedTxInBlock {
    #[serde(flatten)]
    pub sig_txad: SignedTxWithAD,

    #[serde(rename = "hgi", default, skip_serializing_if = "is_default")]
    pub has_genesis_id: bool,
    #[serde(rename = "hgh", default, skip_serializing_if = "is_default")]
    pub has_genesis_hash: bool,
}

/// A (decoded) SignedTx with associated ApplyData.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedTxWithAD {
    #[serde(flatten)]
    pub tx: SignedTx,
    #[serde(flatten)]
    pub ad: ApplyData,
}

/// Contains information about the transaction's execution.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyData {
    /// Closing amount for transaction.
    #[serde(rename = "ca", default, skip_serializing_if = "is_default")]
    pub closing_amount: MicroAlgos,

    /// Closing amount for asset transaction.
    #[serde(rename = "ca", default, skip_serializing_if = "is_default")]
    pub asset_closing_amount: u64,

    // Rewards applied to the Sender, Receiver, and CloseRemainderTo accounts.
    #[serde(rename = "rs", default, skip_serializing_if = "is_default")]
    pub sender_rewards: MicroAlgos,
    #[serde(rename = "rr", default, skip_serializing_if = "is_default")]
    pub receiver_rewards: MicroAlgos,
    #[serde(rename = "rc", default, skip_serializing_if = "is_default")]
    pub close_rewards: MicroAlgos,
    #[serde(rename = "dt", default, skip_serializing_if = "is_default")]
    pub eval_delta: EvalDelta,

    #[serde(rename = "caid", default, skip_serializing_if = "is_default")]
    pub config_asset: u64,
    #[serde(rename = "apid", default, skip_serializing_if = "is_default")]
    pub application_id: u64,
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalDelta {
    #[serde(rename = "gd", default, skip_serializing_if = "is_default")]
    pub global_delta: StateDelta,

    /// When decoding EvalDeltas, the integer key represents an offset into
    /// `[tx.Sender, tx.Accounts[0], tx.Accounts[1], ...]`.
    #[serde(rename = "ld", default, skip_serializing_if = "is_default")]
    pub local_deltas: HashMap<u64, StateDelta>,

    #[serde(rename = "lg", default, skip_serializing_if = "is_default")]
    pub logs: Vec<String>,

    #[serde(rename = "itx", default, skip_serializing_if = "is_default")]
    pub inner_txs: Vec<SignedTxWithAD>,
}

// StateDelta is a map from key/value store keys to ValueDeltas, indicating
// what should happen for that key
//msgp:allocbound StateDelta config.MaxStateDeltaKeys
pub type StateDelta = HashMap<String, ValueDelta>;

/// Links a DeltaAction with a value to be set.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueDelta {
    #[serde(rename = "at", default, skip_serializing_if = "is_default")]
    pub action: DeltaAction,
    #[serde(rename = "bs", default, skip_serializing_if = "is_default")]
    pub bytes: String,
    #[serde(rename = "ui", default, skip_serializing_if = "is_default")]
    pub uint: u64,
}

/// Actions that may be performed when applying a delta to a TEAL key/value store.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeltaAction {
    Invalid,
    /// Indicates that a TEAL byte slice should be stored at a key.
    SetBytes,
    /// Indicates that a Uint should be stored at a key.
    SetUint,
    /// Indicates that the value for a particular key should be deleted.
    Delete,
}

impl Default for DeltaAction {
    fn default() -> Self {
        DeltaAction::Invalid
    }
}
