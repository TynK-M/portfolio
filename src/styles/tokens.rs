pub const SPACE_2: &str = "0.5rem";
pub const SPACE_4: &str = "1rem";
pub const SPACE_6: &str = "1.5rem";
pub const SPACE_8: &str = "2rem";
pub const SPACE_12: &str = "3rem";
pub const SPACE_16: &str = "4rem";
pub const SPACE_24: &str = "6rem";

pub const FONT_FAMILY: &str = "\"Ubuntu Mono\", monospace";

pub const FONT_SIZE_SMALL: &str = "0.875rem";
pub const FONT_SIZE_BASE: &str = "1rem";
pub const FONT_SIZE_LARGE: &str = "1.25rem";
pub const FONT_SIZE_HERO: &str = "clamp(2.5rem, 7vw, 4.5rem)";

pub const FONT_WEIGHT_BOLD: &str = "700";

pub const CONTENT_WIDTH: &str = "860px";
pub const TEXT_WIDTH: &str = "600px";

pub const BORDER_RADIUS_MEDIUM: &str = "8px";

pub const LINE_HEIGHT_BASE: &str = "1.7";
pub const LINE_HEIGHT_HEADING: &str = "1.1";

pub const BREAKPOINT_MOBILE: &str = "600px";
pub const HERO_PADDING_TOP: &str = "18vh";
pub const HERO_PADDING_TOP_MOBILE: &str = "12vh";

pub fn stylesheet() -> String {
    format!(
        r#"
        :root {{
            --space-2: {};
            --space-4: {};
            --space-6: {};
            --space-8: {};
            --space-12: {};
            --space-16: {};
            --space-24: {};

            --font-family: {};

            --font-size-small: {};
            --font-size-base: {};
            --font-size-large: {};
            --font-size-hero: {};

            --font-weight-bold: {};

            --content-width: {};
            --text-width: {};

            --border-radius-medium: {};

            --line-height-base: {};
            --line-height-heading: {};

            --breakpoint-mobile: {};
            --hero-padding-top: {};
            --hero-padding-top-mobile: {};
        }}
        "#,
        SPACE_2,
        SPACE_4,
        SPACE_6,
        SPACE_8,
        SPACE_12,
        SPACE_16,
        SPACE_24,
        FONT_FAMILY,
        FONT_SIZE_SMALL,
        FONT_SIZE_BASE,
        FONT_SIZE_LARGE,
        FONT_SIZE_HERO,
        FONT_WEIGHT_BOLD,
        CONTENT_WIDTH,
        TEXT_WIDTH,
        BORDER_RADIUS_MEDIUM,
        LINE_HEIGHT_BASE,
        LINE_HEIGHT_HEADING,
        BREAKPOINT_MOBILE,
        HERO_PADDING_TOP,
        HERO_PADDING_TOP_MOBILE,
    )
}
