// Author Name: Mohammed Hashem Alsayegh

pub fn parse_buffer(arr_n: &[u8], size_buffer: u8, size_parsing: u8) -> (Vec<u8>, bool, usize) {
    // Length of the input data
    let len = arr_n.len();

    // Buffer-related variables
    let mut buf: u8 = 0;
    let mut bitwise_location: u8 = size_buffer;
    let mut _bit_left: u8 = 0;
    let mut _bit_fill_left: u8 = 0;
    let mut buf_cycle: u8 = 0;
    let mut buf_full = false;

    // Variable to store the last value of `i` outside the loop
    let mut last_i = 0;

    // Iterate through each byte of the input array
    let mut values: Vec<u8> = Vec::new();
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

            //println!("Value: {}", buf);
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
        //println!("Value: {}", buf);
        values.push(buf);
        is_incomplete = true;
    }

    (values, is_incomplete, bitwise_location.into())
}

// This function prints the bytes in groups of `n` bytes.
pub fn print_bytes_in_groups(bytes: &[u8], n: usize, buf_size: usize) -> Vec<String> {
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
                joined_bytes.push_str(format!("{:08b}", &bytes[i + k]).as_str());
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