pub fn arm_fn_mul(lhs: u32, rhs: u32, _: u32) -> u32 {
	lhs * rhs
}

pub fn arm_fn_mla(lhs: u32, rhs: u32, acc: u32) -> u32 {
	lhs * rhs + acc
}

pub fn arm_fn_umull(lhs: u32, rhs: u32, _: u32, _: u32) -> u64 {
	(lhs as u64) * (rhs as u64)
}

pub fn arm_fn_umlal(lhs: u32, rhs: u32, acc_hi: u32, acc_lo: u32) -> u64 {
	(lhs as u64) * (rhs as u64) + combine32to64(acc_hi, acc_lo)
}

pub fn arm_fn_smull(lhs: u32, rhs: u32, _: u32, _: u32) -> u64 {
	let lhs = (lhs as i32) as i64;
	let rhs = (rhs as i32) as i64;
	(lhs * rhs) as u64
}

pub fn arm_fn_smlal(lhs: u32, rhs: u32, acc_hi: u32, acc_lo: u32) -> u64 {
	let lhs = (lhs as i32) as i64;
	let rhs = (rhs as i32) as i64;
	let m_result = (lhs * rhs) as u64;
	m_result + combine32to64(acc_hi, acc_lo)
}

fn combine32to64(hi: u32, lo: u32) -> u64 {
	(lo as u64)  | ((hi as u64) << 32)
}