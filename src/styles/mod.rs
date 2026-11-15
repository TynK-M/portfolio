pub mod layout;
pub mod theme;
pub mod tokens;
pub mod typography;

pub fn stylesheet() -> String {
    [
        theme::stylesheet(),
        typography::stylesheet(),
        layout::stylesheet(),
    ]
    .join("\n")
}
