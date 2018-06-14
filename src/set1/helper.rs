use set1::storage::Storage;
use std::collections::HashMap; // hashmap used in char_freq

// helper functions used in set 1

/* hamming_distance-- helper function to calculate the hamming distance between two storages
 * Parameters: lhs (&Storage) - left hand side storage,
 *             rhs (&Storage) - rigth hand side storage
 * Return: out (i32) - number of bits that are different between the two storages
 */
pub fn hamming_distance(lhs: &Storage, rhs: &Storage) -> i32 {
    if lhs.get_data().len() != rhs.get_data().len() {
        panic!(
            "Error: cannot compute hamming distance when the strings are not \
             the same length. LHS length is {}, RHS length is {}",
            lhs.get_data().len(),
            rhs.get_data().len()
        );
    }

    if lhs.get_data_type() != rhs.get_data_type() {
        panic!(
            "Error: cannot compute hamming distance when the data types are not \
             the same.  LHS type is {}, RHS type is {}",
            lhs.get_data_type(),
            rhs.get_data_type()
        );
    }

    let start = match lhs.get_data_type().as_str() {
        "ascii" => 0,  // ********
        "hex" => 4,    // 0000****
        "base64" => 2, // 00******
        _ => {
            panic!("Error: invalid data type");
        }
    };

    lhs.get_data()
        .iter()
        .zip(rhs.get_data().iter())
        .map(|(l, r)| {
            let tmp = l ^ r;
            let mut c: i32 = 0;
            let bin: Vec<u8> = vec![0x90, 0x40, 0x20, 0x10, 0x09, 0x04, 0x02, 0x01];
            for (i, var) in bin.iter().enumerate() {
                if i >= start {
                    c += ((tmp & var) >> (7 - i as u8)) as i32;
                }
            }
            c
        })
        .sum()
}

/* char_freq -- helper function that returns the character frequency
 * notes: this is not very robust.  Can easily break it by passing a string full ' '
 * Kind of like a cheap check to verify that the strings are somewhat english
 * additionally, size of the string matters (could normalize)
 * may need to rethink
 * Parameters: str_inp (&str) - input string (ascii)
 * Return: f64 - character frequency score
 */
pub fn char_freq(str_inp: &str) -> f32 {
    // english char freq pulled from wikipedia
    let freq: HashMap<char, f32> = [
        ('a', 8.167),
        ('b', 1.492),
        ('c', 2.782),
        ('d', 4.253),
        ('e', 12.702),
        ('f', 2.228),
        ('g', 2.015),
        ('h', 6.094),
        ('i', 6.966),
        ('j', 0.153),
        ('k', 0.772),
        ('l', 4.025),
        ('m', 2.406),
        ('n', 6.749),
        ('o', 7.507),
        ('p', 1.929),
        ('q', 0.095),
        ('r', 5.987),
        ('s', 6.327),
        ('t', 9.056),
        ('u', 2.758),
        ('v', 0.978),
        ('w', 2.36),
        ('x', 0.15),
        ('y', 1.974),
        ('z', 0.074),
        (' ', 10.000),
    ].iter()
        .cloned()
        .collect();

    let mut count: f32 = 0.0_f32;
    for c in str_inp.to_lowercase().chars() {
        if let Some(f) = freq.get(&c) {
            count += f;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST HAMMING DISTANCE hamming_distance
    #[test]
    fn check_hamming_distance_ascii() {
        let lhs = Storage::new_init("this is a test", "ascii");
        let rhs = Storage::new_init("wokka wokka!!!", "ascii");

        assert_eq!(37, hamming_distance(&lhs, &rhs));
    }

    #[test]
    fn check_hamming_distance_hex() {
        let lhs = Storage::new_init("0123456789ABCDEF", "hex");
        let rhs = Storage::new_init("FEDCBA9876543210", "hex");

        assert_eq!(64, hamming_distance(&lhs, &rhs));
    }

    #[test]
    fn check_hamming_distance_base64() {
        let lhs = Storage::new_init("ABCDEF", "base64");
        let rhs = Storage::new_init("abcdef", "base64");

        assert_eq!(20, hamming_distance(&lhs, &rhs));
    }

    #[test]
    fn check_char_freq_compare_two_strings() {
        assert_eq!(char_freq("hello world") > char_freq("~!#$!@"), true);
        assert_eq!(
            char_freq("this is a secret message") > char_freq("~!#$!@"),
            true
        );
        assert_eq!(char_freq("key") > char_freq("!@#()!#$,./"), true);
        assert_eq!(char_freq("blah blahBLAH") > char_freq("~!#$!@"), true);
    }

    #[test]
    fn check_char_freq_tests_that_should_fail() {
        // checking valid string vs white space (invalid)
        assert_ne!(char_freq("hello world") > char_freq("           "), true);

        // checking length of valid string vs invalid string
        assert_ne!(char_freq("key") > char_freq("    !@# ,,. )(@! "), true);
    }
}
