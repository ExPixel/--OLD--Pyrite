// An almost direct translation of VBA's armdis.cpp to Rust.
// Slightly modified to be more rusty (just expecting to do this might not have happened)
// and for more functionality (like resolving GBA Swi instructions).
use ::gba::core::memory::GbaMemory;

pub struct Opcodes {
	pub mask: u32,
	pub cval: u32,
	pub mnemonic: &'static str
}

pub const DIS_WRITE_ADDRESS: u32 = 0b00000001;
pub const DIS_WRITE_CODE: u32 = 0b00000010;
pub const DIS_WRITE_GBA_SWI: u32 = 0b00000100;
#[allow(dead_code)] 
pub const DIS_WRITE_NONE: u32 = 0b00000000;
#[allow(dead_code)] 
pub const DIS_WRITE_ALL: u32 = DIS_WRITE_ADDRESS
							| DIS_WRITE_CODE
							| DIS_WRITE_GBA_SWI;

static SWI: [&'static str; 43] = [
	"SoftReset", "RegisterRamReset",
	"Halt", "Stop/Sleep", "IntrWait",
	"VBlankIntrWait", "Div", "DivArm",
	"Sqrt", "ArcTan", "ArcTan2", "CpuSet",
	"CpuFastSet", "GetBiosChecksum", "BgAffineSet",
	"ObjAffineSet", "BitUnPack", "LZ77UnCompReadNormalWrite8bit",
	"LZ77UnCompReadNormalWrite16bit", "HuffUnCompReadNormal",
	"RLUnCompReadNormalWrite8bit", "RLUnCompReadNormalWrite16bit",
	"Diff8bitUnFilterWrite8bit", "Diff8bitUnFilterWrite16bit",
	"Diff16bitUnFilter", "SoundBias", "SoundDriverInit", "SoundDriverMode",
	"SoundDriverMain", "SoundDriverVSync", "SoundChannelClear",
	"MidiKey2Freq", "SoundWhatever0", "SoundWhatever1", "SoundWhatever2",
	"SoundWhatever3", "SoundWhatever4", "MultiBoot", "HardReset", "CustomHalt", 
	"SoundDriverVSyncOff", "SoundDriverVSyncOn", "SoundGetJumpList"
];

/*
  GBA  NDS7 NDS9 DSi7 DSi9 Sound (and Multiboot/HardReset/CustomHalt)
  1Fh  -    -    -    -    MidiKey2Freq
  20h  -    -    -    -    SoundWhatever0

  28h  -    -    -    -    SoundDriverVSyncOff
  29h  -    -    -    -    SoundDriverVSyncOn
  2Ah  -    -    -    -    SoundGetJumpList
  GBA  NDS7 NDS9 DSi7 DSi9 New NDS Functions
  -    03h  03h  03h  03h  WaitByLoop
  -    0Eh  0Eh  0Eh  0Eh  GetCRC16
  -    0Fh  0Fh  -    -    IsDebugger
  -    1Ah  -    1Ah  -    GetSineTable
  -    1Bh  -    1Bh  -    GetPitchTable (DSi7: bugged)
  -    1Ch  -    1Ch  -    GetVolumeTable
  -    1Dh  -    1Dh  -    GetBootProcs (DSi7: only 1 proc)
  -    -    1Fh  -    1Fh  CustomPost
  GBA  NDS7 NDS9 DSi7 DSi9 New DSi Functions (RSA/SHA1)
  -    -    -    20h  20h  RSA_Init_crypto_heap
  -    -    -    21h  21h  RSA_Decrypt
  -    -    -    22h  22h  RSA_Decrypt_Unpad
  -    -    -    23h  23h  RSA_Decrypt_Unpad_GetChunk04
  -    -    -    24h  24h  SHA1_Init
  -    -    -    25h  25h  SHA1_Update
  -    -    -    26h  26h  SHA1_Finish
  -    -    -    27h  27h  SHA1_Init_update_fin
  -    -    -    28h  28h  SHA1_Compare_20_bytes
  -    -    -    29h  29h  SHA1_Random_maybe
  GBA  NDS7 NDS9 DSi7 DSi9 Invalid Functions
  2Bh+ 20h+ 20h+ -    -    Crash (SWI xxh..FFh do jump to garbage addresses)
  -    xxh  xxh  -    -    Jump to 0   (on any SWI numbers not listed above)
  -    -    -    12h  12h  No function (ignored)
  -    -    -    2Bh  2Bh  No function (ignored)
  -    -    -    40h+ 40h+ Mirror      (SWI 40h..FFh mirror to 00h..3Fh)
  -    -    -    xxh  xxh  Hang        (on any SWI numbers not listed above)

*/

