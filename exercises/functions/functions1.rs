// functions1.rs
// Update get_body so that it works when its called in main()
// and returns a Vec<u8>.
// And make the rest compile!

struct Block {
    header: u32,
    details: Vec<u8>,
}

impl Block { 
    fn get_body() -> Vec<u8> {
        let mut vec = Vec::new();
        return vec;
    }
}

fn is_academy_word(attempt: &str) -> bool {
    attempt == "cryptography" || attempt == "economics" || attempt == "genesis block"
}

fn main() {

    let block = Block {
        header: 12345,
        details: Block::get_body()
    };

    let word = Block::get_body(); // TODO
    if is_academy_word("no") {
        println!("That's definitely a Polkadot Academy word!");
    } else {
        println!("That's not a Polkadot Academy word.");
    }

    println!("{}", block.header );
    println!("{:?}", block.details );
}