use anyhow::{bail, Result};
use base64::{engine::general_purpose, Engine as _};

pub const ED25519_PUBLIC_KEY_LENGTH: usize = 32;

pub trait PublicKey: Send + Sync + Clone + TryFrom<String> {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn get_bytes(&self) -> &[u8];
    fn age(&self) -> std::time::Duration;
}
/// A public key fetched from the server.
#[derive(Clone)]
pub struct ED25519PublicKey {
    pub created: std::time::Instant,
    pub value: [u8; ED25519_PUBLIC_KEY_LENGTH],
}

impl PublicKey for ED25519PublicKey {
    fn len(&self) -> usize {
        ED25519_PUBLIC_KEY_LENGTH
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_bytes(&self) -> &[u8] {
        &self.value
    }

    fn age(&self) -> std::time::Duration {
        std::time::Instant::now().duration_since(self.created)
    }
}

impl TryFrom<String> for ED25519PublicKey {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        log::debug!("Key response: {}", value);
        let decoded = general_purpose::STANDARD.decode(value)?;
        if decoded.len() != ED25519_PUBLIC_KEY_LENGTH {
            bail!("unsupported public key length");
        }
        let mut target: [u8; ED25519_PUBLIC_KEY_LENGTH] = [0; ED25519_PUBLIC_KEY_LENGTH];
        target.copy_from_slice(&decoded);
        log::debug!("Decoded public key: {:x?}", target);
        Ok(ED25519PublicKey::new(target))
    }
}

impl std::fmt::Display for ED25519PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Age: {:.3}s, Key: {:02X?}", self.age().as_secs_f32(), self.value)
    }
}

impl ED25519PublicKey {
    pub fn new(value: [u8; ED25519_PUBLIC_KEY_LENGTH]) -> Self {
        ED25519PublicKey {
            created: std::time::Instant::now(),
            value,
        }
    }
}
