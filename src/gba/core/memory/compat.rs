use rustc_serialize::Encodable;
use rustc_serialize::Decodable;
use rustc_serialize::Decoder;
use rustc_serialize::Encoder;

pub const FIFO_OUT_BUFFER_SIZE: usize = 0x4000;
pub const FIFO_OUT_BUFFER_MASK: usize = 0x3fff;

pub struct FifoOutArray {
	pub data: [i8; FIFO_OUT_BUFFER_SIZE]	
}

impl Default for FifoOutArray {
	fn default() -> FifoOutArray {
		FifoOutArray {
			data: [0i8; FIFO_OUT_BUFFER_SIZE]
		}
	}
}

impl Decodable for FifoOutArray {
	fn decode<D: Decoder>(_: &mut D) -> Result<FifoOutArray, D::Error> {
		Ok(Default::default())
	}
}

impl Encodable for FifoOutArray {
	fn encode<S: Encoder>(&self, _: &mut S) -> Result<(), S::Error> {
		Ok(())
	}
}