static HDIG: [char; 16] = [
	'0', '1', '2', '3', '4', '5', '6', '7',
	'8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
];

pub static REGS: [&'static str; 16] = [
	"r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7",
	"r8", "r9", "r10", "r11", "r12", "sp",
	"lr", "pc"
];

static CONDITIONS: [&'static str; 16] = [
	"eq", "ne", "cs", "cc", "mi", "pl", "vs", "vc",
	"hi", "ls", "ge", "lt", "gt", "le", "", "nv"
];

static SHIFTS: [&'static str; 5] = [
	"lsl", "lsr", "asr", "ror", "rrx"
];

static ARM_MULT_LOAD_STORE: [&'static str; 12] = [
	// non-stack
	"da", "ia", "db", "ib",
	// stack store
	"ed", "ea", "fd", "fa",
	// stack load
	"fa", "fd", "ea", "ed"
];

static THUMB_OPCODES: [Opcodes; 63] = [
	// Format 1
	Opcodes { mask: 0xf800, cval: 0x0000, mnemonic: "lsl %r0, %r3, %o"},
	Opcodes { mask: 0xf800, cval: 0x0800, mnemonic: "lsr %r0, %r3, %o"},
	Opcodes { mask: 0xf800, cval: 0x1000, mnemonic: "asr %r0, %r3, %o"},
	// Format 2
	Opcodes { mask: 0xfe00, cval: 0x1800, mnemonic: "add %r0, %r3, %r6"},
	Opcodes { mask: 0xfe00, cval: 0x1a00, mnemonic: "sub %r0, %r3, %r6"},
	Opcodes { mask: 0xfe00, cval: 0x1c00, mnemonic: "add %r0, %r3, %i"},
	Opcodes { mask: 0xfe00, cval: 0x1e00, mnemonic: "sub %r0, %r3, %i"},
	// Format 3
	Opcodes { mask: 0xf800, cval: 0x2000, mnemonic: "mov %r8, %O"},
	Opcodes { mask: 0xf800, cval: 0x2800, mnemonic: "cmp %r8, %O"},
	Opcodes { mask: 0xf800, cval: 0x3000, mnemonic: "add %r8, %O"},
	Opcodes { mask: 0xf800, cval: 0x3800, mnemonic: "sub %r8, %O"},
	// Format 4
	Opcodes { mask: 0xffc0, cval: 0x4000, mnemonic: "and %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4040, mnemonic: "eor %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4080, mnemonic: "lsl %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x40c0, mnemonic: "lsr %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4100, mnemonic: "asr %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4140, mnemonic: "adc %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4180, mnemonic: "sbc %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x41c0, mnemonic: "ror %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4200, mnemonic: "tst %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4240, mnemonic: "neg %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4280, mnemonic: "cmp %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x42c0, mnemonic: "cmn %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4300, mnemonic: "orr %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4340, mnemonic: "mul %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x4380, mnemonic: "bic %r0, %r3"},
	Opcodes { mask: 0xffc0, cval: 0x43c0, mnemonic: "mvn %r0, %r3"},
	// Format 5
	Opcodes { mask: 0xff80, cval: 0x4700, mnemonic: "bx %h36"},
	Opcodes { mask: 0xfcc0, cval: 0x4400, mnemonic: "[ ??? ]"},
	Opcodes { mask: 0xff00, cval: 0x4400, mnemonic: "add %h07, %h36"},
	Opcodes { mask: 0xff00, cval: 0x4500, mnemonic: "cmp %h07, %h36"},
	Opcodes { mask: 0xff00, cval: 0x4600, mnemonic: "mov %h07, %h36"},
	// Format 6
	Opcodes { mask: 0xf800, cval: 0x4800, mnemonic: "ldr %r8, [%I] (=%J)"},
	// Format 7
	Opcodes { mask: 0xfa00, cval: 0x5000, mnemonic: "str%b %r0, [%r3, %r6]"},
	Opcodes { mask: 0xfa00, cval: 0x5800, mnemonic: "ldr%b %r0, [%r3, %r6]"},
	// Format 8
	Opcodes { mask: 0xfe00, cval: 0x5200, mnemonic: "strh %r0, [%r3, %r6]"},
	Opcodes { mask: 0xfe00, cval: 0x5600, mnemonic: "ldsb %r0, [%r3, %r6]"},
	Opcodes { mask: 0xfe00, cval: 0x5a00, mnemonic: "ldrh %r0, [%r3, %r6]"},
	Opcodes { mask: 0xfe00, cval: 0x5e00, mnemonic: "ldsh %r0, [%r3, %r6]"},
	// Format 9
	Opcodes { mask: 0xe800, cval: 0x6000, mnemonic: "str%B %r0, [%r3, %p]"},
	Opcodes { mask: 0xe800, cval: 0x6800, mnemonic: "ldr%B %r0, [%r3, %p]"},
	// Format 10
	Opcodes { mask: 0xf800, cval: 0x8000, mnemonic: "strh %r0, [%r3, %e]"},
	Opcodes { mask: 0xf800, cval: 0x8800, mnemonic: "ldrh %r0, [%r3, %e]"},
	// Format 11
	Opcodes { mask: 0xf800, cval: 0x9000, mnemonic: "str %r8, [sp, %w]"},
	Opcodes { mask: 0xf800, cval: 0x9800, mnemonic: "ldr %r8, [sp, %w]"},
	// Format 12
	Opcodes { mask: 0xf800, cval: 0xa000, mnemonic: "add %r8, pc, %w (=%K)"},
	Opcodes { mask: 0xf800, cval: 0xa800, mnemonic: "add %r8, sp, %w"},
	// Format 13
	Opcodes { mask: 0xff00, cval: 0xb000, mnemonic: "add sp, %s"},
	// Format 14
	Opcodes { mask: 0xffff, cval: 0xb500, mnemonic: "push {lr}"},  
	Opcodes { mask: 0xff00, cval: 0xb400, mnemonic: "push {%l}"},
	Opcodes { mask: 0xff00, cval: 0xb500, mnemonic: "push {%l,lr}"},
	Opcodes { mask: 0xffff, cval: 0xbd00, mnemonic: "pop {pc}"},
	Opcodes { mask: 0xff00, cval: 0xbd00, mnemonic: "pop {%l,pc}"},  
	Opcodes { mask: 0xff00, cval: 0xbc00, mnemonic: "pop {%l}"},
	// Format 15
	Opcodes { mask: 0xf800, cval: 0xc000, mnemonic: "stmia %r8!, {%l}"},
	Opcodes { mask: 0xf800, cval: 0xc800, mnemonic: "ldmia %r8!, {%l}"},
	// Format 17
	Opcodes { mask: 0xff00, cval: 0xdf00, mnemonic: "swi %m"},
	// Format 16
	Opcodes { mask: 0xf000, cval: 0xd000, mnemonic: "b%c %W"},
	// Format 18
	Opcodes { mask: 0xf800, cval: 0xe000, mnemonic: "b %a"},
	// Format 19
	Opcodes { mask: 0xf800, cval: 0xf000, mnemonic: "bl %A"},
	Opcodes { mask: 0xf800, cval: 0xf800, mnemonic: "blh %Z"},
	Opcodes { mask: 0xff00, cval: 0xbe00, mnemonic: "bkpt %O"},
	// Unknown
	Opcodes { mask: 0x0000, cval: 0x0000, mnemonic: "[ ??? ]"}
];

