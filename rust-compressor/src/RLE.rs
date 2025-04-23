use std::str::Bytes;

pub fn compress(input: Vec<Bytes>) -> Vec<(u8, Vec<u8>)> {
    let mut compress_data: Vec<(u8, Vec<u8>)> = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let current: Vec<u8> = input[i].clone().collect();
        let mut count = 1;

        while i + count < input.len() && current == input[i + count].clone().collect::<Vec<u8>>() {
            count += 1;
        }

        compress_data.push((count as u8, current));
        i += count;
    }

    compress_data
}
