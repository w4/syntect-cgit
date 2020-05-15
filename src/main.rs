use std::io::Read;
use std::path::Path;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Ok(());
    }

    let mut buffer = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    highlight(&buffer, &args[1]);

    Ok(())
}

fn highlight(content: &str, filename: &str) {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = {
        let path = Path::new(filename);
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        let ext_syntax = ss
            .find_syntax_by_extension(file_name)
            .or_else(|| ss.find_syntax_by_extension(extension));

        let line_syntax = if ext_syntax.is_none() {
            content
                .lines()
                .next()
                .map(|v| ss.find_syntax_by_first_line(v))
                .flatten()
        } else {
            None
        };

        ext_syntax.or(line_syntax)
    };

    let syntax = syntax.unwrap_or_else(|| ss.find_syntax_plain_text());

    println!(
        "{}",
        highlighted_html_for_string(content, &ss, syntax, &ts.themes["InspiredGitHub"])
    );
}
