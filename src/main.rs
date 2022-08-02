fn takes_ownership(somestring: String) {
    println!("{}", somestring);
}

fn makes_copy(num: i16) {
    println!("{}", num)
}

fn takes_and_givesback(input: String) -> String {
    return input
}

fn main() {
    // // println!("{}", double_the_num("the num doubled is ", 2));

    // let mut s = String::from("hello");
    // s.push_str(", Judd Misael R .Baguio");

    // println!("{}",s);

    // //
    // println!("What is your name?");
    // let mut input_str = String::new();
    // stdin().lock().read_line(&mut input_str).expect("Read Failure");

    // let mut base_str = String::from("Nice to meet you, ");
    // base_str.push_str(&input_str);

    // println!("{}",base_str);

    let s1 = String::from("I'm the owner!");
    takes_ownership(s1);

    // takes_ownership fn takes hold of s1's previous value,
    // s1 is now considered no-value or shallowed
    // because it is not taking ownership to the previously referenced memory address of its value.
    // println!("{}", s1); 


    let s2 = "To be given back".to_string();

    // takes ownership of s2's value
    // returns s2's value
    // and give the ownership to s3
    let s3 = takes_and_givesback(s2);

    println!("{}",s3);

    let num = 12;
    makes_copy(num);

}

