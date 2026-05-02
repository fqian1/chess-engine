use std::sync::OnceLock;

pub struct XorShift64 {
    value: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        if seed == 0 {
            XorShift64 { value: 0xDEADC0DEBAADF00D }
        } else {
            XorShift64 { value: seed }
        }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.value;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.value = x;
        x
    }

    // 0.0 to 1.0
    pub fn next_f32(&mut self) -> f32 {
        let x = self.next();
        let bits = ((x >> 41) as u32) | 0x3f800000;
        f32::from_bits(bits) - 1.0
    }
}

pub struct ZobristKeys {
    // [color][piece][square]
    pub pieces: [[[u64; 64]; 6]; 2],
    pub castling: [u64; 16],
    pub en_passant: [u64; 8],
    pub side_to_move: u64,
}

impl Default for ZobristKeys {
    fn default() -> Self {
        Self::new(123456789)
    }
}

impl ZobristKeys {
    pub fn new(seed: u64) -> Self {
        let mut rng = XorShift64::new(seed);

        let mut pieces = [[[0u64; 64]; 6]; 2];

        for color in pieces.iter_mut() {
            for piece_type in color.iter_mut() {
                for square in piece_type.iter_mut() {
                    *square = rng.next();
                }
            }
        }

        let mut castling = [0; 16];
        for i in castling.iter_mut() {
            *i = rng.next();
        }

        let mut en_passant = [0; 8];
        for i in en_passant.iter_mut() {
            *i = rng.next();
        }

        ZobristKeys { pieces, castling, en_passant, side_to_move: rng.next() }
    }

    pub fn get() -> &'static Self {
        static INSTANCE: OnceLock<ZobristKeys> = OnceLock::new();
        INSTANCE.get_or_init(Self::default)
    }
}
