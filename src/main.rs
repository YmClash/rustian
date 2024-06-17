use rand::{Rng, thread_rng};
use std::thread;
use std::time::Duration;
use std::io;


// struct Player {
//     nom: String,
//     age:i32,
//     taille:i32,
//     score:i32,
// }


fn main() {

    println!("Player 1 ");
    let player_1 = input();
    println!("Player 2");
    let player_2 = input();
    let mut score_1 :i32 = 0;
    let mut score_2 :i32 = 0;
    println!("Nombre de game ");
    let game:i32 = input_numb();
    let mut runde= 0;

    println!("PLayer 1: {}",player_1);
    println!("Player 2: {}",player_2);
    println!("Number of Runde : {}",runde);
    println!("Bienvenue a {} et {}",player_1,player_2);
    thread::sleep(Duration::from_secs(2));
    println!("On Commence ......");
    thread::sleep(Duration::from_secs(5));


    while runde < game {
        runde = runde+1;
        println!("Rund :{}",runde);
        let  dice_1 = random();
        let dice_2 = random();
        thread::sleep(Duration::from_secs(2));
        println!("{} : Dice:{} ",player_1 , dice_1 );
        // let dice_1 = random();
        thread::sleep(Duration::from_secs(1));
        println!("{} : Dice:{} ",player_2,dice_2);
        if dice_1 > dice_2 {
            thread::sleep(Duration::from_secs(1));
            println!("{} Won this {} rund",player_1,runde);
            score_1 = score_1+1;
            thread::sleep(Duration::from_secs(2));

        } else {
            thread::sleep(Duration::from_secs(1));
            println!("{} Won This {}",player_2,runde);
            score_2 = score_2 + 1 ;
            thread::sleep(Duration::from_secs(2));


        }





    }


    if score_1 > score_2 {
        thread::sleep(Duration::from_secs(2));

        println!("{} Won the Match",player_1);

    } else {
        thread::sleep(Duration::from_secs(2));

        println!("{} Won the match",player_2);
    }





    //
    // println!("Combien de nombre aléatoire voulez-vous ?");
    // let mut numb_inter = String::new();
    // io::stdin().read_line(&mut numb_inter).expect("Failed to read line");
    // let numb_inter: i32 = numb_inter.trim().parse().expect("Please type a number!");
    //
    //


    // println!("Hello, world!");
    // println!("Nom: {}", player1.nom);
    // println!("Age: {}", player1.age);
    // println!("Taille: {}", player1.taille);
    // println!("Score:{}",player1.score);

    println!("#######FIN DU JEUX#########");
    //println!("Random Number: {}",random())


    // for i in 1..runde {
    //     println!("{} random numb ; {}",i,random());
    //     thread::sleep(Duration::from_secs(1));
    //
    // }



}

fn input() -> String {
    // println!("enregistrer votre nom");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("impossible");
    return input;
}

fn input_numb() -> i32{
    let mut input_numb = String::new();
    io::stdin().read_line(&mut input_numb).expect("impossible");
    let input : i32 = input_numb.trim().parse().expect("Please type a number!");
    return input

}

fn random() -> i32 {
    let mut gen = thread_rng();
    let random_numb = gen.gen_range(0..6);
    return random_numb ;

}