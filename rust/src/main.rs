use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use imgico::{imgico_core, imgsvg_core};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "imgico")]
#[command(about = "Convert images to ICO or SVG", long_about = None)]
struct Cli {
    /// Input file path
    input: PathBuf,

    /// Output format: 'ico' or 'svg'
    #[arg(short, long, default_value = "ico")]
    format: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let input_data = fs::read(&cli.input).context("Failed to read input file")?;

    // Match Node.js timestamp format: YYYY-MM-DDTHH-mm-ss-sssZ
    let timestamp = Utc::now().format("%Y-%m-%dT%H-%M-%S-%3fZ").to_string();
    let dir_name = format!("imgico-{}", timestamp);
    fs::create_dir_all(&dir_name).context("Failed to create output directory")?;

    let sizes = [16, 32, 48, 64, 128, 256];
    let format = cli.format.to_lowercase();
    let is_svg = format == "svg";

    for size in sizes {
        let ext = if is_svg { "svg" } else { "ico" };
        let output_path = PathBuf::from(&dir_name).join(format!("{}.{}", size, ext));

        let buffer = if is_svg {
            imgsvg_core(&input_data, Some(size)).map_err(|e| anyhow::anyhow!(e))?
        } else {
            // Create a single-size ICO for each size
            imgico_core(&input_data, Some(vec![size])).map_err(|e| anyhow::anyhow!(e))?
        };

        fs::write(&output_path, buffer)
            .context(format!("Failed to write output file {:?}", output_path))?;
    }

    println!(
        "Extracted {} images to {}",
        if is_svg { "SVG" } else { "ICO" },
        dir_name
    );

    Ok(())
}
