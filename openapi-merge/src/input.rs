use super::*;

pub use description::Description;
pub use dispute::Dispute;
pub use operation::OperationSelection;
pub use path::PathModification;
pub use source::InputSource;

mod description;
mod dispute;
mod operation;
mod path;
mod source;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(flatten)]
    pub source: InputSource,
    pub operation_selection: Option<OperationSelection>,
    pub description: Option<Description>,
    pub path_modification: Option<PathModification>,
    #[doc(hidden)]
    #[serde(skip)]
    pub openapi: OpenAPI,
    #[doc(hidden)]
    #[serde(skip)]
    pub load_time: time::Duration,
}

impl Input {
    pub fn load(self, base: &Path) -> io::Result<Self> {
        let now = time::Instant::now();
        let openapi = self.source.load(base)?;
        let load_time = now.elapsed();

        Ok(Self {
            openapi,
            load_time,
            ..self
        })
    }

    pub fn operations(&self) -> impl Iterator<Item = (&str, &str, &openapiv3::Operation)> {
        self.openapi
            .operations()
            .filter(|(_path, _method, operation)| self.filter_operation(operation))
    }

    fn filter_operation(&self, operation: &openapiv3::Operation) -> bool {
        self.operation_selection
            .as_ref()
            .map_or(true, |selection| selection.filter_operation(operation))
    }

    pub fn components(&mut self) -> Option<openapiv3::Components> {
        self.openapi.components.take()
    }

    pub fn security(&mut self) -> Option<Vec<openapiv3::SecurityRequirement>> {
        self.openapi.security.take()
    }

    pub fn tags(&mut self) -> Vec<openapiv3::Tag> {
        mem::take(&mut self.openapi.tags)
    }

    pub fn extensions(&mut self) -> indexmap::IndexMap<String, json::Value> {
        mem::take(&mut self.openapi.extensions)
    }
}
