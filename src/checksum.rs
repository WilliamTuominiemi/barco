pub fn get_checksum(barcode: &Vec<u8>) -> u8 {
    let mut checksum: Vec<u8> = vec![];

    for i in 0..barcode.len() {
        let position = 12 - i;
        let weight = get_weight(position);
        let partial_sum = barcode[i] * weight;
        checksum.push(partial_sum);
    }

    checksum.iter().sum()
}

fn get_weight(position: usize) -> u8 {
    match position {
        12 => 1,
        11 => 3,
        10 => 1,
        9 => 3,
        8 => 1,
        7 => 3,
        6 => 1,
        5 => 3,
        4 => 1,
        3 => 3,
        2 => 1,
        1 => 3,
        _ => panic!("Invalid position {} detected getting weight", position),
    }
}

#[cfg(test)]
mod tests {
    use crate::checksum::{get_checksum, get_weight};

    #[test]
    fn test_get_weight() {
        assert_eq!(get_weight(12), 1);
        assert_eq!(get_weight(7), 3);
        assert_eq!(get_weight(2), 1);
        assert_eq!(get_weight(1), 3);
    }

    #[test]
    fn test_get_checksum() {
        let barcode = vec![4, 0, 0, 6, 3, 8, 1, 3, 3, 3, 9, 3];

        assert_eq!(get_checksum(&barcode), 89);
    }
}