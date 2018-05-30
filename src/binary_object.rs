pub struct BinaryObject {
  data: Vec<u8>,
  data_type: String,
  ending: u8 // for hex, ending has two states: 0 if end on last 4 (even)
                                             // 1 if end on first 4 (odd)
              // for base64, ending has four states: 0 if end on last 6 
                                                  // 1 if end on first 6
                                                  // 2 if end on last 4 + first 2
                                                  // 3 if end on last 6
             // for hex, we are storing 4 bits for each char
             // for base64, we are storing 6 bits for each char
             // we are fitting those bits in a 8 bit object
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
    let (data, ending) = BinaryObject::build_data(str_inp, data_type);

    BinaryObject {
      data: data,
      data_type: String::from(data_type),
      ending: ending
    }
  }
  

  /* set_data -- helper function to set self.data and self.data_type
   * Parameters: str_inp (&str) - input string, 
   *             data_type (&str) - data type of input string (hex or base64)
   * Return: void 
   */
  pub fn set_data(mut self, str_inp: &str, data_type: &str) {
    let (data, ending) = BinaryObject::build_data(str_inp, data_type);
    self.data = data;
    self.ending = ending;
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
  fn build_data(str_inp: &str, data_type: &str) -> (Vec<u8>, u8) {
    let mut output: Vec<u8> = Vec::new();

    let mut temp: u8 = 0;
    let mut ending: u8 = 0; 
    let mut item: u8;

    for c in str_inp.chars() {
      if data_type == "hex" {
        // convert hex decimal char (0-9, A-F) to u8 (00000000-00001111)
        item = match u8::from_str_radix(&c.to_string(), 16) {
            Ok(result) => result,
            Err(error) => {
              panic!("There was a problem with from_str_radix: {:?}", error)
            },
        };
    
        match ending {
          0 => {
            // temp has nothing in it so we want to push the digit to the left side
            // i.e. so we have (00000000-11110000)
            temp = item << 4;
            ending = 1;
          },
          1 => {
            // temp has digit on left side so we push to right side 
            // (00000000-11111111)
            temp |= item;
            output.push(temp);
            temp = 0;
            ending = 0;
          },
          _ => {
            panic!("Error: there was problem with adding Hex String to Vec");
          }
        }
      } else if data_type == "base64" {
        item = BinaryObject::base64char_to_u8(c);

        // for base64, ending has four states: 0 if end on last 6 
                                            // 1 if end on first 6
                                            // 2 if end on last 4 + first 2
                                            // 3 if end on last 6
        // (first 6), (last 2 + first 4), (last 4 + first 2), (last 6)
        match ending {
          0 => {
            // temp has nothing in it so we push the 6 bits all the way to left
            // first 6
            temp = item << 2; // (00111111 << 2) = 11111100
            ending = 1;
          },
          1 => {
            // temp has first 6 bits in it so we push 2 bits in temp and add to vector
            // then we push 4 bits to temp 
            // (last 2 + first 4)
            temp |= (item & 0x30) >> 4; // 11111100 | ((00111111 & 00110000) >> 4) = 11111111
            output.push(temp);
            temp = (item & 0x0F) << 4; // (00111111 & 00001111) << 4 = 11110000
            ending = 2;
          },
          2 => {
            // temp has 4 bits in it so we push the first 4 digits to temp and add to vector
            // then we push 2 bits to temp
            // (last 4 + first 2)
            temp |= (item & 0x3C) >> 2; // 11110000 | ((00111111 & 00111100) >> 2) = 11111111
            output.push(temp);
            temp = (item & 0x03) << 6; // (00111111 & 00000011) << 6 = 11000000
            ending = 3;
          },
          3 => {
            // ending has 2 bits in it so we add our new digit to temp and push to vector
            // last 6
            temp &= item; // 11000000 & 00111111 = 11111111
            output.push(temp);
            ending = 0;
          },
          _ => {
            panic!("Error: there was problem with adding base64 String to Vec");
          }
        }
      }
    }

    if data_type == "hex" && ending == 1 { 
      // if we have a odd number of hexidecimals then push to vec
      // temp has 4 bits of information in it
      // (11110000)
      output.push(temp);
    } else if data_type == "base64" && 
      (ending == 1 || ending == 2 || ending == 3) {
      // push to vector if temp has something left in it
      // temp has either has 6 bits, 4 bits, or 2 bits of info in it
      // depending on if it's 1, 2, 3 respectfully
      // (11111100, 11110000, 11000000)
      output.push(temp);
    }

    return (output, ending);
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
    // let mut end: u8 = 0;
    let len = self.data.len();

    for (i, item) in self.data.iter().enumerate() {
      if self.data_type == "hex" {
        if i == len && self.ending == 1 {
          out.push_str(&format!("{:x}", item >> 4)); 
        } else {
          out.push_str(&format!("{:x}", item));
        }
      } else if self.data_type == "base64" {
        // ignore for now
        // TODO: implement base64 print
      }
    }
    return out;
  }
}
