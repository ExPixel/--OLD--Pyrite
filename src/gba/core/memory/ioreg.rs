use super::compat::*;
pub use super::*;

#[derive(Copy, Clone)]
pub struct IORegister8(pub usize);

#[derive(Copy, Clone)]
pub struct IORegister16(pub usize);

#[derive(Copy, Clone)]
pub struct IORegister32(pub usize);

// Offsets from 0x4000000

pub const POSTFLG: IORegister8 = IORegister8(0x0000300);
pub const HALTCNT: IORegister8 = IORegister8(0x0000301);
pub const DISPCNT: IORegister16 = IORegister16(0x0000000);
pub const DISPSTAT: IORegister16 = IORegister16(0x0000004);
pub const VCOUNT: IORegister16 = IORegister16(0x0000006);
pub const BG0CNT: IORegister16 = IORegister16(0x0000008);
pub const BG1CNT: IORegister16 = IORegister16(0x000000a);
pub const BG2CNT: IORegister16 = IORegister16(0x000000c);
pub const BG3CNT: IORegister16 = IORegister16(0x000000e);
pub const BG0HOFS: IORegister16 = IORegister16(0x0000010);
pub const BG0VOFS: IORegister16 = IORegister16(0x0000012);
pub const BG1HOFS: IORegister16 = IORegister16(0x0000014);
pub const BG1VOFS: IORegister16 = IORegister16(0x0000016);
pub const BG2HOFS: IORegister16 = IORegister16(0x0000018);
pub const BG2VOFS: IORegister16 = IORegister16(0x000001a);
pub const BG3HOFS: IORegister16 = IORegister16(0x000001c);
pub const BG3VOFS: IORegister16 = IORegister16(0x000001e);
pub const BG2PA: IORegister16 = IORegister16(0x0000020);
pub const BG2PB: IORegister16 = IORegister16(0x0000022);
pub const BG2PC: IORegister16 = IORegister16(0x0000024);
pub const BG2PD: IORegister16 = IORegister16(0x0000026);
pub const BG3PA: IORegister16 = IORegister16(0x0000030);
pub const BG3PB: IORegister16 = IORegister16(0x0000032);
pub const BG3PC: IORegister16 = IORegister16(0x0000034);
pub const BG3PD: IORegister16 = IORegister16(0x0000036);
pub const WIN0H: IORegister16 = IORegister16(0x0000040);
pub const WIN1H: IORegister16 = IORegister16(0x0000042);
pub const WIN0V: IORegister16 = IORegister16(0x0000044);
pub const WIN1V: IORegister16 = IORegister16(0x0000046);
pub const WININ: IORegister16 = IORegister16(0x0000048);
pub const WINOUT: IORegister16 = IORegister16(0x000004a);
pub const MOSAIC: IORegister16 = IORegister16(0x000004c);
pub const BLDCNT: IORegister16 = IORegister16(0x0000050);
pub const BLDALPHA: IORegister16 = IORegister16(0x0000052);
pub const BLDY: IORegister16 = IORegister16(0x0000054);
pub const SOUND1CNT_L: IORegister16 = IORegister16(0x0000060);
pub const SOUND1CNT_H: IORegister16 = IORegister16(0x0000062);
pub const SOUND1CNT_X: IORegister16 = IORegister16(0x0000064);
pub const SOUND2CNT_L: IORegister16 = IORegister16(0x0000068);
pub const SOUND2CNT_H: IORegister16 = IORegister16(0x000006c);
pub const SOUND3CNT_L: IORegister16 = IORegister16(0x0000070);
pub const SOUND3CNT_H: IORegister16 = IORegister16(0x0000072);
pub const SOUND3CNT_X: IORegister16 = IORegister16(0x0000074);
pub const SOUND4CNT_L: IORegister16 = IORegister16(0x0000078);
pub const SOUND4CNT_H: IORegister16 = IORegister16(0x000007c);
pub const SOUNDCNT_L: IORegister16 = IORegister16(0x0000080);
pub const SOUNDCNT_H: IORegister16 = IORegister16(0x0000082);
pub const SOUNDCNT_X: IORegister16 = IORegister16(0x0000084);
pub const SOUNDBIAS: IORegister16 = IORegister16(0x0000088);
pub const WAVE_RAM0_L: IORegister16 = IORegister16(0x0000090);
pub const WAVE_RAM0_H: IORegister16 = IORegister16(0x0000092);
pub const WAVE_RAM1_L: IORegister16 = IORegister16(0x0000094);
pub const WAVE_RAM1_H: IORegister16 = IORegister16(0x0000096);
pub const WAVE_RAM2_L: IORegister16 = IORegister16(0x0000098);
pub const WAVE_RAM2_H: IORegister16 = IORegister16(0x000009a);
pub const WAVE_RAM3_L: IORegister16 = IORegister16(0x000009c);
pub const WAVE_RAM3_H: IORegister16 = IORegister16(0x000009e);
pub const FIF0_A_L: IORegister16 = IORegister16(0x00000a0);
pub const FIFO_A_H: IORegister16 = IORegister16(0x00000a2);
pub const FIFO_B_L: IORegister16 = IORegister16(0x00000a4);
pub const FIFO_B_H: IORegister16 = IORegister16(0x00000a6);
pub const DMA0CNT_L: IORegister16 = IORegister16(0x00000b8);
pub const DMA0CNT_H: IORegister16 = IORegister16(0x00000ba);
pub const DMA1CNT_L: IORegister16 = IORegister16(0x00000c4);
pub const DMA1CNT_H: IORegister16 = IORegister16(0x00000c6);
pub const DMA2CNT_L: IORegister16 = IORegister16(0x00000d0);
pub const DMA2CNT_H: IORegister16 = IORegister16(0x00000d2);
pub const DMA3CNT_L: IORegister16 = IORegister16(0x00000dc);
pub const DMA3CNT_H: IORegister16 = IORegister16(0x00000de);
pub const TM0CNT_L: IORegister16 = IORegister16(0x0000100);
pub const TM0CNT_H: IORegister16 = IORegister16(0x0000102);
pub const TM1CNT_L: IORegister16 = IORegister16(0x0000104);
pub const TM1CNT_H: IORegister16 = IORegister16(0x0000106);
pub const TM2CNT_L: IORegister16 = IORegister16(0x0000108);
pub const TM2CNT_H: IORegister16 = IORegister16(0x000010a);
pub const TM3CNT_L: IORegister16 = IORegister16(0x000010c);
pub const TM3CNT_H: IORegister16 = IORegister16(0x000010e);
pub const SIOMULTI0: IORegister16 = IORegister16(0x0000120);
pub const SIOMULTI1: IORegister16 = IORegister16(0x0000122);
pub const SIOMULTI2: IORegister16 = IORegister16(0x0000124);
pub const SIOMULTI3: IORegister16 = IORegister16(0x0000126);
pub const SIOCNT: IORegister16 = IORegister16(0x0000128);
pub const SIOMLT_SEND: IORegister16 = IORegister16(0x000012a);
pub const KEYINPUT: IORegister16 = IORegister16(0x0000130);
pub const KEYCNT: IORegister16 = IORegister16(0x0000132);
pub const RCNT: IORegister16 = IORegister16(0x0000134);
pub const IR: IORegister16 = IORegister16(0x0000136);
pub const JOYCNT: IORegister16 = IORegister16(0x0000140);
pub const JOY_STAT: IORegister16 = IORegister16(0x0000158);
pub const IE: IORegister16 = IORegister16(0x0000200);
pub const IF: IORegister16 = IORegister16(0x0000202);
pub const WAITCNT: IORegister16 = IORegister16(0x0000204);
pub const IME: IORegister16 = IORegister16(0x0000208);
pub const BG2X: IORegister32 = IORegister32(0x0000028);
pub const BG2Y: IORegister32 = IORegister32(0x000002c);
pub const BG3X: IORegister32 = IORegister32(0x0000038);
pub const BG3Y: IORegister32 = IORegister32(0x000003c);
pub const FIFO_A: IORegister32 = IORegister32(0x00000a0);
pub const FIFO_B: IORegister32 = IORegister32(0x00000a4);
pub const DMA0SAD: IORegister32 = IORegister32(0x00000b0);
pub const DMA0DAD: IORegister32 = IORegister32(0x00000b4);
pub const DMA1SAD: IORegister32 = IORegister32(0x00000bc);
pub const DMA1DAD: IORegister32 = IORegister32(0x00000c0);
pub const DMA2SAD: IORegister32 = IORegister32(0x00000c8);
pub const DMA2DAD: IORegister32 = IORegister32(0x00000cc);
pub const DMA3SAD: IORegister32 = IORegister32(0x00000d4);
pub const DMA3DAD: IORegister32 = IORegister32(0x00000d8);
pub const SIODATA32: IORegister32 = IORegister32(0x0000120);
pub const JOY_RECV: IORegister32 = IORegister32(0x0000150);
pub const JOY_TRANS: IORegister32 = IORegister32(0x0000154);

