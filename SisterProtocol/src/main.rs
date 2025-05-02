mod classes;
mod utils;
use crate::utils::affichage::Affichage;

fn main() {
    use classes::personnage::Hero;

    let mut hero = Hero {
        nom: String::from("John Doe"),
        vie: 100,
        force: 10,
        intelligence: 15,
        argent: 50,
        position: String::from("Quartier bleu"),
        inventaire: classes::inventaire::Inventaire { objets: vec![] },
        amelioration: None,
    };

    Affichage::afficher_message("Bienvenue dans SisterProtocol.");
    Affichage::afficher_message("Voici tes statistiques:");
    Affichage::afficher_hero(&hero);

    // boucle des choix possibles
}
