#[derive(Debug, PartialEq)]
pub enum StateChair {
    EMPTY(String),
    BUSY(String),
}

#[derive(Debug, PartialEq)]
pub struct ChairCinema {
    pub chair: String,
    pub state: StateChair,
}