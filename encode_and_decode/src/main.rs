struct Codec;

impl Codec {
    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(&s.len().to_string());
            result.push('#');
            result.push_str(&s);
        }
        result
    }

    pub fn decode(&self, s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut result = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let len: usize = std::str::from_utf8(&bytes[i..j])
                .unwrap()
                .parse()
                .unwrap();
            j += 1;
            let end = j + len;
            result.push(
                String::from_utf8(bytes[j..end].to_vec()).unwrap()
            );
            i = end;
        }
        result
    }
}