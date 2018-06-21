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
 * Using frequencies from http://www.fitaly.com/board/domper3/posts/136.html
 * Parameters: str_inp (&str) - input string (ascii)
 * Return: f64 - character frequency score
 */
pub fn char_freq(str_inp: &str) -> f32 {
    // english char freq pulled from wikipedia
    let freq: HashMap<u8, f32> = [
        (9, 0.0057),
        (10, 10.0),
        (23, 0.0000),
        (32, 17.1662),
        (33, 0.0072),
        (34, 0.2442),
        (35, 0.0179),
        (36, 0.0561),
        (37, 0.0160),
        (38, 0.0226),
        (39, 0.2447),
        (40, 0.2178),
        (41, 0.2233),
        (42, 0.0628),
        (43, 0.0215),
        (44, 0.7384),
        (45, 1.3734),
        (46, 1.5124),
        (47, 0.1549),
        (48, 0.5516),
        (49, 0.4594),
        (50, 0.3322),
        (51, 0.1847),
        (52, 0.1348),
        (53, 0.1663),
        (54, 0.1153),
        (55, 0.1030),
        (56, 0.1054),
        (57, 0.1024),
        (58, 0.4354),
        (59, 0.1214),
        (60, 0.1225),
        (61, 0.0227),
        (62, 0.1242),
        (63, 0.1474),
        (64, 0.0073),
        (65, 0.3132),
        (66, 0.2163),
        (67, 0.3906),
        (68, 0.3151),
        (69, 0.2673),
        (70, 0.1416),
        (71, 0.1876),
        (72, 0.2321),
        (73, 0.3211),
        (74, 0.1726),
        (75, 0.0687),
        (76, 0.1884),
        (77, 0.3529),
        (78, 0.2085),
        (79, 0.1842),
        (80, 0.2614),
        (81, 0.0316),
        (82, 0.2519),
        (83, 0.4003),
        (84, 0.3322),
        (85, 0.0814),
        (86, 0.0892),
        (87, 0.2527),
        (88, 0.0343),
        (89, 0.0304),
        (90, 0.0076),
        (91, 0.0086),
        (92, 0.0016),
        (93, 0.0088),
        (94, 0.0003),
        (95, 0.1159),
        (96, 0.0009),
        (97, 5.1880),
        (98, 1.0195),
        (99, 2.1129),
        (100, 2.5071),
        (101, 8.5771),
        (102, 1.3725),
        (103, 1.5597),
        (104, 2.7444),
        (105, 4.9019),
        (106, 0.0867),
        (107, 0.6753),
        (108, 3.1750),
        (109, 1.6437),
        (110, 4.9701),
        (111, 5.7701),
        (112, 1.5482),
        (113, 0.0747),
        (114, 4.2586),
        (115, 4.3686),
        (116, 6.3700),
        (117, 2.0999),
        (118, 0.8462),
        (119, 1.3034),
        (120, 0.1950),
        (121, 1.1330),
        (122, 0.0596),
        (123, 0.0026),
        (124, 0.0007),
        (125, 0.0026),
        (126, 0.0003),
    ].iter()
        .cloned()
        .collect();

    let mut count: f32 = 0.0_f32;
    for c in str_inp.chars() {
        if let Some(f) = freq.get(&(c as u8)) {
            count += f;
        }
    }
    count
}

/* split_into_blocks -- splits a storage into keysizes and then splits each keysize into blocks
 * Parameters: keysize (usize) - Number of characters that we want to split by
 * Return: out Vec<Storage> - Vector of Storage where each Storage contains the nth elements in each keysize
 */
pub fn split_into_blocks(s: &Storage, keysize: usize) -> Vec<Storage> {
    // create an empty Vec<Vec<u8>> with the length of keysize
    let mut holder: Vec<String> = (0..keysize).map(|_| String::new()).collect();

    // add the nth item to the respective vec in holder
    // if data contains "helloworld" then w/ keysize 5
    // the result should be
    // "hw", "eo", "lr", "ll", "od"
    // because we split "helloworld" into "hello" and "world"
    // then we append the first characters to the first vec...
    for (i, d) in s.to_string().chars().enumerate() {
        holder[i % keysize].push(d);
    }

    let dt: &str = &s.get_data_type();
    holder.iter().map(|v| Storage::new_init(v, dt)).collect()
}

