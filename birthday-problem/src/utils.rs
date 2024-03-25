use rand::{rngs::ThreadRng, Rng};

pub fn simulate_prob_of_same_birthday(
    n_simulations: i32,
    n_people: i32,
    generator: &mut ThreadRng,
) -> f32 {
    let mut n_duplicated_birthdays: i32 = 0;
    for _ in 0..n_simulations {
        let birthdays: Vec<i32> = simulate_birthdays(n_people, generator);
        let any_duplicates: bool = contains_duplicates(&birthdays);
        if any_duplicates {
            n_duplicated_birthdays += 1;
        }
    }
    let prob_same_birthday: f32 = n_duplicated_birthdays as f32 / n_simulations as f32;
    return prob_same_birthday;
}

pub fn simulate_birthdays(number_of_people: i32, rng: &mut ThreadRng) -> Vec<i32> {
    let mut i: i32 = 0;
    let mut birthdays: Vec<i32> = Vec::new();
    birthdays.reserve(number_of_people as usize);
    while i < number_of_people {
        let birthday: i32 = rng.gen_range(1..365);
        birthdays.push(birthday);
        i += 1;
    }
    return birthdays;
}

// the "problem" with this function is that it requires the reference to
// the input vector to be mutable, but ideally detecting a duplicated
// value in a vector doesn't require mutability
pub fn has_duplicates(birthdays: &mut Vec<i32>) -> bool {
    birthdays.sort();
    let birthdays_copy: Vec<i32> = birthdays.clone();
    birthdays.dedup();
    return birthdays_copy.len() != birthdays.len();
}

// this function is "better" because the underlying vector doesn't need
// to be mutable through its reference
pub fn contains_duplicates(birthdays: &Vec<i32>) -> bool {
    let mut tally: Vec<i32> = Vec::new();
    tally.push(birthdays[0]);
    for bday in &birthdays[1..] {
        if tally.contains(bday) {
            return true;
        } else {
            tally.push(*bday);
        }
    }
    return false;
}

