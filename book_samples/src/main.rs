fn main() {
    println!("Hello, world!");

    another_function(5, 'm');
    let x:i32 = five();
    println!("the value of x is: {x}");

    let test_degree = f_to_c(60.5);
    println!("60.5 degrees F is {test_degree} degrees C");

    let test_fibb = fibb_calc(6);
    println!("the 6th fibb number is {test_fibb}");
}

fn another_function(value: i32, unit_lable: char) {
    println!("The measurment is: {value}{unit_lable}");
}

fn five() -> i32 {
    5
}

fn f_to_c(degree_f: f64) -> f64
{
    (degree_f - 32.0) * (5.0/9.0)
}

fn fibb_calc(nth_value: i32) -> i32
{
    if nth_value <= 0
    {
        return 0;
    }
    else
    {
        let mut counter: i32 = 1;
        let mut fibb_value:i32 = 1;
        let mut fibb_prior:i32 = 0;
        while counter < nth_value && nth_value > 0
        {
            let temp: i32 = fibb_value;
            fibb_value += fibb_prior;
            fibb_prior = temp;
            counter += 1;
        }
        fibb_value
    }
    
}