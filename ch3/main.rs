fn main(){
  let x = 5;

  let y = {
    let x = 3;
    x + 1
  };

  let res = plus_one(y);

  println!("The value of res is {}", res)
}

fn plus_one(x: i32) -> i32{
  x + 1
}