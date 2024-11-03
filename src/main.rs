use Flag::*;

#[derive(PartialEq, Eq, Debug)]
enum Flag {
    Force,
    Help,
    Version,
}

fn main() {
    let (appname, version) = (env!("CARGO_BIN_NAME"), env!("CARGO_PKG_VERSION"));
    let (flag, arg_start) = std::env::args()
        .enumerate()
        .skip(1)
        .next()
        .map(|(i, arg)| match arg.as_str() {
            "-f" | "--force" => (Some(Force), i),
            "-h" | "--help" => (Some(Help), i),
            "-V" | "--version" => (Some(Version), i),
            _ => (None, 0),
        })
        .unwrap_or_default();

    if flag == Some(Help) {
        println!("Convert space-separated strings to `snake_case` and print to stdout.");
        println!("If input contains invalid symbols, exits with code 1.");
        println!("");
        println!("Usage: {appname} [Option] [<string>...]");
        println!("");
        println!("Arguments:");
        println!("  [<string>...]  One or more strings treated as a single input");
        println!("");
        println!("Options:");
        println!("  -f, --force    Force successful conversion, ignoring invalid chars");
        println!("  -h, --help     Print this usage information");
        println!("  -V, --version  Print version information");
        std::process::exit(0);
    } else if flag == Some(Version) {
        println!("{appname} {version}");
        std::process::exit(0);
    }

    let mut arg_warned = false;
    let mut output = String::new();
    for arg in std::env::args().skip(1 + arg_start) {
        if !arg_warned
            && matches!(
                arg.as_str(),
                "-f" | "--force" | "-h" | "--help" | "-V" | "--version"
            )
        {
            eprintln!("Warning: You might have passed an option where input was expected.");
            eprintln!("         If this was unintentional: pass options *before* input.");
            eprintln!("");
            arg_warned = true;
        }

        let last_was_separator = output.bytes().last().map(|x| x == b'_').unwrap_or(true);
        for c in arg.chars() {
            match c {
                ' ' | '-' | '_' if last_was_separator => {}
                ' ' | '-' | '_' => output.push('_'),

                'a'..='z' | 'A'..='Z' | '0'..='9' => output.push(c.to_ascii_lowercase()),
                _ if flag == Some(Force) => output.push(c),

                _ => {
                    eprintln!("Error: Invalid snake_case character: `{c}`");
                    std::process::exit(1);
                }
            }
        }
        output.push('_')
    }
    output.pop();

    println!("{output}");
}
