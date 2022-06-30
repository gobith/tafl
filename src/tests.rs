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
    fn tfl_test_brandubh_old() {
        use super::super::tafl_old::Tafl;
        let tfl = Tafl::brandubh();
        println!("{}", tfl);
        println!("{:?}", tfl)
    }

    #[test]
    fn tfl_test_brandubh_loop_old() {
        use super::super::tafl_old::Tafl;
        let mut count = 0u32;
        loop {
            count += 1;
            // Tafl::brandubh();
            Tafl::new(7);
            if count == 1000000 {
                println!("done {}", count);
                break;
            }
        }
    }

    #[test]
    fn tfl_test_hnefatafl() {
        use super::super::hnefatafl::Hnefatafl;
        let tfl = Hnefatafl::new();
        println!("{}", tfl);
        println!("{:?}", tfl)
    }

    #[test]
    fn tfl_test_brandubh() {
        use super::super::brandubh::Brandubh;
        let tfl = Brandubh::new();
        println!("{}", tfl);
        println!("{:?}", tfl)
    }

    #[test]
    fn tfl_test_brandubh_loop() {
        use super::super::brandubh::Brandubh;
        let mut count = 0u32;
        loop {
            count += 1;
            Brandubh::new();
            if count == 1000000000 {
                println!("done {}", count);
                break;
            }
        }
    }

    #[test]
    fn tfl_test_hnefatafl_loop() {
        use super::super::hnefatafl::Hnefatafl;
        let mut count = 0u32;
        loop {
            count += 1;
            Hnefatafl::new();
            if count == 1000000000 {
                println!("done {}", count);
                break;
            }
        }
    }

    #[test]
    fn array_equality() {
        let array1 = [
            4, 0, 3, 3, 3, 0, 4, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 3, 2, 1, 2, 3, 3, 3,
            0, 0, 2, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 4, 0, 3, 3, 3, 0, 4,
        ];
        let array2 = [
            4, 0, 3, 3, 3, 0, 4, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 2, 0, 0, 3, 3, 3, 2, 1, 2, 3, 3, 3,
            0, 0, 2, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 4, 0, 3, 3, 3, 0, 4,
        ];
        assert_eq!(array1, array2);
    }

    #[test]
    fn tfl_test_tafl_brandubh() {
        let tfl = crate::tafl::brandubh();
        println!("{}", tfl);
        println!("{:?}", tfl)
    }

    #[test]
    fn tfl_test_tafl_hnefatafl() {
        let tfl = crate::tafl::hnefatafl();
        let tfl_clone = tfl.clone();
        println!("{}", tfl_clone);
        println!("{:?}", tfl_clone)
    }


    #[test]
    fn tfl_test_move_piece() {
        let tfl = crate::tafl::brandubh();
        let tfl_new = tfl.move_piece(10 , 7);
        let tfl_new1 = tfl_new.move_piece(17 , 10);
        println!("{}", tfl_new1);
        
    }

}
