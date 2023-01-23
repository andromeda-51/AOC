use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_input(filename: &str) -> String {
        let path = Path::new(filename);
        let display = path.display();

        let mut file = match File::open(&path) {
                Err(why) => panic!("Could not open {}: {}", display, why),
                Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_tostring(&mut s) {
                Err(why) => panic!("Could not read {}: {}", display, why),
                Ok() => (),
        };
        s
}

fn main() {
        let input = readinput("input.txt");
        let mut result = 0;
        for (x, i) in input.chars().enumerate() {
                result = match i {
                        '(' => result + 1,
                        ')' => result - 1,
                         => result,
                };
                if result < 0 { 
                        println!("{}", x + 1);
                        break;
                };
        };
}