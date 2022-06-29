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
    fn tfl_test_brandubh() {
        use super::super::tafl::Tafl;
        let tfl = Tafl::brandubh();
        println!("{}", tfl);
        println!("{:?}", tfl)
    }

    fn tfl_test_brandubh_loop() {
        use super::super::tafl::Tafl;
        let mut count = 0u32;
        loop {
            count += 1;
            Tafl::brandubh();
            if count == 3 {
                break;
            }
        }
    }
}
