pub fn vec_to_number(binary_number: &[bool]) -> i32 {
    binary_number
        .iter()
        .rev()
        .enumerate()
        .map(|(pos, digit)| 2_i32.pow(pos as u32) * *digit as i32)
        .sum()
}
