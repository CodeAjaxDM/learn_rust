fn main() {
    let number = 6;

    if number % 4 == 0 
    {
        println!("the number is divisble by 4");
    }
    else if number % 3 == 0
    {
        println!("the number is divisble by 3");
    }
    else if number %2 == 0
    {
        println!("the nubmer is divisble by 2");
    }
    else
    {
        println!("the number is not divisible by 2, 3, or 4");
    }

    let condition = true;
    let value = if condition { 5 } else { 6 };

    println!("The value of value is: {value}");
}
