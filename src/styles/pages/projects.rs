use super::super::tokens;

pub fn stylesheet() -> String {
    format!(
        r#"
        .projects {{
            display: flex;
            flex-direction: column;
            gap: var(--space-8);
        }}

        .project-legend {{
            display: flex;
            flex-wrap: wrap;
            align-items: center;
            gap: var(--space-4);
            font-size: var(--font-size-small);
        }}

        .projects-grid {{
            display: grid;
            grid-template-columns: repeat(2, minmax(0, 1fr));
            gap: var(--space-6);
        }}

        .project-card {{
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            gap: var(--space-4);
            padding: var(--space-4);
            background: var(--color-surface);
            border: 1px solid var(--color-border);
            border-radius: var(--border-radius-medium);
            transition: border-color 150ms ease, transform 150ms ease;
        }}

        .project-card:hover {{
            border-color: var(--color-aqua);
            transform: translateY(-2px);
        }}

        .project-card-header {{
            display: flex;
            align-items: center;
            justify-content: space-between;
            gap: var(--space-2);
        }}

        .project-card h2 {{
            margin: 0;
            font-size: var(--font-size-large);
        }}

        .project-card h2 a {{
            color: var(--color-foreground);
            text-decoration: none;
            display: inline-flex;
            align-items: center;
            gap: var(--space-2);
        }}

        .project-card h2 a img {{
            width: var(--space-6);
            height: var(--space-6);
        }}

        .project-card h2 a:hover {{
            color: var(--color-aqua);
        }}

        .project-card-description {{
            color: var(--color-muted);
            margin: 0;
            line-height: var(--line-height-base);
        }}

        .project-card-meta {{
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: var(--space-4);
            color: var(--color-muted);
            font-size: var(--font-size-small);
        }}

        .project-card-meta span {{
            white-space: nowrap;
        }}

        .project-language {{
            color: var(--color-purple);
        }}

        .project-stars {{
            color: var(--color-yellow);
        }}

        .project-forks {{
            color: var(--color-aqua);
        }}

        .project-issues {{
            color: var(--color-red);
        }}

        .project-updated {{
            color: var(--color-muted);
        }}

        @media (max-width: {}) {{
            .projects-grid {{
                grid-template-columns: 1fr;
            }}
        }}
        "#,
        tokens::BREAKPOINT_MOBILE,
    )
}
