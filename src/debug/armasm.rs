/* 
 * Shield your eyes :(
 * horribly optimized and written ARM assembler incoming.
 */
use std::ascii::AsciiExt;
use std::collections::HashMap;

static ARM_MNEMONICS: [&'static str; 46] = [
	"ADC", "ADD", "AND", "B", "BIC", "BL", "BX", "CDP", "CMN", "CMP",
	"EOR", "LDC", "LDM", "LDR", "MCR", "MLA", "MOV", "MRC", "MRS", "MSR",
	"MUL", "MVN", "ORR", "RSB", "RSC", "SBC", "STC", "STM", "STR", "SUB",
	"SWI", "SWP", "TEQ", "TST", "NOP", "PUSH", "POP", "UMULL", "UMLAL",
	"SMULL", "SMLAL", "LSL", "LSR", "ASR", "ROR", "RRX"
];


static ARM_CONDITIONS: [&'static str; 15] = [
	"EQ", "NE", "CS", "CC", "MI", "PL", "VS", "VC", "HI", "LS", "GE", "LT",
	"GT", "LE", "AL"
];

struct ArmSuffix {
	suffixes: &'static [&'static str],

	// #CLEANUP consider removing this as it doesn't currently do anything
	// not sure if "just" and LDM/STM instruction is allowed in ARM.
	#[allow(dead_code)]
	required: bool,

	name: &'static str
}

static ARM_SUFFIXES: [ArmSuffix; 6] = [
	ArmSuffix {suffixes: &["S"], required: false, name: ""},
	ArmSuffix {suffixes: &["BT", "B", "T", "H", "SH", "SB"], required: false, name: "mode"},
	ArmSuffix {suffixes: &["BT", "B", "T", "H"], required: false, name: "mode"},
	ArmSuffix {suffixes: &["FD", "ED", "FA", "EA", "IA", "IB", "DA", "DB"], required: true, name: "mode"},
	ArmSuffix {suffixes: &["B"], required: false, name: ""}, 
	ArmSuffix {suffixes: &["L"], required: false, name: ""},
];

fn get_suffix_for_mnemonic<'a>(mnemonic_index: usize) -> Result<usize, String> {
	let mnemonic = ARM_MNEMONICS[mnemonic_index];
	match mnemonic {
		"AND" | "EOR" | "SUB" | "RSB" | "ADD" |
		"ADC" | "SBC" | "RSC" | "ORR" | "BIC" |
		"MUL" | "MLA" | "MOV" | "MVN"  => Ok(0),

		"LDR" => Ok(1),
		"STR" => Ok(2),
		"LDM" | "STM" => Ok(3),

		"SWP" => Ok(4),
		"LDC" => Ok(5),
		"STC" => Ok(5),

		"UMULL" | "UMLAL" | "SMULL" | "SMLAL" => Ok(0),
		"LSL" | "LSR" | "ASR" | "ROR" | "RRX" => Ok(0),

		_ => Err(format!("Failed to find suffix for mnemonic {}", mnemonic))
	}
}

#[derive(Default, Debug)]
struct TempRet<'a> {
	remaining: Option<String>,
	mnemonic: Option<usize>,
	condition: Option<usize>,
	mode: Option<&'a str>,
	s: bool,
	b: bool,
	l: bool
}

enum ArmOperand {
	Register(u8)
}

/// Assembles a single line of ARM assembly.
pub fn arm_assemble_instr(source: &str) { // 233
	let info = parse_mnemonic(source);
	println!("{:#?}", info);
}

fn parse_suffix<'a>(source: &'a str, suffix_index: usize) -> Option<usize> {
	let usource = source.to_ascii_uppercase();
	for i in 0..ARM_SUFFIXES[suffix_index].suffixes.len() {
		let suffix = ARM_SUFFIXES[suffix_index].suffixes[i];
		if usource.starts_with(suffix) { return Some(i) }
	}
	None
}