static ARM_OPCODES: [Opcodes; 40] = [
	// Undefined
	Opcodes { mask: 0x0e000010, cval: 0x06000010, mnemonic: "[ undefined ]"},
	// Branch instructions
	Opcodes { mask: 0x0ff000f0, cval: 0x01200010, mnemonic: "bx%c %r0"},
	Opcodes { mask: 0x0f000000, cval: 0x0a000000, mnemonic: "b%c %o"},
	Opcodes { mask: 0x0f000000, cval: 0x0b000000, mnemonic: "bl%c %o"},
	Opcodes { mask: 0x0f000000, cval: 0x0f000000, mnemonic: "swi%c %q"},
	// PSR transfer
	Opcodes { mask: 0x0fbf0fff, cval: 0x010f0000, mnemonic: "mrs%c %r3, %p"},
	Opcodes { mask: 0x0db0f000, cval: 0x0120f000, mnemonic: "msr%c %p, %i"},
	// Multiply instructions
	Opcodes { mask: 0x0fe000f0, cval: 0x00000090, mnemonic: "mul%c%s %r4, %r0, %r2"},
	Opcodes { mask: 0x0fe000f0, cval: 0x00200090, mnemonic: "mla%c%s %r4, %r0, %r2, %r3"},
	Opcodes { mask: 0x0fa000f0, cval: 0x00800090, mnemonic: "%umull%c%s %r3, %r4, %r0, %r2"},
	Opcodes { mask: 0x0fa000f0, cval: 0x00a00090, mnemonic: "%umlal%c%s %r3, %r4, %r0, %r2"},
	// Load/Store instructions
	Opcodes { mask: 0x0fb00ff0, cval: 0x01000090, mnemonic: "swp%c%b %r3, %r0, [%r4]"},
	Opcodes { mask: 0x0fb000f0, cval: 0x01000090, mnemonic: "[ ??? ]"},
	Opcodes { mask: 0x0c100000, cval: 0x04000000, mnemonic: "str%c%b%t %r3, %a"},
	Opcodes { mask: 0x0c100000, cval: 0x04100000, mnemonic: "ldr%c%b%t %r3, %a"},
	Opcodes { mask: 0x0e100090, cval: 0x00000090, mnemonic: "str%c%h %r3, %a"},
	Opcodes { mask: 0x0e100090, cval: 0x00100090, mnemonic: "ldr%c%h %r3, %a"},
	Opcodes { mask: 0x0e100000, cval: 0x08000000, mnemonic: "stm%c%m %r4%l"},
	Opcodes { mask: 0x0e100000, cval: 0x08100000, mnemonic: "ldm%c%m %r4%l"},
	// Data processing
	Opcodes { mask: 0x0de00000, cval: 0x00000000, mnemonic: "and%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00200000, mnemonic: "eor%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00400000, mnemonic: "sub%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00600000, mnemonic: "rsb%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00800000, mnemonic: "add%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00a00000, mnemonic: "adc%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00c00000, mnemonic: "sbc%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x00e00000, mnemonic: "rsc%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01000000, mnemonic: "tst%c%s %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01200000, mnemonic: "teq%c%s %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01400000, mnemonic: "cmp%c%s %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01600000, mnemonic: "cmn%c%s %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01800000, mnemonic: "orr%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01a00000, mnemonic: "mov%c%s %r3, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01c00000, mnemonic: "bic%c%s %r3, %r4, %i"},
	Opcodes { mask: 0x0de00000, cval: 0x01e00000, mnemonic: "mvn%c%s %r3, %i"},
	// Coprocessor operations
	Opcodes { mask: 0x0f000010, cval: 0x0e000000, mnemonic: "cdp%c %P, %N, %r3, %R4, %R0%V"},
	Opcodes { mask: 0x0e100000, cval: 0x0c000000, mnemonic: "stc%c%L %P, %r3, %A"},
	Opcodes { mask: 0x0f100010, cval: 0x0e000010, mnemonic: "mcr%c %P, %N, %r3, %R4, %R0%V"},
	Opcodes { mask: 0x0f100010, cval: 0x0e100010, mnemonic: "mrc%c %P, %N, %r3, %R4, %R0%V"},
	// Unknown
	Opcodes { mask: 0x00000000, cval: 0x00000000, mnemonic: "[ ??? ]"}
];

