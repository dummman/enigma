#[derive(Default)]
pub struct Reflector {
    forward_wiring: Vec<usize>,
}

impl Reflector {
    pub fn new(encoding: &str) -> Self {
        Reflector{forward_wiring: Self::decode_wiring(encoding),}
    }

    pub fn create(name: &str) -> Self {
        match name {
            "B" => Self::new("YRUHQSLDPXNGOKMIEBFZCWVJAT"),
            "C" => Self::new("FVPJIAOYEDRZXWGCTKUQSBNMHL"),
            _ => Self::new("ZYXWVUTSRQPONMLKJIHGFEDCBA"),
        }
    }

    fn decode_wiring(encoding: &str) -> Vec<usize> {
        let char_vec: Vec<char> = encoding.chars().collect();
        let mut wiring = vec![0usize; char_vec.len()];
        let iter = wiring.iter_mut().zip(char_vec.into_iter());
        for (w, c) in iter {
            *w = c as usize - 65;
        }
        wiring
    }

    pub fn forward(&self, c: usize) -> usize {
        self.forward_wiring[c]
    }
}
