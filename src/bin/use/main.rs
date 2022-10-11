
enum Status{
    Rich,
    Poor,
}

enum Work {
    Civilian, 
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("Rich"),
        Poor => println!("Poor"),
    }

    match work {
        Civilian => println!("Civilian"),
        Soldier => println!("Soldier"),
    }
}