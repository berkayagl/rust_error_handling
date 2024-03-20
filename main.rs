#[warn(unused_imports)]
#[allow(unused)]

use std::{fs::File, io::{Read}};

fn divide(x: i32, y: i32) -> Result<i32, String> {
     
    if y == 0 {
        return Err(String::from("divide = 0"));
    }

    Ok(x / y)
}

fn divide2(x: i32, y: i32) -> i32 {
     
    if y == 0 {
        panic!("divide2 = 0")
    }

    x / y
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return  Err(e),
    };

    let mut contents = String::new();
    
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(e),
    }
}

fn read_file2(path: &str) -> Result<String, std::io::Error> {
    
    let mut file =  File::open(path)?;

    let mut contents = String::new();
    
    file.read_to_string(&mut contents)?;
    Ok(contents)

    
}

fn main() {

    // Result enum - Result<x, y>
    let result = divide(10, 5);
    
    match result {
        Ok(value) => println!("Result: {} / {} = {}", 10, 5, value),
        Err(msg) => println!("Error: {}", msg),
    }

    println!("Finish!");

    // uwnrap
    let result = divide(10, 5).unwrap();
    println!("Result: {} / {} = {}", 10, 5, result);

    println!("Finish!");

    // except
    let result = divide(10, 5).expect("divide = 0");
    println!("Result: {} / {} = {}", 10, 5, result);

    println!("Finish!");

    // panic
    let result = divide2(10, 5);
    println!("Result : {} / {} = {}", 10, 5, result);

    println!("Finish!");    

    // ----------------------------------------------

    // file read

    let result = read_file("file.txt");

    match result {
        Ok(contents) => println!("File contents : {}", contents),
        Err(err) => println!("File error : {}", err),
    }

    // ? operator

    let result = read_file2("file.txt");
    
    match result {
        Ok(contents) => println!("File contents : {}", contents),
        Err(err) => println!("File error : {}", err),
    }



}
