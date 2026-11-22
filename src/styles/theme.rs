pub fn stylesheet() -> String {
    r#"
        body {
            background-color: var(--color-background);
            color: var(--color-foreground);
        }

        ::selection {
            background-color: var(--color-yellow);
            color: var(--color-background-hard);
        }

        a {
            color: var(--color-aqua);
        }

        a:hover {
            color: var(--color-green);
        }
    "#
    .to_string()
}
