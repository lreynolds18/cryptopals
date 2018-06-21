use std::ops;

// TODO: ownership? who owns what and why
// TODO: clean up change base
// TODO: implement display trait instead of to_string and print
// TODO: only use Storage {} dec and only use self.data and self.data_type in here
// TODO: consider to_owned() instead of to_string()

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
            data_type: String::new(),
        }
    }

    /* new_init -- constructor for storage
     * converts string to vec<u8>
     * assuming str_inp is in it's respected format of data_type (hex, base64, or ascii)
     * Parameters: str_inp (&str) - input string,
     *             data_type (&str) - data type of input string (hex, base64, or ascii)
     * Return: Storage (w/ data and data_type)
     */
    pub fn new_init(str_inp: &str, data_type: &str) -> Storage {
        Storage {
            data: Storage::build_data(str_inp, data_type),
            data_type: data_type.to_owned(),
        }
    }

    /* new_init_vec -- constructor for storage
     * Parameters: vec_inp (&vec) - input vector,
     *             data_type (&str) - data type of input string (hex, base64, or ascii)
     * Return: Storage (w/ data and data_type)
     */
    pub fn new_init_vec(vec_inp: &Vec<u8>, data_type: &str) -> Storage {
        Storage {
            data: vec_inp.to_vec(),
            data_type: data_type.to_owned(),
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

        str_inp
            .chars()
            .map(|c| Storage::char_to_u8(c, data_type))
            .collect()
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

    /* set_data_vec -- helper function to set self.data and self.data_type
     * Parameters: str_inp (&str) - input string,
     *             data_type (&str) - data type of input string (hex or base64)
     * Return: void
     */
    pub fn set_data_vec(&mut self, vec_inp: &Vec<u8>, data_type: &str) {
        self.data = vec_inp.to_vec();
        self.data_type = data_type.to_string();
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

    /* len -- helper function to get self.data.len()
     * Parameters: void
     * Return: self.data.len() (usize) - length of data
     */
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /* char_to_u8 -- helper function to convert (hex/base64) char to u8
     *               (note: we don't want self here because we want to be able to use this
     *               outside of this struct / want to use this in constructor)
     * Parameters: c (char) - Character between (0-9, A-F, a-f) or (A-Z, a-z, 0-9, +, /)
     *             data_type (&str) - base of the char to convert to
     * Return: u (u8) - binary representation of character (0000-1111) or (000000-111111)
     */
    pub fn char_to_u8(c: char, data_type: &str) -> u8 {
        let u = c as u8;
        if data_type == "hex" {
            // HEX
            match u {
                48...57 => u - 48,  // 0 - 9
                97...102 => u - 87, // a - f
                65...70 => u - 55,  // A - F
                _ => panic!("Error: this is not a valid hex digit"),
            }
        } else if data_type == "base64" {
            // BASE64
            match u {
                65...90 => u - 65,  // A - Z
                97...122 => u - 71, // a - z
                48...57 => u + 4,   // 0 - 9
                43 => 62,           // +
                47 => 63,           // /
                61 => 255,          // = (padding character)
                _ => panic!("Error: this is not a valid base64 digit"),
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
    pub fn u8_to_char(u: u8, data_type: &str) -> char {
        if data_type == "hex" {
            // HEX
            match u {
                0...9 => (u + 48) as char,   // 0 - 9
                10...15 => (u + 87) as char, // a - f
                _ => panic!("Error: this is not a valid hex digit"),
            }
        } else if data_type == "base64" {
            // BASE64
            match u {
                0...25 => (u + 65) as char,  // A - Z
                26...51 => (u + 71) as char, // a - z
                52...61 => (u - 4) as char,  // 0 - 9
                62 => '+',                   // +
                63 => '/',                   // /
                255 => '=',                  // = (padding character)
                _ => panic!("Error: this is not a valid base64 digit"),
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
        self.data
            .iter()
            .map(|i| Storage::u8_to_char(*i, self.data_type.as_str()))
            .collect()
    }

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
                    if i % 2 == 0 {
                        // temp has 0 bits (00000000)
                        temp = item << 4;
                    // temp has 4 bits (****0000)
                    } else if i % 2 == 1 {
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
                    panic!(
                        "Error: base64 doesn't fit nicely into an ASCII string. \
                         Length of base64 is {}",
                        self.data.len()
                    );
                }

                for (i, item) in self.data.iter().enumerate() {
                    if i % 4 == 0 {
                        // temp starts as 00000000
                        temp = item << 2;
                    // temp is now ******00
                    } else if i % 4 == 1 {
                        // temp starts as ******00
                        temp |= (item & 0x30) >> 4;
                        // temp is now ********
                        output.push(temp);
                        // temp is now 00000000
                        temp = (item & 0x0F) << 4;
                    // temp is now ****0000
                    } else if i % 4 == 2 {
                        // temp start at ****0000
                        temp |= (item & 0x3C) >> 2;
                        // temp is now ********
                        output.push(temp);
                        // temp is now 00000000
                        temp = (item & 0x03) << 6;
                    // temp is now **000000
                    } else if i % 4 == 3 {
                        // temp starts as **000000
                        temp |= item;
                        // temp is now ********
                        output.push(temp);
                        // temp is now 00000000
                    }
                }

                // padding
                let data_l = self.data.len();
                let output_l = output.len();

                if data_l >= 2 && self.data[data_l - 2] == 0xFF && self.data[data_l - 1] == 0xFF {
                    output.truncate(output_l - 2);
                } else if self.data[data_l - 1] == 0xFF {
                    output.truncate(output_l - 1);
                }
            } else if self.data_type == "ascii" && new_init_base == "base64" {
                // ascii -> base64
                for (i, item) in self.data.iter().enumerate() {
                    if i % 3 == 0 {
                        // temp starts at 00000000
                        temp = (item & 0xFC) >> 2;
                        // temp is now 00******
                        output.push(temp);
                        // temp is now 00000000
                        temp = (item & 0x03) << 4;
                    // temp is now 00**0000
                    } else if i % 3 == 1 {
                        // temp starts as 00**0000
                        temp |= (item & 0xF0) >> 4;
                        // temp is now 00******
                        output.push(temp);
                        // temp is now 00000000
                        temp = (item & 0x0F) << 2;
                    // temp is now 00****00
                    } else if i % 3 == 2 {
                        // temp starts at 00****00
                        temp |= (item & 0xC0) >> 6;
                        // temp is now 00******
                        output.push(temp);
                        temp = item & 0x3F;
                        // temp is now 00000000
                        output.push(temp);
                        // temp is now 00000000
                    }
                }
                // Incorporate padding
                // if len%3 == 0 then we can split the string evenly into base64
                // if len%3 == 1 then we have to encode the last bit of information with 2 unique base64 chars and 2 =
                // if len%3 == 2 then we have to encode the last bit of information with 3 unique base64 chars and 1 =
                if self.data.len() % 3 == 1 {
                    // encode with 2 unique base64 chars and 2 =
                    // temp has 00**0000
                    output.push(temp);
                    output.push(0xFF); // representing = as 0xFF
                    output.push(0xFF);
                } else if self.data.len() % 3 == 2 {
                    // encode with 3 unique base64 chars and 1 =
                    // temp has 00****00
                    output.push(temp);
                    output.push(0xFF);
                }
            } else {
                panic!(
                    "Error: unsupported opeartion to convert {} base into {} base",
                    self.data_type, new_init_base
                );
            }

            self.data = output;
            self.data_type = new_init_base.to_string();
        }
    }

    /* index -- returns a storage that contains the elements inside of the range.
     * Parameters: left (usize) - starting index (starting element included)
     *             right (usize) - ending index (ending element not included)
     * Return: Storage - Storage containing the data of the previous storage
     * in range of the two indices
     */
    pub fn index(&self, left: usize, right: usize) -> Storage {
        if left >= self.data.len() {
            panic!("Error: left index must be in range of data");
        }
        if right > self.data.len() {
            panic!("Error: right index must be in range of data");
        }
        if left > right {
            panic!("Error: left index cannot be greater than right index");
        }

        Storage {
            data: self.data[left..right].to_vec(),
            data_type: self.get_data_type().to_string(),
        }
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
            panic!(
                "Error: Storage are not the same type.  \
                 LHS type is {}. RHS type is {}.",
                self.data_type, rhs.data_type
            );
        }

        if self.data.len() == rhs.data.len() {
            Storage {
                data: self.data
                    .iter()
                    .zip(rhs.data.iter())
                    .map(|(l, r)| l ^ r)
                    .collect(),
                data_type: self.get_data_type().to_string(),
            }
        } else if self.data.len() > rhs.data.len() {
            Storage {
                data: self.data
                    .iter()
                    .enumerate()
                    .map(|(i, item)| item ^ rhs.data[i % rhs.data.len()])
                    .collect(),
                data_type: self.get_data_type().to_string(),
            }
        } else {
            panic!(
                "Error: Storage cannot be XOR'd against each other.  \
                 LHS length is {}, RHS length is {}",
                self.data.len(),
                rhs.data.len()
            );
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
            "base64",
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
        let hex_vec = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ];

        assert_eq!(
            hex_vec,
            Storage::build_data("0123456789ABCDEFabcdef", "hex")
        );

        let mut base64_vec = Vec::new();
        for i in 0u8..64u8 {
            base64_vec.push(i);
        }

        assert_eq!(
            base64_vec,
            Storage::build_data(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",
                "base64"
            )
        );

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

        let hex_vec = vec![
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
            0x0E, 0x0F, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        ];
        s.set_data("0123456789abcdefABCDEF", "hex");
        assert_eq!("0123456789abcdefabcdef", s.to_string());
        assert_eq!("hex", s.get_data_type());
        assert_eq!(&hex_vec, s.get_data());
    }

    #[test]
    fn check_set_and_get2() {
        let mut s = Storage::new_init("abcd", "ascii");
        let hex_vec = vec![0x00, 0x01, 0x02, 0x03];
        s.set_data("0123", "hex");
        assert_eq!("0123", s.to_string());
        assert_eq!("hex", s.get_data_type());
        assert_eq!(&hex_vec, s.get_data());

        s.set_data("ABCabc123+/", "base64");
        let base64_vec = vec![
            0x00, 0x01, 0x02, 0x1A, 0x1B, 0x1C, 0x35, 0x36, 0x37, 0x3E, 0x3F,
        ];
        assert_eq!("ABCabc123+/", s.to_string());
        assert_eq!("base64", s.get_data_type());
        assert_eq!(&base64_vec, s.get_data());

        s.set_data("tESt One!32/(*&", "ascii");
        let ascii_vec = vec![
            0x74, 0x45, 0x53, 0x74, 0x20, 0x4F, 0x6E, 0x65, 0x21, 0x33, 0x32, 0x2F, 0x28, 0x2A,
            0x26,
        ];
        assert_eq!("tESt One!32/(*&", s.to_string());
        assert_eq!("ascii", s.get_data_type());
        assert_eq!(&ascii_vec, s.get_data());
    }

    // TEST len
    #[test]
    fn check_len() {
        let s1 = Storage::new();
        let s2 = Storage::new_init("0123456789abcdef", "hex");
        let s3 = Storage::new_init(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+",
            "base64",
        );
        let s4 = Storage::new_init(
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789,./;'[]<>?:\"{{}}-_=+`~!@#$%^&*()", 
            "ascii"
        );

        assert_eq!(0, s1.len());
        assert_eq!(16, s2.len());
        assert_eq!(64, s3.len());
        assert_eq!(94, s4.len());
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

        for (ch, u) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .zip(base64_vec)
        {
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

        for (ch, u) in "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
            .chars()
            .zip(base64_vec)
        {
            assert_eq!(ch, Storage::u8_to_char(u, "base64"));
        }
    }

    #[test]
    #[should_panic]
    fn check_invalid_u8_to_char_base64() {
        Storage::u8_to_char(0xC0, "base64");
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
    #[test]
    fn check_hex_to_base64() {
        let mut s = Storage::new_init("0123456789abcdefFf", "hex");
        s.change_base("base64");
        assert_eq!("ASNFZ4mrze//", s.to_string());
    }

    #[test]
    fn check_base64_to_hex() {
        let mut s = Storage::new_init("ABCabc0123+/", "base64");
        s.change_base("hex");
        assert_eq!("00109a6dcd35db7fbf", s.to_string());
    }

    #[test]
    fn check_hex_to_ascii() {
        let mut s = Storage::new_init("4c75636173", "hex");
        s.change_base("ascii");
        assert_eq!("Lucas", s.to_string());
    }

    #[test]
    fn check_ascii_to_hex() {
        let mut s = Storage::new_init("Hello World!", "ascii");
        s.change_base("hex");
        assert_eq!("48656c6c6f20576f726c6421", s.to_string());
    }

    #[test]
    fn check_base64_to_ascii() {
        let mut s = Storage::new_init("aGVsbG9vb29vIHdvcmxk", "base64");
        s.change_base("ascii");
        assert_eq!("hellooooo world", s.to_string());
    }

    #[test]
    fn check_base64_to_ascii_padding_and_change_back() {
        // test 1 - no padding
        let mut s = Storage::new_init("TWFu", "base64");
        s.change_base("ascii");
        assert_eq!("Man", s.to_string());
        s.change_base("base64");
        assert_eq!("TWFu", s.to_string());

        // test 2 - 1 padding
        s.set_data("TWE=", "base64");
        s.change_base("ascii");
        assert_eq!("Ma", s.to_string());
        s.change_base("base64");
        assert_eq!("TWE=", s.to_string());

        // test 3 - 2 padding
        s.set_data("TQ==", "base64");
        s.change_base("ascii");
        assert_eq!("M", s.to_string());
        s.change_base("base64");
        assert_eq!("TQ==", s.to_string());
    }

    #[test]
    fn check_ascii_to_base64() {
        let mut s = Storage::new_init("hello world!", "ascii");
        s.change_base("base64");
        assert_eq!("aGVsbG8gd29ybGQh", s.to_string());
    }

    #[test]
    fn check_ascii_to_base64_padding() {
        let mut s = Storage::new_init("Man", "ascii");
        s.change_base("base64");
        assert_eq!("TWFu", s.to_string());
        s.set_data("Ma", "ascii");
        s.change_base("base64");
        assert_eq!("TWE=", s.to_string());
        s.set_data("M", "ascii");
        s.change_base("base64");
        assert_eq!("TQ==", s.to_string());
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
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            hex.to_string()
        );
        hex.change_base("hex");
        assert_eq!("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d", hex.to_string());
    }

    // TEST index
    // TODO: add indexing tests for other bases
    #[test]
    fn check_index() {
        let s = Storage::new_init("hello world", "ascii");

        assert_eq!("h", s.index(0, 1).to_string());
        assert_eq!("e", s.index(1, 2).to_string());
        assert_eq!("he", s.index(0, 2).to_string());
        assert_eq!("ll", s.index(2, 4).to_string());
        assert_eq!("hel", s.index(0, 3).to_string());
        assert_eq!("lo ", s.index(3, 6).to_string());
        assert_eq!("hell", s.index(0, 4).to_string());
        assert_eq!("o wo", s.index(4, 8).to_string());
        assert_eq!("hello", s.index(0, 5).to_string());
        assert_eq!(" worl", s.index(5, 10).to_string());
        assert_eq!("hello world", s.index(0, 11).to_string());
        assert_eq!("", s.index(0, 0).to_string());
    }

    #[test]
    #[should_panic]
    fn check_invalid_left_index() {
        let s = Storage::new_init("hello world", "ascii");
        s.index(11, 11);
    }

    #[test]
    #[should_panic]
    fn check_invalid_right_index() {
        let s = Storage::new_init("hello world", "ascii");
        s.index(0, 400);
    }

    #[test]
    #[should_panic]
    fn check_invalid_left_index_greater_than_right_index() {
        let s = Storage::new_init("hello world", "ascii");
        s.index(5, 0);
    }

    // TEST XOR - overloaded Bitwise XOR operator
    #[test]
    fn check_xor_full() {
        let mut lhs = Storage::new_init("01234abcd", "hex");
        let mut rhs = Storage::new_init("abcd01234", "hex");
        let mut ans = &lhs ^ &rhs;
        assert_eq!("aaee4b9f9", ans.to_string());
        assert_eq!("hex", ans.get_data_type());

        lhs.set_data("ABCabc01+/", "base64");
        rhs.set_data("+/abc01ABC", "base64");
        ans = &lhs ^ &rhs;
        assert_eq!("++YBHoB1/9", ans.to_string());
        assert_eq!("base64", ans.get_data_type());

        lhs.set_data("ABCc1+/", "ascii");
        rhs.set_data("+/a0ABC", "ascii");
        ans = &lhs ^ &rhs;
        assert_eq!("jm\"Spil", ans.to_string());
        assert_eq!("ascii", ans.get_data_type());
    }

    #[test]
    fn check_xor_one_char_repeating() {
        let mut lhs = Storage::new_init("01234abcd", "hex");
        let mut rhs = Storage::new_init("d", "hex");
        let mut ans = &lhs ^ &rhs;
        assert_eq!("dcfe97610", ans.to_string());
        assert_eq!("hex", ans.get_data_type());

        lhs.set_data("ABCabc01+/", "base64");
        rhs.set_data("+", "base64");
        ans = &lhs ^ &rhs;
        assert_eq!("+/8kliKLAB", ans.to_string());
        assert_eq!("base64", ans.get_data_type());

        lhs.set_data("{btvd", "ascii");
        rhs.set_data("7", "ascii");
        ans = &lhs ^ &rhs;
        assert_eq!("LUCAS", ans.to_string());
        assert_eq!("ascii", ans.get_data_type());
    }

    #[test]
    fn check_xor_multi_char_repeating() {
        let mut lhs = Storage::new_init("01234abcd0", "hex");
        let mut rhs = Storage::new_init("def", "hex");
        let mut ans = &lhs ^ &rhs;
        assert_eq!("dfdea5622d", ans.to_string());
        assert_eq!("hex", ans.get_data_type());

        lhs.set_data("ABCDabc01+/", "base64");
        rhs.set_data("z7e", "base64");
        ans = &lhs ^ &rhs;
        assert_eq!("z6cwhFvPrNE", ans.to_string());
        assert_eq!("base64", ans.get_data_type());

        lhs.set_data("longplaintext", "ascii");
        rhs.set_data("key", "ascii");
        ans = &lhs ^ &rhs;
        // hello control characters
        let ans_vec: Vec<u8> = vec![
            0x07, 0x0a, 0x17, 0x0c, 0x15, 0x15, 0x0a, 0x0c, 0x17, 0x1f, 0x00, 0x01, 0x1f,
        ];
        assert_eq!(&ans_vec, ans.get_data());
        assert_eq!("ascii", ans.get_data_type());
    }

    #[test]
    #[should_panic]
    fn check_invalid_xor_empty_storages() {
        let lhs: Storage = Storage::new();
        let rhs: Storage = Storage::new();
        &lhs ^ &rhs;
    }

    #[test]
    #[should_panic]
    fn check_invalid_xor_empty_and_full() {
        let lhs = Storage::new_init("abc", "ascii");
        let rhs = Storage::new();
        &lhs ^ &rhs;
    }

    #[test]
    #[should_panic]
    fn check_invalid_xor_full_and_empty() {
        let lhs = Storage::new();
        let rhs = Storage::new_init("abc", "ascii");
        &lhs ^ &rhs;
    }
    #[test]
    #[should_panic]
    fn check_invalid_xor_different_types() {
        let lhs = Storage::new_init("abc", "ascii");
        let rhs = Storage::new_init("abc", "hex");
        &lhs ^ &rhs;
    }

    #[test]
    #[should_panic]
    fn check_invalid_xor_right_side_bigger() {
        let lhs = Storage::new_init("abc", "hex");
        let rhs = Storage::new_init("01234abcd", "hex");
        &lhs ^ &rhs;
    }
}
