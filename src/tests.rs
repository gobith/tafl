#[cfg(test)]

mod tests {
    #[test]
    fn tfl_test() {
        let tfl = "7+24+11-17-22-23-25-36-31-38+2-3-4-10-14-20-21-22-26-27-28-34-38-44-45-46";
        let mut v = tfl.split('+');
        let number: u8 = v.next().unwrap().parse().unwrap();
        println!("{:?}", number)
    }
    #[test]
    fn tfl_test_1() {
        use super::super::tafl::Tafl;
        let tfl = Tafl::from(
            "7+24+17-23-25-31+3-10-21-22-26-27-38-45",
        );
        println!("{}" , tfl);
        println!("{:?}" , tfl)
    }
}
