#[derive(Debug)]
pub enum EnnemiHackable {
    Ordinateur { id: u8, name: String },
    Serveur { id: u8, name: String },
}
