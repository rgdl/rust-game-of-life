use rand::Rng;

pub struct Game { columns: i32, rows: i32, state_counter: i32 }

static INITIAL_PROPORTION: f32 = 0.5;

impl Game {
    pub fn new(columns: i32, rows: i32) -> Game {
        Game { columns: columns, rows: rows, state_counter: 0 }
    }

    pub fn get_next_state(&self) -> Vec<Vec<bool>> {
        // Randomly set values if this is the first state, otherwise, use the rules
        let mut rng = rand::thread_rng();
        let mut result = vec![];
        for _row in 0..self.rows {
            let mut row_result = vec![];
            for _column in 0..self.columns {
                row_result.push(rng.gen_range(0.0, 1.0) < INITIAL_PROPORTION);
            }
            result.push(row_result);
        }
        result
    }
}

fn dumb_function_to_trial_unit_testing(a: u32) -> u32 {
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dumb_passing_test() {
        let test_value = 5u32;
        assert_eq!(test_value, dumb_function_to_trial_unit_testing(test_value));
    }

    //#[test]
    //fn dumb_failing_test() {
    //    let test_value = 5u32;
    //    assert_eq!(test_value + 2, dumb_function_to_trial_unit_testing(test_value));
    //}
}
