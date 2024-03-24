
use rand::{rngs::ThreadRng, Rng};

fn simulate_birthdays(number_of_people: i32, rng: &mut ThreadRng) -> Vec<i32> {
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

fn has_duplicates(birthdays: &mut Vec<i32>) -> bool {
    birthdays.sort();
    let birthdays_copy: Vec<i32> = birthdays.clone();
    birthdays.dedup();
    return birthdays_copy.len() != birthdays.len();
}

fn simulate_prob_of_same_birthday(n_simulations: i32, n_people: i32, generator: &mut ThreadRng) -> f32 {
    let mut n_duplicated_birthdays: i32 = 0;
    for _ in 0..n_simulations {
        let mut birthdays: Vec<i32> = simulate_birthdays(n_people, generator);
        let any_duplicates: bool = has_duplicates(&mut birthdays);
        if any_duplicates {
            n_duplicated_birthdays += 1;
        }         
    }

    let prob_same_birthday: f32 = n_duplicated_birthdays as f32 / n_simulations as f32;
    return prob_same_birthday;
}

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    for n_people in 5..50 {
        let prob: f32 = simulate_prob_of_same_birthday(
            1000, n_people, &mut rng
        );
        println!("{}", prob);
    } 
}
