pub fn import_font() -> String {
    "@import url('https://fonts.googleapis.com/css2?family=Ubuntu+Mono:wght@400;500;700&display=swap');".to_string()
}

pub fn stylesheet() -> String {
    r#"
        body {
            font-family: var(--font-family);
            line-height: var(--line-height-base);
        }

        h1, h2, p {
            margin-top: 0;
        }

        h1 {
            font-size: var(--font-size-hero);
            line-height: var(--line-height-heading);
            letter-spacing: -0.04em;
            font-weight: var(--font-weight-bold);
        }

        h2 {
            font-size: var(--font-size-large);
            font-weight: 500;
            letter-spacing: -0.02em;
        }

        p {
            font-size: var(--font-size-base);
        }

        a {
            text-decoration: none;
        }
    "#
    .to_string()
}
