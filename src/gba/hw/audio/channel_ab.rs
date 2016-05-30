use super::super::super::core::cpu::ArmCpu;
use super::super::super::device::audio::AudioDevice;
use super::super::super::core::memory::*;
use super::super::super::core::memory::ioreg::GbaChannel4;
use super::{AudioState, apply_volume_stereo};

pub fn init(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) {
}

pub fn tick(cpu: &mut ArmCpu, device: &AudioDevice, state: &mut AudioState) -> (i16, i16) {
	return (0, 0)
}