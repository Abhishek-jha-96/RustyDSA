pub fn bitwise_and() {
    let a = 4;
    let b = 8;
    println!("bitwise AND operator: {}", a & b);
}

pub fn bitwise_or() {
    let a = 4;
    let b = 8;
    println!("bitwise OR operator: {}", a | b);
}

pub fn bitwise_xor() {
    let a = 4;
    let b = 8;
    println!("bitwise XOR operator: {}", a ^ b);
}

pub fn bitwise_left_shift() {
    let a = 10;
    let b = 2;
    println!("bitwise left shift operator: {}", a << b);
}

pub fn bitwise_right_shift() {
    let a = 8;
    let b = 2;
    println!("bitwise right shift operator: {}", a >> b);
}

pub fn power_of_two(n: u32) {
    // A power of 2 has exactly one bit set in its binary representation
    // Subtracting 1 from a power of 2 flips all the bits after the single set bit to 1, including the set bit itself.
    // Performing a bitwise AND (&) between 𝑛 and 𝑛−1 results in 0 if n is a power of 2.
    let res = n & (n - 1) == 0;
    println!("{} is a power of two: {}", n, res);
}

pub fn power_of_two_v2(n: u32) -> bool {
    // Repeatedly shift the number to the right by one bit.
    // Count the number of 1 bits in the process.
    // If there is exactly one 1 bit, the number is a power of 2.
    if n == 0 {
        return false;
    }
    let mut count = 0;
    let mut num = n;
    while num > 0 {
        count += num & 1;
        num >>= 1;
    }

    count == 1

}

pub fn reverse_integer(mut n: u32) {
    let mut rev_digit = 0;
    while n > 0 {
       let digit = n % 10;
        rev_digit = rev_digit * 10 + digit;
        n = n / 10;
    }
    println!("Reverse is {}", rev_digit);
}