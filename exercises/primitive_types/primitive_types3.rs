// primitive_types3.rs
// Fill in the missing code to complete the morning_evening function.
// It should take a boolean and output a String.
//
// Then, create an array with at least 100 elements in it where the ??? is.
// Finally, write tests for both functions.

    fn morning_evening (n:bool) -> String {

        if n {
            return "Yes".to_string();
        } else {
            return "No".to_string();
        }
    }

    fn do_array() {
        let mut a: [i32; 100] = [0; 100];
        if a.len() >= 100 {
            println!("Wow, that's a big array!");
        } else {
            println!("Meh, I eat arrays like that for breakfast.");
        }
    }

    #[test]
    fn evening_works() {
        // TODO write test here using assert_eq
        //assert_eq!(morning_evening(true));
    }
    
    #[test]
    fn morning_works() {
        // TODO write test here using assert_eq
    }

    #[test]
    fn do_array_works() {
        // TODO ensure that the array prints correctly
    }