pub fn flag_on(flags: u32, check: u32) -> bool {
	(flags & check) != 0
}

pub fn write_hex(dest: &mut String, s: u32, n: u32) {
	let val = n as usize;
	let mut size = s as usize;
	if size == 0 {
		size = 28usize;
		while size > 0 {
			let shifted = val >> size;
			if shifted != 0 { // Gets rid of leading zeroes.
				dest.push( HDIG[shifted & 0xf] );
			}
			size -= 4;
		}
		size += 4;
	}

	while size > 0 {
		size -= 4;
		dest.push( HDIG[(val >> size) & 0xf] );
	}
}

pub fn disasm_arm(offset: u32, memory: &GbaMemory, flags: u32) -> String {
	let mut ret = String::with_capacity(128);
	disasm_arm_into(&mut ret, offset, memory, flags);
	ret
}

pub fn disasm_arm_into(ret: &mut String, offset: u32, memory: &GbaMemory, flags: u32) {
	let opcode = memory.read32(offset);
	let mut sp: &Opcodes = &ARM_OPCODES[0]; // So the compiler knows its initialized.
	for i in 0..ARM_OPCODES.len() {
		sp = &ARM_OPCODES[i];
		if sp.cval == (opcode & sp.mask) {
			break;
		}
	}

	if flag_on(flags, DIS_WRITE_ADDRESS) {
		write_hex(ret, 32, offset);
		ret.push_str(" ");
	}

	if flag_on(flags, DIS_WRITE_ADDRESS) {
		write_hex(ret, 32, opcode);
		ret.push_str(" ");
	}

	let opcodei = opcode as usize; // used for indexing shit.

	let mut src = (*sp).mnemonic.chars();

	while let Some(_c) = src.next() {
		if _c != '%' {
			ret.push(_c);
		} else {
			let c = src.next().unwrap();
			// ret.push_str("[%");
			// ret.push(c);
			// ret.push_str("=]");
			match c {
				'c' => ret.push_str( CONDITIONS[(opcodei >> 28) & 0xf] ),
				'r' => ret.push_str( REGS[(opcodei >> ((src.next().unwrap() as u8 - '0' as u8) * 4)) & 15] ),
				'o' => {

					ret.push('$');
					let mut off = opcode & 0xffffff;
					if off & 0x800000 != 0 {
						off |= 0xff000000;
					}
					off <<= 2;
					write_hex(ret, 32, (offset as i32 + 8 + off as i32) as u32);
				},
				'i' => {
					if (opcode & (1 << 25)) != 0 {
						ret.push_str("#0x");
						let imm = opcode & 0xff;
						let rot = (opcode & 0xf00) >> 7;
						let val = if rot == 0 {
							imm
						} else {
							(imm<<(32-rot))|(imm>>rot)
						};
						write_hex(ret, 0, val);
					} else {
						ret.push_str( REGS[opcodei & 0x0f] );
						let mut shi = (opcode >> 5) & 3;
						let mut sdw = (opcode >> 7) & 0x1f;
						if sdw == 0 && shi == 3 { shi = 4 }
						if sdw != 0 || opcode & 0x10 != 0 || shi != 0 {
							ret.push(',');
							ret.push(' ');
							ret.push_str(SHIFTS[shi as usize]);
							if opcode & 0x10 != 0 {
								ret.push(' ');
								ret.push_str(REGS[(opcodei >> 8) & 15]);
							} else {
								if sdw==0 && ( (shi==1) || (shi==2) ) {
									sdw = 32
								}
								if shi != 4 {
									ret.push_str("#0x");
									write_hex(ret, 8, sdw);
								}
							}
						}
					}
				},
				'p' => {
					if (opcode & (1 << 22)) != 0 {
						ret.push_str("spsr");
					} else {
						ret.push_str("cpsr");
					}

					if (opcode & 0x00F00000) != 0 {
						ret.push('_');
						if (opcode & 0x00080000) != 0 {
							ret.push('f');
						}
						if (opcode & 0x00040000) != 0 {
							ret.push('s');
						}
						if (opcode & 0x00020000) != 0 {
							ret.push('x');
						}
						if (opcode & 0x00010000) != 0 {
							ret.push('c');
						}
					}
				},
				's' => if (opcode & (1 << 20)) != 0 {
					ret.push('s');
				},
				'S' => if (opcode & (1 << 22)) != 0 {
					ret.push('s');
				},
				'u' => if (opcode & (1 << 22)) != 0 {
					if (opcode & (1 << 22)) != 0 {
						ret.push('s');
					} else {
						ret.push('u');
					}
				},
				'b' => if (opcode & (1 << 22)) != 0 {
					ret.push('b');
				},
				'a' => {
					if (opcode & 0x076f0000)==0x004f0000 {
						ret.push('[');
						ret.push('$');
						let mut addr = offset + 8;
						let add = (opcode & 15) | ((opcode>>8) & 0xf0);
						if (opcode & (1 << 23)) != 0 {
							addr += add;
						} else {
							addr -= add;
						}
						write_hex(ret, 32, addr);
						ret.push_str("] (=$");
						write_hex(ret, 32, memory.read32(addr));
						ret.push(')');
					}

					if (opcode & 0x072f0000)==0x050f0000 {
						ret.push('[');
						ret.push('$');
						let mut addr = offset + 8;
						if (opcode & (1 << 23)) != 0 {
							addr += opcode & 0xfff;
						} else {
							addr -= opcode & 0xfff;
						}
						write_hex(ret, 32, addr);
						ret.push_str("] (=$");
						write_hex(ret, 32, memory.read32(addr));
						ret.push(')');
					} else {
						let reg = (opcodei >> 16) & 15;
						ret.push('[');
						ret.push_str(REGS[reg]);
						if (opcode & (1 << 24)) == 0 { ret.push(']'); }
						if ((opcode & (1 << 25)) & (opcode & (1 << 26))) != 0 || ((opcode & (1 << 22)) | (opcode & (1 << 26))) == 0 {
							ret.push(',');
							ret.push(' ');
							if (opcode & (1 << 23)) == 0 { ret.push('-'); }
							ret.push_str( REGS[opcodei & 0x0f] );
							let shi = (opcodei >> 5) & 3;
							if (opcode & (1 << 26)) != 0 {
								if (opcode >> 7) & 0x1f != 0 || (opcode & 0x10) != 0 || (shi == 1) || (shi == 2) {
									ret.push(',');
									ret.push(' ');
									ret.push_str(SHIFTS[shi]);
									if (opcode & 0x10) != 0 {
										ret.push(' ');
										ret.push_str(REGS[(opcodei >> 7) & 15]);
									} else {
										let mut sdw = (opcode >> 7) & 0x1f;
										if sdw==0 && (shi == 1 || shi == 2) {
											sdw = 32;
										}
										ret.push_str("#0x");
										write_hex(ret, 8, sdw);
									}
								}
							}
						} else {
							let off;
							if (opcode & (1 << 26)) != 0 {
								off = opcode & 0xfff;
							}  else {
								off = (opcode & 15) | ((opcode >> 4) & 0xf0);
							}
							if off != 0 {
								ret.push(',');
								ret.push(' ');
								if (opcode & (1 << 23)) == 0 {
									ret.push('-');
								}
								ret.push_str("#0x");
								write_hex(ret, 0, off);
							}
						}
						if (opcode & (1 << 24)) != 0 {
							ret.push(']');
							if (opcode & (1 << 21)) != 0 {
								ret.push('!');
							}
						}
					}
				},
				't' => if (opcode & 0x01200000) == 0x01200000 {
					ret.push('t');
				},
				'h' => {
					if (opcode & (1 << 6)) != 0 { ret.push('s'); }
					if (opcode & (1 << 5)) != 0 { ret.push('h'); }
					else { ret.push('b'); }
				}
				'm' => if ((opcode >> 16) & 15) == 13 {
					if (opcode & 0x00100000) != 0 {
						ret.push_str( ARM_MULT_LOAD_STORE[8 + ((opcodei >> 23) & 3)] );
					} else {
						ret.push_str( ARM_MULT_LOAD_STORE[4 + ((opcodei >> 23) & 3)] );
					}
				} else {
					ret.push_str( ARM_MULT_LOAD_STORE[(opcodei >> 23) & 3] );
				},
				'l' => {
					if (opcode & (1 << 21)) != 0 {
						ret.push('!');
					}
					ret.push_str(", {");
					{
						let rlst = opcodei & 0xffff;
						let mut msk = 0;
						let mut not_first = false;

						while msk < 16 {
							if (rlst & (1 << msk)) != 0 {
								let fr = msk;
								while (rlst & (1 << msk)) != 0 {
									msk += 1;
								}
								let to = msk - 1;
								if not_first {
									ret.push(',');
								}
								ret.push_str(REGS[fr]);
								if fr != to {
									if fr == to-1 {
										ret.push(',');
									} else {
										ret.push('-');
									}
									ret.push_str(REGS[to]);
								}
								not_first = true;
							} else {
								msk += 1;
							}
						}
						ret.push('}');
						if (opcode & 1 << (22)) != 0 {
							ret.push('^');
						}
					}
				},
				'q' => {
					ret.push('$');
					write_hex(ret, 24, opcode & 0xffffff);

					if flag_on(flags, DIS_WRITE_GBA_SWI) {
						ret.push_str(" ;");
						let swi = (opcode >> 16) & 0xff; // upper 8bits of the comment.
						if swi < 43 {
							ret.push_str( SWI[swi as usize] );
						} else if swi >= 0x2B {
							ret.push_str("*CRASH*");
						} else {
							ret.push_str("???");
						}
					}
				},
				'P' => {
					ret.push('p');
					ret.push_str(&((opcode >> 8) & 15).to_string());
				},
				'N' => {
					if (opcode & 0x10) != 0 {
						ret.push_str(&((opcode >> 21) & 7).to_string());
					} else {
						ret.push_str(&((opcode>>20)&15).to_string());
					}
				},
				'R' => {
					let cc = src.next().unwrap();
					let reg = 4 * (cc as u8 - '0' as u8);
					ret.push('c');
					ret.push_str(&((opcode >> reg) & 15).to_string());
				},
				'V' => {
					let val = (opcode >> 5) & 7;
					if val != 0 {
						ret.push(',');
						ret.push(' ');
						ret.push_str( &val.to_string() );
					}
				},
				'L' => {
					if (opcode & (1 << 22)) != 0 {
						ret.push('l');
					}
				},
				'A' => {
					if (opcode & 0x012f0000) == 0x010f0000 {
						let mut addr = offset + 8;
						let add = (opcode & 0xff) << 2;
						if (opcode & (1 << 23)) != 0 {
							addr += add;
						} else {
							addr -= add;
						}
						ret.push('$');
						write_hex(ret, 32, addr);
					} else {
						ret.push('[');
						ret.push_str(REGS[(opcodei >> 16) & 15]);
						if (opcode & (1 << 24)) == 0 {
							ret.push(']');
						}
						let off = (opcode & 0xff) << 2;
						if off != 0 {
							ret.push(',');
							ret.push(' ');
							if (opcode & (1 << 23)) == 0 {
								ret.push('-');
							}
							ret.push_str("#0x");
							write_hex(ret, 0, off);
						}
						if (opcode & (1 << 24)) != 0 {
							ret.push(']');
							if (opcode & (1 << 21)) != 0 {
								ret.push('!');
							}
						}
					}
				},
				_ => { ret.push('%'); ret.push(c); }
			}
		}
	}
}

