use std::env;
use std::io;

// use tracing_subscriber::{fmt, EnvFilter};

use openapi_merge::Merge;
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
            .into_merge()
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

fn report_merge(merge: &Merge) {
    println!(
        "## Inputs merged, writing the results out to '{}' ({:?})",
        merge.config.output.display(),
        merge.merge_time,
    )
}

fn report_save(merge: &Merge) {
    println!(
        "## Finished writing to '{}' ({:?})",
        merge.config.output.display(),
        merge.save_time
    );
}
