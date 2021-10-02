use rand::Rng;

fn main() {
    //println!("Hello, world!");

    let random_number = rand::thread_rng().gen_range(0..10);
 
    println!("Generated number: {}", random_number);
}
