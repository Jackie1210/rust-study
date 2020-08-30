use std::collections::HashMap;

fn main() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }

    let mut arr = vec![100,37,22];

    for i in &arr {
        println!("{}", i)
    }
    for item in &mut arr {
        *item += 50;
    }

    let mut s1 = String::from("foo");

    let s2 = "bar";

    s1.push_str(s2);

    println!("s2 is {}", s2);

    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");

    let res = format!("{}-{}-{}", s3,s4,s5); // does not take ownership~
    println!("res is {}, s3 is {}", res, s3);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut teams = vec![String::from("Blue"), String::from("Yello")];
    let initial_scores = vec![10, 50];

    let mut scores1: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for(key, value) in scores1 {
        println!("{}: {}", key, value);
    }

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 10);

    scores3.entry(String::from("Yellow")).or_insert(50);
    scores3.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores3);

    let text = "cl cll cl ll";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        // or_insert returns a &mut V
        *count += 1;
    }

    println!("{:?}", map);
}
