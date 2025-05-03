use crate::classes::personnage::Hero;
use crate::io;
use std::io::Write;

pub fn deplacement(hero: &mut Hero){
    println!("🏠 Quel quartier veux-tu aller ?");
                println!("1. Quartier bleu");
                println!("2. Quartier rouge");
                println!("3. Quartier vert");
                println!("4. Quartier jaune");
                print!("> Choisis le quartier : ");
                io::stdout().flush().unwrap();
                let mut choix1 = String::new();
                io::stdin().read_line(&mut choix1).unwrap();
        
                
                match choix1.trim() {
                    "1" => hero.position = String::from("bleu"),
                    "2" => hero.position = String::from("rouge"),
                    "3" => hero.position = String::from("vert"),
                    "4" => hero.position = String::from("jaune"),
                    _ => println!("⛔ Quartier inconnu."),
                    
                }
                println!("🚶 Tu te déplaces vers le quartier {}...", hero.position);
    println!("🚶 Tu marches vers un autre quartier");
}