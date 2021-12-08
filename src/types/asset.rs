// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use serde::{Deserialize, Serialize};

use super::*;
use crate::util::is_default;

/// Maximum length (in bytes) for the asset name.
const ASSET_NAME_MAX_LEN: usize = 32;

/// Maximum length (in bytes) for the asset unit name.
const ASSET_UNIT_NAME_MAX_LEN: usize = 8;

/// Maximum length (in bytes) for the asset url
const ASSET_URL_MAX_LEN: usize = 96;

/// Length of the Asset's `metadata_hash` (in bytes).
const ASSET_METADATA_HASH_LEN: usize = 32;

/// Maximum value of the `decimals` field.
const ASSET_MAX_NUMBER_OF_DECIMALS: u32 = 19;

/// Unique integer index of an asset that can be used to look up the creator of the asset,
/// whose balance record contains the `AssetParams`.
pub type AssetIndex = u64;

/// Describes the parameters of an asset.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetParams {
    /// Specifies the total number of units of this asset created.
    #[serde(rename = "t", default, skip_serializing_if = "is_default")]
    pub total: u64,

    /// Number of digits to display after the decimal place when displaying this asset:
    ///   - 0 represents an asset that is not divisible
    ///   - 1 represents an asset divisible into tenths
    ///   ... and so on
    /// This value must be between `0` and `ASSET_MAX_NUMBER_OF_DECIMALS` (inclusive).
    #[serde(rename = "dc", default, skip_serializing_if = "is_default")]
    pub decimals: u32,

    /// Whether slots for this asset in user accounts are frozen by default or not.
    #[serde(rename = "df", default, skip_serializing_if = "is_default")]
    pub default_frozen: bool,

    /// Hint for the name of a unit of this asset.
    #[serde(rename = "un", default, skip_serializing_if = "is_default")]
    pub unit_name: String,

    /// Hint for the name of the asset.
    #[serde(rename = "an", default, skip_serializing_if = "is_default")]
    pub asset_name: String,

    /// URL where more information about the asset can be retrieved.
    #[serde(rename = "au", default, skip_serializing_if = "is_default")]
    pub url: String,

    /// Commitment to some unspecified asset metadata.
    /// The format of this metadata is up to the application.
    #[serde(rename = "am", default, skip_serializing_if = "is_default")]
    pub metadata_hash: [u8; ASSET_METADATA_HASH_LEN],

    /// An account that is allowed to change the non-zero addresses in this `AssetParams`.
    #[serde(rename = "m", default, skip_serializing_if = "is_default")]
    pub manager: Address,

    /// An account whose holdings of this asset should be reported as "not minted".
    #[serde(rename = "r", default, skip_serializing_if = "is_default")]
    pub reserve: Address,

    /// An account that is allowed to change the frozen state of holdings of this asset.
    #[serde(rename = "f", default, skip_serializing_if = "is_default")]
    pub freeze: Address,

    /// An account that is allowed to take units of this asset from any account.
    #[serde(rename = "c", default, skip_serializing_if = "is_default")]
    pub clawback: Address,
}
