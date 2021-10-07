// this file originally here: 
//   https://rust-lang.github.io/rust-bindgen/tutorial-3.html

extern crate bindgen;

fn main() {
    
    let src = [
        "src/mygeometry.cpp",
        "src/mymath.cpp",
    ];
    
    
    println!("cargo:rerun-if-changed=src/wrapper.hpp");

    cc::Build::new()
        .cpp(true)    
        .files(src.iter())
        //.file("src/mymath.cpp")
        .compile("cpp_math");
    
    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/cppmath.rs")
        .expect("Couldn't write bindings!");

    

    
}