// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::*;
use crate::util::is_default;

/// Represents a bid by a user as part of an auction.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Bid {
    /// Identifies the bidder placing this bid.
    #[serde(rename = "bidder", skip_serializing_if = "is_default")]
    pub bidder_key: Address,

    /// Specifies how much external currency the bidder is putting in with this bid.
    #[serde(rename = "cur", skip_serializing_if = "is_default")]
    pub bid_currency: u64,

    /// Specifies the maximum price, in units of external currency per Algo, that the bidder is willing to pay.
    /// This must be at least as high as the current price of the auction in the block in which this bid appears.
    #[serde(rename = "price", skip_serializing_if = "is_default")]
    pub max_price: u64,

    /// Identifies this bid.
    /// The first bid by a bidder (identified by `bidder_key`)
    /// with a particular `bid_id` on the blockchain will be considered, preventing replay of bids.
    /// Specifying a different `bid_id` allows the bidder to place multiple bids in an auction.
    #[serde(rename = "id", skip_serializing_if = "is_default")]
    pub bid_id: u64,

    /// Specifies the auction for this bid.
    #[serde(rename = "auc", skip_serializing_if = "is_default")]
    pub auction_key: Address,

    /// Identifies the auction for which this bid is intended.
    #[serde(rename = "aid", skip_serializing_if = "is_default")]
    pub auction_id: u64,
}

/// Represents a signed bid by a bidder.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct SignedBid {
    /// Contains information about the bid.
    #[serde(rename = "bid", skip_serializing_if = "is_default")]
    pub bid: Bid,

    /// Signature by the bidder, as identified in the bid (Bid.BidderKey) over the hash of the Bid.
    #[serde(rename = "sig", skip_serializing_if = "is_default")]
    pub sig: Signature,
}

/// Indicates a type of auction messages encoded into a transaction's `note` field.
type NoteFieldType = String;

lazy_static! {
    pub static ref NOTE_DEPOSIT: NoteFieldType = "d".to_owned();
    pub static ref NOTE_BID: NoteFieldType = "b".to_owned();
    pub static ref NOTE_SETTLEMENT: NoteFieldType = "s".to_owned();
    pub static ref NOTE_PARAMS: NoteFieldType = "p".to_owned();
}

/// The struct that represents an auction message.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoteField {
    /// Indicates which type of a message this is
    #[serde(rename = "t", default, skip_serializing_if = "is_default")]
    pub note_type: NoteFieldType,

    /// SignedBid, for NoteBid type
    #[serde(rename = "b", default, skip_serializing_if = "is_default")]
    pub signed_bid: SignedBid,
}
