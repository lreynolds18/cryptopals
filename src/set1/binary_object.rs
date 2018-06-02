use std::ops;

pub struct BinaryObject {
  data: Vec<u8>,
  data_type: String,
}

// XOR implementation for BinaryObject ^ BinaryObject = BinaryObject
// handles repeating XOR so if lhs is bigger than rhs
// it will repeatable XOR the rhs on the lhs
impl ops::BitXor for BinaryObject {
  type Output = Self;

  fn bitxor(self, rhs: Self) -> Self {
    if self.data_type != rhs.data_type {
      panic!("Error: Binary Objects are not the same type.  LHS type is {}. RHS type is {}.", self.data_type, rhs.data_type);
    }
  
    if self.data.len() == rhs.data.len() {
      BinaryObject {
        data: self.data.iter()
                       .zip(rhs.data.iter())
                       .map(|(l, r)| l ^ r)
                       .collect(),
        data_type: self.data_type
      }
    } else if self.data.len() % rhs.data.len() == 0 {
      let mut out: Vec<u8> = Vec::new();
      let rhs_len: i64 = rhs.data.len() as i64;
        
      for (i, item) in self.data.iter().enumerate() {
        out.push(item ^ rhs.data[((i as i64) % rhs_len) as usize]);
      }
      BinaryObject {
        data: out,
        data_type: self.data_type
      }
    } else {
      panic!("Error: Binary Objects cannot be XOR'd against each other");
    }
  }
}

// TODO: ownership? who owns what and why
// -- get_data method is a problem
impl BinaryObject {

  /* new -- constructor for binary_object 
   * converts string to vec<u8>
   * assuming str_inp is in it's respected format of data_type (hex / base64)
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: BinaryObject (w/ data, data_type, and ending)
   */
  pub fn new(str_inp: &str, data_type: &str) -> BinaryObject {
    BinaryObject {
      data: BinaryObject::build_data(str_inp, data_type),
      data_type: String::from(data_type),
    }
  }
  
  /* set_data -- helper function to set self.data and self.data_type
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: void 
   */
  pub fn set_data(&mut self, str_inp: &str, data_type: &str) {
    self.data = BinaryObject::build_data(str_inp, data_type);
    self.data_type = String::from(data_type);
  }
  
  /* get_data -- helper function to get self.data
   * Parameters: void
   * Return: self.data (Vec<u8>) - data in vector format
   */
  pub fn get_data(self) -> Vec<u8> {
    self.data
  }

  /* get_data_type -- helper function to get self.data_type
   * Parameters: void
   * Return: self.data_type (&str) - data_typ 
   */
  pub fn get_data_type(&self) -> &str {
    self.data_type.as_str()
  }

  /* build_data -- helper function to build self.data
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: self.data (Vec<u8>) - vector representation of our str_inp
   */
  fn build_data(str_inp: &str, data_type: &str) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();

