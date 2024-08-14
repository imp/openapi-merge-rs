use std::env;
use std::io;

// use tracing_subscriber::{fmt, EnvFilter};

use openapi_merge::MergeConfig;

fn main() -> io::Result<()> {
    // fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let mut args = env::args();
    let name = args.next().unwrap();
    if let Some(path) = args.next() {
        MergeConfig::from_path(path)
            .inspect(report_config)?
            .load_inputs()
            .inspect(report_inputs)?
            .merge()
            .inspect(report_merge)?
            .save()
            .inspect(report_save)?;
    } else {
        eprintln!("Usage: {name} <config.json>");
    }
    Ok(())
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

fn report_merge(config: &MergeConfig) {
    println!(
        "## Inputs merged, writing the results out to '{}' ({:?})",
        config.output.display(),
        config.merge_time,
    )
}

fn report_save(config: &MergeConfig) {
    println!(
        "## Finished writing to '{}' ({:?})",
        config.output.display(),
        config.save_time
    );
}
