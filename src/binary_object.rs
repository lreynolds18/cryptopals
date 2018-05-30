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
  pub fn set_data(mut self, str_inp: &str, data_type: &str) {
    self.data = BinaryObject::build_data(str_inp, data_type);
    self.data_type = String::from(data_type);
  }
  
  /* get_data -- helper function to get self.data
   * Parameters: void
   * Return: self.data (Vec<u8>) - data in vector format
   */
  pub fn get_data(self) -> Vec<u8> {
    return self.data;
  }

  /* get_data_type -- helper function to get self.data_type
   * Parameters: void
   * Return: self.data_type (&str) - data_typ 
   */
  // pub fn get_data_type(self) -> &str {
  //   return &self.data_type[..];
  // }

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

    return output;
  }

  /* base64char_to_u8 -- helper function to convert base64 char to u8
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

  /* print -- helper function to print self.data Vec<u8>
   * Parameters: void 
   * Return: void
   */
  pub fn print(self) {
    println!("{}", BinaryObject::to_string(self));
  }

  /* to_string -- helper function to convert self.data Vec<u8> to string
   * Parameters: void 
   * Return: out (&str) - Hex/Base64 data in string format 
   */
  pub fn to_string(self) -> String {
    let mut out = String::from("");

    for (i, item) in self.data.iter().enumerate() {
      if self.data_type == "hex" {
        out.push_str(&format!("{:x}", item));
      } else if self.data_type == "base64" {
        out.push(BinaryObject::base64u8_to_char(*item));
      }
    }
    return out;
  }
}
