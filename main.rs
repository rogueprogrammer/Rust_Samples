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

    println!("Hello, world!");
}
