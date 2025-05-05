use serde::Deserialize;

#[derive(Deserialize)]
pub struct Lore {
    pub debut: String,
    pub bleu: String,
    pub vert: String,
    pub jaune: String,
    pub violet: String,
    pub rouge: String,
    pub fin: String,
    pub fin_good: String,
    pub fin_bad: String,
    pub end: String,
}