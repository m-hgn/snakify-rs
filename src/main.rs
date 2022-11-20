#![feature(iter_intersperse)]
fn main() {
    let args = std::env::args();
    let separators: Vec<char> = vec!['-', '_', ' '];

    let input: String = args.skip(1).intersperse(" ".to_string()).collect();

    let output: String = input
        .chars()
        .map(|c| {
            if separators.contains(&c) {
                '_'
            } else {
                let x = c.to_lowercase().next();
                x.expect(&format!(
                    "Error: couldn't convert character to lowercase: `{}`",
                    c
                ));
                x.unwrap()
            }
        })
        .collect();

    println!("{}", output);
}
