use crate::classes::inventaire::{Objet, ObjetQuantifie};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Hero {
    pub name: String,
    pub vie: i32,
    pub force: i32,
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
    pub name: String,
    pub id: u8,
    pub force: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PNJs {
    pub merchants: Vec<Marchand>,
    pub residents: Vec<Resident>,
    pub guards: Vec<Garde>,
    pub boss: Boss,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Boss {
    pub name: String,
    pub alias: String,
    pub force: i32,
}