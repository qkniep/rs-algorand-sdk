// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use serde::{Deserialize, Serialize};

use super::*;
use crate::util::is_default;

pub type AppIndex = u64;

/// Allocation bound for the maximum number of ApplicationArgs that a transaction decoded off of the wire can contain.
/// Its value is verified against consensus parameters in TestEncodedAppTxnAllocationBounds.
const ENCODED_MAX_APPLICATION_ARGS: u32 = 32;

/// Allocation bound for the maximum number of Accounts that a transaction decoded off of the wire can contain.
/// Its value is verified against consensus parameters in TestEncodedAppTxnAllocationBounds
const ENCODED_MAX_ACCOUNTS: u32 = 32;

/// Allocation bound for the maximum number of ForeignApps that a transaction decoded off of the wire can contain.
/// Its value is verified against consensus parameters in TestEncodedAppTxnAllocationBounds
const ENCODED_MAX_FOREIGN_APPS: u32 = 32;

/// Allocation bound for the maximum number of ForeignAssets that a transaction decoded off of the wire can contain.
/// Its value is verified against consensus parameters in TestEncodedAppTxnAllocationBounds
const ENCODED_MAX_FOREIGN_ASSETS: u32 = 32;

/// Captures the transaction fields used for all interactions with applications.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppCallFields {
    #[serde(rename = "apid", default, skip_serializing_if = "is_default")]
    pub application_id: AppIndex,
    #[serde(rename = "apan", default, skip_serializing_if = "is_default")]
    pub on_completion: OnCompletion,
    #[serde(rename = "apaa", default, skip_serializing_if = "is_default")]
    pub application_args: Vec<Vec<u8>>,
    #[serde(rename = "apat", default, skip_serializing_if = "is_default")]
    pub accounts: Vec<Address>,
    #[serde(rename = "apfa", default, skip_serializing_if = "is_default")]
    pub foreign_apps: Vec<AppIndex>,
    #[serde(rename = "apas", default, skip_serializing_if = "is_default")]
    pub foreign_assets: Vec<AssetIndex>,

    #[serde(rename = "apls", default, skip_serializing_if = "is_default")]
    pub local_state_schema: StateSchema,
    #[serde(rename = "apgs", default, skip_serializing_if = "is_default")]
    pub global_state_schema: StateSchema,
    #[serde(rename = "apap", default, skip_serializing_if = "is_default")]
    pub approval_program: Vec<u8>,
    #[serde(rename = "apsu", default, skip_serializing_if = "is_default")]
    pub clear_state_program: Vec<u8>,
    #[serde(rename = "apep", default, skip_serializing_if = "is_default")]
    pub extra_program_pages: u32,
}

/// Represents some layer 1 side effect that an `ApplicationCall` transaction will have if it is included in a block.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OnCompletion {
    /// NoOpOC indicates that an application transaction will simply call its ApprovalProgram.
    NoOpOC,

    /// OptInOC indicates that an application transaction will allocate some
    /// LocalState for the application in the sender's account
    OptInOC,

    /// CloseOutOC indicates that an application transaction will deallocate
    /// some LocalState for the application from the user's account
    CloseOutOC,

    /// ClearStateOC is similar to CloseOutOC, but may never fail.
    /// This allows users to reclaim their minimum balance from an application they no longer wish to opt in to.
    /// When an ApplicationCall transaction's OnCompletion is ClearStateOC, the ClearStateProgram
    /// executes instead of the ApprovalProgram.
    ClearStateOC,

    /// Indicates that an application transaction will
    /// update the ApprovalProgram and ClearStateProgram for the application.
    UpdateApplicationOC,

    /// DeleteApplicationOC indicates that an application transaction will
    /// delete the AppParams for the application from the creator's balance record
    DeleteApplicationOC,
}

/// Sets maximums on the number of each type that may be stored.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSchema {
    #[serde(rename = "nui", default, skip_serializing_if = "is_default")]
    pub num_uint: u64,
    #[serde(rename = "nbs", default, skip_serializing_if = "is_default")]
    pub num_byte_slice: u64,
}

impl Default for OnCompletion {
    fn default() -> Self {
        Self::NoOpOC
    }
}
