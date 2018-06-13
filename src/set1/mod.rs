pub mod storage;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

/* hex_to_base64 -- Set 1, Challenge 1
 * http://cryptopals.com/sets/1/challenges/1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: String - output of converting hex string to base64 string
 */
pub fn hex_to_base64(hex_str: &str) -> String {
  let mut s = storage::Storage::new_init(hex_str, "hex");

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
  let lhs = storage::Storage::new_init(lhs_str, lhs_type);
  let rhs = storage::Storage::new_init(rhs_str, rhs_type);

  let ans = &lhs ^ &rhs;

  ans.to_string()
}


/* char_freq -- helper function that returns the character frequency
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
    (' ', 10.000)
  ].iter().cloned().collect();

  let mut count: f32 = 0.0_f32;
  for c in str_inp.to_lowercase().chars() {
    if let Some(f) = freq.get(&c) {
      count += f;
    } 
  }
  count
}


/* single_byte_xor_cipher -- Set 1, Challenge 3
 * http://cryptopals.com/sets/1/challenges/3
 * The hex string has been XOR'd against a single character. 
 * Find the key, decrypt the message.  
 * Parameters: filename(&str) - File to detect single-character XOR
 * Return: (String, Char) - (hidden message, key that was used)
 */
pub fn single_byte_xor_cipher(str_inp: &str, str_type: &str) -> (String, char) {
  let s = storage::Storage::new_init(str_inp, str_type);

  let mut result_string: String = s.to_string();
  let mut result_char: char = '0';
  let mut max_freq: f32 = 0_f32;
  let mut tmp_freq: f32;

  for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
    let mut char_obj = storage::Storage::new_init(&i.to_string(), "ascii");
    char_obj.change_base("hex");
    let mut ans = &s ^ &char_obj;
    ans.change_base("ascii");
    
    tmp_freq = char_freq(ans.to_string().as_str());
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
pub fn detect_single_character_xor(filename: &str) -> (String, char, i32) {

  let mut f = File::open(filename).expect("Error: File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Error: Something went wrong when reading the file");

  // results that are going to be returned
  let mut result_string: String = String::new();
  let mut result_char: char = ' ';
  let mut result_num: i32 = 0;

  let mut max_freq: f32 = 0_f32; // keep track of the winner char_freq
  let mut tmp_freq: f32; // tmp variable to store char_freq of current string
  let mut count: i32 = 0; // keep track of line number

  // can we do better?
  // we are creating a new obj every line (60 objs in memory)
  // we are creating a new char_obj every line * every char (60*(26+26+10))= 3720
  // we are creating a ans every line * every char (60*(26+26+10)) = 3720
  for l in contents.lines() {
    let mut obj = storage::Storage::new_init(&l, &"hex");

    for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
      let mut char_obj = storage::Storage::new_init(&i.to_string(), "ascii");
      char_obj.change_base("hex");
      let mut ans = &obj ^ &char_obj;
      ans.change_base("ascii");

      tmp_freq = char_freq(&ans.to_string().as_str());
      if tmp_freq > max_freq {
        result_string = ans.to_string();
        result_char = i;
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
pub fn repeating_key_xor_encrypt(lhs_str: &str, lhs_type: &str, rhs_str: &str, rhs_type: &str) -> String {
  // TODO: handle \n -- newlines
  let lhs = storage::Storage::new_init(lhs_str, lhs_type);
  let rhs = storage::Storage::new_init(rhs_str, rhs_type);
  
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
    
  // open file and put in contents
  let mut f = File::open(filename).expect("Error: File not found");

  let mut contents = String::new();
  f.read_to_string(&mut contents)
    .expect("Error: Something went wrong when reading the file");

  // get rid of newlines in contents
  let file_contents: String = contents.lines().collect();
  println!("{}", file_contents.chars().count());

  let mut file_obj = storage::Storage::new_init(&file_contents, "base64");
  file_obj.change_base("ascii");
  // file_obj.print();
  // file_obj.change_base("base64");
  // file_obj.change_base("hex");

  // Step 1-4 - Figure out keysize
  let mut keysize: Vec<usize> = vec!(0, 0, 0);  
  let mut min_nor_dist: Vec<f64> = vec!(1.0f64 / 0.0f64, 1.0f64 / 0.0f64, 1.0f64 / 0.0f64); // set as MAX
  let mut tmp: f64;
  let mut t = vec!();

  for i in 2usize..41usize {
    let (lhs, rhs) = file_obj.split_by_keysize(i);

    tmp = storage::Storage::hamming_distance(&lhs, &rhs) as f64 / i as f64;
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
  
  let mut key_string: String = String::new();
  let mut res = vec!();

  // let v: Vec<usize> = vec!(4, 8, 12, 16, 20, 24, 28, 32, 36, 40);
  let v: Vec<usize> = vec!(36);
  // for key in 2usize..41usize {
  for key in &v {
    // Step 6
    // if file_obj.get_data_type() != "base64" {
    //  file_obj.change_base("base64");
    // }
    // let blocks = file_obj.split_into_blocks(key);
    let blocks = file_obj.split_into_blocks(*key);
    key_string = String::new();

    let mut key_char: char = ' ';
    let mut max_freq: f32; // keep track of the winner char_freq
    let mut tmp_freq: f32; // tmp variable to store char_freq of current string

    for block in blocks.iter() {
      let inp: &str = &block.to_string().to_string();
      let mut b = storage::Storage::new_init(&inp[..inp.chars().count() - (inp.chars().count() % 4)], block.get_data_type());

      key_char = ' ';
      max_freq = 0_f32;
      for ch in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        let mut char_obj = storage::Storage::new_init(&ch.to_string(), "ascii");
        let mut ans = &b ^ &char_obj;

        tmp_freq = char_freq(ans.to_string().as_str());
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
  (String::new(), key_string, keysize[2] as i32, 0) 
}

