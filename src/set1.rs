/* str_to_hex -- helper function to convert string to vec<u8>
 * assuming str is in hex format
 * Parameters: str_inp (&str) - hexidecimal string
 * Return: output (vec<u8>) - hexidecimal in vector 
 */
pub fn str_to_vec(str_inp: &str) -> Vec<u8> {

  let mut output: Vec<u8> = Vec::new();

  let mut temp: u8 = 0x00;
  let mut switch: bool = false; 

  for s in str_inp.chars() {
    let item = match u8::from_str_radix(&s.to_string(), 16) {
        Ok(result) => result,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
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

/* print_hex_vec -- helper function to print Vec<u8> in 
 * format of '0x****'
 * Parameters: vec_inp (&Vec<u8>) - vector to print
 * Return: void
 */
pub fn print_hex_vec(vec_inp: &Vec<u8>) {
  print!("0x");
  for item in vec_inp.iter() {
    print!("{:x}", item);
  }
  println!("");
}

pub fn hex_to_base64(input: &str) {
  // Set 1, Challenge 1
  // Convert hex to base64
  // Always operate on raw bytes, never on encoded strings. Only use hex and base64 for pretty-printing. 

  println!("0x{}", input);  
}
