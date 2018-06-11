use std::ops;

// TODO: ownership? who owns what and why
// TODO: use .iter().map().collect() instead of for item in &self.data????
// TODO: clean up change base
// TODO: use boxes and error checking
// TODO: figure out error message in constructor

pub struct Storage {
  data: Vec<u8>,
  data_type: String,
}

impl Storage {
  /* new -- empty constructor for storage 
   * Parameters: void
   * Return: Storage (w/ data and data_type)
   */
  pub fn new() -> Storage {
    Storage {
      data: Vec::new(),
      data_type: String::new()
    }
  }


  /* new_init -- constructor for storage 
   * converts string to vec<u8>
   * assuming str_inp is in it's respected format of data_type (hex / base64)
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: Storage (w/ data and data_type)
   */
  pub fn new_init(str_inp: &str, data_type: &str) -> Storage {
    Storage {
      data: Storage::build_data(str_inp, data_type),
      data_type: String::from(data_type),
    }
  }


  /* build_data -- helper function to build self.data
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: self.data (Vec<u8>) - vector representation of our str_inp
   */
  fn build_data(str_inp: &str, data_type: &str) -> Vec<u8> {
    if data_type != "hex" && data_type != "base64" && data_type != "ascii" {
      panic!("Error: invalid type ({})", data_type);
    }

    let mut data: Vec<u8> = Vec::new();

    for c in str_inp.chars() {
      data.push(Storage::char_to_u8(c, data_type));
    }
    data
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


  /* hamming_distance-- helper function to calculate the hamming distance between two storages
   * Parameters: lhs (&Storage) - left hand side storage,
   *             rhs (&Storage) - rigth hand side storage
   * Return: out (i32) - number of bits that are different between the two storages
   */
  pub fn hamming_distance(lhs: &Storage, rhs: &Storage) -> i32 {
    if lhs.get_data().len() != rhs.get_data().len() {
      panic!("Error: cannot compute hamming distance when the strings are not \
        the same length. LHS length is {}, RHS length is {}", lhs.get_data().len(), rhs.get_data().len());
    }

    if lhs.get_data_type() != rhs.get_data_type() {
      panic!("Error: cannot compute hamming distance when the data types are not \
        the same.  LHS type is {}, RHS type is {}", lhs.get_data_type(), rhs.get_data_type());
    }

    let start = match lhs.get_data_type().as_str() {
      "ascii" => 0,  // ********
      "hex" => 4,    // 0000****
      "base64" => 2, // 00******
      _ => { panic!("Error: invalid data type"); }
    };

    lhs.data.iter()
            .zip(rhs.data.iter())
            .map(|(l, r)| {
                let tmp = l ^ r;
                let mut c: i32 = 0;
                let bin: Vec<u8> = vec!(0x90, 0x40, 0x20, 0x10, 0x09, 0x04, 0x02, 0x01);
                for (i, var) in bin.iter().enumerate() {
                  if i >= start { 
                    c += ((tmp & var) >> (7-i as u8)) as i32;
                  }
                }
                c
              }
            )
            .sum()
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
   * Return: out (String) - Hex/Base64/Ascii data in string format 
   */
  pub fn to_string(&self) -> String {
    let mut out = String::new();

    for item in &self.data {
      out.push(Storage::u8_to_char(*item, self.data_type.as_str()));
    }
    out
  }


  // TODO: implement as_str.  Complains about lifetime
  /* as_str -- helper function to convert self.data Vec<u8> to &str
   * Parameters: void 
   * Return: out (&str) - Hex/Base64 data in string format 
   */
  /*
  pub fn as_str(&self) -> &str {
    let mut out = String::new();

    for item in &self.data {
      out.push(Storage::u8_to_char(*item, self.data_type.as_str()));
    }
    &out
  }
  */


  /* change_base -- convert old_base to new_init_base
   * handles hex -> base64, base64 -> hex, ascii -> hex, hex -> ascii,
   * ascii -> base64, base64 -> ascii
   * changes self.data and self.data_type in struct
   * Parameters: new_init_base (&str) - New base to convert old base to
   * Return: void 
   */
  pub fn change_base(&mut self, new_init_base: &str) {
    // REFACTOR

    if self.data_type != new_init_base {

      let mut output: Vec<u8> = Vec::new();
      let mut temp: u8 = 0x00;

      if self.data_type == "hex" && new_init_base == "base64" {
        // hex -> base64

        // if converting to base64, we add elements to the vec by 3 hex values
        // 00001111, 00001122, 00002222 -> 00111111, 00222222
        if new_init_base == "base64" && self.data.len() % 3 != 0 {
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

      } else if self.data_type == "base64" && new_init_base == "hex" {
        // base64 -> hex

        // if converting to hex, we add elements to the vec by 2 base64 values
        // 00111122 00223333 -> 00001111, 00002222, 00003333
        if new_init_base == "hex" && self.data.len() % 2 != 0 {
          panic!("Error: base64 doesn't fit nicely into hex.");
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
      } else if self.data_type == "hex" && new_init_base == "ascii" {
        // hex -> ascii
        if self.data.len() % 2 != 0 {
          panic!("Error: hex doesn't fit nicely into an ASCII string");
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
      } else if self.data_type == "ascii" && new_init_base == "hex" {
        // ascii -> hex
        for item in &self.data {
          // push first 4 bits to vec (****0000) >> 4 = (0000****)
          output.push((item & 0xF0) >> 4);
          // push last 4 bits to vec (0000****)
          output.push(item & 0x0F);
        }
      } else if self.data_type == "base64" && new_init_base == "ascii" {
        // base64 -> ascii
        if self.data.len() % 4 != 0 {
          panic!("Error: base64 doesn't fit nicely into an ASCII string");
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
      } else if self.data_type == "ascii" && new_init_base == "base64" {
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
      } else {
        panic!("Error: unsupported opeartion to convert {} base into {} base", 
          self.data_type, new_init_base);
      }

      self.data = output;
      self.data_type = new_init_base.to_string();
    }
  }

  /* split_by_keysize -- returns two storages with the first keysize elements
   * and the second keysize elements.
   * Parameters: keysize (usize) - Number of characters that we want to split by
   * Return: (Storage, Storage) - lhs is first keysize elements in a Storage and rhs 
   * is second keysize elements in a Storage
   */
  pub fn split_by_keysize(&self, keysize: usize) -> (Storage, Storage) {
    // TODO: make sure indexing is correct (2 * keysize) 
    // TODO: for loop can forsure be written better
    // TODO: vec or tuple for return?
    // TODO: write description a little better

    if self.data.len() < 2 * keysize {
      panic!("Error: not enough items in data");
    }

    let mut lhs: Vec<u8> = vec!();
    let mut rhs: Vec<u8> = vec!();

    for (i, d) in self.data.iter().cloned().enumerate() {
      if i < keysize {
        lhs.push(d);
      } else if i < keysize * 2 {
        rhs.push(d);
      } else {
        break;
      }
    }

    (
      Storage {
        data: lhs,
        data_type: self.get_data_type().to_string()
      },
      Storage {
        data: rhs,
        data_type: self.get_data_type().to_string()
      }
    )
  }


  /* split_into_blocks -- splits a storage into keysizes and then splits each keysize into blocks
   * Parameters: keysize (usize) - Number of characters that we want to split by
   * Return: out Vec<Storage> - Vector of Storage where each Storage contains the nth elements in each keysize
   */
  pub fn split_into_blocks(&self, keysize: usize) -> Vec<Storage> {
    // TODO: add some kind of functionality incase len doesn't evenly go into keysize
    if self.data.len() % keysize != 0 {
      panic!("Error: cannot evenly distribute blocks by keysize.  Keysize is {}. Data.len() is {}", keysize, self.data.len());
    }

    // create an empty Vec<Vec<u8>> with the length of keysize
    let mut holder: Vec<Vec<u8>> = vec!();
    for _i in 0..keysize {
      holder.push(vec!());
    }

    // add the nth item to the respective vec in holder
    // if data contains "helloworld" then w/ keysize 5
    // the result should be
    // "hw", "eo", "lr", "ll", "od"
    // because we split "helloworld" into "hello" and "world"
    // then we append the first characters to the first vec...
    for (i, d) in self.data.iter().cloned().enumerate() {
      holder[i % keysize].push(d);
    }

    // create Storage with the results
    let mut out: Vec<Storage> = vec!();
    for v in &holder {
      out.push(
        Storage {
          data: v.to_vec(),
          data_type: self.get_data_type().to_string()
        }
      );
    }
    out
  }
}

// XOR implementation for Storage ^ Storage = Storage
// handles repeating XOR so if lhs is bigger than rhs
// it will repeatable XOR the rhs on the lhs
// we want to always pass references
impl<'a> ops::BitXor<&'a Storage> for &'a Storage {
  type Output = Storage;

  fn bitxor(self, rhs: &Storage) -> Storage {
    if self.data_type == "" || rhs.data_type == "" {
      panic!("Error: cannot XOR on empty storage");
    }

    if self.data_type != rhs.data_type {
      panic!("Error: Storage are not the same type.  \
        LHS type is {}. RHS type is {}.", self.data_type, rhs.data_type);
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
      panic!("Error: Storage cannot be XOR'd against each other.  \
        LHS length is {}, RHS length is {}", self.data.len(), rhs.data.len());
    }
  }
}



#[cfg(test)]
mod tests {
  use super::*;

  // TEST EMPTY CONSTRUCTOR - new
  #[test]
  fn create_blank_storage() {
    Storage::new();
  }
 
  
  // TEST INIT CONSTRUCTOR - new_init
  #[test]
  fn check_init_constructor() {
    Storage::new_init("0123456789abcdef", "hex");
    Storage::new_init(
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+",
      "base64"
    );
    Storage::new_init(
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,./;'[]<>?:\"{{}}-_=+`~!@#$%^&*()", 
      "ascii"
    );
  }

  #[test]
  #[should_panic]
  fn check_invalid_init_constructor() {
    Storage::new_init("asdbasdf", "invalid type");
  }


  // TEST BUILD DATA - build_data
  #[test]
  fn check_build_data() {
    let hex_vec = vec!(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                       0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x0A, 0x0B,
                       0x0C, 0x0D, 0x0E, 0x0F);

    assert_eq!(hex_vec, Storage::build_data("0123456789ABCDEFabcdef", "hex"));

    let mut base64_vec = Vec::new();
    for i in 0u8..64u8 {
      base64_vec.push(i);
    }

    assert_eq!(base64_vec, Storage::build_data(
      "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
      "base64"
    ));

    let mut ascii_vec = Vec::new();
    for i in 32u8..127u8 {
      ascii_vec.push(i);
    }
    
    assert_eq!(ascii_vec, Storage::build_data(
      " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
      "ascii"
    ));
  }

  #[test]
  #[should_panic]
  fn check_invalid_type_build_data() {
    Storage::build_data("asdbasdf", "invalid type");
  }

  #[test]
  #[should_panic]
  fn check_invalid_hex_build_data() {
    Storage::build_data("123AMNJKHDGWU12", "hex");
  }

  #[test]
  #[should_panic]
  fn check_invalid_base64_build_data() {
    Storage::build_data("abAB@$%$@%)(-=+ab", "base64");
  }


  // TEST set_data, get_data, get_data_type, to_string
  #[test]
  fn check_set_and_get1() {
    let mut s = Storage::new();
    let blank_vec: Vec<u8> = Vec::new();
    assert_eq!("", s.to_string());
    assert_eq!("", s.get_data_type());
    assert_eq!(&blank_vec, s.get_data());

    let hex_vec = vec!(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                       0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x0A, 0x0B,
                       0x0C, 0x0D, 0x0E, 0x0F);
    s.set_data("0123456789abcdefABCDEF", "hex");
    assert_eq!("0123456789abcdefabcdef", s.to_string());
    assert_eq!("hex", s.get_data_type());
    assert_eq!(&hex_vec, s.get_data());
  }
  
  #[test]
  fn check_set_and_get2() { 
    let mut s = Storage::new_init("abcd", "ascii");
    let hex_vec = vec!(0x00, 0x01, 0x02, 0x03);
    s.set_data("0123", "hex");
    assert_eq!("0123", s.to_string());
    assert_eq!("hex", s.get_data_type());
    assert_eq!(&hex_vec, s.get_data());

    s.set_data("ABCabc123+/", "base64");
    let base64_vec = vec!(0x00, 0x01, 0x02, 0x1A, 0x1B, 0x1C,
                          0x35, 0x36, 0x37, 0x3E, 0x3F);
    assert_eq!("ABCabc123+/", s.to_string());
    assert_eq!("base64", s.get_data_type());
    assert_eq!(&base64_vec, s.get_data());

    s.set_data("tESt One!32/(*&", "ascii");
    let ascii_vec = vec!(0x74, 0x45, 0x53, 0x74, 0x20, 
                         0x4F, 0x6E, 0x65, 0x21, 0x33,
                         0x32, 0x2F, 0x28, 0x2A, 0x26);
    assert_eq!("tESt One!32/(*&", s.to_string());
    assert_eq!("ascii", s.get_data_type());
    assert_eq!(&ascii_vec, s.get_data());
  }

  
  // TEST HAMMING DISTANCE hamming_distance
   #[test]
  fn check_hamming_distance_ascii() {
    let lhs = Storage::new_init("this is a test", "ascii");
    let rhs = Storage::new_init("wokka wokka!!!", "ascii");

    assert_eq!(37, Storage::hamming_distance(&lhs, &rhs));
  }

  #[test]
  fn check_hamming_distance_hex() {
    let lhs = Storage::new_init("0123456789ABCDEF", "hex");
    let rhs = Storage::new_init("FEDCBA9876543210", "hex");

    assert_eq!(64, Storage::hamming_distance(&lhs, &rhs));
  }

  #[test]
  fn check_hamming_distance_base64() {
    let lhs = Storage::new_init("ABCDEF", "base64");
    let rhs = Storage::new_init("abcdef", "base64");

    assert_eq!(20, Storage::hamming_distance(&lhs, &rhs));
  }


  // TEST char_to_u8
  #[test]
  fn check_char_to_u8_hex() {
    let mut hex_vec = Vec::new();
    for i in 0u8..16u8 {
      hex_vec.push(i);
    } 

    for (ch, u) in "0123456789abcdefABCDEF".chars().zip(hex_vec) {
      assert_eq!(u, Storage::char_to_u8(ch, "hex"));
    }
  }

  #[test]
  #[should_panic]
  fn check_invalid_char_to_u8_hex() {
    Storage::char_to_u8('Z', "hex");
  }
 
  #[test]
  fn check_char_to_u8_base64() {
    let mut base64_vec = Vec::new();
    for i in 0u8..64u8 {
      base64_vec.push(i);
    } 

    for (ch, u) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().zip(base64_vec) {
      assert_eq!(u, Storage::char_to_u8(ch, "base64"));
    }
  }

  #[test]
  #[should_panic]
  fn check_invalid_char_to_u8_base64() {
    Storage::char_to_u8('!', "base64");
  }

  #[test]
  fn check_char_to_u8_ascii() {
    let mut ascii_vec = Vec::new();
    for i in 32u8..127u8 {
      ascii_vec.push(i);
    }

    for (ch, u) in " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".chars().zip(ascii_vec) {
      assert_eq!(u, Storage::char_to_u8(ch, "ascii"));
    }
  }


  // TEST u8_to_char
  #[test]
  fn check_u8_to_char_hex() {
    let mut hex_vec = Vec::new();
    for i in 0u8..16u8 {
      hex_vec.push(i);
    } 

    for (ch, u) in "0123456789abcdefABCDEF".chars().zip(hex_vec) {
      assert_eq!(ch, Storage::u8_to_char(u, "hex"));
    }
  }

  #[test]
  #[should_panic]
  fn check_invalid_u8_to_char_hex() {
    Storage::u8_to_char(0xFF, "hex");
  }
 
  #[test]
  fn check_u8_to_char_base64() {
    let mut base64_vec = Vec::new();
    for i in 0u8..64u8 {
      base64_vec.push(i);
    } 

    for (ch, u) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".chars().zip(base64_vec) {
      assert_eq!(ch, Storage::u8_to_char(u, "base64"));
    }
  }

  #[test]
  #[should_panic]
  fn check_invalid_u8_to_char_base64() {
    Storage::u8_to_char(0xFF, "base64");
  }

  #[test]
  fn check_u8_to_char_ascii() {
    let mut ascii_vec = Vec::new();
    for i in 32u8..127u8 {
      ascii_vec.push(i);
    }

    for (ch, u) in " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".chars().zip(ascii_vec) {
      assert_eq!(ch, Storage::u8_to_char(u, "ascii"));
    }
  }


  // TEST change base - change_base
  // TODO: test change base
  #[test]
  fn check_hex_to_base64() {
  }

  #[test]
  fn check_base64_to_hex() {
  }

  #[test]
  fn check_hex_to_ascii() {
  }

  #[test]
  fn check_ascii_to_hex() {
  }

  #[test]
  fn check_base64_to_ascii() {
  }

  #[test]
  fn check_ascii_to_base64() {
  }

  #[test]
  #[should_panic]
  fn check_invalid_change_base() {
    let mut s = Storage::new();
    s.change_base("hex");
  }

  #[test]
  fn check_change_base() {
    let mut hex = Storage::new_init(
      "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", 
      "hex"
    );
    hex.change_base("base64");
    assert_eq!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", hex.to_string());
    hex.change_base("hex");
    assert_eq!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", hex.to_string());
  }


  // TEST SPLIT ON KEYSIZE INTO SEPERATE STRING - split_on_keysize_into_seperate_string 
  // TODO: add tests and test invalid cases
  #[test]
  fn check_split_by_keysize() {
    let s = Storage::new_init("hello world", "ascii");
    let ans_vec = vec!(
      vec!("h", "e"),         // 1
      vec!("he", "ll"),       // 2
      vec!("hel", "lo "),     // 3
      vec!("hell", "o wo"),   // 4
      vec!("hello", " worl")  // 5
    );

    for i in 1usize..6usize {
      let (lhs, rhs) = s.split_by_keysize(i);
      assert_eq!(ans_vec[i-1][0], lhs.to_string());
      assert_eq!(ans_vec[i-1][1], rhs.to_string());
    }
  }

  // TEST SPLIT INTO BLOCKS - split_into_blocks
  // TODO: add tests and test invalid cases
  #[test]
  fn check_split_into_blocks() {
    let s = Storage::new_init("helloworld", "ascii");
  
    let test1_res = s.split_into_blocks(1);
    assert_eq!("helloworld", test1_res[0].to_string());

    let test2_res = s.split_into_blocks(2);
    assert_eq!("hlool", test2_res[0].to_string());
    assert_eq!("elwrd", test2_res[1].to_string());

    let test3_res = s.split_into_blocks(5);
    assert_eq!("hw", test3_res[0].to_string());
    assert_eq!("eo", test3_res[1].to_string());
    assert_eq!("lr", test3_res[2].to_string());
    assert_eq!("ll", test3_res[3].to_string());
    assert_eq!("od", test3_res[4].to_string());
  }


  // TEST XOR - overloaded Bitwise XOR operator
  // TODO: write tests for XOR (full + repeating) and invalid
  #[test]
  fn check_xor_full_noref() {
     
    assert_eq!("", "");
  }

  #[test]
  fn check_xor_one_char_repeating() {
  }

  #[test]
  fn check_xor_multi_char_repeating() {
  }

  #[test]
  #[should_panic]
  fn check_invalid_hex_operation() {
    let lhs: Storage = Storage::new();
    let rhs: Storage = Storage::new();
    &lhs ^ &rhs;
  }



}
