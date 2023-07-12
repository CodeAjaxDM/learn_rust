fn main() {
    println!("Hello, world!");

    another_function(5, 'm');
}

fn another_function(value: i32, unit_lable: char) {
    println!("The measurment is: {value}{unit_lable}");
}