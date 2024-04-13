use color_request::colors::ColorRep;
use hsluv::{hex_to_rgb, hsluv_to_rgb, rgb_to_hex, rgb_to_hsluv};
use image::{Rgb, RgbImage};
use std::io;

fn main() {
    'main_loop: loop {
        let color_representation;
        println!("Welcome to Color Requester");

        println!("Select which color value representation to use:\n");

        println!("1: Binary\n2: Hex\n3: HSL");

        let mut choice_1 = String::new();

        'color_rep: loop {
            io::stdin().read_line(&mut choice_1).unwrap();

            let _cfinal: u8 = match choice_1.trim().parse() {
                Ok(num) => match num {
                    1 => {
                        color_representation = ColorRep::new_binary();
                        break 'color_rep;
                    }
                    2 => {
                        color_representation = ColorRep::new_hex();
                        break 'color_rep;
                    }
                    3 => {
                        color_representation = ColorRep::new_hsl();
                        break 'color_rep;
                    }
                    _ => {
                        println!("Please enter either 1, 2, or 3");
                        continue;
                    }
                },
                Err(_error) => {
                    println!("Please enter a number");
                    choice_1 = String::from("");
                    continue;
                }
            };
        }

        match color_representation {
            ColorRep::Binary(mut binary) => {
                println!("-- Binary Mode --");
                println!("Please insert values seperated with a space:");

                let mut str = String::new();

                io::stdin().read_line(&mut str).unwrap();

                binary.insert_values(&str);

                let rgb = (binary.red as f64, binary.green as f64, binary.blue as f64);

                generate_image(&rgb);
            }
            ColorRep::HSL(mut hsl) => {
                println!("-- HSL Mode --");
                println!("Please insert values seperated with a space:");

                let mut str = String::new();

                io::stdin().read_line(&mut str).unwrap();

                hsl.insert_values(&str);

                let hsl_rgb = hsluv_to_rgb((hsl.hue as f64, hsl.sat as f64, hsl.light as f64));

                let (mut red, mut green, mut blue) = hsl_rgb;

                red *= 255.0;
                green *= 255.0;
                blue *= 255.0;

                let new_hsl_vals = (red, green, blue);

                generate_image(&new_hsl_vals);
            }
            ColorRep::Hex(mut hex) => {
                println!("-- Hex Mode --");
                println!("Please insert values seperated with a space:");

                let mut str = String::new();

                io::stdin().read_line(&mut str).unwrap();

                hex.insert_values(&str);

                let hex_rgb = hex_to_rgb(&hex.string);

                let (mut red, mut green, mut blue) = hex_rgb;

                red *= 255.0;
                green *= 255.0;
                blue *= 255.0;

                let new_hex_vals = (red, green, blue);

                generate_image(&new_hex_vals);
            }
        }

        println!("Go again?");
        println!("1: Yes\n2: No");

        let mut choice_2 = String::new();

        io::stdin().read_line(&mut choice_2).unwrap();

        let number: u8 = match choice_2.trim().parse() {
            Ok(num) => num,
            Err(_error) => continue,
        };

        if number == 1 {
            continue;
        } else {
            break 'main_loop;
        }
    }
}

fn generate_image(rgb_val: &(f64, f64, f64)) {
    let mut img = RgbImage::new(128, 128);

    let (red, green, blue) = rgb_val;

    for x in 1..128 {
        for y in 1..128 {
            img.put_pixel(x, y, Rgb([*red as u8, *green as u8, *blue as u8]));
        }
    }

    println!("\nPlease insert the full path of the directory for this image to be saved to:\n");

    let mut path = String::new();

    io::stdin().read_line(&mut path).unwrap();

    img.save(format!(
        "{}/{}{}{}.png",
        path.trim(), *red as u8, *green as u8, *blue as u8
    ))
    .unwrap();

    println!("Image generated!\n");

    println!("Here are the values of the image:\n");

    let rgb = (*red as u8, *green as u8, *blue as u8);

    let hex_colors = (red / 255.0, green / 255.0, blue / 255.0);
    let hex = rgb_to_hex((
        hex_colors.0 as f64,
        hex_colors.1 as f64,
        hex_colors.2 as f64,
    ));

    let hsl_colors = (red / 255.0, green / 255.0, blue / 255.0);
    let hsl = rgb_to_hsluv((
        hsl_colors.0 as f64,
        hsl_colors.1 as f64,
        hsl_colors.2 as f64,
    ));

    println!("RGB: {}, {}, {}", rgb.0, rgb.1, rgb.2);
    println!("Hex: {}", hex);
    println!(
        "HSL: {} deg, {}%, {}%\n",
        hsl.0 as u16, hsl.1 as u16, hsl.2 as u16
    );
}
