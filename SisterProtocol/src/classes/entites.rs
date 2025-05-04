#[derive(Debug)]
pub enum EnnemiHackable {
    Ordinateur { id: u8, name: String },
    Serveur { id: u8, name: String },
}

impl EnnemiHackable {
    pub fn nom_type(&self) -> &'static str {
        match self {
            EnnemiHackable::Ordinateur { .. } => "ordinateur",
            EnnemiHackable::Serveur { .. } => "serveur",
        }
    }

    pub fn id(&self) -> u8 {
        match self {
            EnnemiHackable::Ordinateur { id, .. } => *id,
            EnnemiHackable::Serveur { id, .. } => *id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            EnnemiHackable::Ordinateur { name, .. } => name,
            EnnemiHackable::Serveur { name, .. } => name,
        }
    }
}


pub struct Serveur {
    pub id: u8,
    pub name: String,
}

pub struct Ordinateur {
    pub id: u8,
    pub name: String,
}
