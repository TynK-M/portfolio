pub mod contact;
pub mod layout;
pub mod projects;
pub mod theme;
pub mod tokens;
pub mod typography;

pub fn global() -> String {
    [
        theme::stylesheet(),
        typography::stylesheet(),
        layout::stylesheet(),
    ]
    .join("\n")
}
