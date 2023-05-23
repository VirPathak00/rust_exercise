#[cfg(test)]
use super::*;
//in this first test, the first three blocks match. Then we do not get a match until position 43 in the byte vector. This matches with the 4th block of
//the test.txt file's signature
#[test]
fn test1() {
    let data1 = read("../test.txt").unwrap();
    let data2 = read("../delta_text.txt").unwrap();
    let x = compute_signature(&data1, 10);
    let q = compute_deltas(&x, &data2, 10);
    assert_eq!(
        q,
        vec![
            (0, vec![0]),
            (10, vec![1]),
            (20, vec![2]),
            (
                30,
                vec![115, 10, 105, 115, 10, 110, 111, 116, 10, 116, 104, 101, 10]
            ),
            (43, vec![4]),
            (53, vec![5])
        ]
    );
}

//in this test the files are the exact same, so the delta vector should only have matches.
#[test]
fn test2() {
    let data1 = read("../test.txt").unwrap();
    let data2 = read("../test.txt").unwrap();
    let x = compute_signature(&data1, 10);
    let q = compute_deltas(&x, &data2, 10);
    assert_eq!(
        q,
        vec![
            (0, vec![0]),
            (10, vec![1]),
            (20, vec![2]),
            (30, vec![3]),
            (40, vec![4]),
            (50, vec![5])
        ]
    );
}
