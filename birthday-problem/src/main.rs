mod tests;
mod utils;

use rand::rngs::ThreadRng;
use utils::simulate_prob_of_same_birthday;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();
    for n_people in 5..51 {
        let prob: f32 = simulate_prob_of_same_birthday(1000, n_people, &mut rng);
        println!("{},{}", n_people, prob);
    }
}
