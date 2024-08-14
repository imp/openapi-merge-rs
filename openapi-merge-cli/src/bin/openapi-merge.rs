use std::env;
use std::io;

// use tracing_subscriber::{fmt, EnvFilter};

use openapi_merge::MergeConfig;

fn main() -> io::Result<()> {
    // fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let mut args = env::args();
    let name = args.next().unwrap();
    if let Some(path) = args.next() {
        let openapi = MergeConfig::from_path(path)?;
        openapi.merge()
    } else {
        eprintln!("Usage: {name} <config.json>");
        Ok(())
    }
}

pub fn report_config(config: &MergeConfig) {
    println!(
        "## Loaded the configuration: {} inputs ({:?})",
        config.inputs.len(),
        config.load_time,
    );
}

pub fn report_inputs(config: &MergeConfig) {
    config.inputs.iter().enumerate().for_each(|(idx, input)| {
        println!(
            "## Loaded input {idx}: '{}' ({:?})",
            input.source, input.load_time,
        )
    })
}

// #[cfg(test)]
// mod tests {
//     use assert_json_diff::assert_json_eq;
//     use openapiv3::OpenAPI;
//     use pretty_assertions::assert_eq;
//     use serde_json as json;

//     fn load_json<T>(text: &str) -> json::Result<T>
//     where
//         T: serde::de::DeserializeOwned,
//     {
//         json::from_str(text)
//     }

//     const OLD: &str = include_str!("../user-facing-openapi-old.json");
//     const NEW: &str = include_str!("../user-facing-openapi-new.json");

//     #[test]
//     fn openapi_compatibility() {
//         let old: OpenAPI = load_json(OLD).unwrap();
//         let new: OpenAPI = load_json(NEW).unwrap();
//         assert_eq!(old, new);
//     }

//     #[test]
//     fn json_value_compatibility() {
//         let old: json::Value = load_json(OLD).unwrap();
//         let new: json::Value = load_json(NEW).unwrap();
//         assert_json_eq!(old, new);
//     }
// }
