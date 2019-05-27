use storage::Storage;

/// Challenge - Struct to display each challenge
pub struct Challenge {
    header: String,
    input: Option<String>,
    input_file: Option<String>,
    input_type: Option<String>,
    input2: Option<String>,
    input2_type: Option<String>,
    key: Option<String>,
    key_type: Option<String>,
    expected_type: Option<String>,
    actual_type: Option<String>,
    expected_line: Option<i32>,
    actual_line: Option<i32>,
    expected_size: Option<usize>,
    actual_size: Option<usize>,
    expected_key: Option<String>,
    actual_key: Option<String>,
    expected_key_type: Option<String>,
    actual_key_type: Option<String>,
    expected_answer: Option<String>,
    actual_answer: Option<String>,
}

trait ChallengeBuilder {
    fn header(&mut self, header: String) -> &mut Self;
    fn input(&mut self, input: Option<String>) -> &mut Self;
    fn input_file(&mut self, input_file: Option<String>) -> &mut Self;
    fn input_type(&mut self, input_type: Option<String>) -> &mut Self;
    fn input2(&mut self, input2: Option<String>) -> &mut Self;
    fn input2_type(&mut self, input2_type: Option<String>) -> &mut Self;
    fn key(&mut self, key: Option<String>) -> &mut Self;
    fn key_type(&mut self, key_type: Option<String>) -> &mut Self;
    fn expected_type(&mut self, expected_type: Option<String>) -> &mut Self;
    fn actual_type(&mut self, actual_type: Option<String>) -> &mut Self;
    fn expected_line(&mut self, expected_line: Option<i32>) -> &mut Self;
    fn actual_line(&mut self, actual_line: Option<i32>) -> &mut Self;
    fn expected_size(&mut self, expected_size: Option<usize>) -> &mut Self;
    fn actual_size(&mut self, actual_answer: Option<usize>) -> &mut Self;
    fn expected_key(&mut self, expected_key: Option<String>) -> &mut Self;
    fn actual_key(&mut self, actual_key: Option<String>) -> &mut Self;
    fn expected_key_type(&mut self, expected_key_type: Option<String>) -> &mut Self;
    fn actual_key_type(&mut self, actual_key_type: Option<String>) -> &mut Self;
    fn expected_answer(&mut self, expected_answer: Option<String>) -> &mut Self;
    fn actual_answer(&mut self, actual_answer: Option<String>) -> &mut Self;
    fn build(&self) -> Challenge;
}

struct Builder {
    header: String,
    input: Option<String>,
    input_file: Option<String>,
    input_type: Option<String>,
    input2: Option<String>,
    input2_type: Option<String>,
    key: Option<String>,
    key_type: Option<String>,
    expected_type: Option<String>,
    actual_type: Option<String>,
    expected_line: Option<i32>,
    actual_line: Option<i32>,
    expected_size: Option<usize>,
    actual_size: Option<usize>,
    expected_key: Option<String>,
    actual_key: Option<String>,
    expected_key_type: Option<String>,
    actual_key_type: Option<String>,
    expected_answer: Option<String>,
    actual_answer: Option<String>,
}

