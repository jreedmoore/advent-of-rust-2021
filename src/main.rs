mod days;
use days::*;

fn main() {
    println!("{}", day11::part_one().unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_foo() {
        assert!(true)
    }
}