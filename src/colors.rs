const HEX_VALUES: [char; 22] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'a', 'b', 'c', 'd', 'e', 'f'];

#[derive(Debug)]
pub struct ColorBinary{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorBinary {
    pub fn new() -> Self {
        ColorBinary {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn insert_values(&mut self, str: &String) {
        let mut c_iter: u8 = 0;

        for num in str.split_whitespace() {
            let number: u8 = match num.parse() {
                Ok(num) => num,
                Err(error) => {
                    panic!("String is not a u8: {}", error);
                }
            };
            if c_iter == 0 {
                self.red = number;
                c_iter += 1;
            } else if c_iter == 1 {
                self.green = number;
                c_iter += 1;
            } else {
                self.blue = number;
                break;
            }
        }
    }
}

#[derive(Debug)]
pub struct ColorHex {
    pub string: String,
}

impl ColorHex {
    pub fn new() -> Self {
        ColorHex {
            string: String::from("#"),
        }
    }

    pub fn insert_values(&mut self, str: &String) {
        if str.trim().len() != 6 {
            panic!("Invalid hex value");
        }
        for char in str.trim().chars() {
            if !HEX_VALUES.contains(&char) {
                panic!("'{char}' is not a Hex value");
            }
            self.string.push(char);
        }
    }
}

#[derive(Debug)]
pub struct ColorHSL {
    pub hue: u16,
    pub sat: u16,
    pub light: u16,
}

impl ColorHSL {
    pub fn new() -> Self {
        ColorHSL {
            hue: 0,
            sat: 0,
            light: 0,
        }
    }

    pub fn insert_values(&mut self, str: &String) {
        let mut c_iter: u8 = 0;

        for num in str.split_whitespace() {
            let number: u16 = match num.parse() {
                Ok(num) => num,
                Err(error) => {
                    panic!("String is not a u16: {}", error);
                }
            };
            if c_iter == 0 {
                if number > 360 {
                    panic!("Number for hue is not within range of 0-360");
                }
                self.hue = number;
                c_iter += 1;
            } else if c_iter == 1 {
                if number > 100 {
                    panic!("Number for saturation is not within range of 0-100");
                }
                self.sat = number;
                c_iter += 1;
            } else {
                if number > 100 {
                    panic!("Number for lightness is not within range of 0-100");
                }
                self.light = number;
                break;
            }
        }
    }
}

pub enum ColorRep {
    Binary(ColorBinary),
    Hex(ColorHex),
    HSL(ColorHSL),
}

impl ColorRep {
    pub fn new_binary() -> Self {
        ColorRep::Binary(ColorBinary::new())
    }
    pub fn new_hex() -> Self {
        ColorRep::Hex(ColorHex::new())
    }
    pub fn new_hsl() -> Self {
        ColorRep::HSL(ColorHSL::new())
    }
}