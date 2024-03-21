use std::path::Path;
use std::path::PathBuf;

use crate::utils::gifsicle;
use crate::utils::webp;
pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[must_use]
pub fn path_without_extension(input_path: &Path) -> Option<String> {
    let file_stem = input_path.file_stem().map(|s| s.to_str())?;
    let parent = input_path.parent().map(|p| p.to_str())?;
    parent.and_then(|p| file_stem.map(|f| format!("{p}/{f}")))
}

pub fn convert(config: &Config) -> std::result::Result<(), std::io::Error> {
    let output_path_without_extension = path_without_extension(config.output_path).unwrap();
    let step1_output_path = &PathBuf::from(format!("{output_path_without_extension}_step1.gif"));
    gifsicle::optimize(gifsicle::Config {
        input_path: config.input_path,
        output_path: step1_output_path,
        width: config.width,
        height: config.height,
    })?;
    webp::optimize_gif(&webp::GifConfig {
        input_path: step1_output_path,
        output_path: config.output_path,
    })?;
    std::fs::remove_file(step1_output_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn gif2webp() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/gif2webp_test1.gif"),
            output_path: &PathBuf::from("target/gif2webp_test1.webp"),
            width: Some(100),
            height: Some(100),
        })
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn gif2webp_panic() {
        use super::*;

        convert(&Config {
            input_path: &PathBuf::from("tests/files/gif2webp_panic_test1.gif"),
            output_path: &PathBuf::from("target/gif2webp_test1.webp"),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
