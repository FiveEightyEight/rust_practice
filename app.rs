fn main() {
    println!("Hello, World!");

    println!("In rust you do not need to define the type of a variable, but you can if you want to.");
    let x: f64 = 1.1;
    let y = 2.2;
    println!("possible types are: f32, f64, i32, i64, u32, u64, bool, char, str, String");
    
    let answer = multiply_both(x, y);
    println!("{} * {} = {}", x, y, answer);
}

fn multiply_both(x: f64, y: f64) -> f64 {
    return x * y;
}
