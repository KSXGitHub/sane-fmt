use super::super::{
    diff::diff_lines,
    term::color::*,
    DetailLevel::{self, *},
};
use std::path::Path;

/// Log found filesystem object and maybe diff if `--details` is not `count`.
pub type Act<'a> = Box<dyn Fn(&Path, &str, &str) + 'a>;

/// Lookup a function that may log found filesystem object according to `--details`.
/// * If `--details=count`, the returning function would do nothing.
/// * If `--details=name`, the returning function would log names.
/// * If `--details=diff`, the returning function would log names and diffs.
pub fn get<'a>(details: DetailLevel, theme: &'a BoxedColorScheme) -> Act {
    let print_name = move |path: &Path| {
        let message = format!("✗ {}", path.to_string_lossy());
        println!("{}", theme.find().paint(message));
    };
    match details {
        Count => Box::new(|_, _, _| ()),
        Name => Box::new(move |path, _, _| print_name(path)),
        Diff => Box::new(move |path, old, new| {
            print_name(path);
            for line in diff_lines(old, new) {
                println!("{}", line);
            }
        }),
    }
}
