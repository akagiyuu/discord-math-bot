mod image_format;
mod latex;

use anyhow::Result;
use image_format::ImageFormat;
use latex::TexFile;
use std::{process::Command, path::PathBuf};

const BINARY: &str = "wolframscript";
pub const TEMP_FOLDER: &str = "/tmp";

/// Evaluate an expression and output as an image
pub fn evaluate(expression: &str) -> Result<PathBuf> {
    let mut mathematica = Command::new(BINARY);
    mathematica
        .args(["-c", expression])
        .args(["-format", "TeX"]);
    let output = mathematica.output().expect("Failed when trying to evaluate expression using mathematica");
    let mut latex = TexFile::create_with_random_name(&output.stdout).expect("Failed trying to create latex file");
    let generated_image_path = latex.create_img(ImageFormat::Png).expect("Failed to convert latex file to image");
    Ok(generated_image_path)
}
