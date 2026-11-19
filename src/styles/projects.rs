use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        .projects {{
            display: flex;
            flex-direction: column;
            gap: {section_gap};
        }}

        .project-legend {{
            display: flex;
            flex-wrap: wrap;
            align-items: center;
            gap: {meta_gap};
            font-size: {meta_size};
        }}

        .projects-grid {{
            display: grid;
            grid-template-columns: repeat(2, minmax(0, 1fr));
            gap: {grid_gap};
        }}

        .project-card {{
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            gap: {card_gap};
            padding: {card_padding};
            background: {surface};
            border: 1px solid {border};
            border-radius: {radius};
            transition: border-color 150ms ease, transform 150ms ease;
        }}

        .project-card:hover {{
            border-color: {accent};
            transform: translateY(-2px);
        }}

        .project-card-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: {header_gap};
        }}

        .project-card h2 {{
            margin: 0;
            font-size: {title_size};
        }}

        .project-card h2 a {{
            color: {foreground};
            text-decoration: none;
        }}

        .project-card h2 a:hover {{
            color: {accent};
        }}

        .project-card-description {{
            color: {muted};
            margin: 0;
            line-height: {line_height};
        }}

        .project-card-meta {{
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: {meta_gap};
            color: {muted};
            font-size: {meta_size};
        }}

        .project-card-meta span {{
            white-space: nowrap;
        }}

        .project-language {{
            color: {language_color};
        }}

        .project-stars {{
            color: {stars_color};
        }}

        .project-forks {{
            color: {forks_color};
        }}

        .project-issues {{
            color: {issues_color};
        }}

        .project-updated {{
            color: {muted};
        }}

        @media (max-width: {breakpoint}) {{
            .projects-grid {{
                grid-template-columns: 1fr;
            }}
        }}
        "#,
        section_gap = tokens::SPACE_8,
        grid_gap = tokens::SPACE_6,
        card_gap = tokens::SPACE_4,
        card_padding = tokens::SPACE_4,
        surface = theme::SURFACE,
        border = theme::BORDER,
        radius = tokens::BORDER_RADIUS_MEDIUM,
        accent = theme::AQUA,
        header_gap = tokens::SPACE_2,
        title_size = tokens::FONT_SIZE_LARGE,
        foreground = theme::FOREGROUND,
        muted = theme::MUTED,
        line_height = tokens::LINE_HEIGHT_BASE,
        meta_gap = tokens::SPACE_4,
        meta_size = tokens::FONT_SIZE_SMALL,
        language_color = theme::PURPLE,
        stars_color = theme::YELLOW,
        forks_color = theme::AQUA,
        issues_color = theme::RED,
        breakpoint = tokens::BREAKPOINT_MOBILE,
    )
}
