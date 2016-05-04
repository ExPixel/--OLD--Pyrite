pub trait BitDescriptor {
	fn property_count(&self) -> usize;
	fn get_property_name(&self, index: usize) -> &'static str;
	fn get_property_value(&self, index: usize) -> u32;
}

macro_rules! bit_descriptor {
	($desc_name: ident, $($name:ident: $length:expr)+) => (
		pub struct $desc_name {
			properties: &'static [&'static str],
			values: Vec<u32>,
			pub length: u32,
			pub value: u32
		}

		impl $desc_name {
			pub fn from(value: u32) -> $desc_name {
				static _PROPERTIES: &'static [&'static str] = &[$( stringify!($name), )+];
				let mut values = Vec::new();
				let mut off = 0;
				let mut temp;
				$(
					temp = (value >> off) & ((1 << $length) - 1);
					values.push(temp);
					off += $length;
				)+
				$desc_name {
					properties: _PROPERTIES,
					values: values,
					value: value,
					length: off
				}
			}
		}

		impl BitDescriptor for $desc_name {
			fn property_count(&self) -> usize {
				return self.properties.len();
			}

			#[allow(unused_assignments)]
			fn get_property_name(&self, index: usize) -> &'static str {
				return self.properties[index]
			}

			#[allow(unused_assignments)]
			fn get_property_value(&self, index: usize) -> u32 {
				return self.values[index]
			}
		}
	);
}

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
bit_descriptor!(RegDispcntDesc,
	bg_mode: 3
	cgb_mode: 1
	frame_select: 1
	hblank_interval_free: 1
	vram_obj_mapping: 1
	forced_blank: 1
	display_bg0: 1
	display_bg1: 1
	display_bg2: 1
	display_bg3: 1
	display_obj: 1
	window_0_display: 1
	window_1_display: 1
	obj_window_display: 1
);

/*
4000004h - DISPSTAT - General LCD Status (Read/Write)
Display status and Interrupt control. The H-Blank conditions are generated once per scanline, including for the 'hidden' scanlines during V-Blank.
  Bit   Expl.
  0     V-Blank flag   (Read only) (1=VBlank) (set in line 160..226; not 227)
  1     H-Blank flag   (Read only) (1=HBlank) (toggled in all lines, 0..227)
  2     V-Counter flag (Read only) (1=Match)  (set in selected line)     (R)
  3     V-Blank IRQ Enable         (1=Enable)                          (R/W)
  4     H-Blank IRQ Enable         (1=Enable)                          (R/W)
  5     V-Counter IRQ Enable       (1=Enable)                          (R/W)
  6     Not used (0) / DSi: LCD Initialization Ready (0=Busy, 1=Ready)   (R)
  7     Not used (0) / NDS: MSB of V-Vcount Setting (LYC.Bit8) (0..262)(R/W)
  8-15  V-Count Setting (LYC)      (0..227)                            (R/W)
*/
bit_descriptor!(RegDispStat,
	vblank_flag: 1
	hblank_flag: 1
	vcounter_flag: 1
	vblank_irq_flag: 1
	hblank_irq_flag: 1
	vcounter_irq_flag: 1
	unused: 2
	vcounter_setting: 8
);

/*

4000008h - BG0CNT - BG0 Control (R/W) (BG Modes 0,1 only)
400000Ah - BG1CNT - BG1 Control (R/W) (BG Modes 0,1 only)
400000Ch - BG2CNT - BG2 Control (R/W) (BG Modes 0,1,2 only)
400000Eh - BG3CNT - BG3 Control (R/W) (BG Modes 0,2 only)
  Bit   Expl.
  0-1   BG Priority           (0-3, 0=Highest)
  2-3   Character Base Block  (0-3, in units of 16 KBytes) (=BG Tile Data)
  4-5   Not used (must be zero)
  6     Mosaic                (0=Disable, 1=Enable)
  7     Colors/Palettes       (0=16/16, 1=256/1)
  8-12  Screen Base Block     (0-31, in units of 2 KBytes) (=BG Map Data)
  13    Display Area Overflow (0=Transparent, 1=Wraparound; BG2CNT/BG3CNT only)
  14-15 Screen Size (0-3)
Internal Screen Size (dots) and size of BG Map (bytes):
  Value  Text Mode      Rotation/Scaling Mode
  0      256x256 (2K)   128x128   (256 bytes)
  1      512x256 (4K)   256x256   (1K)
  2      256x512 (4K)   512x512   (4K)
  3      512x512 (8K)   1024x1024 (16K)
*/
bit_descriptor!(RegBGCnt,
	bg_priority: 2
	character_base_block: 2
	unused_0: 2
	mosaic: 1
	palette_256: 1
	screen_base_block: 5
	wraparound: 1
	screen_size: 2
);