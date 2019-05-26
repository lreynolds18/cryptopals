pub mod helper;
pub mod storage;

use self::storage::Storage;
use std::f64;
use std::fs;

/// Challenge - Struct to display each challenge
// TODO: implement builder pattern (https://en.wikipedia.org/wiki/Builder_pattern#Rust)
pub struct Challenge {
    header: String,
    input: String,
    input_file: String,
    input_type: String,
    input2: String,
    input2_type: String,
    expected_answer: String,
    expected_type: String,
    expected_line: i32,
    expected_size: usize,
    expected_key: String,
    expected_key_type: String,
    actual_answer: String,
    actual_type: String,
    actual_key: String,
    actual_line: i32,
    actual_size: usize
}

impl Challenge {
    /// new -- construct for challenge
    /// Parameters: void 
    /// Return: Challenge
    pub fn new() -> Challenge {
      Challenge {}
    }

    /// new_init -- basic constructor for challenge 
    /// Parameters: header (&str)
    ///             input (&str)
    ///             input_type (&str)
    ///             expected_answer (&str)
    ///             expected_type (&str)
    /// Return: Challenge
    pub fn new_init(
        header: &str,
        input: &str,
        input_type: &str,
        expected_answer: &str,
        expected_type: &str
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_type: expected_type.to_owned()
        }
    }

    /// new_init -- basic constructor for challenge 
    /// Parameters: header (&str)
    ///             input (&str)
    ///             input_type (&str)
    ///             expected_answer (&str)
    ///             expected_type (&str)
    /// Return: Challenge
    pub fn new_init_file(
        header: &str,
        input_file: &str,
        input_type: &str,
        expected_answer: &str,
        expected_type: &str
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_type: expected_type.to_owned()
        }
    }

    /// new_init_2inputs -- constructor for challenge w/ two inputs
    /// Parameters: header (&str)
    ///             input (&str)
    ///             input_type (&str)
    ///             input2 (&str)
    ///             input2_type (&str)
    ///             expected_answer (&str)
    ///             expected_type (&str)
    /// Return: Challenge
    pub fn new_init_2inputs(
        header: &str,
        input: &str,
        input_type: &str,
        input2: &str,
        input2_type: &str,
        expected_answer: &str,
        expected_type: &str
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            input2: input2.to_owned(),
            input2_type: input2_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_type: expected_type.to_owned()
        }
    }
  
    /// new_init_decryption -- decryption constructor for challenge 
    /// Parameters: header (&str)
    ///             input (&str)
    ///             input_type (&str)
    ///             expected_answer (&str)
    ///             expected_key (&str)
    /// Return: Challenge
    pub fn new_init_decryption(
        header: &str,
        input: &str,
        input_type: &str,
        expected_answer: &str,
        expected_key: &str,
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_key: expected_key.to_owned()
        }
    }

    /// new_init_decryption_line -- decryption constructor for challenge 
    /// Parameters: header (&str)
    ///             input (&str)
    ///             input_type (&str)
    ///             expected_answer (&str)
    ///             expected_key (&str)
    ///             expected_line (i32)
    /// Return: Challenge
    pub fn new_init_decryption_line(
        header: &str,
        input: &str,
        input_type: &str,
        expected_answer: &str,
        expected_key: &str,
        expected_line: i32
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_key: expected_key.to_owned(),
            expected_line: expected_line 
        }
    }

    /// new_init_file_decryption_line -- decryption constructor for challenge 
    /// Parameters: header (&str)
    ///             input_file (&str)
    ///             input_type (&str)
    ///             expected_answer (&str)
    ///             expected_key (&str)
    ///             expected_line (i32)
    /// Return: Challenge
    pub fn new_init_file_decryption_line(
        header: &str,
        input_file: &str,
        input_type: &str,
        expected_answer: &str,
        expected_key: &str,
        expected_line: i32
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input_file: input_file.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_key: expected_key.to_owned(),
            expected_line: expected_line 
        }
    }

    /// new_init_file_decryption_size -- decryption constructor for challenge 
    /// Parameters: header (&str)
    ///             input_file (&str)
    ///             input_type (&str)
    ///             expected_key (&str)
    ///             expected_size (i32)
    /// Return: Challenge
    pub fn new_init_file_decryption_size(
        header: &str,
        input_file: &str,
        input_type: &str,
        expected_key: &str,
        expected_size: usize
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input_file: input_file.to_owned(),
            input_type: input_type.to_owned(),
            expected_key: expected_key.to_owned(),
            expected_size: expected_size
        }
    }
    
    /// new_init_file_decryption -- constructor for challenge 
    /// Parameters: header (&str)
    ///             input_file (&str)
    ///             input_type (&str)
    ///             key (&str)
    ///             key_type(&str)
    /// Return: Challenge
    pub fn new_init_file_decryption(
        header: &str,
        input: &str,
        input_type: &str,
        expected_answer: &str,
        expected_type: &str
    ) -> Challenge {
        Challenge {
            header: header.to_owned(),
            input: input.to_owned(),
            input_type: input_type.to_owned(),
            expected_answer: expected_answer.to_owned(),
            expected_type: expected_type.to_owned()
        }
    }

    pub fn set_actual_answer_type(&mut self, s: Storage) {
        self.actual_answer = s.to_string();
        self.actual_type = s.get_data_type().to_string();
    }

    pub fn set_actual_answer_key(&mut self, ans: &str, key: &str) {
        self.actual_answer = ans.to_owned();
        self.actual_key = key.to_owned();
    }

    pub fn set_actual_answer_key_line(&mut self, ans: &str, key: &str, line: i32) {
        self.actual_answer = ans.to_owned();
        self.actual_key = key.to_owned();
        self.actual_line = line;
    }

    pub fn set_actual_answer_key_size(&mut self, ans: &str, key: &str, size: usize) {
        self.actual_answer = ans.to_owned();
        self.actual_key = key.to_owned();
        self.actual_size = size;
    }


    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_input_file(&self) -> &str {
        &self.input_file
    }

    pub fn get_input_type(&self) -> &str {
        &self.input_type
    }

    pub fn get_input2(&self) -> &str {
        &self.input2
    }

    pub fn get_input2_type(&self) -> &str {
        &self.input2_type
    }
    
    // TODO: move to input2?
    pub fn get_key(&self) -> &str {
        &self.expected_key
    }

    pub fn get_key_type(&self) -> &str {
        &self.expected_key_type
    }
    
    /// print -- Print out challenge
    /// Parameters: void
    /// Return: void
    pub fn print(&self) {
        print!("{}", self.header);
        print!("Input: {}", self.input_type);
        print!("Input Type: {}", self.input_type);
        if !self.input2.is_empty() {
            print!("Input: {}", self.input2_type);
            print!("Input Type: {}", self.input2_type);
        }
        print!("Expected Answer: {}", self.expected_answer);
        print!("Expected Type: {}", self.expected_type);
        print!("Actual Answer: {}", self.actual_answer);
        print!("Actual Type: {}", self.actual_type);
    }
}


