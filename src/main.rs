pub mod set1;

fn main() {
    // Set 1, Challenge 1
    println!("Set 1, Challenge 1 - hex to base64");
    println!(
        "Input: 49276d206b696c6c696e6720796f757220627261696e206c696b65206120\
         706f69736f6e6f7573206d757368726f6f6d"
    );

    println!("Ans: SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    println!(
        "Res: {}",
        set1::hex_to_base64(
            "49276d206b696c6c696e6720796f75722062\
             7261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        )
    );

    println!();

    // Set 1, Challenge 2
    println!("Set 1, Challenge 2 - fixed XOR");
    println!("Input: 1c0111001f010100061a024b53535009181c");
    println!("Input: 686974207468652062756c6c277320657965");
    println!("Ans: 746865206b696420646f6e277420706c6179");
    println!(
        "Res: {}",
        set1::fixed_xor(
            "1c0111001f010100061a024b53535009181c",
            "hex",
            "686974207468652062756c6c277320657965",
            "hex"
        )
    );

    println!();

    // Set 1, Challenge 3
    println!("Set 1, Challenge 3 - Single-byte XOR cipher");
    println!("Input: 1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let (ans_message3, ans_key3) = set1::single_byte_xor_cipher(
        "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        "hex",
    );

    println!("Ans message: Cooking MC's like a pound of bacon");
    println!("Res message: {}", ans_message3);
    println!("Ans key: X");
    println!("Res key: {}", ans_key3);
    println!();

    // Set 1, Challenge 4
    println!("Set 1, Challenge 4 - Detect single-character XOR");
    println!("Input File: ./input_files/set1challenge4.txt");
    let (ans_message4, ans_key4, ans_line4) =
        set1::detect_single_character_xor("./input_files/set1challenge4.txt");

    print!("Ans message: Now that the party is jumping\n");
    print!("Res message: {}", ans_message4);
    println!("Ans key: 5");
    println!("Res key: {}", ans_key4);
    println!("Ans line: 5");
    println!("Res line: {}", ans_line4);
    println!();

    // Set 1, Challenge 5
    println!("Set 1, Challenge 5 - repeating-key XOR");
    println!(
        "Input String: Burning 'em, if you ain't quick and nimble\nI go craz\
         y when I hear a cymbal"
    );
    println!("Input key: ICE");
    println!(
        "Ans: 0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a2622\
         6324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
    );
    println!(
        "Res: {}",
        set1::repeating_key_xor_encrypt(
            "Burning 'em, if you ain't \
             quick and nimble\nI go crazy when I hear a cymbal",
            "ascii",
            "ICE",
            "ascii"
        )
    );
    println!();

    // Set 1, Challenge 6
    println!("Set 1, Challenge 6 - Break repeating-key XOR");
    println!("Input File: ./input_files/set1challenge6.txt");
    let (ans_message6, ans_key6, ans_keysize6) =
        set1::break_repeating_key_xor("./input_files/set1challenge6.txt");

    println!("Res message: {}", ans_message6);
    println!("Ans key: Terminator X: Bring the noise");
    println!("Res key: {}", ans_key6);
    println!("Ans keysize: 29");
    println!("Res keysize: {}", ans_keysize6);
    println!();

    // Set 1, Challenge 7
    println!("Set 1, Challenge 7 - Decrypt AES-128-ECB given key");
    println!("Input File: ./input_files/set1challenge7.txt");
    let ans_message7 =
        set1::decrypt_aes_128_ecb("./input_files/set1challenge7.txt", "YELLOW SUBMARINE");
    println!("Res message: {}", ans_message7);
    println!();
}
