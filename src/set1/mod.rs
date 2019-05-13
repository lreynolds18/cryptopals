pub mod helper;
pub mod storage;

use self::storage::Storage;
use std::f64;
use std::fs;

/* hex_to_base64 -- Set 1, Challenge 1
 * http://cryptopals.com/sets/1/challenges/1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: String - output of converting hex string to base64 string
 */
pub fn hex_to_base64(hex_str: &str) -> String {
    let mut s = Storage::new_init(hex_str, "hex");

    s.change_base("base64");

    s.to_string()
}

/* fixed_xor -- Set 1, Challenge 2
 * http://cryptopals.com/sets/1/challenges/2
 * xor on two hex strings
 * Parameters: lhs_str (&str) - left hand side input
 *             lhs_type (&str) - left hand side data type (hex/base64)
 *             rhs_str (&str) - right hand side input
 *             rhs_type (&str) - right hand side data type (hex/base64)
 * Return: String - output of xor operation on lhs_str and rhs_str
 */
pub fn fixed_xor(lhs_str: &str, lhs_type: &str, rhs_str: &str, rhs_type: &str) -> String {
    let lhs = Storage::new_init(lhs_str, lhs_type);
    let rhs = Storage::new_init(rhs_str, rhs_type);

    let ans = &lhs ^ &rhs;

    ans.to_string()
}

/* single_byte_xor_cipher -- Set 1, Challenge 3
 * http://cryptopals.com/sets/1/challenges/3
 * The hex string has been XOR'd against a single character.
 * Find the key, decrypt the message.
 * Parameters: filename(&str) - File to detect single-character XOR
 * Return: (String, Char) - (hidden message, key that was used)
 */
pub fn single_byte_xor_cipher(str_inp: &str, str_type: &str) -> (String, char) {
    let s = Storage::new_init(str_inp, str_type);
    let freq = helper::freq::get_char_freq_table();

    let mut result_string: String = s.to_string();
    let mut result_char: char = '0';
    let mut max_freq: f32 = 0_f32;
    let mut tmp_freq: f32;

    for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        let mut char_obj = storage::Storage::new_init(&i.to_string(), "ascii");
        char_obj.change_base("hex");
        let mut ans = &s ^ &char_obj;
        ans.change_base("ascii");

        tmp_freq = helper::char_freq(ans.to_string().as_str(), &freq);
        if tmp_freq > max_freq {
            result_string = ans.to_string();
            result_char = i;
            max_freq = tmp_freq;
        }
    }

    (result_string, result_char)
}

/* detect_single_character_xor -- Set 1, Challenge 4
 * http://cryptopals.com/sets/1/challenges/4
 * One of the 60-character strings in this file has been encrypted by single-character XOR. Find it.
 * Parameters: filename(&str) - File to detect single-character XOR
 * Return: (String, char, i32) - (Secret message, key that was used, line number)
 */
pub fn detect_single_character_xor(filename: &str) -> (String, String, i32) {
    let contents = fs::read_to_string(filename).expect("Error: Unable to read file");

    let file_contents: Vec<Storage> = contents
        .lines()
        .map(|l| Storage::new_init(l, "hex"))
        .collect();

    let char_objs: Vec<Storage> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| Storage::new_init(&c.to_string(), "ascii"))
        .collect();

    let freq = helper::freq::get_char_freq_table();

    // results that are going to be returned
    let mut result_string: String = String::new();
    let mut result_char: String = String::new();
    let mut result_num: i32 = 0;

    let mut max_freq: f32 = 0_f32; // keep track of the winner char_freq
    let mut tmp_freq: f32; // tmp variable to store char_freq of current string
    let mut count: i32 = 0; // keep track of line number
    let mut ans: Storage;

    for mut fc in file_contents {
        fc.change_base("ascii");
        for co in &char_objs {
            ans = &fc ^ co;
            tmp_freq = helper::char_freq(&ans.to_string().as_str(), &freq);

            if tmp_freq > max_freq {
                result_string = ans.to_string();
                result_char = co.to_string();
                result_num = count;
                max_freq = tmp_freq;
            }
        }
        count += 1;
    }

    (result_string, result_char, result_num)
}

/* repeating_key_xor_encrypt -- Set 1, Challenge 5
 * http://cryptopals.com/sets/1/challenges/5
 * Parameters: lhs_str (&str) - left hand side input
 *             lhs_type (&str) - left hand side data type (hex/base64)
 *             rhs_str (&str) - right hand side input
 *             rhs_type (&str) - right hand side data type (hex/base64)
 * Return: String - Encrypted message
 */
