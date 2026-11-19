use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        .contact-links {{
            display: flex;
            flex-direction: column;
            gap: {};
            margin-top: {};
            padding: 0;
            list-style: none;
        }}

        .contact-links a {{
            display: inline-flex;
            align-items: center;
            gap: {};
            color: {};
            text-decoration: none;
            transition: color 150ms ease, transform 150ms ease;
        }}

        .contact-links a:hover {{
            color: {};
            transform: translateX(4px);
        }}

        .contact-links img {{
            display: block;
            width: {};
            height: {};
        }}
        "#,
        tokens::SPACE_4,
        tokens::SPACE_6,
        tokens::SPACE_2,
        theme::FOREGROUND,
        theme::AQUA,
        tokens::SPACE_6,
        tokens::SPACE_6,
    )
}
