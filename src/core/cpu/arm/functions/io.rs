use super::super::super::ArmCpu;

pub fn arm_fn_ldrb(cpu: &mut ArmCpu, address: u32, dest: u32) -> u32 {
	0
}

pub fn arm_fn_ldr(cpu: &mut ArmCpu, address: u32, dest: u32) -> u32 {
	0
}

pub fn arm_fn_strb(cpu: &mut ArmCpu, address: u32, source: u32) -> u32 {
	0
}

pub fn arm_fn_str(cpu: &mut ArmCpu, address: u32, source: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_imm(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_off(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_nim(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_lsl(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_lsr(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_asr(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}

pub fn arm_fn_sdt_ror(cpu: &ArmCpu, instr: u32) -> u32 {
	0
}