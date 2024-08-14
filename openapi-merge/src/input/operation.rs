use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationSelection {
    pub include_tags: Option<Vec<String>>,
    pub exclude_tags: Option<Vec<String>>,
}

impl OperationSelection {
    pub fn filter_operation(&self, operation: &openapiv3::Operation) -> bool {
        if let Some(exclude) = &self.exclude_tags {
            if exclude.iter().any(|tag| operation.tags.contains(tag)) {
                tracing::debug!(
                    "Dropping {:?} because it is in exclude list",
                    operation.summary
                );
                return false;
            }
        }

        if let Some(include) = &self.include_tags {
            if include.iter().any(|tag| operation.tags.contains(tag)) {
                return true;
            } else {
                tracing::debug!(
                    "Dropping {:?} because it is NOT in include list",
                    operation.summary
                );
                return false;
            }
        }

        true
    }
}
