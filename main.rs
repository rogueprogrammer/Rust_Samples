use std::collections::HashMap;
fn main() {

    // CREATE NEW PROJECT - `cargo new <project_name>`
    // this will create new project with toml file

    // RUN PROJECT - `cargo run`

    // Arrays and Slices - https://doc.rust-lang.org/rust-by-example/primitives/array.html
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First elem = {}", x[0]);

    // y is an array of 500 32 bit integers, all assigned value 0
    let y: [i32; 500] = [0; 500];
    println!("y elem at 1 = {}", y[1]);
    println!("size of y: {}", y.len());

    let slice: &[i32] = &x[1..4]; // slice of 3 elems: indices 1, 2, 3
    for i in 0..slice.len() {
        println!("elem {} of slice = {}", i, slice[i]); // will print: 2, 3, 4
    }

    // STRINGS
    let s = String::new();
    let s = String::from("Hello");
    println!("Value of s: {}", s);
    let s = String::from("world");
    println!("Value of s: {}", s);

    // MUTABLE VARIABLES - allows var to get updated without having to use `let` keyword
    let mut s2 = "hello";
    s2 = "butt";
    println!("val of mutable variable s2 = {}", s2);

    // MUTABLE REFERENCES
    let s_ref = &mut s2; // create mutable ref to s2
    *s_ref = "yo-yo";
    println!("new val of mutable variable s2 = {}", s2); // "yo-yo"

    // STRUCTS
    struct Person{
        name: String,
        age: u8,
    }
    let dev = Person { name:String::from("Dev"), age:34 };
    println!("Dev's age: {}", dev.age);

    impl Person{
        fn new(name: String, age: u8) -> Person{
            return Person { name, age }
        }
        fn get_name(&self) -> &String{
            return &self.name
        }
    }
    let kabir = Person::new(String::from("Kabir"), 1);
    println!("Kabir's name: {}", kabir.get_name());

    // HASHMAPS:
    let mut map = HashMap::new();
    map.insert("Zinnia", 31);
    map.insert("Kabir", 1);
    map.insert("Dev", 34);
    if map.contains_key("Zinnia"){
        let age = map.get("Zinnia").unwrap_or(&-1); // Default value if key is not found
        println!("Found Zinnia! Age={}", age );
        let age2 = map.get("suzy").unwrap_or(&-1); // Default value if key is not found
        println!("suzy Age={}", age2 ); //prints -1
    }
    //iterate over the hashmap:
    // NOTE: Use &map, instead of map since rust requires us to "borrow" the hashmap instead of 
    // providing ownership. If & is not provided, then ownership transferred to for-loop and the map can't be used after loop
    for(key, val) in &map {
        println!("Key: {} Val: {}", key, val);
    }
    
    // ITERATORS
    let weather_types = vec!["sunny", "rainy", "snowy"];
    for weather_type in weather_types.iter() {
        match weather_type{
            &"sunny" => println!("Summer time!"),
            &"rainy" => println!("Spring time!"),
            &"snowy" => println!("Winter time!"),
            _ => {} // handle other types
        }
    }

    // SMART POINTERS (BOX)
    let new_arr = Box::new([1, 2, 3]);
    // dereference the pointer to access the numbers
    let numbers = *new_arr;
    println!("First number: {}", numbers[0]);
    println!("Second number: {}", numbers[1]);
    println!("Third number: {}", numbers[2]);
    
    // CRATE AS A LIB - https://doc.rust-lang.org/rust-by-example/crates/using_lib.html
    // 1. Create new file (rary.rs)
    // 2. Compile the file using: `rustc --crate-type=lib rary.rs` you will see a new lib file created
    // 3. Use the functions from the lib: rary::<function_name>
    // 4. Compile and Create executable for the main executable linking to the lib: `rustc main.rs --extern rary=library.rlib`
    // 5. run the executable now instead of `cargo run`: ./main.exe 
    rary::public_function();

    // CARGO PACKAGE MANAGEMENT - https://doc.rust-lang.org/rust-by-example/cargo/deps.html
    // CARGO TESTS - https://doc.rust-lang.org/rust-by-example/cargo/test.html
    
    println!("Hello, world!");
}
