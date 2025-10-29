use image::ImageReader;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bc = ImageReader::open("barcode.png")?.decode()?;
    let image_height = bc.height();
    let image_width = bc.width();
    let line = bc.crop(0, image_height / 2, image_width, 1);

    let bytes = line.as_bytes();

    println!("{:?}", bytes);

    Ok(())
}
