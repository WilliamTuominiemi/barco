use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = ImageReader::open("barcode.png")?.decode()?;
    let image_height = image.height();
    let image_width = image.width();
    let cropped = image.crop(0, image_height / 2, image_width, 1);
    let line = cropped.as_bytes();

    let barcode = read_barcode(line);
    println!("{:?}", barcode);

    Ok(())
}

fn read_barcode(line: &[u8]) -> Vec<u8> {
    let mut bytes = line.to_vec();

    for i in 0..bytes.len() {
        let byte = bytes[i];

        if byte / 2 < 127 {
            bytes[i] = 1;
        } else {
            bytes[i] = 0;
        }
    }

    let mut start: Option<usize> = None;
    let mut end: Option<usize> = None;
    for i in 0..bytes.len() {
        if bytes[i] == 1 && start.is_none() {
            start = Some(i);
        } else if bytes[i] == 1 && start.is_some() {
            end = Some(i);
        }
    }

    let barcode = match (start, end) {
        (Some(s), Some(e)) => bytes[s..e].to_vec(),
        _ => panic!("barcode not found"),
    };

    barcode
}
