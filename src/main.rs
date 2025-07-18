use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn vector_function() {
    let mut vector = vec![10, 20, 30, 40];

    // 1. len() function
    println!("1. Vector length: {}", vector.len());

    // 2. push() function
    vector.push(50);
    println!("2. Current Vector: {:?}", vector);

    // 3. get() function
    match vector.get(3) {
        Some(val) => println!("3. The value at index 3 is: {}", val),
        None => println!("3. No value in index 3"),
    }

    // 4. filter() function
    let even_numbers: Vec<i32> = vector.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("4. Even numbers in vector: {:?}", even_numbers);

    // 5. retain() function
    vector.retain(|x| *x == 20);
    println!("5. Retained values (only 20): {:?}", vector);
}

fn hashmap_function() {
    let mut user_hash_map = HashMap::new();

    // 1. insert() function
    user_hash_map.insert("value1", 10);
    println!("1. HashMap after insert: {:?}", user_hash_map);

    // 2. get() function
    if let Some(value) = user_hash_map.get("value1") {
        println!("2.1. Value of 'value1': {}", value);
    }
    match user_hash_map.get("value1") {
        Some(val) => println!("2.2. Value of 'value1': {}", val),
        None => println!("2.2. Invalid key"),
    }

    // 3. entry() function
    user_hash_map.entry("value2").insert_entry(80);
    user_hash_map.entry("value3").or_insert(90);
    println!("3. After entry() function: {:?}", user_hash_map);

    // 4. retain() function
    user_hash_map.retain(|_, v| *v == 90);
    println!("4. After retain (only 90 kept): {:?}", user_hash_map);

    // 5. or_insert_with() function
    user_hash_map.entry("value4").or_insert_with(|| 100);
    user_hash_map.entry("value5").or_insert_with_key(|_| 70);
    println!("5. HashMap after or_insert_with(): {:?}", user_hash_map);
}

// Custom error type
#[derive(Debug)]
struct MyCustomError;

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Something went wrong in custom operation!")
    }
}

impl std::error::Error for MyCustomError {}

fn custom_error_demo(my_value: bool) -> Result<(), MyCustomError> {
    if my_value {
        Err(MyCustomError)
    } else {
        Ok(())
    }
}

// read_function using match on Result
fn read_function(path: &str) -> Result<String, io::Error> {
    let file = File::open(path);
    match file {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content)?;
            Ok(content)
        }
        Err(e) => Err(e),
    }
}

// read_function_2 using ? operator
fn read_function_2(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn error_handling() {
    // 1. Option<T> with Some and None
    let value = Some(5);
    match value {
        Some(num) => println!("1. Option value is {}", num),
        None => println!("1. No value found"),
    }

    // 2. unwrap_or()
    let username: Option<i32> = None;
    let name = username.unwrap_or(25);
    println!("2. unwrap_or: The value is {}", name);

    // 3. Result<T, E> using match
    let result = read_function("read.txt");
    match result {
        Ok(content) => println!("3. File content (read_function): {}", content),
        Err(e) => println!("3. Error: {}", e),
    }

    // 4. Result with ? operator
    let result2 = read_function_2("Cargo.toml");
    match result2 {
        Ok(content) => println!("4. File content length: {}", content.len()),
        Err(e) => println!("4. Error: {}", e),
    }

    // 5. Custom error handling
    match custom_error_demo(false) {
        Ok(_) => println!("5. Operation succeeded"),
        Err(e) => println!("5. Custom error: {}", e),
    }
}

fn main() {
    println!("----- Vector Functions -----");
    vector_function();

    println!("\n----- HashMap Functions -----");
    hashmap_function();

    println!("\n----- Error Handling -----");
    error_handling();
}
