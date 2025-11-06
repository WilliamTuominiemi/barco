use image::ImageReader;

pub fn read_image(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut image = ImageReader::open(path)?.decode()?;
    let image_height = image.height();
    let image_width = image.width();
    let cropped = image.crop(0, image_height / 2, image_width, 1);

    let line = cropped.as_bytes();

    return Ok(read_barcode(line));
}

fn read_barcode(line: &[u8]) -> Vec<u8> {
    let mut bytes = line.to_vec();

    for i in 0..bytes.len() {
        let byte = bytes[i];

        if byte / 2 < 100 {
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