// #TODO make the mask table.
// pub const IOREG_MASK_TABLE = [];

/// #FIXME: Not sure about the write status of 4000158h
/// Read write permissions table for IO registers.
/// 2 bits in this table represents a R/W pair for a byte in the IO registers.
/// This is a shortened table that doesn't include 0x4000300, 0x4000301 and, 0x4000800
/// and instead just returns R/W for any address not in the table.
pub const IOREG_PERMISSIONS_TABLE: [u32; 33] = [
	0xffff5fff,0xaaaaaaaa,0xaaaaaaaa,0xaaaaaaaa,0xfaffaaaa,0xfffffaaf,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffaaaa,0xaafaaaaa,0xaaaafaaa,0xfaaaaafa,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffffff,0xfffffff5,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
	0x000fffff
];

pub fn is_ioreg_addr_readable(addr: u32) -> bool {
	let ioreg_index = addr - 0x04000000;
	let permissions_index = (ioreg_index >> 4) as usize; // there are 16 registers stuffed into each index.
	if permissions_index > 32 { return true } // If it's not in our table it's just readable.
	let permission_shift = (addr & 15) << 1;
	((IOREG_PERMISSIONS_TABLE[permissions_index] >> permission_shift) & 1) == 1
}

pub fn is_ioreg_addr_writeable(addr: u32) -> bool {
	let ioreg_index = addr - 0x04000000;
	let permissions_index = (ioreg_index >> 4) as usize; // there are 16 registers stuffed into each index.
	if permissions_index > 32 { return true } // If it's not in our table it's just writeable.
	let permission_shift = ((addr & 15) << 1) + 1;
	((IOREG_PERMISSIONS_TABLE[permissions_index] >> permission_shift) & 1) == 1
}

