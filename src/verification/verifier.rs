use anyhow::{anyhow, bail, Result};
use ed25519_dalek::VerifyingKey;

use super::{
    headers::SignatureHeaders,
    public_key::{ED25519PublicKey, PublicKey},
    signature::Ed25519Signature,
};

/// A trait for verifying signatures.
pub trait Verifier: Send + Sync {
    type KeyType: PublicKey;

    fn verify_signature(
        &self,
        headers: &impl SignatureHeaders,
        body: &impl AsRef<[u8]>,
        key: &impl PublicKey,
    ) -> Result<()>;
}

/// An implementation of the Verifier trait for Ed25519.
pub struct Ed25519Verifier;

impl Default for Ed25519Verifier {
    fn default() -> Self {
        Ed25519Verifier {}
    }
}

impl Verifier for Ed25519Verifier {
    type KeyType = ED25519PublicKey;

    fn verify_signature(
        &self,
        headers: &impl SignatureHeaders,
        body: &impl AsRef<[u8]>,
        key: &impl PublicKey,
    ) -> Result<()> {
        let algorithm = headers.get_algorithm();
        if algorithm.to_lowercase() != "ed25519" {
            bail!("unsupported algorithm: {}", algorithm);
        }
        let signature: Ed25519Signature = headers.get_signature().try_into()?;
        if key.len() != super::public_key::ED25519_PUBLIC_KEY_LENGTH {
            bail!("unsupported public key length for ed25519");
        }
        let key: Box<[u8]> = key.get_bytes().into();
        verify_signature_bytes(body.as_ref(), signature, key)
    }
}

fn verify_signature_bytes(message: &[u8], signature: Ed25519Signature, public_key: Box<[u8]>) -> Result<()> {
    let key = public_key
        .first_chunk::<32>()
        .ok_or(anyhow!("invalid public key length"))?;
    let verifying_key = VerifyingKey::from_bytes(key)?;
    verifying_key.verify_strict(message, &signature.into())?;
    Ok(())
}
