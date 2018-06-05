use std::ops;

// TODO: sepeate this into multiple files (header, methods, and test?)
// TODO: ownership? who owns what and why
// TODO: use .iter().map().collect() instead of for item in &self.data????
// TODO: clean up change base
// TODO: write tests for XOR (full + repeating)
// TODO: write tests for change_base
// TODO: write tests for char_to_u8 and u8_to_char 

pub struct Storage {
  data: Vec<u8>,
  data_type: String,
}

// XOR implementation for Storage ^ Storage = Storage
// handles repeating XOR so if lhs is bigger than rhs
// it will repeatable XOR the rhs on the lhs
impl ops::BitXor for Storage {
  type Output = Storage;

  fn bitxor(self, rhs: Storage) -> Storage {
    if self.data_type != rhs.data_type {
      panic!("Error: Storage are not the same type.  LHS type is {}. RHS type is {}.", self.data_type, rhs.data_type);
    }
  
    if self.data.len() == rhs.data.len() {
      Storage {
        data: self.data.iter()
                       .zip(rhs.data.iter())
                       .map(|(l, r)| l ^ r)
                       .collect(),
        data_type: self.data_type
      }
    } else if self.data.len() > rhs.data.len() {
      let mut out: Vec<u8> = Vec::new();
      let rhs_len: i64 = rhs.data.len() as i64;
        
      for (i, item) in self.data.iter().enumerate() {
        out.push(item ^ rhs.data[((i as i64) % rhs_len) as usize]);
      }
      Storage {
        data: out,
        data_type: self.data_type
      }
    } else  {
      panic!("Error: Storage cannot be XOR'd against each other.  LHS length is {}, RHS length is {}", self.data.len(), rhs.data.len());
    }
  }
}


impl<'a> ops::BitXor<&'a Storage> for &'a Storage {
  type Output = Storage;

  fn bitxor(self, rhs: &Storage) -> Storage {
    if self.data_type != rhs.data_type {
      panic!("Error: Storage are not the same type.  LHS type is {}. RHS type is {}.", self.data_type, rhs.data_type);
    }
  
    if self.data.len() == rhs.data.len() {
      Storage {
        data: self.data.iter()
                       .zip(rhs.data.iter())
                       .map(|(l, r)| l ^ r)
                       .collect(),
        data_type: self.get_data_type().to_string()
      }
    } else if self.data.len() > rhs.data.len() {
      let mut out: Vec<u8> = Vec::new();
      let rhs_len: i64 = rhs.data.len() as i64;
        
      for (i, item) in self.data.iter().enumerate() {
        out.push(item ^ rhs.data[((i as i64) % rhs_len) as usize]);
      }
      Storage {
        data: out,
        data_type: self.get_data_type().to_string()
      }
    } else  {
      panic!("Error: Storage cannot be XOR'd against each other.  LHS length is {}, RHS length is {}", self.data.len(), rhs.data.len());
    }
  }
}

impl Storage {

  /* new -- constructor for storage 
   * converts string to vec<u8>
   * assuming str_inp is in it's respected format of data_type (hex / base64)
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: Storage (w/ data, data_type, and ending)
   */
  pub fn new(str_inp: &str, data_type: &str) -> Storage {
    Storage {
      data: Storage::build_data(str_inp, data_type),
      data_type: String::from(data_type),
    }
  }
  
  /* set_data -- helper function to set self.data and self.data_type
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: void 
   */
  pub fn set_data(&mut self, str_inp: &str, data_type: &str) {
    self.data = Storage::build_data(str_inp, data_type);
    self.data_type = String::from(data_type);
  }
  
  /* get_data -- helper function to get self.data
   * Parameters: void
   * Return: self.data (Vec<u8>) - data in vector format
   */
  pub fn get_data(&self) -> &Vec<u8> {
    &self.data
  }

  /* get_data_type -- helper function to get self.data_type
   * Parameters: void
   * Return: self.data_type (&str) - data_typ 
   */
  pub fn get_data_type(&self) -> &String {
    &self.data_type
  }

