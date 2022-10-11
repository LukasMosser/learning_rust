use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    lat: f32, 
    lon: f32
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        
        write!(f, "{}: {:.3}x{} {:.3}x{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "RGB ({}, {}, {})", self.red, self.green, self.blue);
        write!(f, "0x")?;

        
        for color in [
            self.red, 
            self.green, 
            self.blue,
        ].iter() {
            write!(f, "{:o>2}", color)?;
        }
        
        write!(f, "")
    }
    
}

fn main(){
 for city in [
    City {name: "Dubling", lat: 53.347778, lon: -6.259722}
 ].iter() {
    println!("{}", *city);
 }

 for color in [
    Color {red: 128, green: 255, blue: 90},
    Color {red: 0, green: 3, blue: 254},
    Color {red: 0, green: 0, blue: 0}
 ].iter() {
    println!("{}", color);
 }

}