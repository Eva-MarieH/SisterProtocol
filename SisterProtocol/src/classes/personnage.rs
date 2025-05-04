use crate::classes::inventaire::{Objet, ObjetQuantifie};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Hero {
    pub name: String,
    pub vie: i32,
    pub force: i32,
    pub intelligence: i32,
    pub argent: i32,
    pub position: String,
    pub inventory: Vec<ObjetQuantifie>,
    pub amelioration: Option<Objet>
}

impl Hero {
    pub fn perdre_vie(&mut self, degats: i32) {
        self.vie -= degats;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Garde {
    pub id: u8,
    pub nom: String,
    pub vie: i32,
    pub force: i32,
    pub loot: Vec<Objet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PNJs {
    pub merchants: Vec<Marchand>,
    pub residents: Vec<Resident>,
}

#[derive(Clone,Debug, Deserialize, Serialize )]
pub struct Marchand {
    pub name: String,
    pub id: u8,
    pub money: i32,
    pub inventory: Vec<ObjetQuantifie>,
    pub dialogues: Vec<Dialogue>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resident {
    pub name: String,
    pub id: u8,
    pub dialogues: Vec<Dialogue>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Dialogue {
    pub context: String,
    pub text: String,
}

pub struct Boss {
    pub nom: String,
    pub vie: i32,
    pub force: i32,
}