  /* build_data -- helper function to build self.data
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: self.data (Vec<u8>) - vector representation of our str_inp
   */
  fn build_data(str_inp: &str, data_type: &str) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for c in str_inp.chars() {
      data.push(Storage::char_to_u8(c, data_type));
    }
    data
  }

  /* char_to_u8 -- helper function to convert (hex/base64) char to u8
   *               (note: we don't want self here because we want to be able to use this 
   *               outside of this struct / want to use this in constructor)
   * Parameters: c (char) - Character between (0-9, A-F, a-f) or (A-Z, a-z, 0-9, +, /)
   *             data_type (&str) - base of the char to convert to 
   * Return: u (u8) - binary representation of character (0000-1111) or (000000-111111)
   */
  fn char_to_u8(c: char, data_type: &str) -> u8 {
    let u = c as u8;
    if data_type == "hex" {
      // HEX
      match u {
        48...57 => { u - 48 }, // 0 - 9
        97...102 => { u - 87 }, // a - f
        65...70 => { u - 55 }, // A - F
        _ => { panic!("Error: this is not a valid hex digit") }
      }
    } else if data_type == "base64" {
      // BASE64
      match u {
        65...90 => { u - 65 }, // A - Z 
        97...122 => { u - 71 }, // a - z
        48...57 => { u + 4 }, // 0 - 9
        43 => { 62 }, // +
        47 => { 63 }, // /
        _ => { panic!("Error: this is not a valid base64 digit") }
      }
    } else {
      // ASCII
      u
    }
  }

  /* u8_to_char -- helper function to convert (hex/base64) u8 to char
   *               (note: we don't want self here because we want to be able to use this 
   *               outside of this struct / want to use this in constructor)
   * Parameters: u (u8) - binary representation of character (0000-1111) or (000000-111111)
   *             data_type (&str) - base of the char to convert to 
   * Return: u (u8) - Character between (0-9, a-f) or (A-Z, a-z, 0-9, +, /)
   */
  fn u8_to_char(u: u8, data_type: &str) -> char {
    if data_type == "hex" {
      // HEX
      match u {
        0...9 => { (u + 48) as char }, // 0 - 9
        10...15 => { (u + 87) as char }, // a - f
        _ => { panic!("Error: this is not a valid hex digit") }
      }
    } else if data_type == "base64" {
      // BASE64
      match u {
        0...25 => { (u + 65) as char }, // A - Z
        26...51 => { (u + 71) as char }, // a - z
        52...61 => { (u - 4) as char }, // 0 - 9
        62 => { '+' }, // +
        63 => { '/' }, // /
        _ => { panic!("Error: this is not a valid base64 digit") }
      }
    } else {
      // ASCII
      u as char
    }
  }

  /* print -- helper function to print self.data Vec<u8>
   * Parameters: void 
   * Return: void
   */
  pub fn print(&self) {
    println!("{}", self.to_string());
  }

  /* to_string -- helper function to convert self.data Vec<u8> to string
   * Parameters: void 
   * Return: out (&str) - Hex/Base64 data in string format 
   */
  pub fn to_string(&self) -> String {
    let mut out = String::new();

    for item in &self.data {
      out.push(Storage::u8_to_char(*item, self.data_type.as_str()));
    }
    out
  }

  /* change_base -- convert old_base to new_base
   * currently handles hex -> base64 and base64 -> hex
   * changes self.data and self.data_type in struct
   * Parameters: new_base (&str) - New base to convert old base to
   * Return: void 
   */
  pub fn change_base(&mut self, new_base: &str) {
    // TODO: Is this a good idea???????????
    // YES: THEN REFACTOR
    // NO: COME UP W/ A NEW SOLUTION

    if self.data_type != new_base {

      let mut output: Vec<u8> = Vec::new();
      let mut temp: u8 = 0x00;

      if self.data_type == "hex" && new_base == "base64" {
        // hex -> base64

        // if converting to base64, we add elements to the vec by 3 hex values
        // 00001111, 00001122, 00002222 -> 00111111, 00222222
        if new_base == "base64" && self.data.len() % 3 != 0 {
          panic!("Error: hex input does not fit nicely into base64.");
        }

        for (i, item) in self.data.iter().enumerate() {
          if i % 3 == 0 {
            // nothing in item
            temp = item << 2;
            // temp now has 4 bits in it (00****00)
          } else if i % 3 == 1 {
            temp |= (item & 0x0C) >> 2;
            // fill first 2 bits in temp and push
            // temp has 6 bits (00******)
            output.push(temp);
            temp = (item & 0x03) << 4;
            // push remaining two bits in temp
            // temp has 2 bits (00**0000)
          } else if i % 3 == 2 {
            temp |= item;
            // temp has 6 bits (00******)
            output.push(temp);
            // temp now has 0 bits (00000000)
          }
        }

      } else if self.data_type == "base64" && new_base == "hex" {
        // base64 -> hex

        // if converting to hex, we add elements to the vec by 2 base64 values
        // 00111122 00223333 -> 00001111, 00002222, 00003333
        if new_base == "hex" && self.data.len() % 2 != 0 {
          panic!("Error: base64 does not fit nicely into hex.");
        }

        for (i, item) in self.data.iter().enumerate() {
          if i % 2 == 0 {
            // we want to add first 4 bits to self.data
            // we want to push the remaining 2 bits to temp
            temp = (item & 0x3C) >> 2;
            // temp has 4 bits (0000****)
            output.push(temp);
            temp = (item & 0x03) << 2; 
            // temp has 2 bits (0000**00)
          } else if i % 2 == 1 {
            // we want to add first 2 bits to temp and then push to self.data
            // we want to add the remaining 4 bits to temp and then push to self.data
            temp |= (item & 0x30) >> 4;
            // temp has 4 bits (0000****)
            output.push(temp);
            temp = item & 0x0F;
            output.push(temp);
            // temp has 0 bits (00000000)
          }
        }
      } else if self.data_type == "hex" && new_base == "ascii" {
        // hex -> ascii
        if self.data.len() % 2 != 0 {
          panic!("Error: the data doesn't fit nicely into an ASCII string");
        }

        for (i, item) in self.data.iter().enumerate() {
          if i%2 == 0 {
            // temp has 0 bits (00000000)
            temp = item << 4; 
            // temp has 4 bits (****0000)
          } else if i%2 == 1 {
            // temp has 4 bits (****0000)
            temp |= item;
            // temp has 8 bits (********)
            output.push(temp);
            // temp has 0 bits (00000000)
          } 
        }
      } else if self.data_type == "ascii" && new_base == "hex" {
        // ascii -> hex
        for item in &self.data {
          // push first 4 bits to vec (****0000) >> 4 = (0000****)
          output.push((item & 0xF0) >> 4);
          // push last 4 bits to vec (0000****)
          output.push(item & 0x0F);
        }
      } else if self.data_type == "base64" && new_base == "ascii" {
        // base64 -> ascii
        if self.data.len() % 4 != 0 {
          panic!("Error: the data doesn't fit nicely into an ASCII string");
        }

        for (i, item) in self.data.iter().enumerate() {
          if i%4 == 0 {
            // temp starts as 00000000
            temp = item << 2;
            // temp is now ******00
          } else if i%4 == 1 {
            // temp starts as ******00
            temp |= (item & 0x30) >> 4;
            // temp is now ********
            output.push(temp);
            // temp is now 00000000
            temp = (item & 0x0F) << 2;
            // temp is now ****0000
          } else if i%4 == 2 {
            // temp start at ****0000
            temp |= (item & 0x3C) >> 2;
            // temp is now ********
            output.push(temp);
            // temp is now 00000000
            temp = (item & 0x03) << 6;
            // temp is now **000000
          } else if i%4 == 3 {
            // temp starts as **000000
            temp |= item;
            // temp is now ********
            output.push(temp);
            // temp is now 00000000
          }
        }
      } else if self.data_type == "ascii" && new_base == "base64" {
        // ascii -> base64
        for (i, item) in self.data.iter().enumerate() {
          if i%3 == 0 {
            // temp starts at 00000000
            temp = (item & 0xFC) >> 2;
            // temp is now 00******
            output.push(temp);
            // temp is now 00000000
            temp = (item & 0x03) << 4;
            // temp is now 00**0000
          } else if i%3 == 1 {
            // temp starts as 00**0000
            temp |= (item & 0xF0) >> 4;
            // temp is now 00******
            output.push(temp);
            // temp is now 00000000
            temp = (item & 0x0F) << 2;
            // temp is now 00****00
          } else if i%3 == 2 {
            // temp starts at 00****00
            temp |= (item & 0xC0) >> 6;
            // temp is now 00******
            output.push(temp);
            // temp is now 00000000
            output.push(item & 0x3F);
            // temp is now 00000000
          }
        }
      }

      self.data = output;
      self.data_type = new_base.to_string();
    }
  }
}


