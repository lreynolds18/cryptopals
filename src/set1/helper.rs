pub mod aes128lookup;
pub mod freq;
use set1::storage::Storage;

use std::collections::HashMap; // hashmap used in char_freq, inv_sub_bytes

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
pub fn char_freq(str_inp: &str, freq: &HashMap<u8, f32>) -> f32 {
    str_inp
        .chars()
        .map(|c| match freq.get(&(c as u8)) {
            Some(f) => f.clone(),
            None => 0f32,
        })
        .sum()
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

/* calc_key_expansion_core --
 *
 */
pub fn calc_key_expansion_core(key: &Storage, i: usize, s_box: &Vec<u8>, rcon: &Vec<u8>) -> Storage {

  // 1. Rotate left (example - [12, 62, 54, 126] -> [62, 54, 126, 12])
  let mut v = key.get_data().clone();
  v.swap(0, 1);
  v.swap(1, 2);
  v.swap(2, 3);

  // 2. S-box
  v[0] = s_box[v[0] as usize];
  v[1] = s_box[v[1] as usize];
  v[2] = s_box[v[2] as usize];
  v[3] = s_box[v[3] as usize];

  // 3. RCon
  v[0] ^= rcon[i];
  let temp: Storage = Storage::new_init_vec(&v, key.get_data_type());
  temp 
}

/* calc_key_expansion -- calculate key expansion using algorithm
 * expands a 16 byte keys into 11 different 16 byte keys
 * Parameters: key (&Storage) - original key
 * Return: vec<Storage> - 11 different keys
 */
pub fn calc_key_expansion<'a>(mut keys: Vec<&'a Storage>, s_box: &Vec<u8>, rcon: &Vec<u8>) -> Vec<&'a Storage> {
  for i in 0..10 {
      let key_generated: Storage = calc_key_expansion_core(&keys[i], i+1, s_box, rcon);
      keys.push(&(keys[i] ^ &key_generated));
  }
  keys
}

/* add_round_key -- a Round Key is added to the State by a simple
 * bitwise XOR operation
 * Parameters: state (Storage) - Encrypted objected to decrypt
 *             key (&Storage) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn add_round_key(state: &Storage, key: &Storage) -> Storage {
    state ^ key
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
pub fn inv_shift_rows(state: &Storage) -> Storage {
    // assuming we have 16 bytes (128 bits)
    if state.len() != 16 {
        println!("Uh oh");
    }

    let b = state.get_data();
    let d = vec![
        b[0], b[13], b[10], b[7], b[4], b[1], b[14], b[11], b[8], b[5], b[2], b[15], b[12], b[9],
        b[6], b[3],
    ];

    Storage::new_init_vec(&d, state.get_data_type())
}

/* inv_sub_bytes -- subsitute bytes based on Inverse S-Box
 * Parameters: state (Storage) - Encrypted objected to decrypt
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_sub_bytes(state: &Storage, inverse_s_box: &Vec<u8>) -> Storage {
    Storage::new_init_vec(
        &state
            .get_data()
            .iter()
            .map(|d| inverse_s_box[*d as usize])
            .collect(),
        state.get_data_type(),
    )
}

/* inv_mix_columns -- Reverse MixCol by multiplying by a^-1
 * a^-1 = [0e 0b 0d 09] [S_0,c]
 *        [09 0e 0b 0d] [s_1,c]
 *        [0d 09 0e 0b] [s_2,c]
 *        [0b 0d 09 0e] [s_3,c]
 * 09 - mul_9, 0b - mul_11, 0d - mul_13, 0e - mul_14
 * option 1 is implementing gaussian field 2^8 for mul_9, mul_11, mul_13, mul_14
 * option 2 is implementing gaussian field 2^8 for mul_2 and then doing mul_9 = x*9 = (((x×2)×2)×2)+x
 * option 3 is using lookup tables
 *
 * Parameters: bytes_in (Storage) - Encrypted objected to decrypt
 *             key (&str) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_mix_columns(
    state: &Storage,
    mul_9: &Vec<u8>,
    mul_11: &Vec<u8>,
    mul_13: &Vec<u8>,
    mul_14: &Vec<u8>,
) -> Storage {
    // assuming that d len is 16
    let d = state.get_data();
    let mut out = Vec::new();
    for c in d.chunks(4) {
        let (c0, c1, c2, c3): (usize, usize, usize, usize) =
            (c[0] as usize, c[1] as usize, c[2] as usize, c[3] as usize);
        out.push(mul_14[c0] ^ mul_11[c1] ^ mul_13[c2] ^ mul_9[c3]);
        out.push(mul_9[c0] ^ mul_14[c1] ^ mul_11[c2] ^ mul_13[c3]);
        out.push(mul_13[c0] ^ mul_9[c1] ^ mul_14[c2] ^ mul_11[c3]);
        out.push(mul_11[c0] ^ mul_13[c1] ^ mul_9[c2] ^ mul_14[c3]);
    }
    Storage::new_init_vec(&out, state.get_data_type())
}

/* inv_cipher_aes_128 -- AES decyption algorithm
 * Parameters: bytes_in (Storage) - Encrypted objected to decrypt
 *             key (&str) - Key used to encrypt object
 * Return: state Storage - Bytes after AES decryption
 */
