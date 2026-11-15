use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        @import url('https://fonts.googleapis.com/css2?family=Ubuntu+Mono:wght@400;500;700&display=swap');

        :root {{
            --font-family: {};
            --color-foreground: {};
            --color-muted: {};
            --color-aqua: {};
            --color-green: {};

            --font-size-small: {};
            --font-size-base: {};
            --font-size-large: {};
            --font-size-hero: {};

            --line-height-base: {};
            --line-height-heading: {};
        }}

        body {{
            font-family: var(--font-family);
            color: var(--color-foreground);
            line-height: var(--line-height-base);
        }}

        h1, h2, p {{
            margin-top: 0;
        }}

        h1 {{
            font-size: var(--font-size-hero);
            line-height: var(--line-height-heading);
            letter-spacing: -0.04em;
            font-weight: 700;
        }}

        h2 {{
            font-size: var(--font-size-large);
            font-weight: 500;
            letter-spacing: -0.02em;
        }}

        p {{
            color: var(--color-muted);
            font-size: var(--font-size-base);
        }}

        a {{
            color: var(--color-aqua);
            text-decoration: none;
        }}

        a:hover {{
            color: var(--color-green);
        }}
        "#,
        tokens::FONT_FAMILY,
        theme::FOREGROUND,
        theme::MUTED,
        theme::AQUA,
        theme::GREEN,
        tokens::FONT_SIZE_SMALL,
        tokens::FONT_SIZE_BASE,
        tokens::FONT_SIZE_LARGE,
        tokens::FONT_SIZE_HERO,
        tokens::LINE_HEIGHT_BASE,
        tokens::LINE_HEIGHT_HEADING,
    )
}