#[cfg(test)]
mod storage_unit_tests {
  use super::*;

  /* - Implement?
  #[test]
  fn create_blank_storage() {
    let s: Storage = Storage::new();
  }
  */
  
  #[test]
  fn check_valid_hex_to_string() {
    // check every possible character in hex string
    // check uppercase hex characters go to lowercase
    let s: Storage = Storage::new(&"0123456789ABCDEFabcdef".to_string(), &"hex".to_string());
    assert_eq!("0123456789abcdefabcdef", s.to_string());
  }

  #[test]
  #[should_panic]
  fn check_invalid_hex_to_string() {
    // check invalid character in hex string
    let _s: Storage = Storage::new(&"ghijklmnop".to_string(), &"hex".to_string());
  }
  
  #[test]
  fn check_base64_to_string() {
    let s: Storage = Storage::new(&"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+".to_string(), &"base64".to_string());
    assert_eq!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+", s.to_string());
  }

  #[test]
  #[should_panic]
  fn check_invalid_base64_to_string() {
    // check invalid character in hex string
    let _s: Storage = Storage::new(&"!@#$*^)($%@_".to_string(), &"base64".to_string());
  }

  #[test]
  fn check_ascii_to_string() {
    let s: Storage = Storage::new(&"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,./;'[]<>?:\"{{}}-_=+`~!@#$%^&*()".to_string(), &"ascii".to_string());
    assert_eq!("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,./;'[]<>?:\"{{}}-_=+`~!@#$%^&*()", s.to_string());
  }

