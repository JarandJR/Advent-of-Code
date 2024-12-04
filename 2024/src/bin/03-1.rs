use aoc2024::parse_into_byte_lines;

fn main() {
    dbg!(parse_and_solve("03"));
}

fn count_status_bits(status: &u128) -> usize {
    let mut count = 0;
    // Starts at most significant bit
    for i in (0..6).rev() {
        if (status & (1 << i)) == 0 {
            // Stop counting when a 0 is encountered
            return count;
        }
        count += 1;
    }
    count
}

fn parse_and_solve(day: &str) -> usize {
    if let Some(line_iter) = parse_into_byte_lines(day) {
        let expected_byte_order = [b'm', b'u', b'l', b'(', b',', b')'];
        return line_iter.fold(0, |acc, line| {
            acc + (line.iter().fold(0_u128, |acc, b| {
                /*
                Acc
                0..=10 = num1
                10..=20 = num2
                26 = m
                25 = u
                24 = l
                23 = (
                22 = ,
                21 = )
                27..122 = result
                */
                // Extracts the status bits
                let status = (acc >> 20) & ((1 << 6) - 1);
                let byte_order_idx = count_status_bits(&status);
                // Expects 'mul('
                if byte_order_idx < 4 {
                    if expected_byte_order[byte_order_idx] == *b {
                        // Set the status bit for byte
                        acc | (1 << (25 - byte_order_idx))
                    } else {
                        // Missmatch, resetting
                        acc & (!0 << 26)
                    }
                // Ascii number
                } else if 48 <= *b && *b <= 57 {
                    let digit = (b - 48) as u128;
                    let (num, shift) = if byte_order_idx == 4 {
                        // Num 1
                        (acc & ((1 << 10) - 1), 0)
                    } else {
                        // Num 2
                        ((acc >> 10) & ((1 << 10) - 1), 10)
                    };
                    if num > 0 {
                        // Zero out bits before setting the new number
                        let acc = acc & !(0b1111111111 << shift);
                        acc | ((num * 10 + digit) << shift)
                    } else {
                        // Setting digit
                        acc | (digit << shift)
                    }
                // Either ',' to go to number 2
                // or ')' to finish off the sequence
                } else if *b == expected_byte_order[byte_order_idx] {
                    if byte_order_idx == 5 {
                        let num1 = acc & ((1 << 10) - 1);
                        let num2 = (acc >> 10) & ((1 << 10) - 1);
                        let result = (acc >> 26) + (num1 * num2);
                        let acc_zero_result = acc & ((1u128 << 26) - 1);
                        (acc_zero_result | (result << 26)) & (!0 << 26)
                    } else {
                        // Set the status bit
                        acc | (1 << (25 - (byte_order_idx)))
                    }
                } else {
                    // Missmatch, resetting
                    acc & (!0 << 26)
                }
            }) >> 26) as usize
        });
    }
    0
}

#[test]
fn day03_1() {
    use std::fs::{remove_file, File};
    let file_name = "test_03_1";
    let file_path = format!("inputs/{}.txt", file_name);
    {
        // Setup for test
        use std::io::Write;
        let mut file = File::create(&file_path).expect("Could not create file");
        writeln!(
            file,
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        )
        .expect("Could not write to file");
    }
    let result = parse_and_solve(&file_name);
    // Clean up
    remove_file(file_path).expect("Could not remove file");
    assert_eq!(result, 161);
}
