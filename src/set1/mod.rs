pub mod binary_object;



/* hex_to_base64 -- Set 1, Challenge 1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn hex_to_base64(hex_str: &str) -> String {
  let mut bin_obj = binary_object::BinaryObject::new(&hex_str, &"hex".to_string());

  bin_obj.change_base(&"base64".to_string());

  bin_obj.to_string()
}

pub fn fixed_xor(lhs_str: &str, lhs_type: &str, rhs_str: &str, rhs_type: &str) -> String {
  let lhs = binary_object::BinaryObject::new(&rhs_str, &rhs_type);
  let rhs = binary_object::BinaryObject::new(&lhs_str, &lhs_type);

  let ans = lhs ^ rhs;

  ans.to_string()
}
