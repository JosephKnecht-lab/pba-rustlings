// event_enums
// Make me compile! Execute `rustlings hint event_enums` for hints!


#[derive(Debug)]
enum Event {
    Instantiated(String),
    Transferred { 
        to: u32,
        from: u32,
        balance: u128 
    },
    Terminated
}

impl Event {
    // add a function that prints the events to the console
    fn call() {
        println!();
    }

}

fn main() {
    let events = [        
        Event::Instantiated(String::from("ContractName")),
        Event::Transferred { to: 10, from: 30, balance: u128::MAX },
        Event::Terminated,
    ];

    for event in &events {
        Event::call();
    }
}