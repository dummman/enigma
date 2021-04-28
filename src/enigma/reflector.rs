#[derive(Default)]
struct Reflector {
    forward_wiring: Vec<u32>,
}

impl Reflector {
    pub fn new(encoding: String) -> Self {
        Reflector{forward_wiring: decode_wiring(encoding),}
    }

    pub fn create(name: String) -> Self {
        match name.to_str() {
            "B" => self::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            "C" => self::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
            _ => self::new("ZYXWVUTSRQPONMLKJIHGFEDCBA"),
        }
    }

    fn decode_wiring(encoding: String) -> Vec<u32> {
        let char_vec: Vec<char> = encoding.chars().collect();
        let mut wiring = vec![0; char_vec.len()];
        let iter = wiring.iter_mut().zip(char_vec.into_iter());
        for (w, c) in iter {
            w = c.to_digit() - 65;
        }
        wiring
    }

    pub fn forward(&self, c: u32) -> u32 {
        self.forward_wiring[c]
    }
}
