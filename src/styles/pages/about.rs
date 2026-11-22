use super::super::tokens;

pub fn stylesheet() -> String {
    format!(
        r#"
        .about {{
            display: flex;
            flex-direction: column;
            gap: var(--space-12);
        }}

        .about-intro {{
            max-width: var(--text-width);
        }}

        .about-intro p {{
            color: var(--color-muted);
            font-size: var(--font-size-base);
            line-height: var(--line-height-base);
        }}

        .about-experience,
        .about-education {{
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
        }}

        .experience-item,
        .education-item {{
            display: flex;
            flex-direction: column;
            gap: var(--space-2);
            padding: var(--space-6);
            background: var(--color-surface);
            border: 1px solid var(--color-border);
            border-radius: var(--border-radius-medium);
        }}

        .experience-header,
        .education-header {{
            display: flex;
            align-items: baseline;
            justify-content: space-between;
            flex-wrap: wrap;
            gap: var(--space-2);
        }}

        .experience-item h3,
        .education-item h3 {{
            margin: 0;
            color: var(--color-foreground);
            font-size: var(--font-size-large);
        }}

        .experience-period,
        .education-period {{
            color: var(--color-aqua);
            font-size: var(--font-size-small);
        }}

        .experience-meta,
        .education-meta {{
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            gap: var(--space-2);
            font-size: var(--font-size-small);
        }}

        .experience-company,
        .education-institution {{
            color: var(--color-yellow);
        }}

        .experience-separator,
        .experience-type,
        .education-separator,
        .education-grade {{
            color: var(--color-muted);
        }}

        .experience-description,
        .education-description {{
            color: var(--color-muted);
            font-size: var(--font-size-base);
            line-height: var(--line-height-base);
        }}

        .experience-description p,
        .education-description p {{
            margin: 0 0 var(--space-4);
        }}

        .experience-description p:last-child,
        .education-description p:last-child {{
            margin-bottom: 0;
        }}

        @media (max-width: {}) {{
            .experience-header,
            .education-header {{
                flex-direction: column;
                align-items: flex-start;
                gap: var(--space-2);
            }}
        }}
        "#,
        tokens::BREAKPOINT_MOBILE,
    )
}
