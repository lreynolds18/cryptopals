pub mod set1;

use set1::binary_object;

fn main() {
  // Test 1
  let hex1 = binary_object::BinaryObject::new(&"0123456789ABCDEF".to_string(), &"hex".to_string());
  let test_hex1 = vec!(0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F);
  println!("Test 1 -- testing str_to_vec");
  println!("output = Vec<u8> {{01, 23, 45, 67, 89, AB, CD, EF}}");
  // println!("Result: {}, Type: {}", hex1.get_data() == test_hex1, hex1.get_data_type());
  println!("Result: {}", hex1.get_data() == test_hex1);
  println!();

  // Test 2
  let hex2 = binary_object::BinaryObject::new(&"012".to_string(), &"hex".to_string());
  let test_hex2 = vec!(0x00, 0x01, 0x02);
  println!("Test 2 -- odd str_to_vec"); 
  println!("output = Vec<u8> {{0x01, 0x20}}");
  println!("Result: {}", hex2.get_data() == test_hex2);
  println!();
  
  // Test 3
  let mut hex3 = binary_object::BinaryObject::new(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string(), &"hex".to_string());
  println!("Test 3 -- testing str_to_vec and print_hex_vec");
  println!("Ans: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  print!("Res: ");
  hex3.print();
  println!();

  // Test 4
  hex3.change_base(&"base64".to_string());
  println!("Test 4 -- hex to base64");
  println!("Ans: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  print!("Res: ");
  hex3.print();
  println!();

  // Test 5
  hex3.change_base(&"hex".to_string());
  println!("Test 5 -- now change back, base64 to hex");
  println!("Ans: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  print!("Res: ");
  hex3.print();
  println!();

  // Test 6
  println!("Test 6 - Test hex to base64");
  println!("Input: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  println!("Ans: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  println!("Res: {}", set1::hex_to_base64(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string())); 
  println!();

  // Test 7
  println!("Test 7 - Test fixed XOR");
  println!("Input: 1c0111001f010100061a024b53535009181c");
  println!("Input: 686974207468652062756c6c277320657965");
  println!("Ans: 746865206b696420646f6e277420706c6179");
  println!("Res: {}", set1::fixed_xor(&"1c0111001f010100061a024b53535009181c".to_string(), &"hex".to_string(), 
                                      &"686974207468652062756c6c277320657965".to_string(), &"hex".to_string()));
  println!();

}
