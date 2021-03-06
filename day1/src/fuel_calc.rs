pub fn fuel_calculator(mass: i32) -> i32 {
    let value = (mass as f32) / 3.0;
    let fuel = (value as i32) - 2;
    if fuel <= 0 {
        return 0;
    }
    fuel + fuel_calculator(fuel)
}

#[cfg(test)]
mod fuel_calc_tests {
    use super::*;

    #[test]
    fn it_returns_the_correct_values() {
        let test1 = fuel_calculator(12);
        assert_eq!(test1, 2);
        let test2 = fuel_calculator(14);
        assert_eq!(test2, 2);
        let test3 = fuel_calculator(1969);
        assert_eq!(test3, 966);
        let test4 = fuel_calculator(100756);
        assert_eq!(test4, 50346);
    }
}
