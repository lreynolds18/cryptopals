pub mod storage;

/* hex_to_base64 -- Set 1, Challenge 1
 * http://cryptopals.com/sets/1/challenges/1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: String - output of converting hex string to base64 string
 */
pub fn hex_to_base64(hex_str: &str) -> String {
  let mut s = storage::Storage::new(&hex_str, &"hex".to_string());

  s.change_base(&"base64".to_string());

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
  let lhs = storage::Storage::new(&lhs_str, &lhs_type);
  let rhs = storage::Storage::new(&rhs_str, &rhs_type);

  let ans = lhs ^ rhs;

  ans.to_string()
}


/* char_freq -- helper function that returns the character frequency
 * Parameters: str_inp (&str) - input string (ascii)
 * Return: f64 - character frequency score
 */
pub fn char_freq(str_inp: &str) -> i32 {
  let mut count: i32 = 0;
  for c in str_inp.chars() {
    if c.is_alphanumeric() || c == ' ' || c == '\'' {
      count += 1;
    }
  }
  count
}


/* single_byte_xor_cipher -- Set 1, Challenge 3
 * http://cryptopals.com/sets/1/challenges/3
 * The hex string has been XOR'd against a single character. Find the key, decrypt the message.  
 * Parameters: filename(&str) - File to detect single-character XOR
 * Return: (String, Char) - (hidden message, key that was used)
 */
pub fn single_byte_xor_cipher(str_inp: &str, str_type: &str) -> (String, char) {
  let s = storage::Storage::new(&str_inp, &str_type);

  let mut result_string: String = s.to_string();
  let mut result_char: char = '0';
  let mut max_freq: i32 = 0;
  let mut tmp_freq: i32;

  for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
    let mut char_obj = storage::Storage::new(&i.to_string(), &"ascii");  
    char_obj.change_base(&"hex".to_string());
    let mut ans = &s ^ &char_obj;
    ans.change_base(&"ascii".to_string());
    
    tmp_freq = char_freq(&ans.to_string().as_str());
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
 * Return: (String, Char) - (hidden message, key that was used)
 */
pub fn detect_single_character_xor(filename: &str) -> (char) {

  // ("".to_string(), 'A')
  'A'
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
  let lhs = storage::Storage::new(&lhs_str, &lhs_type);
  let rhs = storage::Storage::new(&rhs_str, &rhs_type);
  
  let mut ans = lhs ^ rhs;

  ans.change_base(&"hex".to_string());

  ans.to_string()
}

