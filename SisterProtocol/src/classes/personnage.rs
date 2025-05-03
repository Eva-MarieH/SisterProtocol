use crate::classes::inventaire::{Objet, Inventaire};
<<<<<<< HEAD
use serde::Deserialize;
=======
use serde::{Deserialize, Serialize};
>>>>>>> be5e855d3d34068140aafc649ef16f260a41c4ec

#[derive(Debug, Deserialize)]
pub struct Hero {
    pub nom: String,
    pub vie: i32,
    pub force: i32,
    pub intelligence: i32,
    pub argent: i32,
    pub position: String,
    pub inventaire: Inventaire,
    pub amelioration: Option<Objet>
}

impl Hero {
    pub fn perdre_vie(&mut self, degats: i32) {
        self.vie -= degats;
    }
}

pub struct Garde {
    pub id: u8,
    pub nom: String,
    pub vie: i32,
    pub force: i32,
    pub loot: Vec<Objet>,
}

#[derive(Debug, Deserialize)]
pub struct Marchand {
    pub id: u8,
    pub name: String,
    pub inventory: Inventaire,
    pub money: i32,
    pub dialogues: Vec<Dialogue>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Resident {
    pub name: String,
    pub id: u8,
    pub dialogues: Vec<Dialogue>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Dialogue {
    pub context: String,
    pub text: String,
}

pub struct Boss {
    pub nom: String,
    pub vie: i32,
    pub force: i32,
}