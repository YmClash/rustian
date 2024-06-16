use rand::{Rng, thread_rng};
use std::thread;
use std::time::Duration;



fn main() {

    let nom = "Momo";
    let age = 25;
    let taille = 1.75;
    println!("Hello, world!");
    println!("Je m'appelle {} et j'ai {} ans et je mesure {}m", nom, age, taille);
    //println!("Random Number: {}",random())
    for i in 1..10 {
        println!("{} random numb ; {}",i,random());
        thread::sleep(Duration::from_secs(1));

    }



}

fn random() -> i32 {
    let mut gen = thread_rng();
    let random_numb = gen.gen_range(0..1000);
    return random_numb ;

}