use std::io;

fn main() {
    println!("Please input your Fahrenheit temp:");

    let mut temp = String::new();

    io::stdin()
        .read_line(&mut temp)
        .expect("failed to read line");

    let temp: f32 = temp.trim().parse().unwrap_or(0.0);
    let cel_temp = (temp - 32.0) * 5.0 / 9.0;
    println!("{temp}ºF = {cel_temp}ºC");
}
