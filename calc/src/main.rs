use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut op: Option<char> = None;
    let mut first: Option<f32> = None;
    let mut second: Option<f32> = None;
    for x in &args {
        if let Ok(n) = x.parse::<f32>() {
            if first.is_none() {
                first = Some(n);
            } else {
                second = Some(n);
            }
        } else {
            op = Some(x.parse().expect("Failed to parse"));
        }
    };
    match op.unwrap() {
        'a' => println!("{}", first.unwrap() + second.unwrap()),
        's' => println!("{}", first.unwrap() - second.unwrap()),
        't' => println!("{}", first.unwrap() * second.unwrap()),
        'd' => println!("{}", first.unwrap() / second.unwrap()),
        c => println!("Unknown operator: {}", c),
    }

}
