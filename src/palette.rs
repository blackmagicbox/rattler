#![allow(dead_code)]
use macroquad::prelude::Color;

// Palette inspired by the "Retrocal-8-palette" at https://lospec.com/palette-list/retrocal-8
pub const DESERT_SKY: Color = Color::from_hex(0x6eb8a8);
pub const SNAKE_SKIN_DARK: Color = Color::from_hex(0x2a584f);
pub const SNAKE_SKIN: Color = Color::from_hex(0x74a33f);
pub const SNAKE_BELLY: Color = Color::from_hex(0xfcffc0);
pub const DESERT_SAND: Color = Color::from_hex(0xee9c5d);
pub const DESERT_SAND_DARK: Color = Color::from_hex(0x774448);
pub const DESERT_NIGHT: Color = Color::from_hex(0x2f142f);
pub const CACTUS_FLOWER: Color = Color::from_hex(0xc6505a);

pub const BACKGROUND: Color = DESERT_SAND;
pub const GRID_LINE: Color = DESERT_SAND_DARK;
