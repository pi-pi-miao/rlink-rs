use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::utils::read_file_as_string;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Ready,
    Running,
    Killed,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    pub(crate) application_id: String,
    pub(crate) execute_file: String,
    pub(crate) status: Status,
}

impl Application {
    pub fn new(application_id: String, execute_file: String) -> Self {
        Application {
            application_id,
            execute_file,
            status: Status::Ready,
        }
    }

    pub fn load(parent_path: PathBuf) -> std::io::Result<Self> {
        let metadata_file = parent_path.join("metadata");

        let s = read_file_as_string(metadata_file)?;

        let job: Application = serde_json::from_str(s.as_str())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(job)
    }

    pub fn storage(&self, parent_path: PathBuf) -> std::io::Result<()> {
        let metadata_file = parent_path.join("metadata");
        let mut p = File::create(metadata_file)?;
        let context = serde_json::to_string(self).unwrap();
        p.write_all(context.as_bytes())?;
        p.flush()
    }
}
