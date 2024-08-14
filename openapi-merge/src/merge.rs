use super::*;

#[derive(Debug, Deserialize)]
pub struct MergeConfig {
    pub inputs: Vec<Input>,
    pub output: PathBuf,
    #[serde(skip)]
    pub source: PathBuf,
    #[serde(skip)]
    pub load_time: time::Duration,
}

impl MergeConfig {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let now = time::Instant::now();
        let source = path.as_ref().to_path_buf();
        let merge = load_json_file(&source)?;
        let load_time = now.elapsed();

        Ok(Self {
            source,
            load_time,
            ..merge
        })
        // load_json_file::<Self>(path).inspect(|oam| {
        //     println!(
        //         "## Loaded the configuration: {} inputs ({:?})",
        //         oam.inputs.len(),
        //         now.elapsed()
        //     )
        // })
    }

    pub fn load_inputs(self) -> io::Result<Self> {
        let base = self.source.parent().unwrap_or(Path::new("."));

        self.inputs
            .into_iter()
            .map(|input| input.load(base))
            .collect::<io::Result<Vec<_>>>()
            .map(|inputs| Self { inputs, ..self })
    }

    pub fn merge(self) -> io::Result<()> {
        // Use first element is a base for merging
        let now = time::Instant::now();
        let mut inputs = self.inputs.into_iter();
        let base = inputs
            .next()
            .ok_or(io::Error::other("At least one input required"))?;
        let merged = inputs.fold(base, merge_into_base);
        println!(
            "## Inputs merged, writing the results out to '{}' ({:?})",
            self.output.display(),
            now.elapsed()
        );

        let now = time::Instant::now();
        save_json_file(&self.output, &merged.openapi)?;
        println!(
            "## Finished writing to '{}' ({:?})",
            self.output.display(),
            now.elapsed()
        );
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;

    const CONFIG: &str = r#"
{
  "inputs": [
    {
      "inputFile": "./base/openapi.json",
      "operationSelection": {
        "excludeTags": [
          "deprecated"
        ]
      }
    },
    {
      "inputFile": "./mod1/openapi.json"
    },
    {
      "inputFile": "./mod2/openapi.json"
    }
  ],
  "output": "./rest-api/user-facing-openapi.json"
}
"#;
    #[test]
    fn merge_config_deserialize() {
        let config: MergeConfig = json::from_str(CONFIG).unwrap();
        assert_eq!(config.inputs.len(), 3);
    }
}
