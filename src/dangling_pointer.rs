#[derive(Debug)] // allow the `println!` macro to print the Cereal enum
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

pub fn dangling_pointer() {
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye);
    drop(grains);

    // ERROR
    // will NOT compile
    // println!("{:?}", grains);
}
