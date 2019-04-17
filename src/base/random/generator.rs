pub struct Generator {
    state: u64,
    inc: u64,
}

impl Generator {
    pub fn new(state: u64, sequence: u64) -> Generator {
        let mut g = Generator {
            state: 0,
            inc: (sequence << 1) | 1,
        };
        g.random_uint();
        g.state += state;
        g.random_uint();
        g
    }

    pub fn start(&mut self, state: u64, sequence: u64) {
        self.state = 0;
        self.inc = (sequence << 1) | 1;

        self.random_uint();
        self.state += state;
        self.random_uint();
    }

    pub fn random_uint(&mut self) -> u32 {
        self.advance_pcg32()
    }

    pub fn random_float(&mut self) -> f32 {
        let mut bits = self.advance_pcg32();

        bits &= 0x007FFFFF;
        bits |= 0x3F800000;

        unsafe { std::mem::transmute::<u32, f32>(bits) - 1.0 }
    }

    fn advance_pcg32(&mut self) -> u32 {
        let old = self.state;

        // Advance internal state
        self.state = old.wrapping_mul(6364136223846793005) + (self.inc | 1);

        // Calculate output function (XSH RR), uses old state for max ILP
        let xrs = (((old >> 18) ^ old) >> 27) as u32;

        let rot = (old >> 59) as u32;

        (xrs >> rot) | (xrs << (0_u32.wrapping_sub(rot) & 31))
    }
}
