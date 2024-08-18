static BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_'
];

pub struct Base64;

impl Base64 {
    
    pub fn encode(input: &[u8]) -> String {

        // Function to encode a &[u8] to base64 encoded string

        let mut result = String::new();
        let mut i = 0;
        let mut bits = 0;
        let mut current: u32 = 0;
    
        for &byte in input {
            current = (current << 8) | u32::from(byte);
            bits += 8;
    
            while bits >= 6 {
                bits -= 6;
                let index = (current >> bits) as usize & 0x3F;
                result.push(BASE64_CHARS[index]);
                i += 1;
            }
        }
    
        if bits > 0 {
            current <<= 6 - bits;
            let index = current as usize & 0x3F;
            result.push(BASE64_CHARS[index]);
        }
    
        while i % 4 != 0 {
            result.push('=');
            i += 1;
        }
    
        result
    }

    pub fn decode(input: &str) -> Vec<u8> {

        // Function to decode a str from base64 to a vector u8

        let mut result = Vec::new();
        let mut bits = 0;
        let mut current: u32 = 0;
    
        for ch in input.bytes() {
            if ch == b'=' {
                break;
            }
    
            let index = BASE64_CHARS.iter().position(|&c| c as u8 == ch).unwrap() as u32;
            current = (current << 6) | index;
            bits += 6;
    
            while bits >= 8 {
                bits -= 8;
                result.push((current >> bits) as u8);
            }
        }
    
        result
    }
}