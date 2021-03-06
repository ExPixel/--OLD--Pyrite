use super::super::core::cpu::ArmCpu;
use super::super::hw::audio::channel_ab;

macro_rules! timer {
	($cpu:expr, $timer:expr) => (
		$cpu.memory.internal_regs.timers[$timer]
	)
}

pub fn increment(cpu: &mut ArmCpu, amount: u32) {
	let mut last_timer_overflowed = false;
	for t in 0..4 {
		if timer!(cpu, t).operate {
			if timer!(cpu, t).count_up {
				if last_timer_overflowed {
					timer!(cpu, t).counter += 1;
				}
			} else {
				timer!(cpu, t).unscaled_counter += amount;
				let scaled = timer!(cpu, t).unscaled_counter >> timer!(cpu, t).prescaler;
				timer!(cpu, t).counter += scaled;
				timer!(cpu, t).unscaled_counter -= scaled << timer!(cpu, t).prescaler;
			}

			if timer!(cpu, t).counter > 0xffff { // The timer!(cpu, t) has overflowed
				if t == 0 {
					::debug::debugger::get_debugger().timer_ov_counter += 1;
				}

				timer!(cpu, t).counter = timer!(cpu, t).reload;
				timer!(cpu, t).unscaled_counter = 0;
				last_timer_overflowed = true;

				channel_ab::timer_overflow(cpu, t as u16);

				if timer!(cpu, t).irq_enabled {
					cpu.hardware_interrupt(0x08 << t);
				}

				continue
			}
		}
		last_timer_overflowed = false
	}
}
