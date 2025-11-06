mod reader;
use reader::read_image;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let barcode = read_image("barcode.png")?;
    let line_width = read_start_and_end_guard(&barcode);
    let binary = read_binary(&barcode, line_width);

    let left = left_part(&binary);

    let mut digits: Vec<u8> = vec![];
    for code in left {
        let digit = get_digit_from_l_code(code);
        match digit {
            Some(d) => digits.push(d),
            _ => println!("Error reading digit"),
        }
    }

    println!("{:?}", digits);

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

fn left_part(binary: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut numbers: Vec<Vec<u8>> = vec![];
    let mut current: Vec<u8> = vec![]; // 7 bits long

    let size = binary.len();

    for i in 3..size / 2 {
        current.push(binary[i]);

        if current.len() >= 7 {
            numbers.push(current);
            current = vec![];
        }
    }

    println!("{:?}", numbers);

    numbers
}

fn calculate_average_size(lengths: Vec<usize>) -> i32 {
    let mut sum = 0;

    for i in 0..lengths.len() {
        sum += lengths[i];
    }

    (sum as f32 / lengths.len() as f32).round() as i32 - 1
}

fn get_digit_from_l_code(code: Vec<u8>) -> Option<u8> {
    let s: String = code
        .iter()
        .map(|&b| match b {
            0 => '0',
            1 => '1',
            _ => panic!("Invalid binary digit: {}", b),
        })
        .collect();

    let digit: Option<u8> = match s.as_str() {
        "0001101" => Some(0),
        "0011001" => Some(1),
        "0010011" => Some(2),
        "0111101" => Some(3),
        "0100011" => Some(4),
        "0110001" => Some(5),
        "0101111" => Some(6),
        "0111011" => Some(7),
        "0110111" => Some(8),
        "0001011" => Some(9),
        _ => None,
    };

    return digit;
}
