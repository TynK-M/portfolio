use super::{theme, tokens};

pub fn stylesheet() -> String {
    format!(
        r#"
        .about {{
            display: flex;
            flex-direction: column;
            gap: {section_gap};
        }}

        .about-intro {{
            max-width: {text_width};
        }}

        .about-intro p {{
            color: {muted};
            font-size: {base_size};
            line-height: {line_height};
        }}

        .about-experience,
        .about-education {{
            display: flex;
            flex-direction: column;
            gap: {item_gap};
        }}

        .experience-item,
        .education-item {{
            display: flex;
            flex-direction: column;
            gap: {item_gap};
            padding: {item_padding};
            background: {surface};
            border: 1px solid {border};
            border-radius: {radius};
        }}

        .experience-header,
        .education-header {{
            display: flex;
            align-items: baseline;
            justify-content: space-between;
            flex-wrap: wrap;
            gap: {item_gap};
        }}

        .experience-item h3,
        .education-item h3 {{
            margin: 0;
            color: {foreground};
            font-size: {title_size};
        }}

        .experience-period,
        .education-period {{
            color: {accent};
            font-size: {small_size};
        }}

        .experience-meta,
        .education-meta {{
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: {small_gap};
            font-size: {small_size};
        }}

        .experience-company,
        .education-institution {{
            color: {yellow};
        }}

        .experience-separator,
        .experience-type,
        .education-separator,
        .education-grade {{
            color: {muted};
        }}

        .experience-description,
        .education-description {{
            color: {muted};
            font-size: {base_size};
            line-height: {line_height};
        }}

        .experience-description p,
        .education-description p {{
            margin: 0 0 {paragraph_gap};
        }}

        .experience-description p:last-child,
        .education-description p:last-child {{
            margin-bottom: 0;
        }}

        @media (max-width: {breakpoint}) {{
            .experience-header,
            .education-header {{
                flex-direction: column;
                align-items: flex-start;
                gap: {small_gap};
            }}
        }}
        "#,
        section_gap = tokens::SPACE_12,
        text_width = tokens::TEXT_WIDTH,
        muted = theme::MUTED,
        line_height = tokens::LINE_HEIGHT_BASE,
        base_size = tokens::FONT_SIZE_BASE,
        item_gap = tokens::SPACE_2,
        item_padding = tokens::SPACE_6,
        surface = theme::SURFACE,
        border = theme::BORDER,
        radius = tokens::BORDER_RADIUS_MEDIUM,
        foreground = theme::FOREGROUND,
        title_size = tokens::FONT_SIZE_LARGE,
        accent = theme::AQUA,
        yellow = theme::YELLOW,
        small_size = tokens::FONT_SIZE_SMALL,
        small_gap = tokens::SPACE_2,
        paragraph_gap = tokens::SPACE_4,
        breakpoint = tokens::BREAKPOINT_MOBILE,
    )
}
