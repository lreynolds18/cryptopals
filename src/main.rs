mod set1;
mod binary_object;

fn main() {
  // Test 1
  let hex1 = binary_object::BinaryObject::new(&"0123456789ABCDEF".to_string(), &"hex".to_string());
  let test_hex1 = vec!(0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF);
  println!("Test 1 -- testing str_to_vec");
  println!("output = Vec<u8> {{01, 23, 45, 67, 89, AB, CD, EF}}");
  // println!("Result: {}, Type: {}", hex1.get_data() == test_hex1, hex1.get_data_type());
  println!("Result: {}", hex1.get_data() == test_hex1);
  println!("");

  // Test 2
  let hex2 = binary_object::BinaryObject::new(&"012".to_string(), &"hex".to_string());
  let test_hex2 = vec!(0x01, 0x20);
  println!("Test 2 -- odd str_to_vec"); 
  println!("output = Vec<u8> {{0x01, 0x20}}");
  println!("Result: {}", hex2.get_data() == test_hex2);
  println!("");
  
  // Test 3
  let hex3 = binary_object::BinaryObject::new(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string(), &"hex".to_string());
  println!("Test 3 -- testing str_to_vec and print_hex_vec");
  println!("Ans: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  print!("Res: ");
  hex3.print();
  println!("");

  // let base64_1 = set1::hex_to_base64(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string());
  // set1::hex_to_base64(&"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  // set1::hex_to_base64(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
}
