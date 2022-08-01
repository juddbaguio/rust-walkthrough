fn double_the_num(format_str: &str, mut x: i32) -> String {
    x = x * 2;

    let res_string = format_str.to_string() + &x.to_string();
    return res_string;
}

fn main() {
    println!("{}", double_the_num("the num doubled is ", 2));
}
