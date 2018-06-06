pub mod set1;

fn main() {
  // TODO: change this to go through every set and challenge

  // Set 1, Challenge 1
  println!("Set 1, Challenge 1 - hex to base64");
  println!("Input: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  println!("Ans: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
  println!("Res: {}", set1::hex_to_base64(&"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string())); 
  println!();

  // Set 1, Challenge 2
  println!("Set 1, Challenge 2 - fixed XOR");
  println!("Input: 1c0111001f010100061a024b53535009181c");
  println!("Input: 686974207468652062756c6c277320657965");
  println!("Ans: 746865206b696420646f6e277420706c6179");
  println!("Res: {}", set1::fixed_xor(&"1c0111001f010100061a024b53535009181c".to_string(), &"hex".to_string(), 
                                      &"686974207468652062756c6c277320657965".to_string(), &"hex".to_string()));
  println!();

  // Set 1, Challenge 3
  println!("Set 1, Challenge 3 - Single-byte XOR cipher");
  println!("Input: 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
  let (ans_message, ans_key) = set1::single_byte_xor_cipher(&"1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string(), &"hex".to_string());
  println!("Ans message: Cooking MC's like a pound of bacon");
  println!("Res message: {}", ans_message);
  println!("Ans key: X");
  println!("Res key: {}", ans_key); 
  println!();

  // Set 1, Challenge 4 
  println!("Set 1, Challenge 4 - Detect single-character XOR");
  println!("Input File: set1challenge4.txt");
  println!("Ans: ?");
  println!("Res: {}", set1::detect_single_character_xor("./set1challenge4.txt"));
  println!();

  // Set 1, Challenge 5
  println!("Set 1, Challenge 5 - repeating-key XOR");
  println!("Input String: Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
  println!("Input key: ICE");
  println!("Ans: 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
  println!("Res: {}", set1::repeating_key_xor_encrypt(&"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string(), &"ascii".to_string(), &"ICE".to_string(), &"ascii".to_string()));
  println!();


}
