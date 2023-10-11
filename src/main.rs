struct OptionTestPassOwnership {
    x: Option<String>,
    y: Option<String>,
}

fn main() {
    let x = Some(String::from("x"));
    let y = Some(String::from("y"));
    let testOption = OptionTestPassOwnership { x, y };
    let z = testOption.x;

    // I was told testrOption.x would be none because it's implicitely set to None when moved, but that is wrong, instead when the value is moved any attempt to access it throws a compiler error
    // println!("{}", testOption.x.is_none());
}
