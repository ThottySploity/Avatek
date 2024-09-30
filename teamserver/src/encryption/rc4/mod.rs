pub struct Rc4;

impl Rc4 {
    pub fn crypt(data: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
        let mut r = convert_to_u128(data.clone());
        let key = convert_to_u128(key);
        
        let mut s: [u128; 256] = [0; 256];
        let mut k: [u128; 256] = [0; 256];
    
        for i in 0..256 {
            s[i] = i as u128;
            k[i] = key[i % key.len()];
        }
    
        let mut j: usize = 0;
        for i in 0..256 {
            j = (j + s[i as usize] as usize + k[i as usize] as usize) % 256;
            s.swap(i, j); // Swap elements
        }
    
        let mut i = 0;
        let mut j = 0;
        for x in 0..r.len() {
            i = (i + 1) % 256;
            j = (j + s[i as usize] as usize) % 256;
    
            s.swap(i, j); 
            
            let t = ((s[i as usize] + s[j as usize]) % 256) as usize;
            r[x] ^= s[t]; 
        }
    
        convert_to_u8(r)
    }
}

fn convert_to_u128(input: Vec<u8>) -> Vec<u128> {
    input.iter().map(|i| *i as u128).collect()
}

fn convert_to_u8(input: Vec<u128>) -> Vec<u8> {
    input.iter().map(|i| *i as u8).collect()
}