// Old version of the table which contains all of the regiters.
// pub const IOREG_PERMISSIONS_TABLE: [u32; 129] = [
// 	0xffff5fff,0xaaaaaaaa,0xaaaaaaaa,0xaaaaaaaa,0xfaffaaaa,0xfffffaaf,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffaaaa,0xaafaaaaa,0xaaaafaaa,0xfaaaaafa,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xfffffff5,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xfffffffb,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,0xffffffff,
// 	0x000000ff
// ];

macro_rules! put_lo16 {
	($key:expr, $value:expr) => (
		($key & (0xFFFF0000)) | ($value as u32)
	)
}

macro_rules! put_hi16 {
	($key:expr, $value:expr) => (
		($key & (0x0000FFFF)) | (($value as u32) << 16)
	)
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct DMAInternalReg {
	pub reload: bool,
	pub repeat: bool,
	pub transfer_word: bool, // transfers halfwords if false
	pub gamepak_drq: bool,  // #TODO I'm not even sure what this is.
	pub start_timing: u16, // (0=Immediately, 1=VBlank, 2=HBlank, 3=Special)
	pub irq: bool,
	pub enabled: bool,

	pub dest_addr_inc: u32,
	pub source_addr_inc: u32,

	// Everything below here is set and controlled by dma.rs:
	pub is_repeat: bool,
	pub units: u32,
	pub original_destination_addr: u32,
	pub destination_addr: u32,
	pub source_addr: u32,
	pub units_remaining: u32,
	pub first_transfer: bool
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct TimerInternalReg {
	pub prescaler: u32,

	/// When Count-up Timing is enabled, the prescaler value is ignored, 
	/// instead the time is incremented each time when the previous counter overflows. 
	/// This function cannot be used for Timer 0 (as it is the first timer).
	pub count_up: bool,

	pub irq_enabled: bool,
	pub operate: bool,

	// The couter before scaling.
	pub unscaled_counter: u32,
	pub counter: u32,
	pub reload: u32
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct GbaChannel1 {
	pub sweep_time_acc: u32,

	// 4000060h - SOUND1CNT_L (NR10) - Channel 1 Sweep register (R/W)
	pub sweep_shift_number: u16,   // 0-2   R/W  Number of sweep shift      (n=0-7)
	pub sweep_frequency_dec: bool, // 3     R/W  Sweep Frequency Direction  (0=Increase, 1=Decrease)
	pub sweep_time: u16,           // 4-6   R/W  Sweep Time; units of 7.8ms (0-7, min=7.8ms, max=54.7ms)
	// Sweep is disabled by setting Sweep Time to zero, if so, the direction bit should be set.
	// The change of frequency (NR13,NR14) at each shift is calculated by the following formula 
	// where X(0) is initial freq & X(t-1) is last freq:
	//  X(t) = X(t-1) +/- X(t-1)/2^n
	// ---- NOTE ----
	// The documentation is lying, it's not really the frequency but the rate which is 
	// that denominator under 131072 in register SOUND1CNT_X.
	// n is also refers to the sweep shift number above.

	// 4000062h - SOUND1CNT_H (NR11, NR12) - Channel 1 Duty/Len/Envelope (R/W)
	pub sound_length: u16,        // 0-5   W    Sound length; units of (64-n)/256s  (0-63)
	pub wave_pattern_duty: u16,   // 6-7   R/W  Wave Pattern Duty                   (0-3, see below)
	pub envelope_step_time: u16,  // 8-10  R/W  Envelope Step-Time; units of n/64s  (1-7, 0=No Envelope)
	pub envelope_inc: bool,       // 11    R/W  Envelope Direction                  (0=Decrease, 1=Increase)
	pub initial_volume: u16,      // 12-15 R/W  Initial Volume of envelope          (1-15, 0=No Sound)
	// Wave Duty:
	//   0: 12.5% ( -_______-_______-_______ )
	//   1: 25%   ( --______--______--______ )
	//   2: 50%   ( ----____----____----____ ) (normal)
	//   3: 75%   ( ------__------__------__ )
	// The Length value is used only if Bit 6 in NR14 is set.

	/// This is the volume that's actually changing through the
	/// envelope function.
	pub current_volume: u16,
	pub envelope_time_acc: u32,

	pub sound_length_time_acc: u32,

	// 4000064h - SOUND1CNT_X (NR13, NR14) - Channel 1 Frequency/Control (R/W)
	pub frequency: u16,            // 0-10  W    Frequency; 131072/(2048-n)Hz  (0-2047)
	pub length_flag: bool,         // 14    R/W  Length Flag  (1=Stop output when length in NR11 expires)
	pub initial: bool,             // 15    W    Initial      (1=Restart Sound)

	pub frequency_f: f32,
	pub frequency_step: f32,

	pub playing: bool
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct GbaChannel2 {
	// 4000062h - SOUND1CNT_H (NR11, NR12) - Channel 1 Duty/Len/Envelope (R/W)
	pub sound_length: u16,        // 0-5   W    Sound length; units of (64-n)/256s  (0-63)
	pub wave_pattern_duty: u16,   // 6-7   R/W  Wave Pattern Duty                   (0-3, see below)
	pub envelope_step_time: u16,  // 8-10  R/W  Envelope Step-Time; units of n/64s  (1-7, 0=No Envelope)
	pub envelope_inc: bool,       // 11    R/W  Envelope Direction                  (0=Decrease, 1=Increase)
	pub initial_volume: u16,      // 12-15 R/W  Initial Volume of envelope          (1-15, 0=No Sound)
	// Wave Duty:
	//   0: 12.5% ( -_______-_______-_______ )
	//   1: 25%   ( --______--______--______ )
	//   2: 50%   ( ----____----____----____ ) (normal)
	//   3: 75%   ( ------__------__------__ )
	// The Length value is used only if Bit 6 in NR14 is set.

	/// This is the volume that's actually changing through the
	/// envelope function.
	pub current_volume: u16,
	pub envelope_time_acc: u32,

	pub sound_length_time_acc: u32,

	// 4000064h - SOUND1CNT_X (NR13, NR14) - Channel 1 Frequency/Control (R/W)
	pub frequency: u16,            // 0-10  W    Frequency; 131072/(2048-n)Hz  (0-2047)
	pub length_flag: bool,         // 14    R/W  Length Flag  (1=Stop output when length in NR11 expires)
	pub initial: bool,             // 15    W    Initial      (1=Restart Sound)

	pub frequency_f: f32,
	pub frequency_step: f32,

	pub playing: bool
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct GbaChannel3 {
	// 4000070h - SOUND3CNT_L (NR30) - Channel 3 Stop/Wave RAM select (R/W)
	pub wav_ram_banked: bool, // 5     R/W  Wave RAM Dimension   (0=One bank/32 digits, 1=Two banks/64 digits)
	pub wav_ram_bank: u16,    // 6     R/W  Wave RAM Bank Number (0-1, see below)
	pub channel_on: bool,     // 7     R/W  Sound Channel 3 Off  (0=Stop, 1=Playback)
	// The currently selected Bank Number (Bit 6) will be played back, while reading/writing to/from wave RAM 
	// will address the other (not selected) bank.
	// When dimension is set to two banks, output will start by replaying the currently selected bank.

	// 4000072h - SOUND3CNT_H (NR31, NR32) - Channel 3 Length/Volume (R/W)
	pub sound_length: u16,    // 0-7   W    Sound length; units of (256-n)/256s  (0-255)
	pub sound_volume: u16,    // 13-14 R/W  Sound Volume  (0=Mute/Zero, 1=100%, 2=50%, 3=25%)
	pub force_volume: bool,   // 15    R/W  Force Volume  (0=Use above, 1=Force 75% regardless of above)
	// The Length value is used only if Bit 6 in NR34 is set.

	// 4000074h - SOUND3CNT_X (NR33, NR34) - Channel 3 Frequency/Control (R/W)
	pub sample_rate: u16,     // 0-10  W    Sample Rate; 2097152/(2048-n) Hz   (0-2047)
	pub length_flag: bool,    // 14    R/W  Length Flag  (1=Stop output when length in NR31 expires)
	pub initial: bool,        // 15    W    Initial      (1=Restart Sound)
	// The above sample rate specifies the number of wave RAM digits per second, the actual tone frequency depends on the wave RAM content, for example:
	//   Wave RAM, single bank 32 digits   Tone Frequency
	//   FFFFFFFFFFFFFFFF0000000000000000  65536/(2048-n) Hz
	//   FFFFFFFF00000000FFFFFFFF00000000  131072/(2048-n) Hz
	//   FFFF0000FFFF0000FFFF0000FFFF0000  262144/(2048-n) Hz
	//   FF00FF00FF00FF00FF00FF00FF00FF00  524288/(2048-n) Hz
	//   F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0  1048576/(2048-n) Hz

	pub wav_ram: [[u16; 8]; 2],
	pub sound_length_time_acc: u32,
	pub current_wav_index: usize,

	pub freq_inc: f32,
	pub freq_acc: f32,

	pub playing: bool
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct GbaChannel4 {
	// 4000078h - SOUND4CNT_L (NR41, NR42) - Channel 4 Length/Envelope (R/W)
	pub sound_length: u16,         // 0-5   W    Sound length; units of (64-n)/256s  (0-63)
	pub envelope_step_time: u16,   // 8-10  R/W  Envelope Step-Time; units of n/64s  (1-7, 0=No Envelope)
	pub envelope_inc: bool,        // 11    R/W  Envelope Direction                  (0=Decrease, 1=Increase)
	pub initial_volume: u16,       // 12-15 R/W  Initial Volume of envelope          (1-15, 0=No Sound)

	// 400007Ch - SOUND4CNT_H (NR43, NR44) - Channel 4 Frequency/Control (R/W)
	pub dividing_ratio: u16,       // 0-2   R/W  Dividing Ratio of Frequencies (r)
	pub counter_width_7: bool,     // 3     R/W  Counter Step/Width (0=15 bits, 1=7 bits)
	pub shift_clock_freq: u16,     // 4-7   R/W  Shift Clock Frequency (s)
	pub length_flag: bool,         // 14    R/W  Length Flag  (1=Stop output when length in NR41 expires)
	pub initial: bool,             // 15    W    Initial      (1=Restart Sound)

	pub current_volume: u16,
	pub envelope_time_acc: u32,
	pub sound_length_time_acc: u32,

	// Noise Random Generator (aka Polynomial Counter)
	// Noise randomly switches between HIGH and LOW levels, the output levels are calculated by a shift register (X), at the selected frequency, as such:
	//   7bit:  X=X SHR 1, IF carry THEN Out=HIGH, X=X XOR 60h ELSE Out=LOW
	//   15bit: X=X SHR 1, IF carry THEN Out=HIGH, X=X XOR 6000h ELSE Out=LOW
	// The initial value when (re-)starting the sound is X=40h (7bit) or X=4000h (15bit). The data stream repeats after 7Fh (7bit) or 7FFFh (15bit) steps.
	pub lfsr: u16,
	pub lfsr_mask: u16,
	pub lfsr_xor: u16,

	pub freq_inc: f32,
	pub freq_acc: f32,
	pub intermediate_freq: f32,

	pub playing: bool,
}

#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct GbaChannelFIFO {
	pub timer: u16,
	pub enable_right: bool,
	pub enable_left: bool,

	pub sample: i8,

	pub frequency: f32,
	pub freq_acc: f32,
	pub freq_inc: f32,

	// #TODO do I really want to store this on save?
	out_data: FifoOutArray,
	out_write_cursor: usize,
	out_read_cursor: usize,
	out_size: usize,

	data: [i8; 32],
	write_cursor: usize,
	read_cursor: usize,

	size: usize
}

impl GbaChannelFIFO {
	pub fn reset(&mut self) {
		// println!("RESET");
		self.out_write_cursor = 0;
		self.out_read_cursor = 0;
		self.out_size = 0;
		self.write_cursor = 0;
		self.read_cursor = 0;
		self.size = 0;
		self.freq_acc = 0.0;
	}

	pub fn out_push(&mut self, sample: i8) {
		// if sample != 0 {
		// println!("OUT <- {} [idx: {}]", sample, self.out_write_cursor);
		// }
		self.out_data.data[self.out_write_cursor] = sample;
		self.out_write_cursor = (self.out_write_cursor + 1) & FIFO_OUT_BUFFER_MASK;
		self.out_size = min!(self.out_size + 1, FIFO_OUT_BUFFER_SIZE);
		if self.out_write_cursor == self.out_read_cursor {
			// If the write cursor "laps" the read cursor, just move the read
			// cursor forward so that it will then be at the new 0 position.
			self.out_read_cursor = (self.out_write_cursor + 1) & FIFO_OUT_BUFFER_MASK;
		}
	}

	pub fn out_pop(&mut self) -> i8 {
		if self.out_remaining() > 0 {
			let sample = self.out_data.data[self.out_read_cursor];
			// println!("OUT -> {} [idx: {}]", sample, self.out_read_cursor);
			self.out_read_cursor = (self.out_read_cursor + 1) & FIFO_OUT_BUFFER_MASK;
			self.out_size -= 1;
			return sample;
		} else {
			// println!("OUT -> NULL [{}]", self.size);
			return 0;
		}
	}

	pub fn next_sample(&mut self) {
		let sample = self.out_pop();
		self.sample = sample;
	}

	pub fn out_remaining(&self) -> usize {
		return self.out_size
	}

	/// Pushes two signed 8 bit samples into the FIFO queue.
	pub fn push16(&mut self, sample2: u16) {
		self.push((sample2 & 0xff) as i8);
		self.push(((sample2 >> 8) & 0xff) as i8);
	}

	// #TODO there is no need for this push function only to have it
	//       only ever be called twice in push16, push16 is the only one
	//       that's needed so I can just change that to only write in 2's.
	/// Pushes a single signed 8bit sample into the FIFO queue.
	pub fn push(&mut self, sample: i8) {
		let debugger = ::debug::debugger::get_debugger();
		debugger.fifo_a_in.plot(sample as f32);
		self.data[self.write_cursor] = sample;
		self.write_cursor = (self.write_cursor + 1) & 0x1f;
		self.size = min!(self.size + 1, 32);

		// if sample != 0 {
		// println!("BUF <- {} [{}]", sample, self.size);
		// }

		if self.write_cursor == self.read_cursor {
			// If the write cursor "laps" the read cursor, just move the read
			// cursor forward so that it will then be at the new 0 position.
			self.read_cursor = (self.write_cursor + 1) & 0x1f;
		}
	}

	/// Pops an 8 bit sample from the FIFO queue. 
	pub fn pop(&mut self) -> i8 {
		let debugger = ::debug::debugger::get_debugger();
		if self.remaining() > 0 {
			let sample = self.data[self.read_cursor];
			self.read_cursor = (self.read_cursor + 1) & 0x1f;
			self.size -= 1;
			// println!("BUF -> {}", sample);
			debugger.fifo_a_out.plot(sample as f32);
			return sample;
		} else {
			// println!("BUF -> NULL [{}]", self.size);
			debugger.fifo_a_out.plot(0.0);
			return 0;
		}
	}

	/// Returns the number of samples remaining in the queue.
	pub fn remaining(&self) -> usize {
		return self.size
	}
}

// Internal IO registers.
#[derive(Default, RustcEncodable, RustcDecodable)]
pub struct InternalRegisters {
	pub bg2x: u32,
	pub bg2y: u32,
	pub bg3x: u32,
	pub bg3y: u32,

	pub halted: bool,
	pub stopped: bool,

	pub dma_dirty: bool,
	pub dma_registers: [DMAInternalReg; 4],
	pub timers: [TimerInternalReg; 4],

	pub audio_channel1: GbaChannel1,
	pub audio_channel2: GbaChannel2,
	pub audio_channel3: GbaChannel3,
	pub audio_channel4: GbaChannel4,
	pub audio_fifo_a: GbaChannelFIFO,
	pub audio_fifo_b: GbaChannelFIFO,
}

impl InternalRegisters {
	pub fn new() -> InternalRegisters {
		Default::default()
	}

	// This should almost never happen.
	pub fn on_write8(&mut self, address: u32, value: u8, iodata: &[u8]) {
		let register = address & 0x3fe;
		self.on_reg_write8(address, value);
		let value16 = if (address & 1) == 1 {
			(iodata.direct_read16(register as usize) & 0xFF) | ((value as u16) << 8)
		} else {
			(iodata.direct_read16(register as usize) & 0xFF00) | ((value as u16))
		};
		self.on_reg_write(register, value16);
	}

	pub fn on_write16(&mut self, address: u32, value: u16) {
		self.on_reg_write(address & 0x3ff, value);
	}

	pub fn on_write32(&mut self, address: u32, value: u32) {
		self.on_reg_write(address & 0x3ff, (value & 0xFFFF) as u16);
		self.on_reg_write((address & 0x3ff) + 2, ((value >> 16) & 0xFFFF) as u16);
	}

	pub fn on_frame_end(&mut self, iodata: &[u8]) {
		// refresh BG2X, BG2Y, BG3X, BG3Y
		self.on_reg_write(0x0000028, iodata.direct_read16(0x0000028));
		self.on_reg_write(0x000002A, iodata.direct_read16(0x000002A));
		self.on_reg_write(0x000002C, iodata.direct_read16(0x000002C));
		self.on_reg_write(0x000002E, iodata.direct_read16(0x000002E));
		self.on_reg_write(0x0000038, iodata.direct_read16(0x0000038));
		self.on_reg_write(0x000003A, iodata.direct_read16(0x000003A));
		self.on_reg_write(0x000003C, iodata.direct_read16(0x000003C));
		self.on_reg_write(0x000003E, iodata.direct_read16(0x000003E));
	}

	pub fn on_reg_write8(&mut self, register: u32, value: u8) {
		match register {
			0x4000301 => {
				let low_power_bit = (value >> 7) & 1;
				if low_power_bit == 1 {
					self.stopped = true;
					self.halted = false;
					// println!(CPU NOW IN STOPPED MODE.); // #TODO remove testing code.
				} else {
					self.halted = true;
					self.stopped = false;
					// println!(CPU NOW IN HALT MODE.); // #TODO remove testing code.
				}
			},
			_ => {}
		}
	}

	pub fn on_reg_write(&mut self, register: u32, value: u16) {
		match register {
			// Gfx:
			0x00000028 => { self.bg2x = (((put_lo16!(self.bg2x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x0000002A => { self.bg2x = (((put_hi16!(self.bg2x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x0000002C => { self.bg2y = (((put_lo16!(self.bg2y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x0000002E => { self.bg2y = (((put_hi16!(self.bg2y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x00000038 => { self.bg3x = (((put_lo16!(self.bg3x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x0000003A => { self.bg3x = (((put_hi16!(self.bg3x, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			0x0000003C => { self.bg3y = (((put_lo16!(self.bg3y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits
			0x0000003E => { self.bg3y = (((put_hi16!(self.bg3y, value) << 4) as i32) >> 4) as u32 }, // sign extension from 28bits to 32bits

			// Timers:
			0x00000100 => { self.update_timer_lo(0, value) },
			0x00000102 => { self.update_timer_hi(0, value) },
			0x00000104 => { self.update_timer_lo(1, value) },
			0x00000106 => { self.update_timer_hi(1, value) },
			0x00000108 => { self.update_timer_lo(2, value) },
			0x0000010A => { self.update_timer_hi(2, value) },
			0x0000010C => { self.update_timer_lo(3, value) },
			0x0000010E => { self.update_timer_hi(3, value) },

			// DMA:
			0x000000BA => { self.update_dma_hi(0, value); },
			0x000000C6 => { self.update_dma_hi(1, value); },
			0x000000D2 => { self.update_dma_hi(2, value); },
			0x000000DE => { self.update_dma_hi(3, value); },

			// Audio Channel 1:
			0x00000060 => {
				self.audio_channel1.sweep_shift_number = value & 0x7;
				self.audio_channel1.sweep_frequency_dec = (value & 0x8) != 0;
				self.audio_channel1.sweep_time = (value >> 4) & 0x7;
			},
			0x00000062 => {
				self.audio_channel1.sound_length = value & 0x3f;
				self.audio_channel1.wave_pattern_duty = (value >> 6) & 0x3;
				self.audio_channel1.envelope_step_time = (value >> 8) & 0x7;
				self.audio_channel1.envelope_inc = (value & 0x800) != 0;
				self.audio_channel1.initial_volume = (value >> 12) & 0xf;
			},
			0x00000064 => {
				self.audio_channel1.frequency = value & 0x7ff;
				self.audio_channel1.frequency_f = 131072.0 / (2048.0 - self.audio_channel1.frequency as f32);
				self.audio_channel1.length_flag = (value & 0x4000) != 0;
				self.audio_channel1.initial = (value & 0x8000) != 0;
			},

			// Audio Channel 2:
			0x0000068 => {
				self.audio_channel2.sound_length = value & 0x3f;
				self.audio_channel2.wave_pattern_duty = (value >> 6) & 0x3;
				self.audio_channel2.envelope_step_time = (value >> 8) & 0x7;
				self.audio_channel2.envelope_inc = (value & 0x800) != 0;
				self.audio_channel2.initial_volume = (value >> 12) & 0xf;
			},
			0x000006C => {
				self.audio_channel2.frequency = value & 0x7ff;
				self.audio_channel2.frequency_f = 131072.0 / (2048.0 - self.audio_channel2.frequency as f32);
				self.audio_channel2.length_flag = (value & 0x4000) != 0;
				self.audio_channel2.initial = (value & 0x8000) != 0;
			},

			// Audio Channel 3:
			0x00000070 => {
				self.audio_channel3.wav_ram_banked = (value & 0x20) != 0;
				self.audio_channel3.wav_ram_bank = (value >> 6) & 1;
				self.audio_channel3.channel_on = (value & 0x80) != 0;
			},
			0x00000072 => {
				self.audio_channel3.sound_length = value & 0xff;
				self.audio_channel3.sound_volume = (value >> 13) & 0x3;
				self.audio_channel3.force_volume = (value & 0x8000) != 0;

				// println!("WRITE(72): {:04X} -> [LENGTH: {}]", value, self.audio_channel3.sound_length);
			},
			0x00000074 => {
				self.audio_channel3.sample_rate = value & 0x7ff;
				self.audio_channel3.length_flag = (value & 0x4000) != 0;
				self.audio_channel3.initial = (value & 0x8000) != 0;

				// println!("WRITE(74): {:04X} -> [SAMPLE RATE: {}] [LENGTH FLAG: {}] [INITIAL: {}]",
				// 	value,
				// 	self.audio_channel3.sample_rate,
				// 	self.audio_channel3.length_flag,
				// 	self.audio_channel3.initial);
			},
			0x00000090 ... 0x0000009E => { // Writing to Wave RAM
				let bank = (self.audio_channel3.wav_ram_bank ^ 1) as usize;
				self.audio_channel3.wav_ram[bank][((register - 0x00000090) >> 1) as usize] = value;
			},

			// Audio Channel 4:
			0x00000078 => {
				// println!("MODIFIED CHANNEL 4(78): {:04X}", value);
				self.audio_channel4.sound_length = value & 0x3f;
				self.audio_channel4.envelope_step_time = (value >> 8) & 0x7;
				self.audio_channel4.envelope_inc = (value & 0x800) != 0;
				self.audio_channel4.initial_volume = (value >> 12) & 0xf;
			}
			0x0000007C => {
				// println!("MODIFIED CHANNEL 4(7C): {:04X}", value);
				self.audio_channel4.dividing_ratio = value & 0x7;
				self.audio_channel4.counter_width_7 = (value & 0x8) != 0;
				self.audio_channel4.shift_clock_freq = (value >> 4) & 0xf;
				self.audio_channel4.length_flag = (value & 0x4000) != 0;
				self.audio_channel4.initial = (value & 0x8000) != 0;
			},

			// FIFO A:
			0x000000A0 | 0x000000A2 => {
				self.audio_fifo_a.push16(value);
				// if value != 0 {
				// 	println!("FIFO A PUSH[{:08X}h] = 0x{:04X} ({}, {})", register, value,
				// 		(value & 0xff) as i8, ((value >> 8) & 0xff) as i8);
				// }
			},

			// FIFO B:
			0x000000A4 | 0x000000A6 => {
				// self.audio_fifo_b.push16(value);
			},

			// SOUNDCNT_H - DMA Sound Control/Mixing (R/W)
			0x00000082 => {
				self.audio_fifo_a.enable_right = (value & 0x100) != 0;
				self.audio_fifo_a.enable_left = (value & 0x200) != 0;
				self.audio_fifo_a.timer = (value >> 10) & 1;
				if (value & 0x800) != 0 {
					self.audio_fifo_a.reset();	
				}
				self.update_fifo_a_frequency(((value >> 10) & 1) as usize);

				self.audio_fifo_a.enable_right = (value & 0x1000) != 0;
				self.audio_fifo_a.enable_left = (value & 0x2000) != 0;
				self.audio_fifo_a.timer = (value >> 14) & 1;
				if (value & 0x8000) != 0 {
					self.audio_fifo_a.reset();	
				}
				self.update_fifo_b_frequency(((value >> 14) & 1) as usize);
			},

			// #TODO when bit 7 of 4000084h - SOUNDCNT_X (NR52) is cleared,
			// all of the sound registers are supposed to be reset to 0.
			// I haven't been doing this and it might cause some issues
			// down the road. Something to keep in mind.

			_ => {}
		}
	}

	fn update_dma_hi(&mut self, dma_index: usize, dma_hi_data: u16) {
		let internal_reg = &mut self.dma_registers[dma_index];
		internal_reg.repeat = ((dma_hi_data >> 9) & 1) == 1;
		internal_reg.transfer_word = ((dma_hi_data >> 10) & 1) == 1;
		internal_reg.gamepak_drq = ((dma_hi_data >> 11) & 1) == 1;
		internal_reg.start_timing = (dma_hi_data >> 12) & 0x3;
		internal_reg.irq = ((dma_hi_data >> 14) & 1) == 1;
		internal_reg.enabled = ((dma_hi_data >> 15) & 1) == 1;
		internal_reg.reload = ((dma_hi_data >> 5) & 0x3) == 3;
		
		internal_reg.is_repeat = false;

		if !internal_reg.enabled {
			// if a DMA is suddenly stopped, we don't want there to be any units waiting to be transferred.
			internal_reg.units_remaining = 0;
		}

		let size: i32 = if internal_reg.transfer_word {4} else {2};

		internal_reg.dest_addr_inc = match (dma_hi_data >> 5) & 0x3 {
			0 | 3  => size as u32,
			1 => (size * -1) as u32,
			2 => 0u32,
			_ => unreachable!()
		};

		internal_reg.source_addr_inc = match (dma_hi_data >> 7) & 0x3 {
			0 | 3 => size as u32, // #FIXME 3 is actually not supported for the source_addr_inc. Maybe it should disable the DMA?
			1 => (size * -1) as u32,
			2 => 0u32,
			_ => unreachable!()
		};

		self.dma_dirty = true;
	}

	fn update_timer_lo(&mut self, t_idx: usize, lo_data: u16) {
		let timer = &mut self.timers[t_idx];
		timer.reload = lo_data as u32;
	}

	fn update_timer_hi(&mut self, t_idx: usize, hi_data: u16) {
		{
			let timer = &mut self.timers[t_idx];
			timer.prescaler = match hi_data & 0x3 {
				0 => 0,  // 1
				1 => 6,  // 64
				2 => 8,  // 256
				3 => 10, // 1024
				_ => unreachable!()
			};

			timer.count_up = ((hi_data >> 2) & 1) == 1;
			timer.irq_enabled = ((hi_data >> 6) & 1) == 1;
			timer.operate = ((hi_data >> 7) & 1) == 1;

			// #FIXME not sure if this is suppose to happen if we enable
			//        an already enabled timer.
			timer.counter = timer.reload;
			timer.unscaled_counter = 0;
		}

		if t_idx == self.audio_fifo_a.timer as usize {
			self.update_fifo_a_frequency(t_idx);
		}

		if t_idx == self.audio_fifo_b.timer as usize {
			self.update_fifo_b_frequency(t_idx);
		}
	}

	fn update_fifo_a_frequency(&mut self, timer: usize) {
		self.audio_fifo_a.frequency = self.get_timer_frequency(timer);
	}

	fn update_fifo_b_frequency(&mut self, timer: usize) {
		self.audio_fifo_b.frequency = self.get_timer_frequency(timer);
	}

	fn get_timer_frequency(&mut self, timer: usize) -> f32 {
		((16777216 >> self.timers[timer].prescaler) as f32) / max!(1.0, 65535.0 - self.timers[timer].reload as f32)
	}

	// pub fn increment_timers(&mut self, amt: u32) -> u16 {
	// 	let mut overflow_int_mask = 0;
	// 	let mut last_timer_overflowed = false;
	// 	for t_idx in 0..4 {
	// 		let timer = &mut self.timers[t_idx];
	// 		last_timer_overflowed = Self::increment_single_timer(timer, amt, last_timer_overflowed);
	// 		if last_timer_overflowed && timer.irq_enabled {
	// 			overflow_int_mask |= 0x08 << t_idx;
	// 		}
	// 	}
	// 	return overflow_int_mask;
	// }

	// fn increment_single_timer(timer: &mut TimerInternalReg, amt: u32, previous_overflowed: bool) -> bool {
	// 	if timer.operate {
	// 		if timer.count_up {
	// 			if previous_overflowed {
	// 				timer.counter += 1;
	// 			}
	// 		} else {
	// 			timer.unscaled_counter += amt;
	// 			let scaled = timer.unscaled_counter >> timer.prescaler;
	// 			timer.counter += scaled;
	// 			timer.unscaled_counter -= scaled << timer.prescaler;
	// 		}

	// 		if timer.counter > 0xffff { // Ther timer has overflowed
	// 			timer.counter = timer.reload;
	// 			timer.unscaled_counter = 0;
	// 			return true
	// 		}
	// 	}
	// 	return false
	// }
}