/// challenge1 -- Hex to Base64
/// http://cryptopals.com/sets/1/challenges/1
/// Convert Hex to Base64
/// Always operate on raw bytes, never on encoded strings.
pub fn challenge1() {
    // Definitions
    let challenge = Challenge::new_init(
        "Set 1, Challenge 1 - hex to base64",
        "49276d206b696c6c696e6720796f757220627261696e206c696b65206120\
         706f69736f6e6f7573206d757368726f6f6d",
        "hex",
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        "base64"
    );

    // Work
    let mut ans = Storage::new_init(challenge.get_input(), challenge.get_input_type());
    ans.change_base(&challenge.expected_type);

    // Output
    challenge.set_actual_answer_type(ans);
    challenge.print();
}

/// challenge2 -- Fixed XOR 
/// http://cryptopals.com/sets/1/challenges/2
/// xor on two hex strings
/// lhs_str (&str) - left hand side input
/// lhs_type (&str) - left hand side data type (hex/base64)
/// rhs_str (&str) - right hand side input
/// rhs_type (&str) - right hand side data type (hex/base64)
pub fn challenge2() {
    // Definitions
    let challenge = Challenge::new_init_2inputs(
        "Set 1, Challenge 2 - fixed XOR",
        "1c0111001f010100061a024b53535009181c",
        "hex",
        "686974207468652062756c6c277320657965",
        "hex",
        "746865206b696420646f6e277420706c6179",
        "hex"
    );

    // Work
    let lhs = Storage::new_init(challenge.get_input(), challenge.get_input_type());
    let rhs = Storage::new_init(challenge.get_input2(), challenge.get_input2_type());
    let ans = &lhs ^ &rhs;

    // Output
    challenge.set_actual_answer_type(ans);
    challenge.print();
}

