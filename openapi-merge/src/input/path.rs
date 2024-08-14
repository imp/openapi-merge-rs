use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathModification {
    /// When copying over the paths from your OpenAPI specification for this input,
    /// it will strip this string from the start of the path if it is found.
    pub strip_start: Option<String>,
    /// When copying over the paths from your OpenAPI specification for this input,
    /// it will prepend this string to the start of the path if it is found.
    /// `prepend`` will always run after stripStart so that it is deterministic.
    pub prepend: Option<String>,
}
