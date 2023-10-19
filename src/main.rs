use crate::controllers::cinema_controller::{fill_matrix, print_matrix_cinema, change_state_of_chair};
use crate::models::cinema_model::ChairCinema;

mod models;
mod controllers;

fn main() {
    let mut matrix_cinema: Vec<Vec<ChairCinema>> = Vec::new();
    let rows: usize = 3;
    let columns: usize = 5;
    fill_matrix(&mut matrix_cinema, *&rows, *&columns);
    change_state_of_chair(&mut matrix_cinema, "a1".to_string(), *&rows, *&columns);
    change_state_of_chair(&mut matrix_cinema, "b3".to_string(), *&rows, *&columns);
    change_state_of_chair(&mut matrix_cinema, "c4".to_string(), *&rows, *&columns);
    print_matrix_cinema(&matrix_cinema);
}
