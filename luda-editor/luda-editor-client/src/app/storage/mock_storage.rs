use super::{
    get_background_image_paths::GithubStorageBackgroundImagePathsGet,
    get_background_image_url::{GetBackgroundImageUrlError, GithubStorageBackgroundImageUrlGet},
    get_character_image_paths::GithubStorageCharacterImagePathsGet,
    get_character_image_url::{GetCharacterImageUrlError, GithubStorageCharacterImageUrlGet},
    get_meta::{GetMetaError, GithubStorageMetaGet},
    get_sequence::{GetSequenceError, GithubStorageSequenceGet},
    get_sequence_list::{GetSequenceListError, GithubStorageSequenceListGet},
    get_sequence_lock_state::{
        GetSequenceLockStateError, SequenceLockState, StorageSequenceLockStateGet,
    },
    get_sequence_titles::{GetSequenceIndexError, GithubStorageSequenceTitlesGet},
    lock_sequence::{GithubStorageSequenceLock, LockSequenceError},
    put_sequence::{GithubStorageSequencePut, PutSequenceError},
    put_sequence_titles::{GithubStorageSequenceTitlesPut, PutSequenceIndexError},
    unlock_sequence::{GithubStorageSequenceUnlock, UnlockSequenceError},
    ExpiredAt, GithubStorage, SequenceName, StorageInitError,
};
use crate::app::types::{Meta, Sequence};
use async_trait::async_trait;
use mockall::mock;
use namui::Url;

#[cfg(test)]
mock! {
    #[derive(Debug)]
    pub Storage {}

    #[async_trait(?Send)]
    impl GithubStorage for Storage {
        async fn init(&self) -> Result<(), StorageInitError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageMetaGet for Storage {
        async fn get_meta(&self) -> Result<Meta, GetMetaError>;
    }

    #[async_trait(?Send)]
    impl StorageSequenceLockStateGet for Storage {
        async fn get_sequence_lock_state(
            &self,
            sequence_name: &str,
        ) -> Result<SequenceLockState, GetSequenceLockStateError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceLock for Storage {
        async fn lock_sequence(&self, sequence_name: &str) -> Result<ExpiredAt, LockSequenceError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceUnlock for Storage {
        async fn unlock_sequence(&self, sequence_name: &str) -> Result<(), UnlockSequenceError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceGet for Storage {
        async fn get_sequence(&self, sequence_name: &str) -> Result<Sequence, GetSequenceError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequencePut for Storage {
        async fn put_sequence(
            &self,
            sequence_name: &str,
            sequence: &Sequence,
        ) -> Result<(), PutSequenceError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceListGet for Storage {
        async fn get_sequence_list(&self) -> Result<Vec<SequenceName>, GetSequenceListError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceTitlesGet for Storage {
        async fn get_sequence_titles(&self) -> Result<Vec<SequenceName>, GetSequenceIndexError>;
    }

    #[async_trait(?Send)]
    impl GithubStorageSequenceTitlesPut for Storage {
        async fn put_sequence_titles(
            &self,
            sequence_titles: &Vec<SequenceName>,
        ) -> Result<(), PutSequenceIndexError>;
    }

    impl GithubStorageBackgroundImagePathsGet for Storage {
        fn get_background_image_paths(&self) -> Vec<String>;
    }

    impl GithubStorageBackgroundImageUrlGet for Storage {
        fn get_background_image_url(
            &self,
            background_image_path: &str,
        ) -> Result<Url, GetBackgroundImageUrlError>;
    }

    impl GithubStorageCharacterImageUrlGet for Storage {
        fn get_character_image_url(
            &self,
            character_image_path: &str,
        ) -> Result<Url, GetCharacterImageUrlError>;
    }

    impl GithubStorageCharacterImagePathsGet for Storage {
        fn get_character_image_paths(&self) -> Vec<String>;
    }
}
