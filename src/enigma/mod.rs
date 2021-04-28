pub mod rotor;
pub mod reflector;
pub mod plugboard;

#[derive(Default)]
struct Enigma {
    left_rotor: Rotor,
    mid_rotor: Rotor,
    right_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}

impl Enigma {
    pub fn new(rotors: Vec<String>, reflector: String, rotor_positions: Vec<u32>, ring_settings: Vec<u32>, plugboard_connections: String) -> Self {
        Enigma {
            left_rotor: Rotor.create(rotors[0], rotor_positions[0], ring_settings[0]),
            mid_rotor: Rotor.create(rotors[1], rotor_positions[1], ring_settings[1]),
            right_rotor: Rotor.create(rotors[2], rotor_positions[2], ring_settings[2]),
            reflector: Reflector.create(reflector),
            plugboard: Plugboard.new(plugboard_connections),
        }
    }

    pub fn new_from_key(key: EnigmaKey) -> Self {
        Self(key.rotors, "B", key.indicators, key.rings, key.plugboard)
    }

    pub fn rotate() {
        if self::mid_rotor.is_at_notch() {
            self::mid_rotor.turnover();
            self::left_rotor.turnover();
        }
        else if self::right_rotor.is_at_notch() {
            self::mid_rotor.turnover();
        }
        self::right_rotor.turnover();
    }

    pub fn encrypt(mut c: u32) -> u32 {
        self::rotate();

        c = self::plugboard.forward(c);

        let c1: u32 = self::right_rotor.forward(c);
        let c2: u32 = self::right_rotor.forward(c1);
        let c3: u32 = self::right_rotor.forward(c2);

        let c4: u32 = self::reflector.forward(c3);

        let c5: u32 = self::left_rotor.backward(c4);
        let c6: u32 = self::left_rotor.backward(c5);
        let c7: u32 = self::left_rotor.backward(c6);

        self::plugboard.forward(c7)
    }
}