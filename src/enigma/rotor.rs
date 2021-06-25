#[derive(Default)]
pub struct Rotor<'a> {
    name: &'a str,
    rotor_position: usize,
    notch_position: [usize; 2],
    ring_setting: usize,
    forward_wiring: Vec<usize>,
    backward_wiring: Vec<usize>,
}

impl<'a> Rotor<'a> {
    pub fn new(name: &'a str, rotor_position: usize, notch_position: [usize; 2], ring_setting: usize, encoding: &str) -> Self {
        let mut r = Self {
            name,
            rotor_position,
            notch_position,
            ring_setting,
            forward_wiring: vec![0; 0],
            backward_wiring: vec![0; 0],
        };
        r.forward_wiring = r.decode_wiring(encoding);
        let fw: &Vec<usize> = r.forward_wiring.as_ref();
        r.backward_wiring = r.inverse_wiring(fw.to_vec());
        r
    }

    pub fn create(name: &'a str, rotor_position: usize, ring_setting: usize) -> Self {
        match name {
            "I" => Self::new(name, rotor_position, [16, 0], ring_setting, "EKMFLGDQVZNTOWYHXUSPAIBRCJ"),
            "II" => Self::new(name, rotor_position,[4, 0], ring_setting, "AJDKSIRUXBLHWTMCQGZNPYFVOE"),
            "III" => Self::new(name, rotor_position, [21, 0], ring_setting, "BDFHJLCPRTXVZNYEIWGAKMUSQO"),
            "IV" => Self::new(name, rotor_position,[9, 0], ring_setting, "ESOVPZJAYQUIRHXLNFTGKDCMWB"),
            "V" => Self::new(name, rotor_position, [25, 0], ring_setting, "VZBRGITYUPSDNHLXAWMJQOFECK"),
            "VI" => Self::new(name, rotor_position, [12, 25], ring_setting, "VZBRGITYUPSDNHLXAWMJQOFECK"),
            "VII" => Self::new(name, rotor_position, [12, 25], ring_setting, "NZJHGRCXMYSWBOUFAIVLPEKQDT"),
            "VIII" => Self::new(name, rotor_position, [12, 25], ring_setting, "FKQHTLXOCBJSPDZRAMEWNIUYGV"),
            _ => Self::new("Identity", rotor_position,[0, 0], ring_setting, "ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
        }
    }

    fn decode_wiring(&self, encoding: &str) -> Vec<usize> {
        let char_vec: Vec<char> = encoding.chars().collect();
        let mut wiring = vec![0usize; char_vec.len()];
        let iter = wiring.iter_mut().zip(char_vec.into_iter());
        for (w, c) in iter {
            *w = c as usize - 65;
        }
        wiring
    }

    fn inverse_wiring(&self, forward_wiring: Vec<usize>) -> Vec<usize> {
        let mut inverse = vec![0usize; forward_wiring.len()];
        for idx in 0..forward_wiring.len() {
            let forward = forward_wiring[idx];
            inverse[forward] = idx;
        }
        inverse
    }

    pub fn is_at_notch(&self) -> bool {
        let mut result: bool = false;
        for notch in self.notch_position.iter() {
            if *notch == 0 {
                continue;
            }
            if self.rotor_position == *notch {
                result = true;
                break;
            }
        }
        result
    }

    fn encipher(k: usize, pos: usize, ring: usize, mapping: &Vec<usize>) -> usize {
        let shift = pos as i32 - ring as i32;
        ((mapping[((k as i32 + shift + 26) % 26) as usize] as i32 - shift + 26) % 26) as usize
    }

    pub fn forward(&self, c: usize) -> usize {
        Self::encipher(c, self.rotor_position, self.ring_setting, &self.forward_wiring)
    }

    pub fn backward(&self, c: usize) -> usize {
        Self::encipher(c, self.rotor_position, self.ring_setting, &self.backward_wiring)
    }

    pub fn turnover(&mut self) {
        self.rotor_position = (self.rotor_position + 1) % 26;
    }
}