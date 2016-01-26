use super::*;
use super::super::memory::*;



/// Handles an SWI instruction in ARM mode.
pub fn handle_arm_swi(cpu: &mut ArmCpu, instr: u32) {
	let label = (instr >> 16) & 0xff;
	println!("%% {}", cpu.disasm_exec());
	println!("Executing ARM SWI 0x{:02x}", label);
	execute_swi(cpu, label); // only the upper 8 bits.
}


/// Handles an SWI instruction in THUMB mode.
pub fn handle_thumb_swi(cpu: &mut ArmCpu, instr: u32) {
	let label = instr & 0xff;
	println!("%% {}", cpu.disasm_exec());
	println!("Executing THUMB SWI 0x{:02x}", label);
	execute_swi(cpu, label);
}


/// Executes the SWI function with the given label.
pub fn execute_swi(cpu: &mut ArmCpu, label: u32) {
	panic!("poopy");
}

/// Holding state for software interrupts.
#[derive(Default)]
pub struct ArmCpuSwiState {
	waiting_for_vblank: bool,
}

impl ArmCpuSwiState {
	pub fn new() -> ArmCpuSwiState {
		Default::default()
	}
	/// Returns true if the CPU should continue halting.
	pub fn check_if_continue_halt(&mut self, memory: &GbaMemory) -> bool {
		// if self.waiting_for_vblank {
		// }
		return true
	}
}


// SWI functions
// 	00h  SoftReset
// 	01h  RegisterRamReset
// 	02h  Halt
// 	03h  Stop/Sleep
// 	04h  IntrWait       ;DSi7/DSi9: both bugged?
// 	05h  VBlankIntrWait ;DSi7/DSi9: both bugged?
// 	06h  Div
// 	07h  DivArm
// 	08h  Sqrt
// 	09h  ArcTan
// 	0Ah  ArcTan2
// 	0Bh  CpuSet
// 	0Ch  CpuFastSet
// 	0Dh  GetBiosChecksum
// 	0Eh  BgAffineSet
// 	0Fh  ObjAffineSet
// GBA  Decompression Functions
// 	10h  BitUnPack
// 	11h  LZ77UnCompReadNormalWrite8bit   ;"Wram"
// 	12h  LZ77UnCompReadNormalWrite16bit  ;"Vram"
// 	13h  HuffUnCompReadNormal
// 	14h  RLUnCompReadNormalWrite8bit     ;"Wram"
// 	15h  RLUnCompReadNormalWrite16bit    ;"Vram"
// 	16h  Diff8bitUnFilterWrite8bit       ;"Wram"
// 	17h  Diff8bitUnFilterWrite16bit      ;"Vram"
// 	18h  Diff16bitUnFilter
// GBA  Sound (and Multiboot/HardReset/CustomHalt)
// 	19h  SoundBias
// 	1Ah  SoundDriverInit
// 	1Bh  SoundDriverMode
// 	1Ch  SoundDriverMain
// 	1Dh  SoundDriverVSync
// 	1Eh  SoundChannelClear
// 	1Fh  MidiKey2Freq
// 	20h  SoundWhatever0
// 	21h  SoundWhatever1
// 	22h  SoundWhatever2
// 	23h  SoundWhatever3
// 	24h  SoundWhatever4
// 	25h  MultiBoot
// 	26h  HardReset
// 	27h  CustomHalt
// 	28h  SoundDriverVSyncOff
// 	29h  SoundDriverVSyncOn
// 	2Ah  SoundGetJumpList
// GBA Invalid Functions
// 	2Bh+ Crash (SWI xxh..FFh do jump to garbage addresses)
