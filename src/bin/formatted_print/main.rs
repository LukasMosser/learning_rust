fn main(){
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}", 
            object="the lazy dog", 
            subject="the quick brown fox", 
            verb="jumps over");

    println!("Base 10 repr: {}", 69420);
    println!("Base 2 repr: {:b}", 69420);
    println!("Base 8 repr: {:o}", 69420);
    println!("Base 16 repr: {:x}", 69420);
    println!("Base 16 repr: {:X}", 69420);
    
    println!("{number:>10}", number=1);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    //println!("This struct wont print {}", Structure(3));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    let decimals: usize = 5;
    println!("Pi is roughly {0:.decimals$}", pi)
}