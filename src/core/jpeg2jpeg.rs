use crate::{utils::magick, Dimensions, InputOutput};

pub fn convert<T>(config: &T) -> std::result::Result<(), magick_rust::MagickError>
where
    T: InputOutput + Dimensions,
{
    magick::optimize(&magick::Config {
        input_path: config.input_path(),
        output_path: config.output_path(),
        width: config.width(),
        height: config.height(),
    })
}

#[cfg(test)]
mod tests {
    use crate::Config;

    #[test]
    fn jpeg2jpeg() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/jpeg2jpeg_test1.jpg".into(),
            output_path: &"target/jpeg2jpeg_test1.jfif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }

    #[test]
    #[should_panic = "MagickError(\"failed to read image\")"]
    fn jpeg2jpeg_panic() {
        use super::*;

        convert(&Config {
            input_path: &"tests/files/jpeg2jpeg_notexisting_test1.jpg".into(),
            output_path: &"target/jpeg2jpeg_test1.jfif".into(),
            width: Some(100),
            height: None,
        })
        .unwrap();
    }
}
