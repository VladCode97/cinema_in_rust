use crate::controllers::cinema_controller::{fill_matrix, print_matrix_cinema, change_state_of_chair};
use crate::models::cinema_model::ChairCinema;

mod models;
mod controllers;

fn main() {
    let mut matrix_cinema: Vec<Vec<ChairCinema>> = Vec::new();
    fill_matrix(&mut matrix_cinema, 3, 3);
    print_matrix_cinema(&matrix_cinema);
    change_state_of_chair(&mut matrix_cinema, "a1".to_string());
    change_state_of_chair(&mut matrix_cinema, "c1".to_string());
    print_matrix_cinema(&matrix_cinema);
}
