fn main() {
    println!("Hello, world!");

    another_function(5, 'm');
    let x:i32 = five();
    println!("the value of x is: {x}");
}

fn another_function(value: i32, unit_lable: char) {
    println!("The measurment is: {value}{unit_lable}");
}

fn five() -> i32 {
    5
}