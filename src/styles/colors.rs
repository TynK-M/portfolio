pub const BACKGROUND: &str = "#282828";
pub const BACKGROUND_HARD: &str = "#1d2021";
pub const SURFACE: &str = "#3c3836";
pub const SURFACE_LIGHT: &str = "#504945";

pub const FOREGROUND: &str = "#ebdbb2";
pub const MUTED: &str = "#a89984";

pub const RED: &str = "#fb4934";
pub const GREEN: &str = "#b8bb26";
pub const YELLOW: &str = "#fabd2f";
pub const BLUE: &str = "#83a598";
pub const PURPLE: &str = "#d3869b";
pub const AQUA: &str = "#8ec07c";
pub const ORANGE: &str = "#fe8019";

pub const BORDER: &str = "#504945";

pub fn stylesheet() -> String {
    format!(
        r#"
        :root {{
            --color-background: {};
            --color-background-hard: {};
            --color-surface: {};
            --color-surface-light: {};

            --color-foreground: {};
            --color-muted: {};

            --color-red: {};
            --color-green: {};
            --color-yellow: {};
            --color-blue: {};
            --color-purple: {};
            --color-aqua: {};
            --color-orange: {};

            --color-border: {};
        }}
        "#,
        BACKGROUND,
        BACKGROUND_HARD,
        SURFACE,
        SURFACE_LIGHT,
        FOREGROUND,
        MUTED,
        RED,
        GREEN,
        YELLOW,
        BLUE,
        PURPLE,
        AQUA,
        ORANGE,
        BORDER,
    )
}
