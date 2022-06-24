mod position;
mod tafl;

use tafl::Tafl;

fn main() {
    let board = Tafl::new(7);
    println!("{}", board);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn tfl_test() {
        let tfl = "7+24+11-17-22-23-25-36-31-38+2-3-4-10-14-20-21-22-26-27-28-34-38-44-45-46";
        let mut v = tfl.split('+');
        let number: u8 = v.next().unwrap().parse().unwrap();
        println!("{:?}", number)
    }
}