    for c in str_inp.chars() {
      data.push(BinaryObject::char_to_u8(c, data_type));
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
      match u {
        48...57 => { u - 48 }, // 0 - 9
        97...102 => { u - 87 }, // a - f
        65...70 => { u - 55 }, // A - F
        _ => { panic!("Error: this is not a valid hex digit") }
      }
    } else {
    // } else if data_type == "base64" {
      match u {
        65...90 => { u - 65 }, // A - Z 
        97...122 => { u - 71 }, // a - z
        48...57 => { u + 4 }, // 0 - 9
        43 => { 62 }, // +
        47 => { 63 }, // /
        _ => { panic!("Error: this is not a valid base64 digit") }
      }
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
      match u {
        0...9 => { (u + 48) as char }, // 0 - 9
        10...15 => { (u + 87) as char }, // a - f
        _ => { panic!("Error: this is not a valid hex digit") }
      }
    } else {
    // } else if data_type == "base64" {
      match u {
        0...25 => { (u + 65) as char }, // A - Z
        26...51 => { (u + 71) as char }, // a - z
        52...61 => { (u - 4) as char }, // 0 - 9
        62 => { '+' }, // +
        63 => { '/' }, // /
        _ => { panic!("Error: this is not a valid base64 digit") }
      }
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
      out.push(BinaryObject::u8_to_char(*item, self.data_type.as_str()));
    }
    out
  }

  /* to_string -- helper function to convert self.data Vec<u8> to ASCII string
   * Parameters: void 
   * Return: out (&str) - Hex/Base64 data in string format 
   */
  pub fn to_ascii_string(&self) -> String {
    let mut out = String::new();
    let mut tmp: u8 = 0;
    let mut ending = 0;
  
    // TODO: refactor ending out?  use index instead -- i%2==1 i&2==0
    if self.data_type == "hex" && self.data.len() % 2 == 0 {
      for hex in &self.data {
        if ending == 0 {
          tmp = hex << 4;
          ending = 1;
        } else if ending == 1 {
          tmp |= hex;
          out.push(tmp as char);
          ending = 0;
        } 
      }
    } else if self.data_type == "base64" && self.data.len() % 4 == 0 {
      for base64 in &self.data {
        if ending == 0 {
          // tmp starts as 0x00000000
          tmp = base64 << 2;
          // tmp is now 0x******00
          ending = 1;
        } else if ending == 1 {
          // tmp starts as 0x******00
          tmp |= (base64 & 0x30) >> 4;
          // tmp is now 0x********
          out.push(tmp as char);
          // tmp is now 0x00000000
          tmp = (base64 & 0x0F) << 2;
          // tmp is now 0x****0000
          ending = 2;
        } else if ending == 2 {
          // tmp start at 0x****0000
          tmp |= (base64 & 0x3C) >> 2;
          // tmp is now 0x********
          out.push(tmp as char);
          // tmp is now 0x00000000
          tmp = (base64 & 0x03) << 6;
          // tmp is now 0x**000000
          ending = 3;
        } else if ending == 3 {
          // tmp starts as 0x**000000
          tmp |= base64;
          // tmp is now 0x********
          out.push(tmp as char);
          // tmp is now 0x00000000
          ending = 0;
        }
      }
    } else {
      panic!("Error: the data doesn't fit nicely into an ASCII string");
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
    if self.data_type != new_base {
      // if converting to base64, we add elements to the vec by 3 hex values
      // 00001111, 00001122, 00002222 -> 00111111, 00222222
      if new_base == "base64" && self.data.len() % 3 != 0 {
        panic!("Error: hex input does not fit nicely into base64.");
      }

      // if converting to hex, we add elements to the vec by 2 base64 values
      // 00111122 00223333 -> 00001111, 00002222, 00003333
      if new_base == "hex" && self.data.len() % 2 != 0 {
        panic!("Error: base64 does not fit nicely into hex.");
      }

      let mut output: Vec<u8> = Vec::new();
      let mut temp: u8 = 0x00;
      let mut ending: u8 = 0;

      for item in &self.data {
        // hex -> base64
        if new_base=="base64" && ending == 0 {
          // nothing in item
          temp = item << 2;
          // temp now has 4 bits in it (00****00)
          ending = 1;
        } else if new_base=="base64" && ending == 1 {
          temp |= (item & 0x0C) >> 2;
          // fill first 2 bits in temp and push
          // temp has 6 bits (00******)
          output.push(temp);
          temp = (item & 0x03) << 4;
          // push remaining two bits in temp
          // temp has 2 bits (00**0000)
          ending = 2;
        } else if new_base=="base64" && ending == 2{
          temp |= item;
          // temp has 6 bits (00******)
          output.push(temp);
          ending = 0;
          // temp now has 0 bits (00000000)
        }

        // base64 -> hex
        else if new_base == "hex" && ending == 0 {
          // we want to add first 4 bits to self.data
          // we want to push the remaining 2 bits to temp
          temp = (item & 0x3C) >> 2;
          // temp has 4 bits (0000****)
          output.push(temp);
          temp = (item & 0x03) << 2; 
          // temp has 2 bits (0000**00)
          ending = 1;
        } else if new_base == "hex" && ending == 1 {
          // we want to add first 2 bits to temp and then push to self.data
          // we want to add the remaining 4 bits to temp and then push to self.data
          temp |= (item & 0x30) >> 4;
          // temp has 4 bits (0000****)
          output.push(temp);
          temp = item & 0x0F;
          output.push(temp);
          // temp has 0 bits (00000000)
          ending = 0;
        }
      }
      self.data = output;
      self.data_type = new_base.to_string();
    }
  }
}
