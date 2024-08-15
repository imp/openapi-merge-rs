use super::*;

#[derive(Debug, Default)]
pub struct Merge {
    pub config: MergeConfig,
    pub merged: OpenAPI,
    pub merge_time: time::Duration,
    pub save_time: time::Duration,
}

impl Merge {
    pub fn merge(mut self) -> io::Result<Self> {
        let now = time::Instant::now();
        let mut inputs = self.config.inputs.drain(..);
        // Use first element is a base for merging
        let base = inputs
            .next()
            .ok_or(io::Error::other("At least one input required"))?;
        let merged = inputs.fold(base, merge_into_base).openapi;
        let merge_time = now.elapsed();

        Ok(Self {
            merged,
            merge_time,
            ..self
        })
    }

    pub fn save(self) -> io::Result<Self> {
        let now = time::Instant::now();
        let path = self.config.output();
        save_json_file(&path, &self.merged)?;
        let save_time = now.elapsed();

        Ok(Self { save_time, ..self })
    }
}

impl From<MergeConfig> for Merge {
    fn from(config: MergeConfig) -> Self {
        Self {
            config,
            ..default()
        }
    }
}

fn merge_into_base(base: Input, mut other: Input) -> Input {
    tracing::info!(other = %other.source, "Processing");

    let mut openapi = base.openapi;
    openapi.merge_components(other.components());
    openapi.merge_security(other.security());
    openapi.merge_tags(other.tags());
    openapi.merge_extensions(other.extensions());

    for (path, method, operation) in other.operations() {
        openapi.merge_operation(path, method, operation);
    }

    Input { openapi, ..base }
}
