// Import the rand crate
use rand::Rng;

fn main(){
    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Generate a random integer in a specific range (inclusive)
    let random_int: i32 = rng.gen_range(1..=100);
    println!("Random Integer: {}", random_int);
}