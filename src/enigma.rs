mod rotor;
mod reflector;
mod plugboard;

#[derive(Default)]
struct Enigma {
    left_rotor: Rotor,
    mid_rotor: Rotor,
    right_rotor: Rotor,
    reflector: Reflector,
    plugboard: Plugboard,
}