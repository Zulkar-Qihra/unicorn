// SPARC registers
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum RegisterSPARC {
    INVALID = 0,
    F0 = 1,
    F1 = 2,
    F2 = 3,
    F3 = 4,
    F4 = 5,
    F5 = 6,
    F6 = 7,
    F7 = 8,
    F8 = 9,
    F9 = 10,
    F10 = 11,
    F11 = 12,
    F12 = 13,
    F13 = 14,
    F14 = 15,
    F15 = 16,
    F16 = 17,
    F17 = 18,
    F18 = 19,
    F19 = 20,
    F20 = 21,
    F21 = 22,
    F22 = 23,
    F23 = 24,
    F24 = 25,
    F25 = 26,
    F26 = 27,
    F27 = 28,
    F28 = 29,
    F29 = 30,
    F30 = 31,
    F31 = 32,
    F32 = 33,
    F34 = 34,
    F36 = 35,
    F38 = 36,
    F40 = 37,
    F42 = 38,
    F44 = 39,
    F46 = 40,
    F48 = 41,
    F50 = 42,
    F52 = 43,
    F54 = 44,
    F56 = 45,
    F58 = 46,
    F60 = 47,
    F62 = 48,
    FCC0 = 49,
    FCC1 = 50,
    FCC2 = 51,
    FCC3 = 52,
    G0 = 53,
    G1 = 54,
    G2 = 55,
    G3 = 56,
    G4 = 57,
    G5 = 58,
    G6 = 59,
    G7 = 60,
    I0 = 61,
    I1 = 62,
    I2 = 63,
    I3 = 64,
    I4 = 65,
    I5 = 66,
    FP = 67,
    I7 = 68,
    ICC = 69,
    L0 = 70,
    L1 = 71,
    L2 = 72,
    L3 = 73,
    L4 = 74,
    L5 = 75,
    L6 = 76,
    L7 = 77,
    O0 = 78,
    O1 = 79,
    O2 = 80,
    O3 = 81,
    O4 = 82,
    O5 = 83,
    SP = 84,
    O7 = 85,
    Y = 86,
    XCC = 87,
    PC = 88,
}

impl From<RegisterSPARC> for i32 {
    fn from(r: RegisterSPARC) -> Self {
        r as i32
    }
}
