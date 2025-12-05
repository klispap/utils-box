//! # Bits utilities
//! A toolbox of small utilities that manipulate bitstream data.
//! Useful for debugging streams from embedded devices.

/// Convert bits stream to vector (MSB is Vec[0])
pub fn bits_to_vec(bits: u64, out_len: u8) -> Vec<u8> {
    let mut in_bits = bits;
    let mut output: Vec<u8> = vec![];

    let mut idx = 0;
    while in_bits > 0 && idx < out_len {
        output.push((in_bits & 0x01) as u8);
        in_bits >>= 1;
        idx += 1;
    }

    for _i in idx..out_len {
        output.push(0);
    }

    output.into_iter().rev().collect()
}

/// Convert vector to bits stream (Vec[0] is MSB)
pub fn bit_vec_to_byte_vec(vec: &[u8]) -> Vec<u8> {
    let mod_vec = vec.len() % 8;
    let mut vec = vec.to_owned();

    let mut res: Vec<u8> = vec![];

    if mod_vec != 0 {
        vec.append(&mut vec![0; mod_vec]);
    }

    for i in (0..vec.len()).step_by(8) {
        let byte = vec[i..i + 8].iter().fold(0b0, |acc, &x| (acc << 1) | x);

        res.push(byte);
    }

    res
}

pub fn vec_to_bits(vec: &[u8]) -> u64 {
    vec.iter().fold(0b0, |acc, &x| (acc << 1) | x as u64)
}

pub fn bit_vec_to_hex_string(vec: &[u8]) -> String {
    let mut hex_string = String::new();

    let mod_vec = vec.len() % 4;
    let mut vec = vec.to_owned();

    if mod_vec != 0 {
        vec.append(&mut vec![0; mod_vec]);
    }

    for i in (0..vec.len()).step_by(4) {
        let bits = vec[i..i + 4].iter().fold(0b0, |acc, &x| (acc << 1) | x);

        let char = match bits {
            0..=9 => bits + 48,
            10..=15 => bits + 55,
            _ => unreachable!(),
        } as char;

        hex_string.push(char);
    }

    hex_string
}

#[cfg(test)]
mod tests {
    use crate::bits::*;

    #[test]
    fn bits_to_vec_test() {
        let received_bit_stream: u64 = 0b110101000100111010110;
        let expected_bytes_array: Vec<u8> = vec![
            1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0,
        ];

        let bytes = bits_to_vec(received_bit_stream, 21);

        assert_eq!(expected_bytes_array, bytes);
    }

    #[test]
    fn bit_vec_to_hex_string_test() {
        let bit_array: Vec<u8> = vec![
            1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0,
        ];
        let expected_string = "D44EB0".to_string();

        let string = bit_vec_to_hex_string(&bit_array);

        assert_eq!(expected_string, string);
    }
}
