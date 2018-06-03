pub mod binary_object;

/* hex_to_base64 -- Set 1, Challenge 1
 * http://cryptopals.com/sets/1/challenges/1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: String - output of converting hex string to base64 string
 */
pub fn hex_to_base64(hex_str: &str) -> String {
  let mut bin_obj = binary_object::BinaryObject::new(&hex_str, &"hex".to_string());

  bin_obj.change_base(&"base64".to_string());

  bin_obj.to_string()
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
  let lhs = binary_object::BinaryObject::new(&lhs_str, &lhs_type);
  let rhs = binary_object::BinaryObject::new(&rhs_str, &rhs_type);

  let ans = lhs ^ rhs;

  ans.to_string()
}

/* single_byte_xor_cipher -- Set 1, Challenge 3
 * http://cryptopals.com/sets/1/challenges/3
 * The hex string has been XOR'd against a single character. Find the key, decrypt the message.  
 * Parameters: filename(&str) - File to detect single-character XOR
 * Return: (String, Char) - (hidden message, key that was used)
 */
pub fn single_byte_xor_cipher(str_inp: &str, str_type: &str) -> (String, char) {

  ("".to_string(), 'A')
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

  "".to_string()
}


