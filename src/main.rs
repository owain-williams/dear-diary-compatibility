use dear_diary_match_maker::{get_dob, random_number_from_seed};

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

    // Get user's date of birth
    println!("Please enter your date of birth in the format DDMMYYYY");
    let dob_user_result = get_dob();
    let dob_user = match dob_user_result {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };
    // Get crush's date of birth
    println!("Please enter your crush's date of birth in the format DDMMYYYY");
    let dob_crush_result = get_dob();
    let dob_crush = match dob_crush_result {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };

    // Sum DOBs
    let dob_sum = dob_user + dob_crush;

    // Get a Random Number using the summed DOBs as a Seed
    let seeded_random_number = random_number_from_seed(dob_sum as u64, results.len() - 1);

    println!("Your advice is: {}", results[seeded_random_number as usize]);
}
