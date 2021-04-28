use std::ptr;

struct EnigmaKey {
    rotors: Vec<String>,
    indicators: Vec<u32>,
    rings: Vec<u32>,
    plugboard: String,
}

impl EnigmaKey {
    pub fn new(rotors: Vec<String>, indicators: Vec<u32>, rings: Vec<u32>, plugboard_connections: String) -> Self {
        self::rotors = if rotors.is_null() {vec!["I", "II", "III"]} else {rotors};
        self::indicators = if indicators.is_null {vec![0,0,0]} else {indicators};
        self.rings = if rings.is_null {vec![0,0,0]} else {rings};
        self.plugboard = if plugboard_connections.is_null {""} else {plugboard_connections};
    }

    pub fn new_from_key(key: EnigmaKey) -> Self {

    }
}