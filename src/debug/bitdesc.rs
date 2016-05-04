pub trait BitDescriptor {
	fn property_count(&self) -> usize;
	fn get_property_name(&self, index: usize) -> &'static str;
	fn get_property_value(&self, index: usize) -> u32;
}

macro_rules! bit_descriptor {
	($desc_name: ident, $($name:ident, $length:expr)+) => (
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
	bg_mode, 3
	cgb_mode, 1
	frame_select, 1
	hblank_interval_free, 1
	vram_obj_mapping, 1
	forced_blank, 1
	display_bg0, 1
	display_bg1, 1
	display_bg2, 1
	display_bg3, 1
	display_obj, 1
	window_0_display, 1
	window_1_display, 1
	obj_window_display, 1
);