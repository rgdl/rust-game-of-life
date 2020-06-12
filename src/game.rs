pub struct Game { columns: i32, rows: i32 }

impl Game {
    pub fn new(columns: i32, rows: i32) -> Game {
        Game { columns: columns, rows: rows }
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
