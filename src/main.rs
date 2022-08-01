fn double_the_num(mut x: i32) -> String {
    x = x * 2;

    let res_string = "the doubled num is ".to_string() + &x.to_string();
    return res_string;
}

fn main() {
    println!("{}", double_the_num(2));
}
