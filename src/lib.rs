// Author Name: Mohammed Hashem Alsayegh

pub fn parse_buffer(arr_n: &[u32], size_buffer: u8, size_parsing: u8) -> (Vec<u32>, bool, usize) {
    // Length of the input data
    let len = arr_n.len();

    // Buffer-related variables
    let mut buf: u32 = 0;
    let mut bitwise_location: u8 = size_buffer;
    let mut _bit_left: u8 = 0;
    let mut _bit_fill_left: u32 = 0;
    let mut buf_cycle: u8 = 0;
    let mut buf_full = false;

    // Variable to store the last value of `i` outside the loop
    let mut last_i = 0;

    // Iterate through each byte of the input array
    let mut values: Vec<u32> = Vec::new();
    let mut is_incomplete = false;
    for i in 0..len {
        // If the buffer is not full and buf_cycle is less than size_parsing,
        // update the buffer with the current byte
        if buf_cycle < size_parsing && buf_full {
            buf |= arr_n[i - 1] << bitwise_location;
        }

        // If the remaining space in the buffer is less than size_parsing,
        // update the buffer and related variables
        if bitwise_location < size_parsing {
            buf_full = true;
            buf_cycle += 1;
            _bit_left = bitwise_location;
            _bit_fill_left = arr_n[i] >> (size_parsing - _bit_left);
            buf |= _bit_fill_left;

            let mask = 0xFFFFFFFF >> (32 - size_buffer);
            buf &= mask;

            // println!("Value: {}", buf);
            values.push(buf);

            // Reset the buffer and update the bitwise_location
            buf = 0;
            bitwise_location = size_buffer - (size_parsing - _bit_left);

            // Reset buf_cycle if it's a multiple of size_parsing
            if buf_cycle % size_parsing == 0 {
                buf_cycle = 0;
            }
        } else {
            // Update the buffer and bitwise_location
            bitwise_location -= size_parsing;
            buf |= arr_n[i] << bitwise_location;
            buf_full = false;
        }

        // Store the last value of `i`
        last_i = i;
    }

    // If there are any remaining bits in the last byte, shift them into the buffer
    if buf_cycle % size_parsing != 0 {
        let i = last_i;
        buf |= arr_n[i] << bitwise_location;

        let mask = 0xFFFFFFFF >> (32 - size_buffer);
        buf &= mask;

        // println!("last Value: {}", buf);
        values.push(buf);
        is_incomplete = true;
    }

    (values, is_incomplete, bitwise_location.into())
}

// This function prints the bytes in groups of `n` bytes.
pub fn print_bytes_in_groups(bytes: &[u32], n: usize, buf_size: usize) -> Vec<String> {
    // Declare variables.
    let mut i = 0;
    let mut _null_padding = String::new(); // The null padding string.
    let mut joined_bytes = String::new(); // The joined bytes string.
    let null_end_buffer = n - (bytes.len() % n); // The number of null bytes at the end of the buffer.

    // Declare `bytes_group_list` as a `Vec<String>`.
    let mut bytes_group_list = Vec::new();

    // If there are null bytes at the end of the buffer, create a null padding string.
    if null_end_buffer != n {
        _null_padding = "0".repeat((n - (bytes.len() % n)) * buf_size);
    }

    // Iterate over the bytes and join them into groups of `n` bytes.
    while i < bytes.len() {
        joined_bytes.clear();

        // For each byte in the group, convert it to a binary string and append it to `joined_bytes`.
        for k in 0..n {
            if i + k < bytes.len() {
                let string = format!("{:0width$b}", &bytes[i + k], width = buf_size);
                let string = string.as_str();
                joined_bytes.push_str(&string);
            }
        }

        // Add `bytes_group_list` to the list of groups.
        i += n;
        if i < bytes.len() {
            bytes_group_list.push(joined_bytes.clone());
        } else {
            joined_bytes += &_null_padding;
            bytes_group_list.push(joined_bytes.clone());
        }
    }

    bytes_group_list
}

// This function parses the bits out of a bytes group.
pub fn parse_out_bits(bytes_group: &str, n: usize) -> Vec<String> {
    let mut parsed_bits = Vec::new();
    let mut i = 0;

    // For each `n` bytes in the bytes group, add the corresponding bits to `parsed_bits`.
    while i < bytes_group.len() {
        parsed_bits.push(bytes_group[i..i + n].to_string());
        i += n;
    }

    parsed_bits
}

// to test the library: `cargo test -- --nocapture`
#[cfg(test)]
mod tests {
    use crate::{parse_buffer, parse_out_bits, print_bytes_in_groups};

    // Constants for parsing and buffer size
    const SIZE_BUFFER: u8 = 8;
    const SIZE_PARSING: u8 = 4;

    #[test]
    fn test_parse_buffer() {
        println!("This part for parsing in 3 bits into a byte buffer:\n");
        let arr_n: [u32; 13] = [
            0b0101, 0b1001, 0b0111, 0b0001, 0b0111, 0b0000, 0b1001, 0b0010, 0b1001, 0b0101, 0b0111,
            0b01001, 0b1110,
        ];

        println!("arr_n is: {:?}", arr_n);

        let (values, is_incomplete, length_occupied) =
            parse_buffer(&arr_n, SIZE_BUFFER, SIZE_PARSING);
        println!("Values: {:?}", values);
        println!("Is incomplete: {}", is_incomplete);
        println!("Length occupied: {}",SIZE_PARSING as usize - length_occupied);

        for value in &values {
            // println!("{:08b}", value);
            println!("value is: {:0width$b}", value, width = SIZE_BUFFER as usize);
        }

        println!("\nThis part for parsing out {} bits out of a group of byte buffer:\n",SIZE_PARSING);

        // Print the bytes in groups of `n` bytes.
        let bytes_group_list = print_bytes_in_groups(&values, SIZE_PARSING.into(), SIZE_BUFFER.into());

        println!("{:?}", bytes_group_list);


        // break point referance
        let mut counter_len = arr_n.len();

        // For each bytes group, print the bits and the parsed bits.
        for bytes_group in &bytes_group_list {
            println!("\nbytes_group: {}", bytes_group);
            let parsed_bits = parse_out_bits(&bytes_group, SIZE_PARSING.into());
            for parsed_bit in parsed_bits {
                println!("{}", parsed_bit);

                counter_len -= 1;
                if 0 == counter_len {
                    break;
                }
            }
        }
    }
}
