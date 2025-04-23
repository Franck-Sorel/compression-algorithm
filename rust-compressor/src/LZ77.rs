#[derive(Debug)]
struct Token {
    offset: usize,
    length: usize,
    next_char: char,
}

fn compress(input: &[u8], window_size: usize) -> Vec<Token> {
    // let chars: Vec<char> = input.chars().collect();
    let mut result: Vec<Token> = Vec::new();
    let chars = input;
    let mut pos = 0;

    while pos < chars.len() {
        let mut match_offset = 0;
        let mut match_length = 0;

        // Set the window boundaries
        let window_start = if pos >= window_size {
            pos - window_size
        } else {
            0
        };

        // Try to find the longest match in the window
        for i in window_start..pos {
            let mut length = 0;

            while pos + length < chars.len() && chars[i + length] == chars[pos + length] {
                length += 1;
                if i + length >= pos {
                    break; // Avoid overlapping the current position
                }
            }

            if length > match_length {
                match_offset = pos - i;
                match_length = length;
            }
        }

        let next_char = if pos + match_length < chars.len() {
            chars[pos + match_length]
        } else {
            '\0' // null char to indicate end of input
        };

        result.push(Token {
            offset: match_offset,
            length: match_length,
            next_char,
        });

        pos += match_length + 1;
    }

    result
}

fn decompress(tokens: &[Token]) -> String {
    let mut output = String::new();

    for token in tokens {
        if token.offset == 0 && token.length == 0 {
            output.push(token.next_char);
        } else {
            let start = output.len() - token.offset;
            let end = start + token.length;
            let slice: String = output.chars().skip(start).take(token.length).collect();
            output.push_str(&slice);
            if token.next_char != '\0' {
                output.push(token.next_char);
            }
        }
    }

    output
}

fn main() {
    let input = "AAAAAABBBBAAAAABBA";
    let window_size = 8;

    println!("Original input: {}", input);

    let compressed = compress(input, window_size);
    println!("Compressed tokens: {:?}", compressed);

    let decompressed = decompress(&compressed);
    println!("Decompressed: {}", decompressed);
}
// const X: u8 = 4;

// pub(crate) fn compress(input: &[u8]) {
//     let mut search_buffer = Vec::new();
//     let mut compare: u8;
//     let mut iter = input.iter().peekable();
//     search_buffer.push((0, 0, input[0]));
//     while let Some(&current) = iter.next() {
//         let count = 0;
//         while let Some(&&next) = iter.peek() {
//             if current == next {

//                 compare = **iter.peek().unwrap();
//                 // for element in &search_buffer {
//                 if next != compare && count != 0 {
//                     let new = (count, count, compare);
//                     search_buffer.push(new);
//                     break;
//                 } else if next == compare {

//                 }
//                 // }
//             } else if current != next {
//                 search_buffer.push((0, 0, next));
//             }
//         }
//         println!("{X}")
//     }
// }
