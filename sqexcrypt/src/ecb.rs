use generic_array::{arr, typenum::U8};

use crate::memorystream::MemoryStream;

pub struct Ecb(Vec<u8>);

impl Ecb {
    pub fn from_plaintext(plain: Vec<u8>) -> Self {
        assert!(plain.len() % 8 == 0, "plaintext length must be divisible by 8");

        Self(plain)
    }

    pub fn encrypt<T>(&mut self, bfish: &mut T) -> MemoryStream
        where T: blowfish::cipher::BlockEncryptMut<BlockSize=U8>
    {
        let block_count = self.0.len() / 8;
        let mut output_blocks = MemoryStream::new();
        for block_num in 0..block_count {
            let block_start = block_num * 8;
            let block_end = (block_num + 1) * 8;
            let block_in = &self.0[block_start..block_end];
            let mut block_in_arr = arr![u8; 0, 0, 0, 0, 0, 0, 0, 0];
            block_in_arr.clone_from_slice(block_in);
            let mut block_out_arr = arr![u8; 0, 0, 0, 0, 0, 0, 0, 0];
            bfish.encrypt_block_b2b_mut(&block_in_arr, &mut block_out_arr);
            output_blocks.merge(block_out_arr);
        }
        output_blocks
    }
}