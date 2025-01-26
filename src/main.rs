// use rand::{Rng, thread_rng};
// use std::thread;
// use std::time::Duration;
// use std::io;
use ymcrust::{input, input_numb, random, pause,};



#[derive(Debug)]
enum Level {
    Debutant,
    Intemediare,
    Expert,
}

struct Player {
    nom : String,
    id : i32,
    score : Vec<i32>,
    xp : i32,
    niveau: Level,
}

fn main() {


    println!("Hallo World");
    println!("Veuillez entre le  numbre  d'iteratation ");

    let  numb_iteration = input_numb("Entrez le nombre d'iteration");
    println!("Nom du Joueur ?  ");
    println!("On vas teste le changement de nom de repertoire");
    println!("Rename Recu ");


    let mut  joueur = Player{
        nom : input("Entrez Nom du Joueur"),
        id : random(1,10),
        score: Vec::new(),
        xp : 0,
        niveau : Level::Debutant
    };


    println!("Player 1 : {} ", joueur.nom);
    println!("id:{}",joueur.id);
    println!("Score : {:?}",joueur.score);

    for i in 1..numb_iteration + 1{
        let dee = random(1,6);
        println!("#{} Nombre genere : {}",i, dee);
        joueur.score.push(dee) ;
    }

    println!("Score : {:?}",joueur.score);
    joueur.xp = joueur.score.iter().sum();
    println!("XP : {}",joueur.xp);



    if joueur.xp > 100 {
        joueur.niveau = Level::Intemediare;

    } else if joueur.xp > 200 {
        joueur.niveau = Level::Expert;

    } else {
        joueur.niveau = Level::Debutant;

    }
    pause(2);

    println!("Niveau : {:?}",joueur.niveau);



////////****////////////////////


   //  println!("Player 2");
   //  let player_2 = input();
   //  let mut score_1 :i32 = 0;
   //  let mut score_2 :i32 = 0;
   //  println!("Nombre de game ");
   //  let game:i32 = input_numb();
   //  let mut runde= 0;
   //
   //  println!("PLayer 1: {}",player_1);
   //  println!("Player 2: {}",player_2);
   //  println!("Number of Runde : {}",runde);
   //  println!("Bienvenue a {} et {}",player_1,player_2);
   //  thread::sleep(Duration::from_secs(2));
   //  println!("On Commence ......");
   //  thread::sleep(Duration::from_secs(5));
   //
   //
   //  while runde < game {
   //      runde = runde+1;
   //      println!("Rund :{}",runde);
   //      let  dice_1 = random();
   //      let dice_2 = random();
   //      thread::sleep(Duration::from_secs(2));
   //      println!("{} : Dice:{} ",player_1 , dice_1 );
   //      // let dice_1 = random();
   //      thread::sleep(Duration::from_secs(1));
   //      println!("{} : Dice:{} ",player_2,dice_2);
   //      if dice_1 > dice_2 {
   //          thread::sleep(Duration::from_secs(1));
   //          println!("{} Won this {} rund",player_1,runde);
   //          score_1 = score_1+1;
   //          thread::sleep(Duration::from_secs(2));
   //
   //      } else {
   //          thread::sleep(Duration::from_secs(1));
   //          println!("{} Won This {}",player_2,runde);
   //          score_2 = score_2 + 1 ;
   //          thread::sleep(Duration::from_secs(2));
   //
   //
   //      }
   //
   //  }
   //
   //  if score_1 > score_2 {
   //      thread::sleep(Duration::from_secs(2));
   //      println!("{} Won the Match",player_1);
   //
   //  } else {
   //      thread::sleep(Duration::from_secs(2));
   //      println!("{} Won the match",player_2);
   //  }
   //
   //
   //  println!("#######FIN DU JEUX#########");
   //

}

//
//
//
//
// fn input() -> String {
//     // println!("enregistrer votre nom");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("impossible");
//     return input;
// }
//
// fn input_numb() -> i32{
//     let mut input_numb = String::new();
//     io::stdin().read_line(&mut input_numb).expect("impossible");
//     let input : i32 = input_numb.trim().parse().expect("Please type a number!");
//     return input
//
// }
//
// fn random(factor:i32) -> i32 {
//     if factor == 1 {
//         let mut gen = thread_rng();
//         let random = gen.gen_range(0..10);
//         return random;
//     }else if factor == 2 {
//         let mut gen = thread_rng();
//         let random = gen.gen_range(0..100);
//         return random;
//     }
//     else {
//         let mut gen = thread_rng();
//         let random = gen.gen_range(0..1000);
//         return random;
//     }
// }
//
// fn pause() {
//     thread::sleep(Duration::from_secs(1));

// }
//
//
//
// fn rando(a:i32,b:i32) ->i32{
//     let mut gen = thread_rng();
//     if a == true{
//         return 0;
//     }
//     let random = gen.gen_range(a..b);
//     return random;
// }