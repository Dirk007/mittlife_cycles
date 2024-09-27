use mockall::{mock, predicate::*};

use crate::verification::public_key::PublicKey;

mock! {
    #[derive(Debug, Clone)]
    pub Key{}

    impl PublicKey for Key {
        fn len(&self) -> usize;
        fn get_bytes(&self) -> &[u8];
        fn is_empty(&self) -> bool;
        fn age(&self) -> std::time::Duration;
    }

    impl Clone for Key {
        fn clone(&self) -> Self;
    }

    impl TryFrom<String> for Key {
        type Error = anyhow::Error;
        fn try_from(value: String) -> Result<Self, <MockKey as TryFrom<String>>::Error>;
    }
}
