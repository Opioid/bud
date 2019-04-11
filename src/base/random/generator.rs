pub struct State {
    state: u64,
    inc: u64,
}

pub struct Generator {
    state: State,
}

impl Generator {
    pub fn new(state: u64, sequence: u64) -> Generator {
        let mut g = Generator {
            state: State {
                state: state,
                inc: sequence,
            },
        };
        g.start(state, sequence);
        g
    }

    pub fn start(&mut self, state: u64, sequence: u64) {
        self.state.state = 0;
        self.state.inc = (sequence << 1) | 1;

        self.random_uint();
        self.state.state += state;
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
        let oldstate = self.state.state;

        // Advance internal state
        self.state.state = oldstate.wrapping_mul(6364136223846793005) + (self.state.inc | 1);

        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;

        let rot = (oldstate >> 59) as u32;

        (xorshifted >> rot) | (xorshifted << ((0xFFFFFFFF - rot) & 31))
    }
}
