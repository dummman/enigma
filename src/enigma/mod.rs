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
}