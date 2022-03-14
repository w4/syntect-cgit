use bat::assets::HighlightingAssets;
use once_cell::sync::Lazy;
use std::{io::Read, path::Path};
use syntect::{html::highlighted_html_for_string, parsing::SyntaxSet};

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
    let bat_assets = HighlightingAssets::from_binary();
    let ss = bat_assets
        .get_syntax_set()
        .unwrap_or_else(|_| syntact_syntax_set());
    let theme = bat_assets.get_theme("GitHub");

    let syntax = {
        let path = Path::new(filename);
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let extension = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        let path_syntax = ss
            .find_syntax_by_extension(file_name)
            .or_else(|| ss.find_syntax_by_extension(extension));

        path_syntax.or_else(|| {
            content
                .lines()
                .next()
                .and_then(|v| ss.find_syntax_by_first_line(v))
        })
    };

    let syntax = syntax.unwrap_or_else(|| ss.find_syntax_plain_text());

    println!(
        "{}",
        highlighted_html_for_string(content, &ss, syntax, theme)
    );
}

fn syntact_syntax_set() -> &'static SyntaxSet {
    static SS: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
    &*SS
}
