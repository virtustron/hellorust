use rand::Rng;

mod cppmath;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..10);
    println!("Generated number: {}", random_number);

    
    unsafe {
        let perimeter = cppmath::CalculatePerimeter(1, 2, 3);
        println!("CalculatePerimeter result: {}", perimeter);
    }

    unsafe {
        let sum = cppmath::CalculateSum(5, 5);
        println!("CalculateSum result: {}", sum);
    }
    
}
