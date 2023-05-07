pub struct PCG32 {
    state: u64,
    multiplier: u64,
    increment: u64,
}

pub const PCG32_MULTIPLIER: u64 = 6364136223846793005_u64;
pub const PCG32_INCREMENT: u64 = 1442695040888963407_u64;

impl PCG32 {
    pub fn new(seed: u64, multiplier: u64, stream: u64) -> Self {
        let increment = (stream << 1) | 1;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };
        pcg.next();
        pcg
    }

    pub fn new_default(seed: u64) -> Self {
        let multiplier = PCG32_MULTIPLIER;
        let increment = PCG32_INCREMENT;
        let mut pcg = PCG32 {
            state: seed.wrapping_add(increment),
            multiplier,
            increment,
        };
        pcg.next();
        pcg
    }

    pub fn next(&mut self) {
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
    }

    #[inline]
    pub fn get_u64(&mut self) -> u64 {
        let oldstate = self.state;
        self.next();
        oldstate
    }
}