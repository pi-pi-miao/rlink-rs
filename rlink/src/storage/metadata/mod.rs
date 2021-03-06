use std::error::Error;
use std::fmt::Debug;

use crate::api::cluster::MetadataStorageType;
use crate::runtime::{ApplicationDescriptor, TaskManagerStatus};
use crate::storage::metadata::mem_metadata_storage::MemoryMetadataStorage;

pub mod mem_metadata_storage;

pub mod metadata_loader;
pub use metadata_loader::MetadataLoader;

pub trait TMetadataStorage: Debug {
    fn save_job_descriptor(
        &mut self,
        metadata: ApplicationDescriptor,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn delete_job_descriptor(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn read_job_descriptor(&self) -> Result<ApplicationDescriptor, Box<dyn Error + Send + Sync>>;
    fn update_job_status(
        &self,
        job_manager_status: TaskManagerStatus,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn update_task_status(
        &self,
        task_manager_id: &str,
        task_manager_address: &str,
        task_manager_status: TaskManagerStatus,
        metrics_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[derive(Debug)]
pub enum MetadataStorage {
    MemoryMetadataStorage(MemoryMetadataStorage),
}

impl MetadataStorage {
    pub fn new(mode: &MetadataStorageType) -> Self {
        match mode {
            MetadataStorageType::Memory => {
                let storage = MemoryMetadataStorage::new();
                MetadataStorage::MemoryMetadataStorage(storage)
            }
        }
    }
}

impl TMetadataStorage for MetadataStorage {
    fn save_job_descriptor(
        &mut self,
        metadata: ApplicationDescriptor,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorage::MemoryMetadataStorage(storage) => {
                storage.save_job_descriptor(metadata)
            }
        }
    }

    fn delete_job_descriptor(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorage::MemoryMetadataStorage(storage) => storage.delete_job_descriptor(),
        }
    }

    fn read_job_descriptor(&self) -> Result<ApplicationDescriptor, Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorage::MemoryMetadataStorage(storage) => storage.read_job_descriptor(),
        }
    }

    fn update_job_status(
        &self,
        job_manager_status: TaskManagerStatus,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorage::MemoryMetadataStorage(storage) => {
                storage.update_job_status(job_manager_status)
            }
        }
    }

    fn update_task_status(
        &self,
        task_manager_id: &str,
        task_manager_address: &str,
        task_manager_status: TaskManagerStatus,
        metrics_address: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match self {
            MetadataStorage::MemoryMetadataStorage(storage) => storage.update_task_status(
                task_manager_id,
                task_manager_address,
                task_manager_status,
                metrics_address,
            ),
        }
    }
}

pub(crate) fn loop_read_job_descriptor(
    metadata_storage: &MetadataStorage,
) -> ApplicationDescriptor {
    loop_fn!(
        metadata_storage.read_job_descriptor(),
        std::time::Duration::from_secs(2)
    )
}

pub(crate) fn loop_save_job_descriptor(
    metadata_storage: &mut MetadataStorage,
    application_descriptor: ApplicationDescriptor,
) {
    loop_fn!(
        metadata_storage.save_job_descriptor(application_descriptor.clone()),
        std::time::Duration::from_secs(2)
    );
}

pub(crate) fn loop_delete_job_descriptor(metadata_storage: &mut MetadataStorage) {
    loop_fn!(
        metadata_storage.delete_job_descriptor(),
        std::time::Duration::from_secs(2)
    );
}

pub(crate) fn loop_update_job_status(
    metadata_storage: &mut MetadataStorage,
    job_manager_status: TaskManagerStatus,
) {
    loop_fn!(
        metadata_storage.update_job_status(job_manager_status.clone()),
        std::time::Duration::from_secs(2)
    );
}
