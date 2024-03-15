use image_resizer::utils::gifsicle;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "tests/files/test1.gif";

    // Specify the output PNG file path (optional)
    let output_path = "target/gifsicle_test1.gif";

    gifsicle::optimize(gifsicle::Config {
        input_path,
        output_path,
        width: Some(100),
        height: Some(100),
    });

    Ok(())
}
