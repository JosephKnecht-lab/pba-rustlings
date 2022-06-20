// macros4.rs
// Make me compile!


#[derive(PartialEq, Debug)]
enum T {
    Ta,
    Tb,
}

fn main() {
    assert_eq!(T::Ta, T::Ta);
}