use magick_rust::{magick_wand_genesis, MagickWand};
use std::{path::PathBuf, sync::Once};

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.

static START: Once = Once::new();

pub struct Config<'a> {
    pub input_path: &'a PathBuf,
    pub output_path: &'a PathBuf,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub fn optimize(config: &Config) -> Result<(), magick_rust::MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    wand.read_image(&config.input_path.display().to_string())?;

    let width = config.width.map_or(wand.get_image_width(), |s| s as usize);
    let height = config
        .height
        .map_or(wand.get_image_height(), |s| s as usize);

    wand.fit(width, height);
    wand.auto_orient();
    wand.strip_image()?;
    wand.set_image_compression_quality(75)?;

    wand.write_image(&config.output_path.display().to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn magic_resize_and_auto_orient() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: &PathBuf::from("tests/files/orientation_test.jpg"),
            output_path: &PathBuf::from("target/magick_out.jpg"),
            width: Some(240),
            height: Some(100),
        })
    }
    #[test]
    fn magic_resize_and_auto_orient_gif() -> Result<(), magick_rust::MagickError> {
        use super::*;

        optimize(&Config {
            input_path: &PathBuf::from("tests/files/test1.gif"),
            output_path: &PathBuf::from("target/magick_out.gif"),
            width: Some(100),
            height: Some(100),
        })
    }
}
