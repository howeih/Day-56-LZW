use std::collections::HashMap;

struct LZW {}

impl LZW {
    fn new() -> Self {
        LZW {}
    }

    fn init_encode_table() -> HashMap<Vec<u32>, u32> {
        let mut code_table = HashMap::<Vec<u32>, u32>::new();
        for c in 0u32..=255u32 {
            code_table.insert(vec![c], c as u32);
        }
        code_table
    }

    fn encode(&mut self, data: &[u8]) -> Vec<u32> {
        let mut code_table = LZW::init_encode_table();
        let mut output_code = Vec::<u32>::new();
        let mut p = Vec::<u32>::new();
        if data.len() > 0 {
            p.push(data[0] as u32);
        }
        let mut next_code = 256;
        for i in 0..data.len() - 1 {
            let c = data[i + 1] as u32;
            let mut p_c = Vec::<u32>::new();
            p_c.extend(p.iter());
            p_c.push(c);
            if code_table.contains_key(&p_c) {
                p = p_c;
            } else {
                let code = code_table.get(&p).unwrap();
                output_code.push(*code);
                code_table.insert(p_c, next_code);
                p = vec![c];
                next_code += 1;
            }
        }
        output_code.push(*code_table.get(&p).unwrap());
        output_code
    }

    fn init_decode_table() -> HashMap<u32, Vec<u32>>{
        let mut code_table = HashMap::<u32, Vec<u32>>::new();
        for c in 0u32..=255u32 {
            code_table.insert(c as u32, vec![c]);
        }
        code_table
    }

    fn decode(&self, encoded_data: &[u32]) -> Vec<u8> {
        let mut code_table = LZW::init_decode_table();
        let mut s = Vec::<u32>::new();
        let mut old= 0x00 ;
        let mut c = Vec::<u32>::new();
        let mut next_code = 256;
        let mut decoded_data = Vec::<u8>::new();
        if encoded_data.len() > 0 {
            old = encoded_data[0];
            let translated_code = code_table.get(&old).unwrap();
            s.extend(translated_code);
            c.extend(translated_code);
            decoded_data.push(old as u8);
        }
        for i in 0..encoded_data.len() - 1 {
            let n = encoded_data[i+1];
            if !code_table.contains_key(&n){
                s = (*code_table.get(&old).unwrap()).clone();
                s.extend(c.clone());
            }else {
                s.clear();
                s.extend(code_table.get(&n).unwrap());
            }
            decoded_data.extend(s.iter().map(|&x|x as u8).collect::<Vec<u8>>());
            c.clear();
            c.push(s[0]);
            let mut count = code_table.get(&old).unwrap().clone();
            count.extend(c.clone());
            code_table.insert(next_code, count);
            next_code +=1;
            old = n;
        }
        decoded_data
    }
}

fn main() {
    let mut lzw = LZW::new();
    let text = "BABAABAAA";
    let encoded = lzw.encode(text.as_bytes());
    let decoded = lzw.decode(&encoded);
    assert_eq!(String::from_utf8(decoded).unwrap(), text);
}
