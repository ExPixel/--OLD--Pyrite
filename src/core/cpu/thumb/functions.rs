use super::super::ArmCpu;

pub fn thumb_fn_neg(cpu: &mut ArmCpu, _: u32, rhs: u32) -> u32 {
	super::super::arm::functions::arm_fn_sub_s(cpu, 0, rhs)
}

pub fn thumb_fn_mul(cpu: &mut ArmCpu, lhs: u32, rhs: u32) -> u32 {
	let result = lhs * rhs;
	super::super::alu::set_nz_flags(cpu, result);
	return result;
}

pub fn thumb_fn_ldr(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread32(address);
	cpu.rset(rd, data);
}

pub fn thumb_fn_ldrb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread8(address);
	cpu.rset(rd, data as u32);
}

pub fn thumb_fn_str(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.rget(rd);
	cpu.mwrite32(address, data);
}

pub fn thumb_fn_strb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.rget(rd) as u8;
	cpu.mwrite8(address, data);
}

pub fn thumb_fn_ldrh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = cpu.mread16(address) as u32;
	cpu.rset(rd, data);
}

pub fn thumb_fn_strh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	// if address == cpu.rget(7) && rd == 1 {
	// 	println!("store r{}=0x{:08x} to [{:08x}]; r2=x{:08x}; r3=0x{:08x}", rd, cpu.rget(rd), address, cpu.rget(2), cpu.rget(3));
	// }
	let data = cpu.rget(rd) as u16;
	cpu.mwrite16(address, data);
}

pub fn thumb_fn_ldrsb(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = ((cpu.mread8(address) as i8) as i32) as u32;
	cpu.rset(rd, data);
}

pub fn thumb_fn_ldrsh(cpu: &mut ArmCpu, address: u32, rd: u32) {
	let data = ((cpu.mread16(address) as i16) as i32) as u32;
	cpu.rset(rd, data);
}

pub fn thumb_sdt_addr_reg(cpu: &ArmCpu, instr: u32) -> u32 {
	let ro = (instr >> 6) & 0x7;
	let rb = (instr >> 3) & 0x7;
	let _ro = cpu.rget(ro);
	let _rb = cpu.rget(rb);
	return _rb + _ro;
}

pub fn thumb_sdt_addr_imm5(cpu: &ArmCpu, instr: u32) -> u32 {
	let rb = (instr >> 3) & 0x7;
	let offset5 = (instr >> 6) & 0x1f;
	return cpu.rget(rb) + offset5;
}

pub fn thumb_sdt_addr_imm6(cpu: &ArmCpu, instr: u32) -> u32 {
	let rb = (instr >> 3) & 0x7;
	let offset6 = ((instr >> 6) & 0x1f) << 1;
	return cpu.rget(rb) + offset6;
}

pub fn thumb_sdt_addr_imm7(cpu: &ArmCpu, instr: u32) -> u32 {
	let rb = (instr >> 3) & 0x7;
	let offset7 = ((instr >> 6) & 0x1f) << 2;
	return cpu.rget(rb) + offset7;
}