impl Challenge {
    fn print(&self) {
        println!("{}", self.header);
        match self.input {
            Some(s) => println!("Input: {}", s),
            None => ()
        }
        match self.input_file {
            Some(s) => println!("Input File: {}", s),
            None => ()
        }
        match self.input_type {
            Some(s) => println!("Input Type: {}", s),
            None => ()
        }
        match self.input2 {
            Some(s) => println!("Input2: {}", s),
            None => ()
        }
        match self.input2_type {
            Some(s) => println!("Input2 Type: {}", s),
            None => ()
        }
        match self.key {
            Some(s) => println!("Key: {}", s),
            None => ()
        }
        match self.key_type {
            Some(s) => println!("Key Type: {}", s),
            None => ()
        } 
        match self.expected_key {
            Some(s) => println!("Expected Key: {}", s),
            None => ()
        }
        match self.actual_key {
            Some(s) => println!("Actual Key: {}", s),
            None => ()
        }
        match self.expected_key_type {
            Some(s) => println!("Expected Key Type: {}", s),
            None => ()
        }
        match self.actual_key_type {
            Some(s) => println!("Actual Key Type: {}", s),
            None => ()
        }
        match self.expected_type {
            Some(s) => println!("Expected Type: {}", s),
            None => ()
        }
        match self.actual_type {
            Some(s) => println!("Actual Type: {}", s),
            None => ()
        }
        match self.expected_line {
            Some(s) => println!("Expected Line Number: {}", s),
            None => ()
        }
        match self.actual_line {
            Some(s) => println!("Actual Line Number: {}", s),
            None => ()
        }
        match self.expected_size {
            Some(s) => println!("Expected Size: {}", s),
            None => ()
        }
        match self.actual_size {
            Some(s) => println!("Actual Size: {}", s),
            None => ()
        }
        match self.expected_answer {
            Some(s) => println!("Expected Answer: {}", s),
            None => ()
        }
        match self.actual_answer {
            Some(s) => println!("Actual Answer: {}", s),
            None => ()
        }
    }

    // Setters
    fn set_actual_type(&mut self, actual_type: &str) {
        self.actual_type = Some(actual_type.to_owned());
    }

    fn set_actual_line(&mut self, actual_line: i32) {
        self.actual_line = Some(actual_line);
    }

    fn set_actual_size(&mut self, actual_size: usize) {
        self.actual_size = Some(actual_size);
    }

    fn set_actual_key(&mut self, actual_key: &str) {
        self.actual_key = Some(actual_key.to_owned());
    }

    fn set_actual_answer(&mut self, actual_answer: &str) {
        self.actual_answer = Some(actual_answer.to_owned());
    }

    // Getters
    fn get_input(&self) -> &str {
        &self.input.unwrap()
    }

    fn get_input_file(&self) -> &str {
        &self.input_file.unwrap()
    }

    fn get_input_type(&self) -> &str {
        &self.input_type.unwrap()
    }

    fn get_input2(&self) -> &str {
        &self.input2.unwrap()
    }

    fn get_input2_type(&self) -> &str {
        &self.input2_type.unwrap()
    }

    fn get_key(&self) -> &str {
        &self.key.unwrap()
    }

    fn get_key_type(&self) -> &str {
        &self.key_type.unwrap()
    }

    fn get_expected_type(&self) -> &str {
        &self.expected_type.unwrap()
    }

    fn get_expected_key_type(&self) -> &str {
        &self.expected_key_type.unwrap()
    }
}

impl Builder {
    fn new() -> Builder {
        Builder {
            header: "",
            input: None,
            input_file: None,
            input_type: None,
            input2: None,
            input2_type: None,
            key: None,
            key_type: None,
            expected_type: None,
            actual_type: None,
            expected_line: None,
            actual_line: None,
            expected_size: None,
            actual_size: None,
            expected_key: None,
            actual_key: None,
            expected_key_type: None,
            actual_key_type: None,
            expected_answer: None,
            actual_answer: None
        }
    }

    fn set_header(&mut self, header: &str) -> &mut Builder {
        self.header = header.to_owned();
        self
    }

    fn set_input(&mut self, input: &str) -> &mut Builder {
        self.input = Some(input.to_owned());
        self
    }
    
    fn set_input_file(&mut self, input_file: &str) -> &mut Builder {
        self.input_file = Some(input_file.to_owned());
        self
    }

    fn set_input_type(&mut self, input_type: &str) -> &mut Builder {
        self.input_type = Some(input_type.to_owned());
        self
    }

    fn set_input2(&mut self, input2: &str) -> &mut Builder {
        self.input2 = Some(input2.to_owned());
        self
    }

