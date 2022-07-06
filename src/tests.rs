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
        println!("{}", tfl);
        println!("{:?}", tfl)
    }


    #[test]
    fn tfl_test_move_piece() {
        let mut tfl = crate::tafl::brandubh();
        println!("{}", tfl);
        tfl.move_piece(10 , 7);
        println!("{}", tfl);
        tfl.move_piece(17 , 10);
        println!("{}", tfl);
        tfl.move_piece(14 , 17);
        println!("{}", tfl);
        println!("{:?}", tfl);
        
    }
    
    #[test]
    fn tfl_test_move_piece_capture_left() {
        let mut tfl = crate::tafl::brandubh();
        println!("{}", tfl);
        tfl.move_piece(14 , 16);
        println!("{}", tfl);
        tfl.move_piece(31 , 32);
        println!("{}", tfl);
        tfl.move_piece(20 , 18);
        println!("{}", tfl);
        println!("{:?}", tfl);
        
    }

    #[test]
    fn tfl_test_move_piece_capture_right() {
        let mut tfl = crate::tafl::brandubh();
        println!("{}", tfl);
        tfl.move_piece(34 , 32);
        println!("{}", tfl);
        tfl.move_piece(17 , 18);
        println!("{}", tfl);
        tfl.move_piece(28 , 30);
        println!("{}", tfl);
        println!("{:?}", tfl);
        
    }

    #[test]
    fn tfl_test_move_wrong_piece() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(24 , 23);
       
        
    }

    #[test]
    fn tfl_test_move_piece_up() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(46, 39);
       
        
    }





}


mod test_move {

    #[test]
    fn tfl_test_move_piece_up_error() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(46, 11);
    }

    #[test]
    fn tfl_test_move_piece_down_error() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(4, 39);  
    }

    #[test]
    fn tfl_test_move_piece_left_error() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(45, 43);
    }

    #[test]
    fn tfl_test_move_piece_right_error() {
        let mut tfl = crate::tafl::brandubh();
        tfl.move_piece(45, 47); 
    }

}