// Import the rand crate
use rand::Rng;

fn main(){
    // Initialize the random number generator
    let mut rng = rand::thread_rng();

    // Generate a random integer in a specific range (inclusive)
    let random_int: i32 = rng.gen_range(1..=100);
    println!("Random Integer: {}", random_int);

    // Generate a random floating point number in the range [0.0,1.0)
    let random_float: f64 = rng.gen();
    println!("Random float: {}", random_float);

    // Generate a random boolean value
    let random_bool: bool = rng.gen();
    println!("Random bool: {}", random_bool);
}