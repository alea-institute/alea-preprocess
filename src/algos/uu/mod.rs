use std::error::Error;

const UU_CHUNK_SIZE: usize = 45;

/// UUencode an input buffer and return the encoded string.
///
/// # Arguments
///
/// * `input_buffer` - The input buffer to uuencode.
/// * `name` - The name to use in the uuencode header.
/// * `mode` - The mode to use in the uuencode header.
///
/// # Returns
///
/// A `String` containing the uuencoded data.
pub fn encode_buffer(input_buffer: &[u8], name: &str, mode: u32) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();

    // Write the header
    output.push_str(&format!("begin {} {}\n", mode, name));

    for chunk in input_buffer.chunks(UU_CHUNK_SIZE) {
        let encoded_line = uuencode_line(chunk);
        output.push_str(&encoded_line);
    }

    // Write the end
    output.push_str("`\nend\n");

    Ok(output)
}

fn uuencode_line(chunk: &[u8]) -> String {
    let mut line = String::new();

    // Length character
    let length_char = ((chunk.len() + 32) & 0x7F) as u8 as char;
    line.push(length_char);

    for i in (0..chunk.len()).step_by(3) {
        let a = *chunk.get(i).unwrap_or(&0);
        let b = *chunk.get(i + 1).unwrap_or(&0);
        let c = *chunk.get(i + 2).unwrap_or(&0);

        let c1 = ((a >> 2) & 0x3F) + 32;
        let c2 = (((a << 4) | (b >> 4)) & 0x3F) + 32;
        let c3 = (((b << 2) | (c >> 6)) & 0x3F) + 32;
        let c4 = (c & 0x3F) + 32;

        line.push(c1 as char);
        line.push(c2 as char);
        line.push(c3 as char);
        line.push(c4 as char);
    }

    line.push('\n');

    line
}

/// Uudecode an input string and return the name and decoded bytes.
///
/// # Arguments
///
/// * `input` - The input string to uudecode.
///
/// # Returns
///
/// A tuple containing the filename and a `Vec<u8>` of the decoded bytes.
pub fn decode_buffer(input: &str) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    let mut lines = input.lines();

    // Read the header
    let header = lines.next().ok_or("Invalid uuencoded input: empty input")?;
    if !header.starts_with("begin ") {
        return Err("Invalid uuencoded input: missing 'begin'".into());
    }

    let parts: Vec<&str> = header.split_whitespace().collect();
    if parts.len() < 3 {
        return Err("Invalid uuencoded input: malformed header".into());
    }

    let name = parts[2].to_string();

    let mut data = Vec::new();

    for line in lines {
        if line.trim() == "end" {
            break;
        }

        // Decode the line
        let decoded_line = uudecode_line(line)?;
        data.extend_from_slice(&decoded_line);
    }

    Ok((name, data))
}

fn uudecode_line(line: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let line = line.trim_end();

    if line.is_empty() {
        return Ok(Vec::new());
    }

    let length_char = line.chars().next().unwrap();
    let length = (length_char as u8 - 32) & 0x3F;

    if length == 0 {
        return Ok(Vec::new());
    }

    let encoded_chars: Vec<u8> = line[1..].bytes().collect();
    let mut result = Vec::new();

    let mut i = 0;
    while i + 3 < encoded_chars.len() {
        let c1 = encoded_chars[i] - 32 & 0x3F;
        let c2 = encoded_chars[i + 1] - 32 & 0x3F;
        let c3 = encoded_chars[i + 2] - 32 & 0x3F;
        let c4 = encoded_chars[i + 3] - 32 & 0x3F;

        let a = (c1 << 2) | (c2 >> 4);
        let b = (c2 << 4) | (c3 >> 2);
        let c = (c3 << 6) | c4;

        result.push(a);
        if result.len() < length as usize {
            result.push(b);
        }
        if result.len() < length as usize {
            result.push(c);
        }

        i += 4;
    }

    result.truncate(length as usize);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuencode_uudecode() -> Result<(), Box<dyn Error>> {
        let input_data = b"Hello, world! This is a test of uuencoding and uudecoding in Rust.";
        let name = "file.txt";
        let mode = 0o666;

        // Encode
        let encoded = encode_buffer(input_data, name, mode)?;

        // Decode
        let (decoded_name, decoded_data) = decode_buffer(&encoded)?;

        assert_eq!(name, decoded_name);
        assert_eq!(input_data.to_vec(), decoded_data);

        Ok(())
    }
}
