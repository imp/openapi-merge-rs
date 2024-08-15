use std::fmt;
use std::fs;
use std::io;
use std::mem;
use std::path::{Path, PathBuf};
use std::time;

use openapiv3::OpenAPI;
use serde::{de, Deserialize, Serialize};
use serde_json as json;

pub use config::MergeConfig;
pub use ext::OpenAPIExt;
pub use input::Description;
pub use input::Dispute;
pub use input::Input;
pub use input::InputSource;
pub use input::OperationSelection;
pub use input::PathModification;
pub use merge::Merge;

mod config;
mod ext;
mod input;
mod merge;

fn load_json_file<T>(path: impl AsRef<Path>) -> io::Result<T>
where
    T: de::DeserializeOwned,
{
    let text = fs::read_to_string(path)?;
    json::from_str(&text).map_err(io::Error::other)
}

fn save_json_file<T>(path: impl AsRef<Path>, value: &T) -> io::Result<()>
where
    T: Serialize,
{
    let text = json::to_string_pretty(value).map_err(io::Error::other)?;
    fs::write(path, text)
}

fn default<T: Default>() -> T {
    T::default()
}
