#[derive(Default)]
struct Rotor {
    name: String,
    rotor_position: u32,
    notch_position: [u32; 2],
    ring_setting: u32,
    forward_wiring: Vec<u32>,
    backward_wiring: Vec<u32>,
}

impl Rotor {
    pub fn new(name: String, rotor_position: u32, notch_position: [u32; 2], ring_setting: u32, encoding: String) -> Self {
        Rotor {
            name,
            rotor_position,
            notch_position,
            ring_setting,
            forward_wiring: decode_wiring(encoding),
            backward_wiring: inverse_wiring(forward_wiring),
        }
    }

    pub fn create(name: String, rotor_position: u32, ring_setting: u32) -> Self {
        match name.to_str() {
            "I" => self::new(name, rotor_position, [16, 0], ring_setting, "EKMFLGDQVZNTOWYHXUSPAIBRCJ"),
            "II" => self::new(name, rotor_position,[4, 0], ring_setting, "AJDKSIRUXBLHWTMCQGZNPYFVOE"),
            "III" => self::new(name, rotor_position, [21, 0], ring_setting, "BDFHJLCPRTXVZNYEIWGAKMUSQO"),
            "IV" => self::new(name, rotor_position,[9, 0], ring_setting, "ESOVPZJAYQUIRHXLNFTGKDCMWB"),
            "V" => self::new(name, rotor_position, [25, 0], ring_setting, "VZBRGITYUPSDNHLXAWMJQOFECK"),
            "VI" => self::new(name, rotor_position, [12, 25], ring_setting, "VZBRGITYUPSDNHLXAWMJQOFECK"),
            "VII" => self::new(name, rotor_position, [12, 25], ring_setting, "NZJHGRCXMYSWBOUFAIVLPEKQDT"),
            "VIII" => self::new(name, rotor_position, [12, 25], ring_setting, "FKQHTLXOCBJSPDZRAMEWNIUYGV"),
            _ => self::new("Identity", rotor_position,[0, 0], ring_setting, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
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

    fn inverse_wiring(forward_wiring: Vec<u32>) -> Vec<u32> {
        let mut inverse = vec![0; forward_wiring.len()];
        for idx in 0..forward_wiring.len() {
            let forward = forward_wiring[idx];
            inverse[forward] = idx;
        }
        inverse
    }

    fn is_at_notch(&self) -> bool {
        let mut result: bool = false;
        for notch in self.notch_position.iter() {
            if notch == 0 {
                continue;
            }
            if self.rotor_position == notch {
                result = true;
                break;
            }
        }
        result
    }

    fn encipher(k: u32, pos: u32, ring: u32, mapping: Vec<u32>) -> u32 {
        let shift = pos - ring;
        (mapping[(k + shift + 26) % 26] - shift + 26) % 26
    }

    pub fn forward(&self, c: u32) -> u32 {
        encipher(c, self.rotor_position, self.ring_setting, &self.forward_wiring)
    }

    pub fn backward(&self, c: u32) -> u32 {
        encipher(c, self.rotor_position, self.ring_setting, &self.backward_wiring)
    }

    pub fn turnover(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26
    }
}