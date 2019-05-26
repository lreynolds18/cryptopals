use storage::Storage;

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
            input_file: input_file.to_owned(),
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

    pub fn get_expected_type(&self) -> &str {
        &self.expected_type
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


