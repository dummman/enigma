use regex::Regex;

#[derive(Default)]
pub struct Plugboard {
    wiring: Vec<usize>,
}

impl Plugboard {
    pub fn new(connections: &str) -> Self {
        Plugboard{wiring: Self::decode_plugboard(connections),}
    }

    pub fn forward(&self, c: usize) -> usize {
        self.wiring[c]
    }

    fn identity_plugboard() -> Vec<usize> {
        (0..26).collect()
    }

    pub fn get_unplugged_chars(plugboard: &str) -> Vec<usize> {
        let mut unplugged_chars: Vec<usize> = (0..26).collect();
        if plugboard == "" {
            return unplugged_chars
        }
        let re = Regex::new(r"[^a-zA-Z]").unwrap();
        let pairings: Vec<&str> = re.split(plugboard).collect();
        for pair in pairings {
            let c1: usize = pair.chars().next().unwrap() as usize - 65;
            let c2: usize = pair.chars().next().unwrap() as usize - 65;
            unplugged_chars.remove(c1);
            unplugged_chars.remove(c2);
        }
        unplugged_chars
    }

    pub fn decode_plugboard(plugboard: &str) -> Vec<usize> {
        if plugboard == "" {
            return Self::identity_plugboard()
        }
        let re = Regex::new(r"[^a-zA-Z]").unwrap();
        let pairings: Vec<&str> = re.split(plugboard).collect();
        let mut plugged_chars = Vec::new();
        let mut mapping = Self::identity_plugboard();

        for pair in pairings {
            if pair.len() != 2 {
                return Self::identity_plugboard()
            }
            let c1: usize = pair.chars().nth(0).unwrap() as usize - 65;
            let c2: usize = pair.chars().nth(1).unwrap() as usize - 65;
            if plugged_chars.contains(&c1) || plugged_chars.contains(&c2) {
                return Self::identity_plugboard()
            }
            plugged_chars.push(c1);
            plugged_chars.push(c2);
            mapping[c1] = c2;
            mapping[c2] = c1;
        }
        mapping
    }
}