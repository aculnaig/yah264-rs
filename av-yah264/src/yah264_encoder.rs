use crate::ffi::*;

pub struct Encoder {
    enc: *mut yah264_encoder_t
}

unsafe impl Send for Encoder {}

impl Encoder {
    pub fn create() -> Result<Encoder, bool> {
        let params = unsafe {
            let mut default_params = std::mem::zeroed();
            yah264_param_default(default_params);
            default_params
        };

        let yah264_encoder = unsafe {
            let encoder = yah264_encoder_open(params);
            encoder
        };

        if yah264_encoder.is_null() {
            return Err(false);
        }

        Ok(Encoder { enc: yah264_encoder })
    }
}
