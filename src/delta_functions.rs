use crate::hash_calculation::*;
use crate::sig_functions::*;
use crate::MODULUS;

//this function receives a signature, a part of the delta file, and start/end indices in order to compute the underlying a/b functions correctly
//as specified in the rsync algorithm
pub fn check_block_equality(
    sig: &Vec<(i64, u64)>,
    new_file_part: &Vec<u8>,
    index_start: usize,
    index_end: usize,
) -> (bool, usize) {
    for i in 0..sig.len() {
        if rolling_checksum(new_file_part, index_start, index_end) == sig[i].0
            && calculate_hash(new_file_part) == sig[i].1
        {
            return (true, i);
        }
    }
    return (false, 0);
}

//this function takes a file signature, a new delta file, and a block length. It calculates the corresponding delta vector.
//The delta vector is a vector of the form Vec<(usize, Vec<usize>)>. The first element in each tuple is a position. The second element is a vec with
//elements you need to insert in the original file in order to change the original file into the delta file. If what needs to be inserted matches with a block
//in the original file signature, the second element in the tuple will just be a vec with a single element. This element i is the ith block of block_length bytes in the original file.
pub fn compute_deltas(
    sig: &Vec<(i64, u64)>,
    new_file: &Vec<u8>,
    block_length: usize,
) -> Vec<(usize, Vec<usize>)> {
    let mut deltas: Vec<(usize, Vec<usize>)> = Vec::new();
    let mut last_equal_position: usize = 0;
    let mut i = 0;
    let mut matches_some_block = false;
    let mut a_value = a_function(&new_file[i..i + block_length].to_vec());
    let mut b_value = b_function(
        &new_file[i..i + block_length].to_vec(),
        i,
        i + block_length - 1,
    );

    let mut x = check_block_equality(
        sig,
        &new_file[i..i + block_length].to_vec(),
        i,
        i + block_length - 1,
    );
    if x.0 {
        matches_some_block = true;
        let mut block_vec = Vec::new();
        for ell in last_equal_position..i {
            block_vec.push(new_file[ell] as usize);
        }

        deltas.push((i, vec![x.1]));
        last_equal_position = i + block_length;
        i += block_length;
    } else {
        i += 1;
    }

    while i < new_file.len() {
        if i + block_length < new_file.len() && matches_some_block {
            x = check_block_equality(
                sig,
                &new_file[i..i + block_length].to_vec(),
                i,
                i + block_length - 1,
            );
            if x.0 {
                matches_some_block = true;
                let mut block_vec = Vec::new();

                for ell in last_equal_position..i {
                    block_vec.push(new_file[ell] as usize);
                }

                deltas.push((i, vec![x.1]));
                a_value = a_function(&new_file[i..i + block_length].to_vec());
                b_value = b_function(
                    &new_file[i..i + block_length].to_vec(),
                    i,
                    i + block_length - 1,
                );

                last_equal_position = i + block_length;
                i += block_length;
            } else {
                a_value = a_function(&new_file[i..i + block_length].to_vec());
                b_value = b_function(
                    &new_file[i..i + block_length].to_vec(),
                    i,
                    i + block_length - 1,
                );
                matches_some_block = false;
            }
        } else if i + block_length < new_file.len() && !matches_some_block {
            //make use of the rolling checksum property described in the rsync algorithm
            a_value = (a_value - new_file[i - 1] as i64 + new_file[i + block_length - 1] as i64)
                % MODULUS;

            b_value =
                (b_value - block_length as i64 * (new_file[i - 1] as i64) + a_value) % MODULUS;
            let rolling_checksum = a_value + 2_i64.pow(16) * b_value;

            for j in 0..sig.len() {
                if rolling_checksum == sig[j].0
                    && calculate_hash(&new_file[i..i + block_length]) == sig[j].1
                {
                    matches_some_block = true;
                    let mut block_vec = Vec::new();

                    for ell in last_equal_position..i {
                        block_vec.push(new_file[ell] as usize);
                    }
                    deltas.push((last_equal_position, block_vec));
                    deltas.push((i, vec![j]));
                    last_equal_position = i + block_length;
                    i += block_length;
                }
            }
        } else {
            x = check_block_equality(
                sig,
                &new_file[last_equal_position..].to_vec(),
                last_equal_position,
                new_file.len(),
            );
            if x.0 {
                deltas.push((i, vec![x.1]));
            } else {
                let mut block_vec = Vec::new();
                for ell in last_equal_position..new_file.len() {
                    block_vec.push(new_file[ell] as usize);
                }
                deltas.push((last_equal_position, block_vec));
            }

            break;
        }

        if !matches_some_block {
            i += 1;
        }
    }
    deltas
}
