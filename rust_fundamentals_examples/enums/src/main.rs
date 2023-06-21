enum Work {
    Civilian,
    Soldier,
}
fn main() {
    use crate::Work::{Civilian, Soldier};
    // Equivalent to `Work::Civilian`.
    let work = Civilian;
    match work {
        // Note the lack of scoping because of the explicit `use` above
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}