mod raindrop1;

use std::env;

fn main() {
    let name = "Cristian";
    println!("Hello, {}!", name);

    let x = raindrop1::raindrop(30);
    println!("{}", x);

    let args = env::args().collect::<Vec<String>>();
    println!("{:?}", args);

    if let Ok(value) = &args[1].parse::<i32>() {
        match value {
            42 => println!("Vida, universo e tudo mais"),
            22 => println!("AA"),
            _ => println!("Failed"),
        }
    } else {
        println!("Not a valid value")
    };



    

}
    