pub fn inv_cipher_aes_128(bytes_in: &Storage, key: &Storage) {
    if bytes_in.len() % 16 != 0 {
        panic!("Error: the length of bytes_in must be divisible by 16");
    }
    let (s_box, inverse_s_box, rcon, mul_9, mul_11, mul_13, mul_14) = aes128lookup::get_aes_128_lookup_tables();
    let end: usize = bytes_in.len() / 16;

    
    let mut keys: Vec<&Storage> = Vec::new();
    keys.push(key);
    let ans = calc_key_expansion(keys, &s_box, &rcon);

    for i in 0usize..end {
        let mut state = bytes_in.index(i * 16, (i + 1) * 16);

        state = add_round_key(&state, &key);

        for _j in 0..9 {
            state = inv_shift_rows(&state);
            state = inv_sub_bytes(&state, &inverse_s_box);
            state = add_round_key(&state, &key);
            state = inv_mix_columns(&state, &mul_9, &mul_11, &mul_13, &mul_14);
        }

        state = inv_shift_rows(&state);
        state = inv_sub_bytes(&state, &inverse_s_box);
        state = add_round_key(&state, &key);

        state.print();
    }
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
        let freq = freq::get_char_freq_table();
        assert_eq!(
            char_freq("hello world", &freq) > char_freq("~!#$!@", &freq),
            true
        );
        assert_eq!(
            char_freq("this is a secret message", &freq) > char_freq("~!#$!@", &freq),
            true
        );
        assert_eq!(
            char_freq("key", &freq) > char_freq("!@#()!#$,./", &freq),
            true
        );
        assert_eq!(
            char_freq("blah blahBLAH", &freq) > char_freq("~!#$!@", &freq),
            true
        );
    }

    #[test]
    fn check_char_freq_tests_that_should_fail() {
        let freq = freq::get_char_freq_table();
        // checking valid string vs white space (invalid)
        assert_ne!(
            char_freq("hello world", &freq) > char_freq("           ", &freq),
            true
        );

        // checking length of valid string vs invalid string
        assert_ne!(
            char_freq("key", &freq) > char_freq("    !@# ,,. )(@! ", &freq),
            true
        );
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

    #[test]
    fn check_key_expansion() {
        let (s_box, _, rcon, _, _, _, _) = aes128lookup::get_aes_128_lookup_tables();
        let test1_key = Storage::new_init("00000000000000000000000000000000", "base64");
        let test1_ans: Vec<Storage> = vec![
          Storage::new_init("00000000000000000000000000000000", "base64"),
          Storage::new_init("62636363626363636263636362636363", "base64"),
          Storage::new_init("9b9898c9f9fbfbaa9b9898c9f9fbfbaa", "base64"),
          Storage::new_init("90973450696ccffaf2f457330b0fac99", "base64"),
          Storage::new_init("ee06da7b876a1581759e42b27e91ee2b", "base64"),
          Storage::new_init("7f2e2b88f8443e098dda7cbbf34b9290", "base64"),
          Storage::new_init("ec614b851425758c99ff09376ab49ba7", "base64"),
          Storage::new_init("217517873550620bacaf6b3cc61bf09b", "base64"),
          Storage::new_init("0ef903333ba9613897060a04511dfa9f", "base64"),
          Storage::new_init("b1d4d8e28a7db9da1d7bb3de4c664941", "base64"),
          Storage::new_init("b4ef5bcb3e92e21123e951cf6f8f188e", "base64"),
        ];
        let test1_res = calc_key_expansion(test1_key, &s_box, &rcon);
        assert_eq!(test1_ans.len(), test2_ans.len());
        
        let test2_key = Storage::new_init("ffffffffffffffffffffffffffffffff", "base64");
        let test2_ans: Vec<Storage> = vec![
          Storage::new_init("ffffffffffffffffffffffffffffffff", "base64"),
          Storage::new_init("e8e9e9e917161616e8e9e9e917161616", "base64"),
          Storage::new_init("adaeae19bab8b80f525151e6454747f0", "base64"),
          Storage::new_init("090e2277b3b69a78e1e7cb9ea4a08c6e", "base64"),
          Storage::new_init("e16abd3e52dc2746b33becd8179b60b6", "base64"),
          Storage::new_init("e5baf3ceb766d488045d385013c658e6", "base64"),
          Storage::new_init("71d07db3c6b6a93bc2eb916bd12dc98d", "base64"),
          Storage::new_init("e90d208d2fbb89b6ed5018dd3c7dd150", "base64"),
          Storage::new_init("96337366b988fad054d8e20d68a5335d", "base64"),
          Storage::new_init("8bf03f233278c5f366a027fe0e0514a3", "base64"),
          Storage::new_init("d60a3588e472f07b82d2d7858cd7c326", "base64"),
        ];
        
        let test3_key = Storage::new_init("000102030405060708090a0b0c0d0e0f", "base64");
        let test3_ans: Vec<Storage> = vec![
          Storage::new_init("000102030405060708090a0b0c0d0e0f", "base64"),
          Storage::new_init("d6aa74fdd2af72fadaa678f1d6ab76fe", "base64"),
          Storage::new_init("b692cf0b643dbdf1be9bc5006830b3fe", "base64"),
          Storage::new_init("b6ff744ed2c2c9bf6c590cbf0469bf41", "base64"),
          Storage::new_init("47f7f7bc95353e03f96c32bcfd058dfd", "base64"),
          Storage::new_init("3caaa3e8a99f9deb50f3af57adf622aa", "base64"),
          Storage::new_init("5e390f7df7a69296a7553dc10aa31f6b", "base64"),
          Storage::new_init("14f9701ae35fe28c440adf4d4ea9c026", "base64"),
          Storage::new_init("47438735a41c65b9e016baf4aebf7ad2", "base64"),
          Storage::new_init("549932d1f08557681093ed9cbe2c974e", "base64"),
          Storage::new_init("13111d7fe3944a17f307a78b4d2b30c5", "base64"),
        ];

        let test4_key = Storage::new_init("6920e299a5202a6d656e636869746f2a", "base64");
        let test4_ans: Vec<Storage> = vec![
          Storage::new_init("6920e299a5202a6d656e636869746f2a", "base64"),
          Storage::new_init("fa8807605fa82d0d3ac64e6553b2214f", "base64"),
          Storage::new_init("cf75838d90ddae80aa1be0e5f9a9c1aa", "base64"),
          Storage::new_init("180d2f1488d0819422cb6171db62a0db", "base64"),
          Storage::new_init("baed96ad323d173910f67648cb94d693", "base64"),
          Storage::new_init("881b4ab2ba265d8baad02bc36144fd50", "base64"),
          Storage::new_init("b34f195d096944d6a3b96f15c2fd9245", "base64"),
          Storage::new_init("a7007778ae6933ae0dd05cbbcf2dcefe", "base64"),
          Storage::new_init("ff8bccf251e2ff5c5c32a3e7931f6d19", "base64"),
          Storage::new_init("24b7182e7555e77229674495ba78298c", "base64"),
          Storage::new_init("ae127cdadb479ba8f220df3d4858f6b1", "base64"),
        ];
    }
}
