use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        * {{
            box-sizing: border-box;
        }}

        body {{
            margin: 0;
            background: {background};
        }}

        .site-header {{
            width: min({content_width}, calc(100% - {horizontal_padding}));
            margin-inline: auto;
            padding-block: {header_padding};

            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: {header_gap};
        }}

        .site-logo {{
            display: inline-flex;
            align-items: center;
            gap: {logo_gap};

            color: {foreground};
            font-weight: {font_weight_bold};
            text-decoration: none;
        }}

        .site-logo img {{
            display: block;
            width: {logo_size};
            height: {logo_size};
            object-fit: contain;
        }}

        .site-header nav ul {{
            display: flex;
            align-items: center;
            gap: {nav_gap};
            list-style: none;
            margin: 0;
            padding: 0;
        }}

        .site-header nav a {{
            color: {muted};
        }}

        .site-header nav a:hover {{
            color: {accent};
        }}

        main {{
            width: min({content_width}, calc(100% - {horizontal_padding}));
            margin-inline: auto;
            padding-top: {hero_padding};
        }}

        .hero {{
            margin-bottom: {hero_margin};
        }}

        .hero h1 {{
            max-width: {heading_width};
            margin-bottom: {heading_margin};
        }}

        .hero p {{
            max-width: {text_width};
            font-size: {font_size_large};
        }}

        section:not(.hero) {{
            margin-bottom: {section_margin};
        }}

        section:not(.hero) h2 {{
            margin-bottom: {heading_margin};
        }}

        footer {{
            width: min({content_width}, calc(100% - {horizontal_padding}));
            margin: {footer_margin} auto 0;
            padding: {footer_padding} 0;
            border-top: 1px solid {border};
            text-align: center;
        }}

        footer p {{
            font-size: {font_size_small};
        }}

        @media (max-width: {mobile_breakpoint}) {{
            .site-header {{
                flex-direction: column;
                align-items: flex-start;
            }}

            main {{
                padding-top: {mobile_hero_padding};
            }}

            .hero {{
                margin-bottom: {mobile_hero_margin};
            }}
        }}
        "#,
        background = theme::BACKGROUND,
        foreground = theme::FOREGROUND,
        muted = theme::MUTED,
        accent = theme::AQUA,
        border = theme::BORDER,
        content_width = tokens::CONTENT_WIDTH,
        horizontal_padding = tokens::SPACE_12,
        header_padding = tokens::SPACE_6,
        header_gap = tokens::SPACE_4,
        logo_gap = tokens::SPACE_2,
        logo_size = tokens::SPACE_8,
        nav_gap = tokens::SPACE_6,
        font_weight_bold = tokens::FONT_WEIGHT_BOLD,
        font_size_large = tokens::FONT_SIZE_LARGE,
        font_size_small = tokens::FONT_SIZE_SMALL,
        hero_padding = tokens::HERO_PADDING_TOP,
        hero_margin = tokens::SPACE_24,
        heading_width = tokens::CONTENT_WIDTH,
        heading_margin = tokens::SPACE_6,
        text_width = tokens::TEXT_WIDTH,
        section_margin = tokens::SPACE_16,
        footer_margin = tokens::SPACE_24,
        footer_padding = tokens::SPACE_6,
        mobile_breakpoint = tokens::BREAKPOINT_MOBILE,
        mobile_hero_padding = tokens::HERO_PADDING_TOP_MOBILE,
        mobile_hero_margin = tokens::SPACE_16,
    )
}
