use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        * {{
            box-sizing: border-box;
        }}

        body {{
            margin: 0;
            background: {};
        }}

        main {{
            width: min({}, calc(100% - {}));
            margin-inline: auto;
            padding-top: {};
        }}

        .hero {{
            margin-bottom: {};
        }}

        .hero h1 {{
            max-width: {};
            margin-bottom: {};
        }}

        .hero p {{
            max-width: {};
            font-size: {};
        }}

        section:not(.hero) {{
            margin-bottom: {};
        }}

        section:not(.hero) h2 {{
            margin-bottom: {};
        }}

        footer {{
            width: min({}, calc(100% - {}));
            margin: {} auto 0;
            padding: {} 0;
            border-top: 1px solid {};
            text-align: center;
        }}

        footer p {{
            font-size: {};
        }}

        @media (max-width: {}) {{
            main {{
                padding-top: {};
            }}

            .hero {{
                margin-bottom: {};
            }}
        }}
        "#,
        theme::BACKGROUND,
        tokens::CONTENT_WIDTH,
        tokens::SPACE_12,
        tokens::HERO_PADDING_TOP,
        tokens::SPACE_24,
        tokens::CONTENT_WIDTH,
        tokens::SPACE_6,
        tokens::TEXT_WIDTH,
        tokens::FONT_SIZE_LARGE,
        tokens::SPACE_16,
        tokens::SPACE_4,
        tokens::CONTENT_WIDTH,
        tokens::SPACE_12,
        tokens::SPACE_24,
        tokens::SPACE_6,
        theme::BORDER,
        tokens::FONT_SIZE_SMALL,
        tokens::BREAKPOINT_MOBILE,
        tokens::HERO_PADDING_TOP_MOBILE,
        tokens::SPACE_16,
    )
}
