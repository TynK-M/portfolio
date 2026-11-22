use super::tokens;

pub fn stylesheet() -> String {
    format!(
        r#"
        * {{
            box-sizing: border-box;
        }}

        body {{
            margin: 0;
        }}

        .site-header {{
            width: min(var(--content-width), calc(100% - var(--space-12)));
            margin-inline: auto;
            padding-block: var(--space-6);

            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-4);
        }}

        .site-logo {{
            display: inline-flex;
            align-items: center;
            gap: var(--space-2);

            color: var(--color-foreground);
            font-weight: var(--font-weight-bold);
            text-decoration: none;
        }}

        .site-logo img {{
            display: block;
            width: var(--space-8);
            height: var(--space-8);
            object-fit: contain;
        }}

        .site-header nav ul {{
            display: flex;
            align-items: center;
            gap: var(--space-6);
            list-style: none;
            margin: 0;
            padding: 0;
        }}

        .site-header nav a {{
            color: var(--color-muted);
        }}

        .site-header nav a:hover {{
            color: var(--color-aqua);
        }}

        main {{
            width: min(var(--content-width), calc(100% - var(--space-12)));
            margin-inline: auto;
            padding-top: var(--hero-padding-top);
        }}

        .hero {{
            margin-bottom: var(--space-24);
        }}

        .hero h1 {{
            max-width: var(--content-width);
            margin-bottom: var(--space-6);
        }}

        .hero p {{
            max-width: var(--text-width);
            font-size: var(--font-size-large);
        }}

        section:not(.hero) {{
            margin-bottom: var(--space-16);
        }}

        section:not(.hero) h2 {{
            margin-bottom: var(--space-6);
        }}

        footer {{
            width: min(var(--content-width), calc(100% - var(--space-12)));
            margin: var(--space-24) auto 0;
            padding: var(--space-6) 0;
            border-top: 1px solid var(--color-border);
            text-align: center;
        }}

        footer p {{
            font-size: var(--font-size-small);
        }}

        @media (max-width: {}) {{
            .site-header {{
                flex-direction: column;
                align-items: flex-start;
            }}

            main {{
                padding-top: var(--hero-padding-top-mobile);
            }}

            .hero {{
                margin-bottom: var(--space-16);
            }}
        }}
        "#,
        tokens::BREAKPOINT_MOBILE,
    )
}
