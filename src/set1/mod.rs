pub mod helper;
pub mod storage;

use self::storage::Storage;
use std::fs::File;
use std::io::prelude::*;

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

    let mut result_string: String = s.to_string();
    let mut result_char: char = '0';
    let mut max_freq: f32 = 0_f32;
    let mut tmp_freq: f32;

    for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        let mut char_obj = storage::Storage::new_init(&i.to_string(), "ascii");
        char_obj.change_base("hex");
        let mut ans = &s ^ &char_obj;
        ans.change_base("ascii");

        tmp_freq = helper::char_freq(ans.to_string().as_str());
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
    let mut f = File::open(filename).expect("Error: File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error: Something went wrong when reading the file");

    let file_contents: Vec<Storage> = contents
        .lines()
        .map(|l| Storage::new_init(l, "hex"))
        .collect();

    let char_objs: Vec<Storage> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| Storage::new_init(&c.to_string(), "ascii"))
        .collect();

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
            tmp_freq = helper::char_freq(&ans.to_string().as_str());

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
 * Return: (String, char, i32) - (Secret message, key that was used, key size, line number)
 */
pub fn break_repeating_key_xor(filename: &str) -> (String, String, i32, i32) {
    let mut f = File::open(filename).expect("Error: File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error: Something went wrong when reading the file");

    let file_contents: Vec<Storage> = contents
        .lines()
        .map(|l| Storage::new_init(l, "base64"))
        .collect();

    let char_objs: Vec<Storage> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| Storage::new_init(&c.to_string(), "ascii"))
        .collect();

    // Step 1-4 - Figure out keysize (theoretically we should use a minheap)
    let mut keysize: Vec<usize> = vec![0, 0, 0];
    let mut min_nor_dist: Vec<f64> = vec![1.0f64 / 0.0f64, 1.0f64 / 0.0f64, 1.0f64 / 0.0f64]; // set as MAX
    let mut tmp: f64;
    let mut t = vec![];

    for i in 2usize..41usize {
        let (lhs, rhs) = file_contents[0].split_by_keysize(i);

        tmp = helper::hamming_distance(&lhs, &rhs) as f64 / i as f64;
        t.push(tmp);
        if tmp < min_nor_dist[0] {
            min_nor_dist[2] = min_nor_dist[1];
            min_nor_dist[1] = min_nor_dist[0];
            min_nor_dist[0] = tmp;
            keysize[2] = keysize[1];
            keysize[1] = keysize[0];
            keysize[0] = i;
        } else if tmp < min_nor_dist[1] {
            min_nor_dist[2] = min_nor_dist[1];
            min_nor_dist[1] = tmp;
            keysize[2] = keysize[1];
            keysize[1] = i;
        } else if tmp < min_nor_dist[2] {
            min_nor_dist[2] = tmp;
            keysize[2] = i;
        }
    }
    println!("{:?}", keysize);
    println!("{:?}", t);

    // keysize = vec!(4, 8, 12, 16, 20, 24, 28, 32, 36, 40);
    // keysize = vec!(36);
    // keysize = vec!(2usize..41usize);

    let mut key_string: String = String::new();
    // let mut res = vec![];
    let mut max_freq = 0_f32; // keep track of the winning string

    let mut key_char: char = ' ';
    let mut max_char_freq: f32; // keep track of the winner char_freq
    let mut tmp_freq: f32; // tmp variable to store char_freq

    for key in &keysize {
        for file_obj in &file_contents {
            // file_obj.change_base("ascii");
            let blocks = helper::split_into_blocks(&file_obj, *key);
            key_string = String::new();

            for block in &blocks {}
        }
    }

    /*
        for block in blocks.iter() {
            let inp: &str = &block.to_string().to_string();
            let mut b = storage::Storage::new_init(
                &inp[..inp.chars().count() - (inp.chars().count() % 4)],
                block.get_data_type(),
            );

            key_char = ' ';
            max_freq = 0_f32;
            for ch in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
                let mut char_obj = storage::Storage::new_init(&ch.to_string(), "ascii");
                let mut ans = &b ^ &char_obj;

                tmp_freq = helper::char_freq(ans.to_string().as_str());
                if tmp_freq > max_freq {
                    key_char = ch;
                    max_freq = tmp_freq;
                }
            }
            key_string.push(key_char);
        }
        res.push(key_string.to_string());

        let key_obj = storage::Storage::new_init(&key_string.as_str(), "ascii");
        let ans = &file_obj ^ &key_obj;
        ans.print();
    }

    println!("{:?}", keysize);
    println!("{:?}", res);
    */
    (String::new(), key_string, keysize[2] as i32, 0)
}
