use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_file;

fn main() {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return;
    }

    let theme = &ts.themes["InspiredGitHub"];
    let html = highlighted_html_for_file(&args[1], &ss, theme).unwrap();
    println!("{}", html);
}
