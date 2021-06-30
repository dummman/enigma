use crate::enigma::plugboard::Plugboard;
use crate::enigma::reflector::Reflector;
use crate::enigma::rotor::Rotor;

// use crate::enigma::Rotor;
pub mod rotor;
pub mod reflector;
pub mod plugboard;

#[derive(Default)]
pub struct Enigma<'a> {
    left_rotor: Rotor<'a>,
    mid_rotor: Rotor<'a>,
    right_rotor: Rotor<'a>,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl<'a> Enigma<'a> {
    pub fn new(rotors: Vec<&'a str>, reflector: &str, rotor_positions: Vec<usize>, ring_settings: Vec<usize>, plugboard_connections: &str) -> Self {
        Enigma {
            left_rotor: Rotor::create(rotors[0], rotor_positions[0], ring_settings[0]),
            mid_rotor: Rotor::create(rotors[1], rotor_positions[1], ring_settings[1]),
            right_rotor: Rotor::create(rotors[2], rotor_positions[2], ring_settings[2]),
            reflector: Reflector::create(reflector),
            plugboard: Plugboard::new(plugboard_connections),
        }
    }

    pub fn rotate(&mut self) {
        if self.mid_rotor.is_at_notch() {
            self.mid_rotor.turnover();
            self.left_rotor.turnover();
        }
        else if self.right_rotor.is_at_notch() {
            self.mid_rotor.turnover();
        }
        self.right_rotor.turnover();
    }

    pub fn encrypt_int(&mut self, c: usize) -> usize {
        self.rotate();

        let c0 = self.plugboard.forward(c);


        let c1: usize = self.right_rotor.forward(c0);
        let c2: usize = self.mid_rotor.forward(c1);
        let c3: usize = self.left_rotor.forward(c2);

        let c4: usize = self.reflector.forward(c3);

        let c5: usize = self.left_rotor.backward(c4);
        let c6: usize = self.mid_rotor.backward(c5);
        let c7: usize = self.right_rotor.backward(c6);

        self.plugboard.forward(c7)
    }

    pub fn encrypt_char(&mut self, c: char) -> char {
        (self.encrypt_int(c as usize - 65) + 65) as u8 as char
    }

    pub fn encrypt_chararray(&mut self, c: &str) -> String {
        let mut output  = String::new();
        for character in c.chars() {
            output.push(self.encrypt_char(character));
        }
        output
    }
}
