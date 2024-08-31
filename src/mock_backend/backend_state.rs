use anyhow::{anyhow, Result};
use poem::http::HeaderMap;

use crate::verification::{
    fetcher::KeyFetcher, headers::SignatureHeaders, Ed25519Verifier, KeyCollection, MappedHeaders, MemoryCache,
    ReqwestFetcher, Verifier,
};

fn extract_heaader<'a>(headers: &'a HeaderMap, key: &str) -> Result<&'a str> {
    let result = headers.get(key).ok_or(anyhow!("Missing {} header", key))?.to_str()?;
    Ok(result)
}

pub struct BackendState<T: Verifier> {
    key_collection: KeyCollection<MemoryCache<T::KeyType>, T::KeyType, ReqwestFetcher>,
    verifier: T,
}

impl TryFrom<&HeaderMap> for MappedHeaders {
    type Error = anyhow::Error;

    fn try_from(headers: &HeaderMap) -> std::result::Result<Self, Self::Error> {
        let serial = extract_heaader(headers, "X-Marketplace-Signature-Serial")?;
        let algorithm = extract_heaader(headers, "X-Marketplace-Signature-Algorithm")?;
        let signature = extract_heaader(headers, "X-Marketplace-Signature")?;

        Ok(MappedHeaders {
            signature: signature.to_string(),
            algorithm: algorithm.to_string(),
            serial: serial.to_string(),
        })
    }
}

#[allow(unused)]
pub fn new_ed25519_verifier(base_url: &str) -> BackendState<Ed25519Verifier> {
    BackendState::new(
        KeyCollection::new(
            MemoryCache::default(),
            ReqwestFetcher::default().with_base_url(base_url),
        ),
        Ed25519Verifier::default(),
    )
}

impl<T: Verifier> BackendState<T> {
    pub fn new(
        key_collection: KeyCollection<MemoryCache<T::KeyType>, T::KeyType, ReqwestFetcher>,
        verifier: T,
    ) -> Self {
        BackendState {
            key_collection,
            verifier,
        }
    }

    pub async fn verify_request(&mut self, body: &Vec<u8>, headers: &HeaderMap) -> Result<()>
    where
        <<T as Verifier>::KeyType as TryFrom<String>>::Error: Sync + std::fmt::Debug,
    {
        let headers: MappedHeaders = headers.try_into()?;
        let serial = headers.get_serial();
        let public_key = self.key_collection.get_or_fetch_key(serial).await?;
        self.verifier.verify_signature(&headers, body, &public_key)
    }
}
