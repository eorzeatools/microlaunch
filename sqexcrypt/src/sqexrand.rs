// Implementation of square enix's horrible
// custom random number generator

// why do you do this to me Square

// https://github.com/goatcorp/FFXIVQuickLauncher/blob/c98a4327a7dbdb43d5ad5f516b7a30618088d128/src/XIVLauncher.Common/Encryption/CrtRand.cs#L3

pub struct Sqexrand {
    seed: u32,
}

impl Sqexrand {
    pub fn new(seed: u32) -> Self {
        Self {
            seed
        }
    }

    pub fn next(&mut self) -> u32 {
        self.seed = 0x343fd_u32.wrapping_mul(self.seed) + 0x269ec3_u32;
        return ((self.seed >> 16) & 0xFFFF) & 0x7FFF;
    }
}