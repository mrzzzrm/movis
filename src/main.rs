mod lib;

fn main() {
    let name = "HalloWeltbeffepdejaijsnd".to_string();
    let mut level = lib::MyLevel::new(&*name);
    println!("{}", level.name);
}
