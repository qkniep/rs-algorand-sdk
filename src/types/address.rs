// Copyright (C) 2021 Quentin M. Kniep <hello@quentinkniep.com>
// Distributed under terms of the MIT license.

use std::fmt;
use std::str::FromStr;

use data_encoding::BASE32_NOPAD;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512_256};
use thiserror::Error;

const CHECKSUM_LEN: usize = 4;
const HASH_LEN: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Error)]
pub enum AddressError {
    #[error("invalid base32 encoding")]
    InvalidBase32,
    #[error("wrong length for address")]
    WrongLength,
    #[error("invalid checksum")]
    InvalidChecksum,
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address(pub [u8; HASH_LEN]);

impl Address {
    /// Returns the checksum as Vec<u8>.
    /// Checksum in Algorand are the last 4 bytes of the shortAddress Hash. H(Address)[28..]
    fn checksum(&self) -> Vec<u8> {
        let short_addr_hash = Sha512_256::digest(&self.0);
        short_addr_hash[short_addr_hash.len() - CHECKSUM_LEN..].to_vec()
    }

    /// Checks if an address is the zero value.
    pub fn is_zero(&self) -> bool {
        *self == Address([0; 32])
    }
}

impl fmt::Display for Address {
    /// Returns a string representation of Address
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut addr_with_checksum = [0_u8; 32 + CHECKSUM_LEN];
        addr_with_checksum[..32].copy_from_slice(&self.0[..]);
        addr_with_checksum[32..].copy_from_slice(&self.checksum());
        f.write_str(&BASE32_NOPAD.encode(&addr_with_checksum))
    }
}

impl FromStr for Address {
    type Err = AddressError;

    /// Tries to unmarshal the checksummed address string.
    /// Algorand address strings (base32 encoded) have a postamble which serves as the checksum of the address.
    /// When converted to an Address object representation, that checksum is dropped (after validation).
    fn from_str(addr: &str) -> Result<Self, AddressError> {
        let decoded = match BASE32_NOPAD.decode(addr.as_bytes()) {
            Ok(d) => d,
            _ => {
                return Err(AddressError::InvalidBase32);
            }
        };

        let mut short = Address([0; 32]);
        if decoded.len() < short.0.len() {
            return Err(AddressError::WrongLength);
        }

        short.0[..].copy_from_slice(&decoded[..32]);
        let incoming_checksum = &decoded[decoded.len() - CHECKSUM_LEN..];
        let calculated_checksum = short.checksum();
        let is_valid = incoming_checksum == calculated_checksum;

        if !is_valid {
            return Err(AddressError::InvalidChecksum);
        }

        // Validate that we had a canonical string representation
        if short.to_string() != addr {
            unreachable!();
        }

        Ok(short)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmarshall_checksum_address() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let result = Address::from_str(&short_addr.to_string());
        assert_eq!(result, Ok(short_addr));
    }

    #[test]
    fn too_short() {
        let addr = "";
        assert_eq!(Address::from_str(addr), Err(AddressError::WrongLength));
    }

    #[test]
    fn wrong_checksum() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let mut short_addr_str = short_addr.to_string();
        short_addr_str.pop();
        short_addr_str.push('1');
        let result = Address::from_str(&short_addr_str);
        assert_eq!(result, Err(AddressError::InvalidBase32));
    }

    #[test]
    fn wrong_checksum_space() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let mut short_addr_str = short_addr.to_string();
        short_addr_str.pop();
        short_addr_str.push(' ');
        let result = Address::from_str(&short_addr_str);
        assert_eq!(result, Err(AddressError::InvalidBase32));
    }

    #[test]
    fn wrong_address_add_char() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let mut s = "4".to_owned();
        s.push_str(&short_addr.to_string());
        let result = Address::from_str(&s);
        assert_eq!(result, Err(AddressError::InvalidBase32));
    }

    #[test]
    fn wrong_address_replace_char() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let mut short_addr_str = short_addr.to_string();
        short_addr_str.remove(0);
        let mut s = "4".to_owned();
        s.push_str(&short_addr_str);
        let result = Address::from_str(&s);
        assert_eq!(result, Err(AddressError::InvalidChecksum));
    }

    #[test]
    fn wrong_address_invalid_char() {
        let addr = Sha512_256::digest(b"randomString");
        let short_addr = Address(addr.into());

        let mut s = " ".to_owned();
        s.push_str(&short_addr.to_string());
        let result = Address::from_str(&s);
        assert_eq!(result, Err(AddressError::InvalidBase32));
    }

    #[test]
    fn human_readable() {
        let s = "J5YDZLPOHWB5O6MVRHNFGY4JXIQAYYM6NUJWPBSYBBIXH5ENQ4Z5LTJELU";
        let addr = Address::from_str(s).unwrap();
        assert_eq!(&addr.to_string(), s);
    }

    #[test]
    fn non_canonical() {
        let addr = "J5YDZLPOHWB5O6MVRHNFGY4JXIQAYYM6NUJWPBSYBBIXH5ENQ4Z5LTJELU";
        let non_canonical = "J5YDZLPOHWB5O6MVRHNFGY4JXIQAYYM6NUJWPBSYBBIXH5ENQ4Z5LTJELV";

        assert_eq!(Address::from_str(addr).is_ok(), true);
        assert_eq!(
            Address::from_str(non_canonical),
            Err(AddressError::InvalidBase32)
        );
    }
}
