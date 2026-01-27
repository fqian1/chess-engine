use std::sync::OnceLock;

struct XorShift64 {
    value: u64,
}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        if seed == 0 {
            XorShift64 {
                value: 0xDEADC0DEBAADF00D,
            }
        } else {
            XorShift64 { value: seed }
        }
    }
    fn next(&mut self) -> u64 {
        let mut x = self.value;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.value = x;
        x
    }
}

pub struct ZobristKeys {
    // [color][piece][square]
    pub pieces: [[[u64; 64]; 6]; 2],
    pub castling: [u64; 16],
    pub en_passant: [u64; 8],
    pub side_to_move: u64,
}

impl ZobristKeys {
    pub fn new() -> Self {
        let mut rng = XorShift64::new(123456789);

        let mut pieces = [[[0; 64]; 6]; 2];
        for color in 0..2 {
            for piece in 0..6 {
                for sq in 0..64 {
                    pieces[color][piece][sq] = rng.next();
                }
            }
        }

        let mut castling = [0; 16];
        for i in 0..16 {
            castling[i] = rng.next();
        }

        let mut en_passant = [0; 8];
        for i in 0..8 {
            en_passant[i] = rng.next();
        }

        ZobristKeys {
            pieces,
            castling,
            en_passant,
            side_to_move: rng.next(),
        }
    }
    pub fn get() -> &'static Self {
        static INSTANCE: OnceLock<ZobristKeys> = OnceLock::new();
        INSTANCE.get_or_init(|| Self::new())
    }
}
