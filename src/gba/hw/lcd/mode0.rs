use super::*;
use super::super::super::core::memory::*;
use super::tiles::*;

const TILE_ADDR: (u32, u32) = (0x06000000, 0x0600FFFF);

pub fn render_mode_0(dispcnt: u16, memory: &GbaMemory, line: u16, lines: &mut GbaDisplayLines) {
}