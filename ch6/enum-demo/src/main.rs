fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 1u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("else")
    }

    let some_value = Some(6u8);

    match some_value{
        Some(3) => println!("three"),
        _ => println!("else value"),
    }

    if let Some(6) = some_value{
        println!("six")
    }

    println!("res is {:?}, {:?}, {:?}", five, six, none)
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1),
    }
}