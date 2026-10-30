#![allow(dead_code)]
use macroquad::prelude::Color;

// Palette inspired by the "Retrocal-8-palette" at https://lospec.com/palette-list/retrocal-8
const DESERT_SKY: Color = Color::from_hex(0x6eb8a8);
const SNAKE_SKIN_DARK: Color = Color::from_hex(0x2a584f);
const SNAKE_SKIN: Color = Color::from_hex(0x74a33f);
const SNAKE_BELLY: Color = Color::from_hex(0xfcffc0);
const DESERT_SAND: Color = Color::from_hex(0xee9c5d);
const DESERT_SAND_DARK: Color = Color::from_hex(0x774448);
const DESERT_NIGHT: Color = Color::from_hex(0x2f142f);
const CACTUS_FLOWER: Color = Color::from_hex(0xc6505a);

pub const BACKGROUND: Color = DESERT_SAND;
pub const GRID_LINE: Color = DESERT_SAND_DARK;
pub const SNAKE_HEAD: Color = SNAKE_SKIN;
pub const SNAKE_BODY: Color = SNAKE_SKIN_DARK;
