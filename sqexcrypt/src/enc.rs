// Horrible stupid no-good "custom" """encryption""" algorithm
// written by Square Enix for FFXIV Steam authentication tickets
//
// (It's actually Blowfish)
// (but they fucked it because of course)
//
// I hate this fucking company so much

// A giant thank you to Andrea @hikari_no_yume
// for helping me port this shitfest from C# to Rust.

use crate::{memorystream::MemoryStream, sqexrand::Sqexrand, ecb::Ecb};

const SQEX_FUCKED_ALPHABET: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

fn to_fucked_se_b64(bytes: Vec<u8>) -> String {
    data_encoding::BASE64.encode(&bytes)
        .replace("+", "-")
        .replace("/", "_")
        .replace("=", "*")
}

pub fn encrypt(bytes: Vec<u8>, steam_time: u32) -> (String, usize) {
    let mut steam_time = steam_time;
    steam_time -= 5;
    steam_time -= steam_time % 60;

    let ticket_string = data_encoding::HEXLOWER.encode(&bytes);
    let raw_ticket_bytes = ticket_string.as_bytes();

    let mut raw_ticket = vec![];
    raw_ticket.extend_from_slice(raw_ticket_bytes);
    raw_ticket.push(0);

    let blowfish_key = format!("{:08x}#un@e=x>", steam_time);

    let mut memory_stream = MemoryStream::new();

    // regular sum + ticket
    let mut ticket_sum = 0_u16;

    for i in &raw_ticket {
        ticket_sum = ticket_sum.wrapping_add((*i).into());
    }

    memory_stream.push_u16(ticket_sum);
    memory_stream.merge(raw_ticket.clone());

    // garbage
    let cast_ticket_sum = ticket_sum as i16 as i32;
    let seed = (steam_time as i32) ^ cast_ticket_sum;
    let mut rand = Sqexrand::new(seed as u32);

    // here comes the pain
    // is this a compiler optimisation?
    let num_random_bytes = ((((&raw_ticket).len() + 9) as u64) & 0xFFFFFFFFFFFFFFF8) - 2 - ((&raw_ticket).len() as u64);
    let mut garbage = vec![0_u8; num_random_bytes as usize];

    let mut fucked_sum = u32::from_le_bytes((&memory_stream.0[0..4]).try_into().unwrap());

    for i in 0..num_random_bytes {
        let rand_char = SQEX_FUCKED_ALPHABET.chars().nth(((fucked_sum + rand.next()) & 0x3F) as usize).unwrap();
        garbage[i as usize] = rand_char as u8;
        fucked_sum += rand_char as u8 as u32;
    }

    memory_stream.merge(garbage);

    memory_stream.merge_at(fucked_sum.to_le_bytes(), 0);

    let mut final_bytes = memory_stream.into_bytes();

    // Horrible byte swap
    let t = final_bytes[0].clone();
    final_bytes[0] = final_bytes[1];
    final_bytes[1] = t;

    let key_bytes = blowfish_key.as_bytes();

    let mut bfish = blowfish::Blowfish::bc_init_state();
    bfish.bc_expand_key(key_bytes);

    let mut ecb = Ecb::from_plaintext(final_bytes);
    let crypt = ecb.encrypt(&mut bfish);

    let crypt_bytes = crypt.into_bytes();

    let fucked_b64 = to_fucked_se_b64(crypt_bytes);

    const CHUNK_SIZE: usize = 300;
    let fucked2 = fucked_b64.clone().into_bytes();
    let parts = fucked2.chunks(CHUNK_SIZE);

    let final_string = parts.collect::<Vec<&[u8]>>();

    let final_parts = final_string.len();
    let really_final_string = final_string.join(&(',' as u8));

    let really_really_final_string = String::from_utf8(really_final_string).unwrap();
    let real_len = really_really_final_string.len();

    (really_really_final_string, real_len - (final_parts - 1))
}

mod tests {
    #[test]
    pub fn test_crypt() {
        let test_bytes =
            data_encoding::HEXLOWER.decode(include_str!("test_ticket.txt").to_lowercase().as_bytes());
        let test_time = 1649762720_u32;
        let _crypt = super::encrypt(test_bytes.unwrap(), test_time);
    }
}