/// Parses a mnemonic and a condition field.
fn parse_mnemonic<'a>(source: &'a str) -> Result<TempRet<'a>, String> {
	let mut t = source.split(" ");
	let t0 = t.next().expect("No mnemonic found in input.");
	let mut ret: TempRet = Default::default();
	let mut matched = false;
	for i in 0..t0.len() {
		// #todo reset ret.
		let c = &t0[0..(i + 1)];
		if let Some(mnemonic) = ARM_MNEMONICS.iter().position(|&a| c.eq_ignore_ascii_case(a)) {	
			ret.mnemonic = Some(mnemonic);
			let mut d = &t0[(i + 1)..t0.len()];

			if d.len() > 1 {
				let cond = &d[0..2];
				if let Some(condition) = ARM_CONDITIONS.iter().position(|&a| cond.eq_ignore_ascii_case(a)) {
					ret.condition = Some(condition);
					d = &d[ARM_CONDITIONS[condition].len()..];
				}
			}

			if d.len() == 0 {
				matched = true;
				break;
			}

			if let Ok(suffixes_idx) = get_suffix_for_mnemonic(mnemonic) {
				if let Some(suffix_idx) = parse_suffix(d, suffixes_idx) {
					if ARM_SUFFIXES[suffixes_idx].name.eq_ignore_ascii_case("mode") {
						ret.mode = Some(ARM_SUFFIXES[suffixes_idx].suffixes[suffix_idx]);
					} else {
						match &(ARM_SUFFIXES[suffixes_idx].suffixes[suffix_idx].to_ascii_uppercase()) as &str {
							"S" => ret.s = true,
							"B" => ret.b = true,
							"L" => ret.l = true,
							_ => {}
						}
					}
					d = &d[ARM_SUFFIXES[suffixes_idx].suffixes[suffix_idx].len()..];
					if d.len() == 0 {
						matched = true;
						break;
					}
				} else {
					continue
				}
			} else {
				continue
			}
		} else {
			continue
		}
	}
	return if !matched {
		Err(format!("Invalid mnemonic {}", source))
	} else {
		ret.remaining = Some(t.collect::<String>());
		Ok(ret)
	}
}

fn parse_operands<'a>(mnemonic_index: usize, remaining: &String) -> HashMap<&'a str, ArmOperand> {
	match ARM_MNEMONICS[mnemonic_index] {
		"BX"										=> parse_operands_0(remaining),
		"B"| "BL"									=> parse_operands_1(remaining),
		"AND"| "EOR"| "SUB"| "RSB"| "ADD"|"ADC"|
		"SBC"|"RSC"| "ORR"| "BIC"| "MOV"| "MVN" 	=> parse_operands_2(remaining),
		"MRS"										=> parse_operands_3(remaining),
		"MSR"										=> parse_operands_4(remaining),
		"MUL"| "MLA"								=> parse_operands_5(remaining),
		"UMULL"| "UMLAL"| "SMULL"| "SMLAL"			=> parse_operands_6(remaining),
		"LDR"| "STR"								=> parse_operands_7(remaining),
		"LDM"| "STM"								=> parse_operands_9(remaining),
		"SWP"										=> parse_operands_10(remaining),
		"SWI"										=> parse_operands_11(remaining),
		"CDP"										=> parse_operands_12(remaining),
		"LDC"| "STC"								=> parse_operands_13(remaining),
		"MRC"| "MCR"								=> parse_operands_14(remaining),
		"PUSH"| "POP"								=> parse_operands_15(remaining),
		"LSL"| "LSR"| "ASR"| "ROR"					=> parse_operands_16(remaining),
		"RRX"										=> parse_operands_17(remaining),
		"NOP"										=> parse_operands_18(remaining),
		"CMP"| "CMN"| "TEQ"| "TST" 					=> parse_operands_19(remaining),
		_ => panic!("Impossible mnemonic match.")
	}
}

/// Parses operands for instructions of assembler syntax:
/// <BX>{cond} Rn
fn parse_operands_0<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> {
	let mut map = HashMap::new();
	map.insert("rn", ArmOperand::Register(12));
	map
}

fn parse_operands_1<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }

fn parse_operands_2<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_3<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_4<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_5<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_6<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_7<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_8<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_9<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_10<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_11<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_12<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_13<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_14<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_15<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_16<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_17<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_18<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }
fn parse_operands_19<'a>(remaining: &String) -> HashMap<&'a str, ArmOperand> { unreachable!() }