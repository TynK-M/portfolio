pub mod colors;
pub mod layout;
pub mod pages;
pub mod theme;
pub mod tokens;
pub mod typography;

pub fn global() -> String {
    [
        typography::import_font(),
        colors::stylesheet(),
        tokens::stylesheet(),
        theme::stylesheet(),
        typography::stylesheet(),
        layout::stylesheet(),
    ]
    .join("\n")
}
