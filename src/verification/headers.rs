use std::{borrow::Borrow, collections::HashMap};

use anyhow::{anyhow, Error, Result};
#[cfg(test)]
use mockall::automock;

pub const MARKETPLACE_HEADER_SIGNATURE: &str = "X-Marketplace-Signature";
pub const MARKETPLACE_HEADER_ALGORITHM: &str = "X-Marketplace-Signature-Algorithm";
pub const MARKETPLACE_HEADER_SERIAL: &str = "X-Marketplace-Signature-Serial";

/// Trait for headers that contain signature, algorithm, and serial information.
#[cfg_attr(test, automock)]
pub trait SignatureHeaders {
    /// Get the signature value.
    fn get_signature(&self) -> &str;
    /// Get the algorithm value. Only Ed25519 is supported at this moment.
    fn get_algorithm(&self) -> &str;
    /// Get the serial value.
    #[allow(unused)]
    fn get_serial(&self) -> &str;
}

/// Simple implementation of SignatureHeaders.
#[derive(Debug)]
pub struct MappedHeaders {
    pub serial: String,
    pub algorithm: String,
    pub signature: String,
}

impl SignatureHeaders for MappedHeaders {
    fn get_signature(&self) -> &str {
        &self.signature
    }
    fn get_algorithm(&self) -> &str {
        &self.algorithm
    }
    fn get_serial(&self) -> &str {
        &self.serial
    }
}

impl<K, V> TryFrom<&HashMap<K, V>> for MappedHeaders
where
    K: std::hash::Hash + Eq + Borrow<str>,
    V: Borrow<str> + std::fmt::Display,
{
    type Error = Error;
    fn try_from(headers: &HashMap<K, V>) -> Result<Self, Self::Error> {
        let signature = headers
            .get(MARKETPLACE_HEADER_SIGNATURE)
            .ok_or_else(|| anyhow!("missing {} header", MARKETPLACE_HEADER_SIGNATURE))?;
        let algorithm = headers
            .get(MARKETPLACE_HEADER_ALGORITHM)
            .ok_or_else(|| anyhow!("missing {} header", MARKETPLACE_HEADER_ALGORITHM))?;
        let serial = headers
            .get(MARKETPLACE_HEADER_SERIAL)
            .ok_or_else(|| anyhow!("missing {} header", MARKETPLACE_HEADER_SERIAL))?;

        log::debug!("Signature: {}, algorithm: {}, serial: {}", signature, algorithm, serial);

        Ok(MappedHeaders {
            signature: signature.borrow().into(),
            algorithm: algorithm.borrow().into(),
            serial: serial.borrow().into(),
        })
    }
}
