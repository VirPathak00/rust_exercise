use crate::hash_calculation::*;
//to create the signature, we follow the rsync algorithm.
//The a, b, and rolling_checksum functions are defined as in the rsync paper.
pub fn a_function(bytes: &Vec<u8>) -> i64 {
    let mut sum = 0;
    for byte in bytes {
        sum += *byte as i64 % crate::MODULUS;
    }
    sum
}

pub fn b_function(bytes: &Vec<u8>, start: usize, end: usize) -> i64 {
    let mut res = 0;
    for i in 0..bytes.len() {
        res += (bytes[i] as i64 * (end - (i + start) + 1) as i64) % crate::MODULUS;
    }

    res
}

pub fn rolling_checksum(bytes: &Vec<u8>, start: usize, end: usize) -> i64 {
    let a_part = a_function(bytes);
    let b_part = 2_i64.pow(16) * b_function(bytes, start, end);

    a_part + b_part
}

//receives file converted to a Vec of u8 and chunk size. Outputs its rsync signature.
pub fn compute_signature(s: &Vec<u8>, block_length: usize) -> Vec<(i64, u64)> {
    let mut signature: Vec<(i64, u64)> = Vec::new();

    let remainder = s.len() % block_length;

    let mut start = 0;

    for i in (block_length..s.len() - remainder + 1).step_by(block_length) {
        signature.push((
            rolling_checksum(&s[start..i].to_vec(), start, i - 1),
            calculate_hash(&s[start..i].to_vec()),
        ));
        start += block_length;
    }

    signature.push((
        rolling_checksum(
            &s[s.len() - remainder..s.len()].to_vec(),
            s.len() - remainder,
            s.len(),
        ),
        calculate_hash(&s[s.len() - remainder..s.len()].to_vec()),
    ));
    signature
}
