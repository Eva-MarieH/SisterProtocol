use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Machines {
    pub computers: Vec<Ordinateur>,
    pub servers: Vec<Serveur>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EnnemiHackable {
    Ordinateur(Ordinateur),
    Serveur(Serveur),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ordinateur {
    pub id: u8,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Serveur {
    pub id: u8,
    pub name: String,
}
