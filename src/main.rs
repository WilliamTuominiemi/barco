use image::ImageReader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image = ImageReader::open("barcode.png")?.decode()?;
    let image_height = image.height();
    let image_width = image.width();
    let cropped = image.crop(0, image_height / 2, image_width, 1);
    let line = cropped.as_bytes();

    let barcode = read_barcode(line);
    println!("{:?}", barcode);
    let line_width = read_start_and_end_guard(&barcode);
    println!("{:?}", line_width);
    let binary = read_binary(&barcode, line_width);
    println!("{:?}", binary);
    println!("{:?}", binary.len());

    Ok(())
}

fn read_binary(barcode: &Vec<u8>, line_width: i32) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let mut current: Vec<u8> = vec![];

    current.push(barcode[0]);

    for i in 1..barcode.len() {
        let byte = barcode[i];

        if barcode[i - 1] != byte {
            let previous_bar_length = if current.len() > line_width as usize {
                current.len()
            } else {
                line_width as usize
            };
            let bar_amount = ((previous_bar_length as f64) / (line_width as f64)).round() as u32;
            for _j in 0..bar_amount {
                result.push(current[0]);
            }
            current = vec![];
        } else {
            current.push(byte);
        }
    }

    result.pop();
    result
}

fn read_start_and_end_guard(barcode: &Vec<u8>) -> i32 {
    let mut start_indexes: Vec<usize> = vec![];
    let mut start_lengths: Vec<usize> = vec![];
    let mut end_indexes: Vec<usize> = vec![];
    let mut end_lengths: Vec<usize> = vec![];

    let bc_length = barcode.len();

    for i in 0..bc_length {
        let byte = barcode[i];

        let entries = start_indexes.len();
        let last = if entries == 0 {
            None
        } else {
            Some(start_indexes[entries - 1])
        };

        if (entries == 0 && byte == 0) || (entries == 1 && byte == 1) || (entries == 2 && byte == 0)
        {
            start_indexes.push(i - 1);

            match last {
                Some(index) => start_lengths.push(i - index - 1),
                _ => start_lengths.push(i),
            }
        }
    }

    for i in 0..bc_length {
        let byte = barcode[bc_length - i - 1];

        let entries = end_indexes.len();
        let last = if entries == 0 {
            None
        } else {
            Some(end_indexes[entries - 1])
        };

        if (entries == 0 && byte == 0) || (entries == 1 && byte == 1) || (entries == 2 && byte == 0)
        {
            end_indexes.push(i - 1);

            match last {
                Some(index) => end_lengths.push(i - index - 1),
                _ => end_lengths.push(i),
            }
        }
    }

    let start_ratio = start_lengths[0] as f64 / start_lengths[2] as f64;
    if (start_ratio < 0.6) || (start_ratio > 1.4) {
        panic!("Error reading start guard, invalid barcode")
    }

    let end_ratio = end_lengths[0] as f64 / end_lengths[2] as f64;
    if (end_ratio < 0.6) || (end_ratio > 1.4) {
        panic!("Error reading end guard, invalid barcode")
    }

    let mut lengths = start_lengths;
    lengths.extend(end_lengths);

    calculate_average_size(lengths)
}

fn calculate_average_size(lengths: Vec<usize>) -> i32 {
    let mut sum = 0;

    for i in 0..lengths.len() {
        sum += lengths[i];
    }

    (sum as f32 / lengths.len() as f32).round() as i32 - 1
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
