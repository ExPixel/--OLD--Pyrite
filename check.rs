#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
pub trait BitDescriptor {
    fn get_property_name(&self, index: usize)
    -> &'static str;
    fn get_property_value(&self, index: usize)
    -> u32;
}

// #TODO I can use a &' static array instead of the current if/else++ mess.
pub struct RegDispcntDesc {
    properties: &'static [&'static str],
    values: Vec<u32>,
    pub length: u32,
    pub value: u32,
}

impl 


 /*

 4000000h - DISPCNT - LCD Control (Read/Write)
   Bit   Expl.
   0-2   BG Mode                (0-5=Video Mode 0-5, 6-7=Prohibited)
   3     Reserved / CGB Mode    (0=GBA, 1=CGB; can be set only by BIOS opcodes)
   4     Display Frame Select   (0-1=Frame 0-1) (for BG Modes 4,5 only)
   5     H-Blank Interval Free  (1=Allow access to OAM during H-Blank)
   6     OBJ Character VRAM Mapping (0=Two dimensional, 1=One dimensional)
   7     Forced Blank           (1=Allow FAST access to VRAM,Palette,OAM)
   8     Screen Display BG0  (0=Off, 1=On)
   9     Screen Display BG1  (0=Off, 1=On)
   10    Screen Display BG2  (0=Off, 1=On)
   11    Screen Display BG3  (0=Off, 1=On)
   12    Screen Display OBJ  (0=Off, 1=On)
   13    Window 0 Display Flag   (0=Off, 1=On)
   14    Window 1 Display Flag   (0=Off, 1=On)
   15    OBJ Window Display Flag (0=Off, 1=On)
 */
 RegDispcntDesc {
    pub fn from(value: u32) -> RegDispcntDesc {
        static _PROPERTIES: &'static [&'static str] =
            &["bg_mode", "cgb_mode", "frame_select", "hblank_interval_free",
              "vram_obj_mapping", "forced_blank", "display_bg0",
              "display_bg1", "display_bg2", "display_bg3", "display_obj",
              "window_0_display", "window_1_display", "obj_window_display"];
        let mut values = Vec::new();
        let mut off = 0;
        let mut temp;
        temp = (value >> off) & ((1 << 3) - 1);
        values.push(temp);
        off += 3;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        temp = (value >> off) & ((1 << 1) - 1);
        values.push(temp);
        off += 1;
        RegDispcntDesc{properties: _PROPERTIES,
                       values: values,
                       value: value,
                       length: off,}
    }
}
impl BitDescriptor for RegDispcntDesc {
    #[allow(unused_assignments)]
    fn get_property_name(&self, index: usize) -> &'static str {
        return self.properties[index]
    }
    #[allow(unused_assignments)]
    fn get_property_value(&self, index: usize) -> u32 {
        return self.values[index]
    }
}
