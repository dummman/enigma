use std::collections::HashSet;

struct Plugboard {
    wiring: Vec<u32>,
}

impl Plugboard {
    pub fn new(connections: String) -> Self {
        Plugboard{wiring: decode_Plugboard(connections),}
    }

    pub fn forward(c: u32) -> u32 {
        wiring[c]
    }

    fn identity_plugboard() -> Vec<u32> {
        (0..26).collect()
    }

    pub fn get_unplugged_chars(plugboard: String) -> HashSet {
        let mut unplugged_chars: HashSet = (0..26).collect();
        // let mut a: HashSet = vec![1i32, 2, 3].into_iter().collect();
        if plugboard == "" {
            unplugged_chars
        }
        let re = Regex::new(r"[^a-zA-Z]").unwrap();
        let pairings: Vec<String> = re.split(plugboard).collect();
        for pair in pairings {
            let c1: u32 = pair.chars().next().unwrap() as u32 - 65;
            let c2: u32 = pair.chars().next().unwrap() as u32 - 65;
            unplugged_chars.remove(&c1);
            unplugged_chars.remove(&c2);
        }
        unplugged_chars
    }

    pub fn decode_plugboard(plugboard: String) -> Vec<u32> {
        if plugboard.is_null() || plugboard == "" {
            Identity_Plugboard
        }
        let re = Regex::new(r"[^a-zA-Z]").unwrap();
        let pairings: Vec<String> = re.split(plugboard).collect();
        let mut plugged_chars = HashSet::new();
        let mapping = Identity_Plugboard;

        for pair in pairings {
            if pair.len() != 2 {
                Identity_Plugboard
            }
            let c1: u32 = pair.chars().next().unwrap() as u32 - 65;
            let c2: u32 = pair.chars().next().unwrap() as u32 - 65;
            if plugged_chars.contains(&c1) || plugged_chars.contains(&c2) {
                identityPlugboard()
            }
            plugged_chars.add(c1);
            plugged_chars.add(c2);
            mapping[c1] = c2;
            mapping[c2] = c1;
        }
        mapping
    }
}