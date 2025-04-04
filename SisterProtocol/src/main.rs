mod classes;
mod utils;

fn main() {
    use classes::personnage::Hero;
    use utils::afficher_message;

    let mut hero = Hero {
        vie: 100,
        force: 10,
        intelligence: 15,
        argent: 50,
        position: String::from("Quartier bleu"),
        inventaire: classes::inventaire::Inventaire { objets: vec![] },
        amelioration: None,
    };

    afficher_message("Bienvenue dans SisterProtocol.");
}
