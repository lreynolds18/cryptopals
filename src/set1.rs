

/* hex_to_base64 -- Set 1, Challenge 1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn hex_to_base64(hex_str: &str) {
  // let hex_vec: Vec<u8> = hexstr_to_vec(hex_str); 

  /*
  let mut u: u8;
  for c in hex_str.chars() {
    u = base64char_to_u8(c); 
    println!("{}", base64u8_to_char(u));
  }
  */

  // 4927 (0100 1001 0010 0111)
  // S (010010 010010 0111)

  // 111111 (11 1111) (1111 11) (111111)
  // (first 6), (last 2 + first 4), (last 4 + first 2), (last 6), ***
  
  // (0000 1111) so need to check if odd or not so we don't get rid of the last 0

  // 00001111 00001111 00001111
  // (last 4 + last first half), (last last half + last 4) 

  // store 11111100 00001111 11000000
}