/* add_round_key -- a Round Key is added to the State by a simple
 * bitwise XOR operation
 * Parameters: state (Storage) - Encrypted objected to decrypt
 *             key (&str) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn add_round_key(state: &Storage, key: &str) {
    state.print();
}

/* inv_shift_rows -- inv shift to the right
 * shift the first column 0 to the right
 * shift the second column 1 to the right
 * shift the third column 2 to the right
 * shift the fourth column 3 to the right
 *
 *  B0  B4  B8 B12       B0  B4  B8 B12
 *  B1  B5  B9 B13  --> B13  B1  B5  B9
 *  B2  B6 B10 B14  --> B10 B14  B2  B6
 *  B3  B7 B11 B15       B7 B11 B15  B3
 *
 * Parameters: bytes_in (Storage) - Encrypted objected to decrypt
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_shift_rows(state: &Storage) {
    state.print();
}

/* inv_sub_bytes -- subsitute bytes based on Inverse S-Box
 * Parameters: state (Storage) - Encrypted objected to decrypt
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_sub_bytes(state: &Storage) {
    state.print();
}

/* inv_mix_columns -- Reverse MixCol by multiplying by a^-1
 * a^-1 = [0e 0b 0d 09] [S_0,c]
 *        [09 0e 0b 0d] [s_1,c]
 *        [0d 09 0e 0b] [s_2,c]
 *        [0b 0d 09 0e] [s_3,c]
 * Parameters: bytes_in (Storage) - Encrypted objected to decrypt
 *             key (&str) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_mix_columns(state: &Storage) {
    state.print();
}

/* inv_cipher -- AES decyption algorithm
 * Parameters: bytes_in (Storage) - Encrypted objected to decrypt
 *             key (&str) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_cipher_aes_128(bytes_in: &Storage, word: &str) {
    let state = bytes_in;

    /*
    add_round_key(state, word);

    for i in 0i32..10i32 {
        inv_shift_rows(state);
        inv_sub_bytes(state);
        add_round_key(state, word);
        inv_mix_columns(state);
    }
    
    inv_shift_rows(state);
    inv_sub_bytes(state);
    add_round_key(state, word);
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST hamming_distance
    #[test]
    fn check_hamming_distance_ascii() {
        let lhs = Storage::new_init("this is a test", "ascii");
        let rhs = Storage::new_init("wokka wokka!!!", "ascii");

        assert_eq!(37, hamming_distance(&lhs, &rhs));
    }

    #[test]
    fn check_hamming_distance_ascii_2() {
        let lhs = Storage::new_init("hEllO ! 2A3", "ascii");
        let rhs = Storage::new_init("good BYE wo", "ascii");

        assert_eq!(37, hamming_distance(&lhs, &rhs));
    }

    #[test]
    fn check_hamming_distance_ascii_3() {
        let lhs = Storage::new_init("123", "ascii");
        let rhs = Storage::new_init("BYE", "ascii");

        assert_eq!(15, hamming_distance(&lhs, &rhs));
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

    // TEST char_freq
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

    // TEST split_into_blocks
    // TODO: add tests and test invalid cases
    #[test]
    fn check_split_into_blocks() {
        let s = Storage::new_init("helloworld", "ascii");

        let test1_res = split_into_blocks(&s, 1);
        assert_eq!("helloworld", test1_res[0].to_string());

        let test2_res = split_into_blocks(&s, 2);
        assert_eq!("hlool", test2_res[0].to_string());
        assert_eq!("elwrd", test2_res[1].to_string());

        let test3_res = split_into_blocks(&s, 5);
        assert_eq!("hw", test3_res[0].to_string());
        assert_eq!("eo", test3_res[1].to_string());
        assert_eq!("lr", test3_res[2].to_string());
        assert_eq!("ll", test3_res[3].to_string());
        assert_eq!("od", test3_res[4].to_string());
    }

}