  #[test]
  fn check_get_data_hex() {
    let hex = Storage::new(&"0123456789ABCDEFabcdef".to_string(), &"hex".to_string());
    let test_hex = vec!(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
    assert_eq!(&test_hex, hex.get_data());
    assert_eq!(test_hex, Storage::build_data(&"0123456789ABCDEFabcdef".to_string(), &"hex".to_string()));
    assert_eq!("hex", hex.get_data_type());
  }

  #[test]
  fn check_get_data_base64() {
    let base64 = Storage::new(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(), &"base64".to_string());
    let test_base64: Vec<u8> = (0u8..64).collect();

    assert_eq!(&test_base64, base64.get_data());
    assert_eq!(test_base64, Storage::build_data(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(), &"base64".to_string()));
    assert_eq!("base64", base64.get_data_type());
  }

  #[test]
  fn check_get_data_ascii() {
    let base64 = Storage::new(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(), &"base64".to_string());
    let test_base64: Vec<u8> = (0u8..64).collect();

    assert_eq!(&test_base64, base64.get_data());
    assert_eq!(test_base64, Storage::build_data(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".to_string(), &"base64".to_string()));
    assert_eq!("base64", base64.get_data_type());
  }

  
}

/*
  // Test 1
  let hex1 = storage::Storage::new(&"0123456789ABCDEF".to_string(), &"hex".to_string());
  let test_hex1 = vec!(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
  println!("Test 1 -- testing str_to_vec");
  println!("output = Vec<u8> {{01, 23, 45, 67, 89, AB, CD, EF}}");
  // println!("Result: {}, Type: {}", hex1.get_data() == test_hex1, hex1.get_data_type());
  println!("Result: {}", hex1.get_data() == test_hex1);
  println!();

  // Test 2
  let hex2 = storage::Storage::new(&"012".to_string(), &"hex".to_string());
  let test_hex2 = vec!(0x00, 0x01, 0x02);
  println!("Test 2 -- odd str_to_vec"); 
  println!("output = Vec<u8> {{0x01, 0x20}}");
  println!("Result: {}", hex2.get_data() == test_hex2);
  println!();
  
  // Test 3
  let mut hex3 = storage::Storage::new(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string(), &"hex".to_string());
  println!("Test 3 -- testing str_to_vec and print_hex_vec");
  println!("Ans: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  print!("Res: ");
  hex3.print();
  println!();

  // Test 4
  hex3.change_base(&"base64".to_string());
  println!("Test 4 -- hex to base64");
  println!("Ans: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  print!("Res: ");
  hex3.print();
  println!();

  // Test 5
  hex3.change_base(&"hex".to_string());
  println!("Test 5 -- now change back, base64 to hex");
  println!("Ans: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  print!("Res: ");
  hex3.print();
  println!();

*/