    fn set_input2_type(&mut self, input2_type: &str) -> &mut Builder {
        self.input2_type = Some(input2_type.to_owned());
        self
    }

    fn set_key(&mut self, key: &str) -> &mut Builder {
        self.key = Some(key.to_owned());
        self
    }

    fn set_key_type(&mut self, key_type: &str) -> &mut Builder {
        self.key_type = Some(key_type.to_owned());
        self
    }

    fn set_expected_type(&mut self, expected_type: &str) -> &mut Builder {
        self.expected_type = Some(expected_type.to_owned());
        self
    }

    fn set_expected_line(&mut self, expected_line: i32) -> &mut Builder {
        self.expected_line = Some(expected_line);
        self
    }

    fn set_expected_size(&mut self, expected_size: usize) -> &mut Builder {
        self.expected_size = Some(expected_size);
        self
    }
    
    fn set_expected_key(&mut self, expected_key: &str) -> &mut Builder {
        self.expected_key = Some(expected_key.to_owned());
        self
    }

    fn set_expected_key_type(&mut self, expected_key_type: &str) -> &mut Builder {
        self.expected_key_type = Some(expected_key_type.to_owned());
        self
    }

    fn set_expected_answer(&mut self, expected_answer: &str) -> &mut Builder {
        self.expected_answer = Some(expected_answer.to_owned());
        self
    }

    fn build_challenge(&self) -> Challenge {
        Challenge {
            header: self.header,
            input: self.input,
            input_file: self.input_file,
            input_type: self.input_type,
            input2: self.input2,
            input2_type: self.input2_type,
            key: self.key,
            key_type: self.key_type,
            expected_type: self.expected_type,
            actual_type: self.actual_type,
            expected_line: self.expected_line,
            actual_line: self.actual_line,
            expected_size: self.expected_size,
            actual_size: self.actual_size,
            expected_key: self.expected_key,
            actual_key: self.actual_key,
            expected_key_type: self.expected_key_type,
            actual_key_type: self.actual_key_type,
            expected_answer: self.expected_answer,
            actual_answer: self.actual_answer
        }
    }
}

impl ChallengeBuilder for Builder {
    fn header(&mut self, header: &str) -> &mut Self {
        self.set_header(header)
    }

    fn input(&mut self, input: &str) -> &mut Self {
        self.set_input(input)
    }

    fn input_file(&mut self, input_file: &str) -> &mut Self {
        self.set_input_file(input_file)
    }

    fn input_type(&mut self, input_type: &str) -> &mut Self {
        self.set_input_type(input_type)
    }

    fn input2(&mut self, input2: &str) -> &mut Self {
        self.set_input2(input2)
    }

    fn input2_type(&mut self, input2_type: &str) -> &mut Self {
        self.set_input2_type(input2_type)
    }

    fn key(&mut self, key: &str) -> &mut Self {
        self.set_key(key)
    }

    fn key_type(&mut self, key_type: &str) -> &mut Self {
        self.set_key_type(key_type)
    }
    
    fn expected_type(&mut self, expected_type: &str) -> &mut Self {
        self.set_expected_type(expected_type)
    }

    fn expected_line(&mut self, expected_line: i32) -> &mut Self {
        self.set_expected_line(expected_line)
    }

    fn expected_size(&mut self, expected_size: usize) -> &mut Self {
        self.set_expected_size(expected_size)
    }

    fn expected_key(&mut self, expected_key: &str) -> &mut Self {
        self.set_expected_key(expected_key)
    }

    fn expected_key_type(&mut self, expected_key_type: &str) -> &mut Self {
        self.set_expected_key_type(expected_key_type)
    }

    fn expected_answer(&mut self, expected_answer: &str) -> &mut Self {
        self.set_expected_answer(expected_answer)
    }
     
    fn build(&self) -> Challenge {
        self.build_challenge()
    }
}

