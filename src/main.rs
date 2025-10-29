use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bc = ImageReader::open("barcode.png")?.decode()?;
    let image_height = bc.height();
    let image_width = bc.width();
    let line = bc.crop(0, image_height / 2, image_width, 1);

    let mut bytes = line.as_bytes().to_vec();

    for i in 0..bytes.len() {
        let byte = bytes[i];

        if byte / 2 < 127 {
            bytes[i] = 0;
        } else {
            bytes[i] = 255;
        }
    }

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    for i in 0..bytes.len() {
        if bytes[i] == 0 && start.is_none() {
            start = Some(i);
        } else if bytes[i] == 0 && start.is_some() {
            end = Some(i);
        }
    }

    let barcode = match (start, end) {
        (Some(s), Some(e)) => bytes[s..e].to_vec(),
        _ => panic!("barcode not found"),
    };

    println!("{:?}", bytes);
    println!("{:?}", barcode);

    Ok(())
}
