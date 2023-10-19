use crate::models::cinema_model::ChairCinema;
use crate::models::cinema_model::StateChair::{BUSY, EMPTY};

fn convert_number_in_chair(number: u8, position: usize) -> String {
    let change = number as char;
    let mut chair = change.to_string();
    chair.push_str(&position.to_string());
    chair
}

pub fn fill_matrix(matrix_cinema: &mut Vec<Vec<ChairCinema>>, rows: usize, columns: usize) {
    let mut letter_number: u8 = 96;
    for _ in 0..rows {
        letter_number += 1;
        let mut row_data = Vec::new();
        for column in 0..columns {
            row_data.push(ChairCinema {
                chair: convert_number_in_chair(letter_number, column),
                state: EMPTY("Empty".to_string()),
            });
        }
        matrix_cinema.push(row_data);
    }
}

pub fn change_state_of_chair(matrix_cinema: &mut Vec<Vec<ChairCinema>>, name_chair: String, rows: usize, columns: usize) {
    'a: for row in 0..rows {
        for column in 0..columns {
            if matrix_cinema[row][column].chair == name_chair && matrix_cinema[row][column].state == EMPTY("Empty".to_string()) {
                matrix_cinema[row][column].state = BUSY("BUSY".to_string());
                break 'a;
            }
        }
    }
}

pub fn print_matrix_cinema(matrix_cinema: &Vec<Vec<ChairCinema>>) {
    dbg!(&matrix_cinema);
}