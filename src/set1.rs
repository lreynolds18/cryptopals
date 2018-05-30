/* hexstr_to_vec -- helper function to convert string to vec<u8>
 * assuming str is in hex format
 * Parameters: str_inp (&str) - hexidecimal string
 * Return: output (vec<u8>) - hexidecimal in vector 
 */
pub fn hexstr_to_vec(str_inp: &str) -> Vec<u8> {

  let mut output: Vec<u8> = Vec::new();

  let mut temp: u8 = 0x00;
  let mut switch: bool = false; 

  for s in str_inp.chars() {
    let item = match u8::from_str_radix(&s.to_string(), 16) {
        Ok(result) => result,
        Err(error) => {
            panic!("There was a problem with from_str_radix: {:?}", error)
        },
    };

    if switch {
      temp |= item;
      output.push(temp);
      temp = 0x00;
      switch = false;
    } else {
      temp = item << 4;
      switch = true;
    }
  }

  // if we have a odd number of hexidecimals then add
  if switch {
    temp >>= 4;
    output.push(temp);
  }

  return output;
}

/* base64char_to_u8 -- helper function to convert base64 char to u8
 * char is 
 * Parameters: c (char) - Character between A-Z, a-z, 0-9, +, /
 * Return: u (u8) - binary representation of character (000000-111111)  
 */
fn base64char_to_u8(c: char) -> u8 {
  let mut u = c as u8;
  if u >= 65 && u <= 90 {
    // A - Z
    u -= 65;
  } else if u >= 97 && u <= 122 {
    // a - z
    u -= 71; // a = 26 in base64 so 97-26=71
             // z = 51 in base64 so 122-71=51
  } else if u >= 48 && u <= 57 {
    // 0-9
    u += 4; // 0 = 52 in base64 so 48+4=52
            // 9 = 61 in base64 so 57+4=61
  } else if u == 43 {
    // +
    u = 62;
  } else if u == 47 {
    // /
    u = 63;
  } else {
    panic!("Error: this is not a valid base64 digit");
  }
  return u;
}

/* base64u8_to_char -- helper function to convert base64 u8 to char
 * char is 
 * Parameters: u (u8) - binary representation of character (000000-111111)
 * Return: u (u8) - Character between A-Z, a-z, 0-9, +, /
 */
fn base64u8_to_char(u: u8) -> char {
  let c: char;
  if u >= 0 && u <= 25 {
    // A - Z
    c = (u + 65) as char;
  } else if u >= 26 && u <= 51 {
    // a - z
    c = (u + 71) as char;
  } else if u >= 52 && u <= 61 {
    // 0 - 9
    c = (u - 4) as char;
  } else if u == 62 {
    // +
    c = '+';
  } else if u == 63 {
    // /
    c = '/';
  } else {
    panic!("Error: this is not a valid base64 digit");
  }
  return c;
}

/* print_hex_vec -- helper function to print Vec<u8> in 
 * hexidecimal format
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn print_hex_vec(vec_inp: &Vec<u8>) {
  for item in vec_inp.iter() {
    print!("{:x}", item);
  }
  println!("");
}

/* print_base64_vec -- helper function to print Vec<u8> in 
 * base64 format
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn print_base64_vec(vec_inp: &Vec<u8>) {
  // (first 6), (last 2 + first 4), (last 4 + first 2), (last 6)
  for i in 0..vec_inp.len() { 
    print!("{:x}", vec_inp[i]); 
  }
}

/* hex_to_base64 -- Set 1, Challenge 1
 * converts hex to base64
 * Always operate on raw bytes, never on encoded strings.  
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn hex_to_base64(hex_str: &str) {
  // let hex_vec: Vec<u8> = hexstr_to_vec(hex_str); 

  let mut u: u8;
  for c in hex_str.chars() {
    u = base64char_to_u8(c); 
    println!("{}", base64u8_to_char(u));
  }

  // 4927 (0100 1001 0010 0111)
  // S (010010 010010 0111)

  // 111111 (11 1111) (1111 11) (111111)
  // (first 6), (last 2 + first 4), (last 4 + first 2), (last 6), ***
  
  // (0000 1111) so need to check if odd or not so we don't get rid of the last 0

  // 00001111 00001111 00001111
  // (last 4 + last first half), (last last half + last 4) 

  // store 11111100 00001111 11000000
}
