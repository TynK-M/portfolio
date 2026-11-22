pub fn stylesheet() -> String {
    r#"
        .contact-links {
            display: flex;
            flex-direction: column;
            gap: var(--space-4);
            margin-top: var(--space-6);
            padding: 0;
            list-style: none;
        }

        .contact-links a {
            display: inline-flex;
            align-items: center;
            gap: var(--space-2);
            color: var(--color-foreground);
            text-decoration: none;
            transition: color 150ms ease, transform 150ms ease;
        }

        .contact-links a:hover {
            color: var(--color-aqua);
            transform: translateX(4px);
        }

        .contact-links img {
            display: block;
            width: var(--space-6);
            height: var(--space-6);
        }
    "#
    .to_string()
}