/// challenge3 - Single Byte Xor Cipher
/// http://cryptopals.com/sets/1/challenges/3
/// The hex string has been XOR'd against a single character.
/// Find the key, decrypt the message.
pub fn challenge3() {
    // Definitions
    let challenge = Challenge::new_init_decryption(
      "Set 1, Challenge 3 - Single-byte XOR cipher",
      "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
      "hex",
      "Cooking MC's like a pound of bacon",
      "X"
    );

    let s = Storage::new_init(challenge.get_input(), challenge.get_input_type());
    let freq = helper::freq::get_char_freq_table();
    let mut result_string: String = s.to_string();
    let mut result_char: char = '0';
    let mut max_freq: f32 = 0_f32;
    let mut tmp_freq: f32;

    // Work
    for i in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".chars() {
        let mut char_obj = storage::Storage::new_init(&i.to_string(), "ascii");
        char_obj.change_base("hex");
        let mut ans = &s ^ &char_obj;
        ans.change_base("ascii");

        tmp_freq = helper::char_freq(ans.to_string().as_str(), &freq);
        if tmp_freq > max_freq {
            result_string = ans.to_string();
            result_char = i;
            max_freq = tmp_freq;
        }
    }

    // Output
    challenge.set_actual_answer_key(&result_string, &result_char.to_string());
    challenge.print();
}

/// challenge4 -- Detect Single Character Xor
/// http://cryptopals.com/sets/1/challenges/4
/// One of the 60-character strings in this file has been encrypted by single-character XOR. Find it.
/// Parameters: filename(&str) - File to detect single-character XOR
/// Return: (String, char, i32) - (Secret message, key that was used, line number)
pub fn challenge4() {
    // Definitions
    let challenge = Challenge::new_init_decryption_line(
        "Set 1, Challenge 4 - Detect single-character XOR",
        "./input_files/set1challenge4.txt",
        "hex",
        "Now that the party is jumping\n",
        "5",
        5
    );

    let contents = fs::read_to_string(challenge.get_input_file()).expect("Error: Unable to read file");
    let file_contents: Vec<Storage> = contents
        .lines()
        .map(|l| Storage::new_init(l, challenge.get_input_type()))
        .collect();

    let char_objs: Vec<Storage> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .map(|c| Storage::new_init(&c.to_string(), "ascii"))
        .collect();
    let freq = helper::freq::get_char_freq_table();

    // results that are going to be returned
    let mut result_string: String = String::new();
    let mut result_char: String = String::new();
    let mut result_num: i32 = 0;

    let mut max_freq: f32 = 0_f32; // keep track of the winner char_freq
    let mut tmp_freq: f32; // tmp variable to store char_freq of current string
    let mut count: i32 = 0; // keep track of line number
    let mut ans: Storage;

    // Work
    for mut fc in file_contents {
        fc.change_base("ascii");
        for co in &char_objs {
            ans = &fc ^ co;
            tmp_freq = helper::char_freq(&ans.to_string().as_str(), &freq);

            if tmp_freq > max_freq {
                result_string = ans.to_string();
                result_char = co.to_string();
                result_num = count;
                max_freq = tmp_freq;
            }
        }
        count += 1;
    }

    // Output
    challenge.set_actual_answer_key_line(&result_string, &result_char.to_string(), result_num);
    challenge.print();
}

/// challenge5 -- Repeating Key Xor Encrypt
/// http://cryptopals.com/sets/1/challenges/5
/// Parameters: lhs_str (&str) - left hand side input
///             lhs_type (&str) - left hand side data type (hex/base64)
///             rhs_str (&str) - right hand side input
///             rhs_type (&str) - right hand side data type (hex/base64)
/// Return: String - Encrypted message
pub fn challenge5() {
    // TODO: handle \n -- newlines
    // Definitions
    let challenge = Challenge::new_init_2inputs(
        "Set 1, Challenge 5 - repeating-key XOR",
        "Burning 'em, if you ain't quick and nimble\nI go crazy \
        when I hear a cymbal",
        "ascii",
        "ICE",
        "ascii",
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a2622\
         6324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f",
        "hex"
    );
    let lhs = Storage::new_init(challenge.get_input(), challenge.get_input_type());
    let rhs = Storage::new_init(challenge.get_input2(), challenge.get_input2_type());

    // Work
    let mut ans = &lhs ^ &rhs;
    ans.change_base("hex");

    // Output
    challenge.set_actual_answer_type(ans);
    challenge.print();
}

