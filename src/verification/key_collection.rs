use std::marker::PhantomData;

use anyhow::{anyhow, Result};

use super::{cache::Cache, fetcher::KeyFetcher, public_key::PublicKey};

pub struct KeyCollection<C, K, F> {
    phantom_data: PhantomData<K>,
    cache: C,
    fetcher: F,
}

impl<C, K, F> KeyCollection<C, K, F> {
    pub fn new(cache: C, fetcher: F) -> Self {
        KeyCollection {
            phantom_data: PhantomData,
            cache,
            fetcher,
        }
    }
}

impl<C, K, F> KeyCollection<C, K, F>
where
    K: PublicKey,
    C: Cache<K>,
    F: KeyFetcher,
{
    pub async fn get_or_fetch_key(&mut self, serial: &str) -> Result<K>
    where
        <K as TryFrom<String>>::Error: Sync + std::fmt::Debug,
    {
        if let Some(key) = self.cache.get(serial).await {
            Ok(key)
        } else {
            self.cache.retire_keys().await;

            let response = self.fetcher.fetch(serial).await?;
            let public_key: K = response
                .key_base64
                .try_into()
                .map_err(|e| anyhow!("malformed key {:?}", e))?;
            self.cache.set(response.serial, public_key.clone()).await?;
            Ok(public_key)
        }
    }
}
