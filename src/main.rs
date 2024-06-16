use rand::{Rng, thread_rng};
use std::thread;
use std::time::Duration;
use std::io;




fn main() {
    println!("Quel est votre nom ?");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom).expect("Failed to read line");
    let mut age =  String::new();
    println!("Quel est votre age ?");
    io::stdin().read_line(&mut age).expect("Failed to read line");
    let age: i32 = age.trim().parse().expect("Please type a number!");
    println!("Quelle est votre taille ?");
    let mut taille = String::new();
    io::stdin().read_line(&mut taille).expect("Failed to read line");
    let taille: i32 = taille.trim().parse().expect("Please type a number!");



    println!("Hello, world!");
    println!("Nom: {}", nom);
    println!("Age: {}", age);
    println!("Taille: {}", taille);
    println!();
    //println!("Random Number: {}",random())


    for i in 1..3 {
        println!("{} random numb ; {}",i,random());
        thread::sleep(Duration::from_secs(1));

    }



}

fn random() -> i32 {
    let mut gen = thread_rng();
    let random_numb = gen.gen_range(0..1000);
    return random_numb ;

}