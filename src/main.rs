use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::io;

fn main() {
    let results = vec![
        "Be more careful in all aspects with your partner",
        "Take it easy whenever you feel bad",
        "The key to happiness - is more patience",
        "It is in your interest - to make your partner happier",
        "A witty word can accomplish far more than a sour remark",
        "Give a little - take a little",
        "Hesitate, and your chance will go by",
        "Be careful, \"forgiving\" is not the same as \"forgetting\"",
        "If you really want your partner, you'll find a way",
        "You cannot improve relations by only looking backwards",
        "Be strong to face new situations with your partner",
        "Make it a habit - give small presents to your partner",
        "A very happy smile to your partner - will make his day",
        "Make a special gesture to your partner - once a day",
        "Do not show a sad face - it will make your partner also sad",
        "Rely on your own good judgement",
        "Smile, always smile, it is very difficult to resist a smile",
        "Cheerfulness is the core of happy relations",
        "Be more careful about your feelings to him",
        "Happiness lies in knowing that you are both happy",
    ];
    let mut dob_user = String::new();
    let mut dob_partner = String::new();

    // Get User's date of birth
    println!("Please enter your date of birth in the format DDMMYYYY");

    io::stdin()
        .read_line(&mut dob_user)
        .expect("Failed to read line");

    // Get Partner's date of birth
    println!("Please enter your crush's date of birth in the format DDMMYYYY");

    io::stdin()
        .read_line(&mut dob_partner)
        .expect("Failed to read line");

    // Parse DOBs
    let dob_user = dob_user.trim().parse::<u64>();
    let dob_user = match dob_user {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };
    let dob_partner = dob_partner.trim().parse::<u64>();
    let dob_partner = match dob_partner {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };

    let dob_sum = dob_user + dob_partner;

    let mut rng = ChaCha8Rng::seed_from_u64(dob_sum);
    let seeded_random_number = rng.gen_range(0..results.len() - 1);

    println!("{}", results[seeded_random_number as usize]);
}
