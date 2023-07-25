// Author Name: Mohammed Hashem Alsayegh

use std::fs::File;
use std::io::{BufReader, Read};
use std::io::{BufWriter, Write};

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
pub fn parse_out_bits(bytes_group: &str, n: usize) -> Vec<u32> {
    let mut parsed_bits = Vec::new();
    let mut i = 0;

    // For each `n` bytes in the bytes group, add the corresponding bits to `parsed_bits`.
    while i < bytes_group.len() {
        let parsed_bit_str = u32::from_str_radix(&(bytes_group[i..i + n].to_string()), 2).unwrap();
        parsed_bits.push(parsed_bit_str);
        i += n;
    }

    parsed_bits
}

// READ A FILE
pub fn read_file_to_u32(file_in_name: String) -> (Vec::<u32>, usize)
{
    // Open the file for reading
    let mut file = BufReader::new(File::open(file_in_name).unwrap());

    // Create a byte vec to allocated read data
    let mut bytes = Vec::new();

    file.read_to_end(&mut bytes).unwrap();

    // Create U32 vec to allocated the grouped data
    let mut _u32_array = Vec::<u32>::new(); 

    // loop to allocated bytes in group of four in U32 vec
    let mut i = 0;
    while (i+4)<bytes.len() {
        let byte_group = [bytes[i+3], bytes[i+2], bytes[i+1], bytes[i]];
        let slice: &[u8] = &byte_group;
        let u32_group = u32::from_le_bytes(slice.try_into().unwrap());
        _u32_array.push(u32_group);
        i+=4;
    }

    // we calulated how many last byte in the last buffer
    let lefts = bytes.len() - i;
    
    // put teh last byte with padding zero in U32 vec
    if (i+4)>bytes.len()-1 {
        let mut _u8_array = Vec::<u8>::new();

        for _o in 0..4-lefts {
            _u8_array.push(0b0);
        }
        
        for left in (0..lefts).step_by(1) {
            _u8_array.push(bytes[bytes.len()-left-1]);
            i+=1;
        }

        let slice: &[u8] = &_u8_array;
        let u32_group = u32::from_le_bytes(slice.try_into().unwrap());
        _u32_array.push(u32_group);
    }

    (_u32_array, lefts)
}

// Write a file from Vec::<u32>
pub fn write_file_u32(file_out_name: String, _u32_array: Vec::<u32>, lefts: usize)
{
    // Create a file for write
    let mut _file = BufWriter::new(File::create(file_out_name).unwrap());


    for i in 0.._u32_array.len() {
        // divide the u32 into bytes to pass to file write
        let u8_array = _u32_array[i].to_be_bytes();
        
        // dedect the last u32 and check how many byte that needed writen
        if i==_u32_array.len()-1 {

            let mut u8_array_last = Vec::<u8>::new();
            let mut lefts_check = lefts;

            if lefts == 0 {lefts_check = 4;}

            for i in 0..lefts_check {
                u8_array_last.push(u8_array[i]);
            }

            _file.write(&u8_array_last).unwrap();
            break;
        }

        // write some data to the file
        _file.write(&u8_array).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use crate::{parse_buffer, parse_out_bits, print_bytes_in_groups};
    use crate::{read_file_to_u32, write_file_u32};

    // Constants for parsing and buffer size
    const SIZE_BUFFER: u8 = 8;
    const SIZE_PARSING: u8 = 3;

    // to test the bit parsing part of library: cargo test test_parse_buffer -- --nocapture
    #[test]
    fn test_parse_buffer() {
        println!("This part for parsing in 3 bits into a byte buffer:\n");
        let arr_n: [u32; 13] = [
            0b101, 0b001, 0b111, 0b001, 0b111, 0b000, 0b001, 0b010, 0b001, 0b101, 0b111,
            0b001, 0b110,
        ];

        println!("arr_n is: {:?}", arr_n);

        let (values, is_incomplete, length_occupied) =
            parse_buffer(&arr_n, SIZE_BUFFER, SIZE_PARSING);
        println!("Values: {:?}", values);
        println!("Is incomplete: {}", is_incomplete);
        println!(
            "Length occupied: {}",
            SIZE_PARSING as usize - length_occupied
        );

        for value in &values {
            // println!("{:08b}", value);
            println!("value is: {:0width$b}", value, width = SIZE_BUFFER as usize);
        }

        println!(
            "\nThis part for parsing out {} bits out of a group of byte buffer:\n",
            SIZE_PARSING
        );

        // Print the bytes in groups of `n` bytes.
        let bytes_group_list =
            print_bytes_in_groups(&values, SIZE_PARSING.into(), SIZE_BUFFER.into());

        println!("{:?}", bytes_group_list);

        // break point referance
        let mut counter_len = arr_n.len();

        // For each bytes group, print the bits and the parsed bits.
        for bytes_group in &bytes_group_list {
            println!("\nbytes_group: {}", bytes_group);
            let parsed_bits = parse_out_bits(&bytes_group, SIZE_PARSING.into());
            for parsed_bit in parsed_bits {
                println!("{}", parsed_bit);
                println!("{:0width$b}", parsed_bit, width = SIZE_PARSING as usize);

                counter_len -= 1;
                if 0 == counter_len {
                    break;
                }
            }
        }
    }

    // to test read/write as u32 part of library: cargo test test_read_write_u32 -- .\beep.jpg --nocapture
    #[test]
    fn test_read_write_u32()
    {
        // Get the file name from the command line arguments.
        let mut file_name : String = std::env::args().nth(2).unwrap().parse().unwrap();
        println!("{:?}", file_name);
        
        println!("Now it will read ...");
        let (_u32_array, lefts) = read_file_to_u32(file_name.clone());

        
        // Extract string suffix for file extension 
        let file_extension = file_name[file_name.len() - 4..].to_string();

        // delete the last four char from file name
        for _i in 0..4 {
            file_name.pop();
        }

        // create new name for the output file with _out in it
        let file_name = format!("{}{}{}", file_name, "_out", file_extension);
        
        // printout the new name
        println!("{:?}", file_name);

        // write th new file using the new name
        println!("Now it will write ...");
        write_file_u32(file_name, _u32_array, lefts);
    }
}