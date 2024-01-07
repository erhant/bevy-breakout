use bevy::prelude::*;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub neutral: Color,
    pub info: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub base100: Color,
}

// https://github.com/dracula/dracula-theme
// https://github.com/saadeghi/daisyui/blob/master/src/theming/themes.js#L117
const DRACULA: Theme = Theme {
    primary: Color::rgb(1.00000000, 0.47450980, 0.77647059),
    secondary: Color::rgb(0.74117647, 0.57647059, 0.97647059),
    accent: Color::rgb(1.00000000, 0.72156863, 0.42352941),
    neutral: Color::rgb(0.25490196, 0.27058824, 0.34509804),
    base100: Color::rgb(0.15686275, 0.16470588, 0.21176471),
    info: Color::rgb(0.54509804, 0.91372549, 0.99215686),
    success: Color::rgb(0.31372549, 0.98039216, 0.48235294),
    warning: Color::rgb(0.94509804, 0.98039216, 0.54901961),
    error: Color::rgb(1.00000000, 0.33333333, 0.33333333),
};

pub const MAIN_THEME: Theme = DRACULA;
