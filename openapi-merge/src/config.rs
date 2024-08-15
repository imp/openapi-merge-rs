use super::*;

#[derive(Debug, Default, Deserialize)]
pub struct MergeConfig {
    pub inputs: Vec<Input>,
    pub output: PathBuf,
    #[serde(skip)]
    pub source: PathBuf,
    #[serde(skip)]
    pub load_time: time::Duration,
    #[serde(skip)]
    pub merged: OpenAPI,
    #[serde(skip)]
    pub merge_time: time::Duration,
    #[serde(skip)]
    pub save_time: time::Duration,
}

impl MergeConfig {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let now = time::Instant::now();
        let source = path.as_ref().to_path_buf();
        let config = load_json_file(path)?;
        let load_time = now.elapsed();

        Ok(Self {
            source,
            load_time,
            ..config
        })
    }

    pub fn into_merge(self) -> Merge {
        self.into()
    }

    pub fn load_inputs(self) -> io::Result<Self> {
        let base = self.source.parent().unwrap_or(Path::new("."));

        self.inputs
            .into_iter()
            .map(|input| input.load(base))
            .collect::<io::Result<Vec<_>>>()
            .map(|inputs| Self { inputs, ..self })
    }

    pub fn output(&self) -> PathBuf {
        self.source
            .parent()
            .unwrap_or(Path::new("."))
            .join(&self.output)
    }
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
