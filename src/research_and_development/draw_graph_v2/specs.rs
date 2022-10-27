use super::drawing_tools::Color;

enum DisplayMode {
    _DETAILLED,
    MINIMAL,
}

pub enum ColorGeneration {
    CYCLIC,
    _RANDOM,
}

pub const COLOR_GEN_MODE: ColorGeneration = ColorGeneration::CYCLIC;
const BLOCK_MODE: DisplayMode = DisplayMode::MINIMAL;

pub const DISPLAY_BYTECODE: bool = match BLOCK_MODE {
    DisplayMode::_DETAILLED => true,
    DisplayMode::MINIMAL => false,
};

pub const HORYZONTAL_IMPROVEMENT_N_ITERS: usize = 100;
pub const BLOCK_WIDTH: f32 = 200.;
pub const TEXT_SIZE: f32 = 15.0;
pub const LEFT_PADDING: f32 = 100.;
pub const TOP_PADDING: f32 = 100.;
pub const X_SPACE_BETWEEN_BLOCKS: f32 = 400.;
pub const Y_SPACE_BETWEEN_BLOCKS: f32 = 200.;

pub const BACKGROUND_COLOR: Color = Color {
    alpha: 255,
    red: 255,
    green: 255,
    blue: 255,
};
pub const DEFAULT_BLOCK_COLOR: Color = Color {
    alpha: 255,
    red: 0,
    green: 0,
    blue: 0,
};

pub const SPECIAL_BLOCK_COLOR: Color = Color {
    alpha: 255,
    red: 0,
    green: 255,
    blue: 0,
};

pub const EXTERNAL_CONNECTIONS_COUNT_COLOR: Color = Color {
    alpha: 255,
    red: 255,
    green: 0,
    blue: 0,
};

pub const DEFAULT_CONNECTIONS_COLOR: Color = Color {
    alpha: 255,
    red: 0,
    green: 0,
    blue: 0,
};

pub const CYCLIC_COLORS: [Color; 4] = [
    Color {
        alpha: 255,
        red: 255,
        green: 0,
        blue: 0,
    },
    Color {
        alpha: 255,
        red: 0,
        green: 255,
        blue: 0,
    },
    Color {
        alpha: 255,
        red: 0,
        green: 0,
        blue: 255,
    },
    Color {
        alpha: 255,
        red: 255,
        green: 0,
        blue: 255,
    },
];
