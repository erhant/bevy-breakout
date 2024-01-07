use bevy::prelude::*;

pub struct Theme {
    pub Primary: Color,
    pub Secondary: Color,
    pub Accent: Color,
    pub Neutral: Color,
    pub Info: Color,
    pub Success: Color,
    pub Warning: Color,
    pub Error: Color,
}

// https://github.com/dracula/dracula-theme
// https://github.com/saadeghi/daisyui/blob/master/src/theming/themes.js#L117
pub const MAIN_THEME: Theme = Theme {
    Primary: Color::rgb(1., 0.4745, 0.7764),
    Secondary: Color::rgb(0.7411, 0.5764, 0.9764),
    Accent: Color::rgb(1., 0.7215, 0.4235),
    Neutral: Color::rgb(0.2549, 0.2705, 0.3450),
    Info: Color::rgb(0.5450, 0.9137, 0.9921),
    Success: Color::rgb(0.3137, 0.9803, 0.4823),
    Warning: Color::rgb(0.9450, 0.9803, 0.5490),
    Error: Color::rgb(1., 0.3333, 0.3333),
};
