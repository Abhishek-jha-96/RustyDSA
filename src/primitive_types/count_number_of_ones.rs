// WAP to count number of bits set to 1 in a integer.
pub fn number_of_ones(mut num: i32) -> i32 {
    let mut count = 0;
    while num != 0 {
        if num & 1 == 1 {
            count += 1;
        }
        num = num >> 1;
    }
    return count;
}