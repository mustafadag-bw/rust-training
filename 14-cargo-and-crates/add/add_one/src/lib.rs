use rand::Rng;

pub fn add_random(x: i32) -> i32 {
    x + rand::thread_rng().gen_range(1..=100)
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add_random(2);
        assert!(result < 103 && result > 1);
    }

    #[test]
    fn add_one_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