pub fn repeating_key_xor_encrypt(
    lhs_str: &str,
    lhs_type: &str,
    rhs_str: &str,
    rhs_type: &str,
) -> String {
    // TODO: handle \n -- newlines
    let lhs = Storage::new_init(lhs_str, lhs_type);
    let rhs = Storage::new_init(rhs_str, rhs_type);

    let mut ans = &lhs ^ &rhs;

    ans.change_base("hex");

    ans.to_string()
}

/* break_repeating_key_xor -- Set 1, Challenge 6
 * http://cryptopals.com/sets/1/challenges/6
 * File has been base64'd after being encrypted with repeating-key XOR.
 * Algorithm:
 *   Step 1: Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
 *   Step 2: Write function to compute edit distance/Hamming distance
 *   Step 3: For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second
 *           KEYSIZE worth of bytes, and find the edit distance between them.
 *           Normalize this result by dividing by KEYSIZE.
 *   Step 4: The KEYSIZE with the smallest normalized edit distance is probably the key.
 *           You could proceed perhaps with the smallest 2-3 KEYSIZE values.
 *           Or take 4 KEYSIZE blocks instead of 2 and average the distances.
 *   Step 5: Now that you probably know the KEYSIZE: break the ciphertext into blocks
 *           of KEYSIZE length.
 *   Step 6: Now transpose the blocks: make a block that is the first byte of every block,
 *           and a block that is the second byte of every block, and so on.
 *   Step 7: Solve each block as if it was single-character XOR. You already have code to do this.
 *   Step 8: For each block, the single-byte XOR key that produces the best looking histogram is
 *           the repeating-key XOR key byte for that block. Put them together and you have the key.
 * Parameters: filename(&str) - File to detect repeating key xor
 * Return: (String, String, usize) - (Secret message, key that was used, key size)
 */
pub fn break_repeating_key_xor(filename: &str) -> (String, String, usize) {
    let contents = fs::read_to_string(filename).expect("Error: Unable to read file");

    let mut file_contents = Storage::new_init(&contents.replace("\n", ""), "base64");
    file_contents.change_base("ascii");

    let char_objs: Vec<Storage> =
        " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz:"
            .chars()
            .map(|c| Storage::new_init(&c.to_string(), "ascii"))
            .collect();

    let freq = helper::freq::get_char_freq_table();

    // Step 1-4 - Figure out keysize (theoretically we should use a minheap)
    let mut keysize: usize = 0;
    let mut min_nor_dist: f64 = f64::INFINITY; // set as MAX
    let mut tmp: f64;

    for i in 4usize..41usize {
        let s1 = file_contents.index(0, i);
        let s2 = file_contents.index(i, 2 * i);
        let s3 = file_contents.index(2 * i, 3 * i);
        let s4 = file_contents.index(3 * i, 4 * i);

        let hd1 = helper::hamming_distance(&s1, &s2);
        let hd2 = helper::hamming_distance(&s2, &s3);
        let hd3 = helper::hamming_distance(&s3, &s4);

        tmp = (hd1 + hd2 + hd3) as f64 / (3 * i) as f64;

        if tmp < min_nor_dist {
            min_nor_dist = tmp;
            keysize = i;
        }
    }

    let mut key_string = String::new();
    let mut result_char = String::new();
    let mut max_freq: f32;
    let mut tmp_freq: f32;
    let mut ans;

    let blocks = helper::split_into_blocks(&file_contents, keysize);

    for block in &blocks {
        max_freq = 0_f32;

        for co in &char_objs {
            ans = block ^ co;
            tmp_freq = helper::char_freq(&ans.to_string().as_str(), &freq);

            if tmp_freq > max_freq {
                result_char = co.to_string();
                max_freq = tmp_freq;
            }
        }
        key_string.push_str(&result_char);
    }

    let key_obj = Storage::new_init(&key_string, "ascii");
    ans = &file_contents ^ &key_obj;

    (ans.to_string(), key_string, keysize)
}

/* decrypt_aes_128_ecb -- Set 1, Challenge 7
 * http://cryptopals.com/sets/1/challenges/7
 * The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode under the key
 * "YELLOW SUBMARINE". (case-sensitive, without the quotes; exactly 16 characters; I like
 * "YELLOW SUBMARINE" because it's exactly 16 bytes long, and now you do too).
 * Decrypt it. You know the key, after all.
 * Parameters: filename (&str) - Input File
 *             key (&str) - String used to encrypt message
 * Return: String - Secret message
 */
pub fn decrypt_aes_128_ecb(filename: &str, key: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Error: Unable to read file");

    let mut input_storage = Storage::new_init(&contents.replace("\n", ""), "base64");
    let key_storage = Storage::new_init(key, "ascii");

    input_storage.change_base("ascii");

    helper::inv_cipher_aes_128(&input_storage, &key_storage);
    "".to_string()
}
