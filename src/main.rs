use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = ImageReader::open("barcode.png")?.decode()?;
    let image_height = image.height();
    let image_width = image.width();
    let cropped = image.crop(0, image_height / 2, image_width, 1);
    let line = cropped.as_bytes();

    let barcode = read_barcode(line);
    println!("{:?}", barcode);
    let start_code_end_index = read_start_guard(barcode);

    Ok(())
}

fn read_start_guard(barcode: Vec<u8>) -> usize {
    let mut indexes: Vec<usize> = vec![];
    let mut lengths: Vec<usize> = vec![];

    for i in 0..barcode.len() {
        let byte = barcode[i];

        let entries = indexes.len();
        let last = if entries == 0 {
            None
        } else {
            Some(indexes[entries - 1])
        };

        if (entries == 0 && byte == 0) || (entries == 1 && byte == 1) || (entries == 2 && byte == 0)
        {
            indexes.push(i - 1);

            match last {
                Some(index) => lengths.push(i - index - 1),
                _ => lengths.push(i),
            }
        }
    }

    let ratio = lengths[0] as f64 / lengths[2] as f64;
    if (ratio < 0.6) || (ratio > 1.4) {
        panic!("Error reading start guard, invalid barcode")
    }

    println!("{:?}", indexes);
    println!("{:?}", lengths);

    indexes[2]
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
