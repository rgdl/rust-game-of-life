use rand::Rng;

pub struct Game { columns: i32, rows: i32, is_new: bool, state: Vec<Vec<bool>>}

fn get_random_state(columns: i32, rows: i32, initial_proportion: f32) -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    let mut result = vec![];
    for _row in 0..rows {
        let mut row_result = vec![];
        for _column in 0..columns {
            row_result.push(rng.gen_range(0.0, 1.0) < initial_proportion);
        }
        result.push(row_result);
    }
    result
}

fn will_cell_survive(n_neighbours: i32, currently_alive: bool) -> bool {
    match n_neighbours {
        0..=1 => false,
        2 => currently_alive,
        3 => true,
        _ => false,
    }
}

impl Game {
    pub fn new(columns: i32, rows: i32, initial_proportion: f32) -> Game {
        Game { columns: columns, rows: rows, is_new: true, state: get_random_state(columns, rows, initial_proportion) }
    }
 
    pub fn get_next_state(&mut self) -> Vec<Vec<bool>> {
        if !self.is_new {
            self.iterate();
        } else {
            self.is_new = false;
        }
        self.state.to_vec()
    }

    fn iterate(&mut self) {
        let mut result = vec![];
        for row in 0..self.rows {
            let mut row_result = vec![];
            for column in 0..self.columns {
                let n_neighbours = self.get_n_neighbours(row, column);
                row_result.push(will_cell_survive(n_neighbours, self.state[row as usize][column as usize]));
            }
            result.push(row_result);
        }
        self.state = result; 
    }

    fn get_n_neighbours(&self, row: i32, column: i32) -> i32 {
        let mut n_neighbours = 0;
        for row_shift in [-1, 0, 1].iter() {
            for column_shift in [-1, 0, 1].iter() {
                if row + row_shift < 0 || row + row_shift >= self.rows { continue }
                if column + column_shift < 0 || column + column_shift >= self.columns { continue }
                if *row_shift == 0 && *column_shift == 0 { continue }
                if self.state[(row + row_shift) as usize][(column + column_shift) as usize] {
                    n_neighbours += 1;
                }
            }
        }
        n_neighbours
    }

}

