use std::io;

fn main() {
    let mut f = String::new();
    loop {
        println!("Introduce temp in Fahrenheit");
        io::stdin().read_line(&mut f)
            .expect("Failed to read line");
        let f: f32 = match f.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => continue,
        };

        let c = (f - 32_f32)/1.8;
        println!("{} F degrees are {} C degrees", f, c);
        break;
    }
}

