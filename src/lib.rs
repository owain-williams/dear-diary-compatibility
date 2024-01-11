use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::{io, num::ParseIntError};

pub fn get_dob() -> Result<u32, ParseIntError> {
    let mut dob = String::new();

    // Get Date of Birth from user
    io::stdin()
        .read_line(&mut dob)
        .expect("Failed to read line");

    // Parse Date of Birth
    let dob = dob.trim().parse::<u32>();

    dob
}

pub fn get_random_number_from_seed(seed: u64, max: usize) -> usize {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let seeded_random_number = rng.gen_range(0..max);
    seeded_random_number
}
