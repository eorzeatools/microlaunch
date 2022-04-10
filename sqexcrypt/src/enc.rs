// Horrible stupid no-good "custom" """encryption""" algorithm
// written by Square Enix for FFXIV Steam authentication tickets
//
// (It's actually Blowfish)
// (but they fucked it because of course)
//
// I hate this fucking company so much

// A giant thank you to Andrea @hikari_no_yume
// for helping me port this shitfest from C# to Rust.

use crate::{memorystream::MemoryStream, sqexrand::Sqexrand};

const SQEX_FUCKED_ALPHABET: &'static str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_";

pub fn encrypt(bytes: Vec<u8>, steam_time: u32) {
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

    // TODO: Finish this algorithm with Andrea tomorrow
}

mod tests {
    #[test]
    pub fn test_crypt() {

    }
}