extern crate libc;

use libc::*;
use std::ffi::CStr;
use std::str;

mod ffi;

pub enum SamplingRate {
	_8000 = 8000,
	_12000 = 12000,
	_16000 = 24000,
	_48000 = 48000
}

pub enum Channels {
	One = 1,
	Two = 2
}

pub enum Application {
    VoIP = 2048,
    Audio = 2049,
    LowDelay = 2051
}

#[derive(Debug)]
pub struct OpusEncoder {
	ptr: size_t,
	error: i32,
}
#[derive(Debug, PartialEq)]
pub enum OpusError {
	Success,
	InvalidArgument,
	BufferTooSmall,
	InternalError,
	CorruptedStream,
	RequestNotImplemented,
	InvalidState,
	MemoryAllocationFailed,
	UnknownError,
}

impl OpusError {
	pub fn new(error_code: i32) -> OpusError {
		use OpusError::*;
		match error_code {
			 0 => Success,
			-1 => InvalidArgument,
			-2 => BufferTooSmall,
			-3 => InternalError,
			-4 => CorruptedStream,
			-5 => RequestNotImplemented,
			-6 => InvalidState,
			-7 => MemoryAllocationFailed,
			 _ => UnknownError
		}
	}
}
#[allow(dead_code)]
enum OpusRequest {
	SetApplication = 4000,
	GetApplication = 4001,
	SetBitrate = 4002,
	GetBitrate = 4003,
	SetMaxBandwidth = 4004,
	GetMaxBandwidth = 4005,
	SetVBR = 4006,
	GetVBR = 4007,
	SetBandwidth = 4008,
	GetBandwidth = 4009,
	SetComplexity = 4010,
	GetComplexity = 4011,
	SetInbandFEC = 4012,
	GetInbandFEC = 4013,
	SetPacketLossPerc = 4014,
	GetPacketLossPerc = 4015,
	SetDtxRequest = 4016,
	GetDtxRequest = 4017,
	SetVBRConstraint = 4020,
	GetVBRConstraint = 4021,
	SetForceChannels = 4022,
	GetForceChannels = 4023,
	SetSignalRequest = 4024,
	GetSignalRequest = 4025,
	GetLookahead = 4027,
	//ResetState = 4028, //commented out in the original header
	GetSampleRate = 4029,
	GetFinalRange = 4031,
	GetPitch = 4033,
	SetGain = 4034,
	GetGain = 4035,
	SetLSBDepth = 4036,
	GetLSBDepth = 4037,
	GetLastPacketDuration = 4039,
	SetExpertFrameDuration = 4040,
	GetExpertFrameDuration = 4041,
	SetPredictionDisabled = 4042,
	GetPredictionDisabled = 4043
	// 4045 is GetGain, for some reason
}

macro_rules! encoder_ctl {
	($target:expr, $request:expr) => {
		{
			let ret = OpusError::new(
				unsafe {
					ffi::opus_encoder_ctl($target, $request as i32)
				});
			match ret {
				OpusError::Success => Ok(()),
				_ => Err(ret)
			}
		}
	};
	($target:expr, $request:expr, $arg1:expr) => {
		{
			let ret = OpusError::new(
				unsafe {
					ffi::opus_encoder_ctl($target, $request as i32, $arg1)
				});
			match ret {
				OpusError::Success => Ok(()),
				_ => Err(ret)
			}
		}
	};
	($target:expr, $request:expr, $arg1:expr, $arg2:expr) => {
		{
			let ret = OpusError::new(
				unsafe {
					ffi::opus_encoder_ctl($target, $request as i32, $arg1, $arg2)
				});
			match ret {
				OpusError::Success => Ok(()),
				_ => Err(ret)
			}
		}
	}
}

impl OpusEncoder {
	pub fn new(sampling_rate: SamplingRate,
			channels: Channels,
			application: Application) -> Result<OpusEncoder, OpusError> {
		let mut error = 0i32;
		let ptr = unsafe { ffi::opus_encoder_create(
			sampling_rate as i32,
			channels as i32,
			application as i32,
			&mut error
			)};
		let mut _error = OpusError::new(error);
		match _error {
			OpusError::Success => Ok(OpusEncoder { ptr: ptr, error: error }),
			_ => Err(_error)
		}
	}

	pub fn get_bitrate(&mut self) -> Result<i32, OpusError> {
		let mut out = Box::new(0i32);
		let ret = OpusError::new(
			unsafe {
				ffi::opus_encoder_ctl(
					&mut self.ptr,
					OpusRequest::GetBitrate as i32,
					*out
				)
			});
		println!("<out>{:x}</out>", *out);
		match ret {
			OpusError::Success => Ok(*out),
			_ => Err(ret)
		}
		
	}

	pub fn set_bitrate(&mut self, bitrate: i32) -> Result<(), OpusError> {
		encoder_ctl!(&mut self.ptr, OpusRequest::SetBitrate, bitrate)
		/*let ret = OpusError::new(
			unsafe {
				ffi::opus_encoder_ctl(
					&mut self.ptr,
					OpusRequest::SetBitrate as i32,
					bitrate
				)
			}
		);
		match ret {
			OpusError::Success => Ok(()),
			_ => Err(ret)
		}*/
	}
}



// no need for this - we re-implement it
pub fn str_error(error_code: i32) -> String {
	let c_buf = unsafe { ffi::opus_strerror(error_code) };
	let c_str = unsafe { CStr::from_ptr(c_buf)};
	let buf = c_str.to_bytes();
	let str_slice = str::from_utf8(buf).unwrap();
	str_slice.to_owned()
}

pub fn version_str() -> String {
	let c_buf: *const c_char = unsafe { ffi::opus_get_version_string() };
	let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
	let buf: &[u8] = c_str.to_bytes();
	let str_slice: &str = str::from_utf8(buf).unwrap();
	str_slice.to_owned()
}

#[cfg(test)]
mod tests {
	fn make_encoder() -> Result<OpusEncoder, OpusError> {
		OpusEncoder::new(
			SamplingRate::_48000,
			Channels::Two,
			Application::Audio
		)
	}

	use super::*;
	#[test]
	fn error_string() {
		assert_eq!("success", &str_error(0));
		assert_eq!("unknown error", &str_error(1));
		assert_eq!("invalid argument", &str_error(-1));
	}
	#[test]
	fn lib_version() {
		assert_eq!("libopus 1.1", &version_str());
	}
	#[test]
	fn create_encoder() {
		let opus_enc = make_encoder();
		if let Err(e) = opus_enc {
			panic!("Test failed with error: {:?}", e);
		}
	}
	#[test]
	fn encoder_set_bitrate() {
		let mut opus_enc = make_encoder().unwrap();
		assert!(opus_enc.set_bitrate(500).is_ok());
		assert!(opus_enc.set_bitrate(0).is_err());
		assert_eq!(opus_enc.set_bitrate(0).err().unwrap(), OpusError::InvalidArgument);
	}
	#[test]
	fn encoder_get_bitrate() {
		let mut opus_enc = make_encoder().unwrap();
		let _ = opus_enc.set_bitrate(4000);
		assert_eq!(opus_enc.get_bitrate().unwrap(), 500)
	}
}

#[test]
fn it_works() {

}
