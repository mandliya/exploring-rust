use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    lon: f32,
}

impl Display for City {
    // 'f' is a buffer, this method must write formatted string into it.
    fn fmt(&self, f : &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        //write! will write the formatted string to buffer f
        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name,
            self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {
     fn fmt(&self, f : &mut Formatter) -> fmt::Result {
         write!(f, "R: 0x{:X}, G: 0x{:X}, B: 0x{:X}",
            self.red, self.green, self.blue)
     }
}

fn main() {
    for city in [
        City { name: "Seattle", lat: 47.608013, lon: -122.335167 }, 
        City { name: "New York", lat: 40.730610, lon: -73.935242 },
        City { name: "New Delhi", lat:28.644800, lon: 77.216721 }
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red:56, green: 255, blue: 90 },
        Color { red:67, green: 12, blue: 255},
        Color { red:123, green: 34, blue: 134},
    ].iter() {
        println!("{}", *color);
    }

    println!("\nWith Debug:\n");

    for color in [
        Color { red:56, green: 255, blue: 90 },
        Color { red:67, green: 12, blue: 255},
        Color { red:123, green: 34, blue: 134},
    ].iter() {
        println!("{:?}", *color);
    }

}