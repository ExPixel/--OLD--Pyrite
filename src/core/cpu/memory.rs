pub trait ArmMemory {
	fn read8(address: u32) -> u8;
	fn write8(address: u32, value: u8);

	fn read16(address: u32) -> u16;
	fn write16(address: u32, value: u16);

	fn read32(address: u32) -> u32;
	fn write32(address: u32, value: u32);
}