pub fn disasm_thumb(offset: u32, memory: &GbaMemory, flags: u32) -> String {
	let mut ret = String::with_capacity(128);
	disasm_thumb_into(&mut ret, offset, memory, flags);
	ret
}

pub fn disasm_thumb_into(ret: &mut String, offset: u32, memory: &GbaMemory, flags: u32) {
	let opcode = memory.read16(offset) as u32;
	let opcodei = opcode as usize;

	let mut sp: &Opcodes = &THUMB_OPCODES[0]; // So the compiler knows its initialized.
	for i in 0..THUMB_OPCODES.len() {
		sp = &THUMB_OPCODES[i];
		if sp.cval == (opcode & sp.mask) {
			break;
		}
	}

	if flag_on(flags, DIS_WRITE_ADDRESS) {
		write_hex(ret, 32, offset);
		ret.push(' ');
	}

	if flag_on(flags, DIS_WRITE_CODE) {
		write_hex(ret, 16, opcode);
		ret.push(' ');
	}

	let mut src = (*sp).mnemonic.chars();

	while let Some(_c) = src.next() {
		if _c != '%' {
			ret.push(_c);
		} else {
			let c = src.next().unwrap();
			match c {
				'r' => ret.push_str( REGS[(opcodei >> (src.next().unwrap() as u8 - '0' as u8)) & 7] ),
				'o' => {
					ret.push_str("#0x");
					let val = (opcode >> 6) & 0x1f;
					write_hex(ret, 8, val);
				},
				'p' => {
					ret.push_str("#0x");
					let mut val = (opcode >> 6) & 0x1f;
					if (opcode & (1 << 12)) == 0 { val <<= 2; }
					write_hex(ret, 0, val);
				},
				'e' => {
					ret.push_str("#0x");
					write_hex(ret, 0, ((opcode >> 6) & 0x1f) << 1);
				},
				'i' => {
					ret.push_str("#0x");
					write_hex(ret, 0, (opcode >> 6) & 7);
				},
				'h' => {
					let mut reg = (opcodei >> (src.next().unwrap() as u8 - '0' as u8)) & 7;
					let cc = src.next().unwrap() as u8;
					if (opcode & (1 << (cc - '0' as u8))) != 0 {
						reg += 8;
					}
					ret.push_str( REGS[reg] );
				},
				'O' => {
					ret.push_str("#0x");
					write_hex(ret, 0, opcode & 0xff);
				},
				'I' => {
					ret.push('$');
					write_hex(ret, 32, (offset & 0xfffffffc) + 4 + ((opcode & 0xff) << 2));
				},
				'J' => {
					let value = memory.read32((offset & 0xfffffffc) + 4 + ((opcode & 0xff) << 2));
					ret.push('$');
					write_hex(ret, 32, value);
				},
				'K' => {
					let value = (offset & 0xfffffffc) + 4 + ((opcode & 0xff) << 2);
					ret.push('$');
					write_hex(ret, 32, value);
				},
				'b' => {
					if (opcode & (1 << 10)) != 0 { ret.push('b'); }
				},
				'B' => {
					if (opcode & (1 << 12)) != 0 { ret.push('b'); }
				},
				'w' => {
					ret.push_str("#0x");
					write_hex(ret, 0, (opcode & 0xff) << 2);
				},
				'W' => {
					ret.push('$');
					let mut add = opcode & 0xff;
					if (add & 0x80) != 0 { add |= 0xffffff00; }
					write_hex(ret, 32, (offset & 0xfffffffe) + 4 + (add << 1));
				},
				'c' => {
					ret.push_str(CONDITIONS[(opcodei >> 8) & 15]);
				},
				's' => {
					if (opcode & (1 << 7)) != 0 {
						ret.push('-');
					}
					ret.push_str("#0x");
					write_hex(ret, 0, (opcode & 0x7f) << 2);
				},
				'l' => {
					let rlst = opcode & 0xff;
					let mut msk = 0;
					let mut not_first = false;
					while msk < 8 {
						if (rlst & (1 << msk)) != 0 {
							let fr = msk;
							while (rlst & (1 << msk)) != 0 { msk += 1; }
							let to = msk - 1;
							if not_first { ret.push(','); }
							ret.push_str(REGS[fr]);
							if fr != to {
								if fr == to-1 { ret.push(','); }
								else { ret.push('-'); }
								ret.push_str(REGS[to]);
							}
							not_first = true;
						} else {
							msk += 1;
						}
					}
				},
				'm' => {
					ret.push('$');
					write_hex(ret, 8, opcode & 0xff);
				},
				'Z' => {
					ret.push('$');
					write_hex(ret, 16, (opcode & 0x7ff) << 1);
				},
				'a' => {
					ret.push('$');
					let mut add = opcode & 0x07ff;
					if (add & 0x400) != 0 { add |= 0xfffff800; }
					add <<= 1;
					write_hex(ret, 32, offset + 4 + add);
				},
				'A' => {
					let nopcode = memory.read16(offset + 2) as u32;
					let mut add = opcode & 0x7ff;
					if (add & 0x400) != 0 { add |= 0xfff800; }
					add = (add << 12) | ((nopcode & 0x7ff) << 1);
					ret.push('$');
					write_hex(ret, 32, offset + 4 + add);
				},
				_ => { ret.push('%'); ret.push(c); }
			}
		}
	}
}
