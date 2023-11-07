pub mod convert {
    
    pub fn format(message: &str) -> String {
        format!("```{}```", message)
    }

    pub fn text_to_bb(text: &str) -> String {
        binary_to_bb(&text_to_binary(&ensecret(text)))
    }

    pub fn bb_to_text(text: &str) -> String {
        unsecret(&binary_to_text(&bb_to_binary(text)))
    }

    pub fn binary_to_text(input: &str) -> String {
        input
            .split_whitespace()
            .map(|chr| {
                u8::from_str_radix(chr, 2)
                    .ok()
                    .and_then(|byte| std::char::from_u32(u32::from(byte)).map(|newc| String::from(newc)))
                    .unwrap_or(String::from(""))
            })
            .collect()
    }

    pub fn text_to_binary(text: &str) -> String {
        text.chars()
            .map(|c| format!("{:08b} ", c as u8))
            .collect()
    }

    fn binary_to_bb(binary: &str) -> String {
        binary.chars()
            .map(|c| match c {
                '0' => 'b',
                '1' => 'B',
                _ => c,
            })
            .collect()
    }

    fn bb_to_binary(bb_str: &str) -> String {
        bb_str.chars()
            .map(|c| match c {
                'b' => '0',
                'B' => '1',
                _ => c,
            })
            .collect()
    }

    fn ensecret(text: &str) -> String {
        let length = text.trim().len() as u32;
        
        let encoded_chars: Vec<Option<char>> = text.chars()
            .map(|c| char::from_u32(c as u32 + length / 16))
            .collect();
    
        let encoded_string: String = encoded_chars
            .into_iter()
            .filter_map(|c| c)
            .collect();
    
        encoded_string
    }
    
    
    fn unsecret(text: &str) -> String {
        let length = text.trim().len() as u32;
        
        let encoded_chars: Vec<Option<char>> = text.chars()
            .map(|c| char::from_u32(c as u32 - length / 16))
            .collect();
    
        let encoded_string: String = encoded_chars
            .into_iter()
            .filter_map(|c| c)
            .collect();
    
        encoded_string
    }
    
}