/// challenge6 -- Break Repeating Key Xor
/// http://cryptopals.com/sets/1/challenges/6
/// File has been base64'd after being encrypted with repeating-key XOR.
/// Algorithm:
///  Step 1: Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.
///  Step 2: Write function to compute edit distance/Hamming distance
///  Step 3: For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second
///          KEYSIZE worth of bytes, and find the edit distance between them.
///          Normalize this result by dividing by KEYSIZE.
///  Step 4: The KEYSIZE with the smallest normalized edit distance is probably the key.
///          You could proceed perhaps with the smallest 2-3 KEYSIZE values.
///          Or take 4 KEYSIZE blocks instead of 2 and average the distances.
///  Step 5: Now that you probably know the KEYSIZE: break the ciphertext into blocks
///          of KEYSIZE length.
///  Step 6: Now transpose the blocks: make a block that is the first byte of every block,
///          and a block that is the second byte of every block, and so on.
///  Step 7: Solve each block as if it was single-character XOR. You already have code to do this.
///  Step 8: For each block, the single-byte XOR key that produces the best looking histogram is
///          the repeating-key XOR key byte for that block. Put them together and you have the key.
/// Parameters: filename(&str) - File to detect repeating key xor
/// Return: (String, String, usize) - (Secret message, key that was used, key size)
pub fn challenge6() {
    // Definitions
    let challenge = Challenge::new_init_file_decryption_size(
        "Set 1, Challenge 6 - Break repeating-key XOR",
        "./input_files/set1challenge6.txt",
        "base64",
        "Terminator X: Bring the noise",
        29
    );

    let contents = fs::read_to_string(challenge.get_input_file()).expect("Error: Unable to read file");
    let mut file_contents = Storage::new_init(&contents.replace("\n", ""), challenge.get_input_type());
    file_contents.change_base("ascii");

    let char_objs: Vec<Storage> =
        " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz:"
            .chars()
            .map(|c| Storage::new_init(&c.to_string(), "ascii"))
            .collect();

    let freq = helper::freq::get_char_freq_table();

    // Work
    // Step 1-4 - Figure out keysize (theoretically we should use a minheap)
    let mut keysize: usize = 0;
    let mut min_nor_dist: f64 = f64::INFINITY; // set as MAX
    let mut tmp: f64;

    for i in 4usize..41usize {
        let s1 = file_contents.index(0, i);
        let s2 = file_contents.index(i, 2 * i);
        let s3 = file_contents.index(2 * i, 3 * i);
        let s4 = file_contents.index(3 * i, 4 * i);

        let hd1 = helper::hamming_distance(&s1, &s2);
        let hd2 = helper::hamming_distance(&s2, &s3);
        let hd3 = helper::hamming_distance(&s3, &s4);

        tmp = (hd1 + hd2 + hd3) as f64 / (3 * i) as f64;

        if tmp < min_nor_dist {
            min_nor_dist = tmp;
            keysize = i;
        }
    }

    let mut key_string = String::new();
    let mut result_char = String::new();
    let mut max_freq: f32;
    let mut tmp_freq: f32;
    let mut ans;

    let blocks = helper::split_into_blocks(&file_contents, keysize);

    for block in &blocks {
        max_freq = 0_f32;

        for co in &char_objs {
            ans = block ^ co;
            tmp_freq = helper::char_freq(&ans.to_string().as_str(), &freq);

            if tmp_freq > max_freq {
                result_char = co.to_string();
                max_freq = tmp_freq;
            }
        }
        key_string.push_str(&result_char);
    }

    let key_obj = Storage::new_init(&key_string, "ascii");
    ans = &file_contents ^ &key_obj;

    // Output
    challenge.set_actual_answer_key_size(&ans.to_string(), &key_string, keysize);
    challenge.print();
}

/// challenge7 -- Decrypt AES 128 ECB
/// http://cryptopals.com/sets/1/challenges/7
/// The Base64-encoded content in this file has been encrypted via AES-128 in ECB mode under the key
/// "YELLOW SUBMARINE". (case-sensitive, without the quotes; exactly 16 characters; I like
/// "YELLOW SUBMARINE" because it's exactly 16 bytes long, and now you do too).
/// Decrypt it. You know the key, after all.
/// Parameters: filename (&str) - Input File
///             key (&str) - String used to encrypt message
/// Return: String - Secret message
pub fn challenge7() {
    // Definitions
    let challenge = Challenge::new_init_file(
        "Set 1, Challenge 7 - Decrypt AES-128-ECB given key",
        "./input_files/set1challenge7.txt",
        "base64",
        "YELLOW SUBMARINE",
        "ascii",
    );
    let contents = fs::read_to_string(challenge.get_input_file()).expect("Error: Unable to read file");
    let mut input_storage = Storage::new_init(&contents.replace("\n", ""), challenge.get_input_type());
    input_storage.change_base("ascii");
    let key_storage = Storage::new_init(challenge.get_key(), challenge.get_key_type());

    // Work
    helper::inv_cipher_aes_128(&input_storage, &key_storage);

    // Output
}
