use rand::Rng;


mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindtest.rs"));
}


fn main() {
    let random_number = rand::thread_rng().gen_range(0..10);
    println!("Generated number: {}", random_number);

    
    unsafe {
        let perimeter = bindings::CalculatePerimeter(1, 2, 3);
        println!("CalculatePerimeter result: {}", perimeter);
    }

    unsafe {
        let sum = bindings::CalculateSum(5, 5);
        println!("CalculateSum result: {}", sum);
    }
}
