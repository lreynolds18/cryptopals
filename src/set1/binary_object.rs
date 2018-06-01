pub struct BinaryObject {
  data: Vec<u8>,
  data_type: String,
}

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
   *         ending (&str) - number from 0-3 that represents how much room is
   *                         left in the last element of self.data
   */
  fn build_data(str_inp: &str, data_type: &str) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    let mut item: u8 = 0;

    for c in str_inp.chars() {
      if data_type == "hex" {
        // convert hex decimal char (0-9, A-F) to u8 (00000000-00001111)
        item = match u8::from_str_radix(&c.to_string(), 16) {
            Ok(result) => result,
            Err(error) => {
              panic!("There was a problem with from_str_radix: {:?}", error)
            },
        };
      } else if data_type == "base64" {
        item = BinaryObject::base64char_to_u8(c);
      }
      output.push(item);
    }

    output
  }

  /* base64char_to_u8 -- helper function to convert base64 char to u8
   * Parameters: c (char) - Character between A-Z, a-z, 0-9, +, /
   * Return: u (u8) - binary representation of character (000000-111111)  
   */
  fn base64char_to_u8(c: char) -> u8 {
    let u = c as u8;
    match u {
      65...90 => { u - 65 }, // A - Z 
      97...122 => { u - 71 }, // a - z
      48...57 => { u + 4 }, // 0 - 9
      43 => { 62 }, // +
      47 => { 63 }, // /
      _ => { panic!("Error: this is not a valid base64 digit") }
    }
  }

  /* base64u8_to_char -- helper function to convert base64 u8 to char
   * Parameters: u (u8) - binary representation of character (000000-111111)
   * Return: u (u8) - Character between A-Z, a-z, 0-9, +, /
   */
  fn base64u8_to_char(u: u8) -> char {
    match u {
      0...25 => { (u + 65) as char }, // A - Z
      26...51 => { (u + 71) as char }, // a - z
      52...61 => { (u - 4) as char }, // 0 - 9
      62 => { '+' }, // +
      63 => { '/' }, // /
      _ => { panic!("Error: this is not a valid base64 digit") }
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
    let mut out = String::from("");

    for item in &self.data {
      if self.data_type == "hex" {
        out.push_str(&format!("{:x}", item));
      } else if self.data_type == "base64" {
        out.push(BinaryObject::base64u8_to_char(*item));
        // out.push(&set1::binary_object::BinaryObject::base64u8_to_char(*item));
      }
    }
    out
  }

  /* to_string -- convert old_base to new_base
   * concurrently handles hex -> base64 and base64 -> hex
   * changes self.data and self.data_type in struct
   * Parameters: new_base (&str) - New base to convert old base to
   * Return: void 
   */
  pub fn change_base(&mut self, new_base: &str) {
    // TODO: make dry, lots of repetition
    if self.data_type != new_base {
      let mut output: Vec<u8> = Vec::new();
      let mut temp: u8 = 0x00;
      let mut ending: u8 = 0;

      if self.data_type == "hex" && new_base == "base64" {
        // hex -> base64
        if self.data.len() % 3 != 0 {
          panic!("Error: hex input does not fit nicely into base64.");
        }
        for item in &self.data {
          if ending == 0 {
            // nothing in item
            temp = item << 2;
            // temp now has 4 bits in it (00****00)
            ending = 1;
          } else if ending == 1 {
            temp |= (item & 0x0C) >> 2;
            // fill first 2 bits in temp and push
            // temp has 6 bits (00******)
            output.push(temp);
            temp = (item & 0x03) << 4;
            // push remaining two bits in temp
            // temp has 2 bits (00**0000)
            ending = 2;
          } else {
            temp |= item;
            // temp has 6 bits (00******)
            output.push(temp);
            ending = 0;
            // temp now has 0 bits (00000000)
          }
        }
      } else if self.data_type == "base64" && new_base == "hex" {
        // base64 -> hex
        if self.data.len() % 2 != 0 {
          panic!("Error: base64 does not fit nicely into hex.");
        }
        for item in &self.data {
          if ending == 0 {
            // we want to add first 4 bits to self.data
            // we want to push the remaining 2 bits to temp
            temp = (item & 0x3C) >> 2;
            // temp has 4 bits (0000****)
            output.push(temp);
            temp = (item & 0x03) << 2; 
            // temp has 2 bits (0000**00)
            ending = 1;
          } else if ending == 1 {
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
      }
      self.data = output;
      self.data_type = new_base.to_string();
    }
  }
}
