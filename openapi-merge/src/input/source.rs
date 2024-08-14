use super::*;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputSource {
    InputFile {
        #[serde(rename = "inputFile")]
        input_file: PathBuf,
    },
    InputUrl {
        #[serde(rename = "inputURL")]
        input_url: String,
    },
}

impl InputSource {
    pub fn load(&self, base: &Path) -> io::Result<OpenAPI> {
        match self {
            Self::InputFile { input_file } => {
                let path = base.join(input_file);
                load_json_file(path)
            }
            Self::InputUrl { .. } => todo!(),
        }
    }
}

impl fmt::Display for InputSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputFile { input_file } => input_file.display().fmt(f),
            Self::InputUrl { input_url } => input_url.fmt(f),
        }
    }
}
