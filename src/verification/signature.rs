use anyhow::bail;
use base64::{engine::general_purpose, Engine as _};

const ED25519_SIGNATURE_LENTH: usize = 64;

// Implementing Ed25519 signature using base64 for encoding/decoding.
pub struct Ed25519Signature {
    data: [u8; ED25519_SIGNATURE_LENTH],
}

impl TryFrom<&str> for Ed25519Signature {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let decoded = general_purpose::STANDARD.decode(value)?;
        if decoded.len() != ED25519_SIGNATURE_LENTH {
            bail!("unsupported signature length {}", decoded.len());
        }
        let mut target: [u8; ED25519_SIGNATURE_LENTH] = [0; ED25519_SIGNATURE_LENTH];
        target.copy_from_slice(&decoded);
        Ok(Ed25519Signature { data: target })
    }
}

impl From<Ed25519Signature> for ed25519::Signature {
    fn from(signature: Ed25519Signature) -> Self {
        ed25519::Signature::from_bytes(&signature.data)
    }
}
