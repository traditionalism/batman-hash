use ggez::{filesystem, Context};

pub fn fix_path(ctx: &Context, path: &str) -> String {
    let slash_path = String::from("/") + path;

    if filesystem::is_file(ctx, &slash_path) || filesystem::is_dir(ctx, &slash_path) {
        slash_path
    } else {
        String::from(path)
    }
}