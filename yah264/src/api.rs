#[cfg(feature = "low-level-api")]
pub mod low_level_api {
    use std::{
        ffi::{CString, c_int, c_void}, mem, ptr::null,
    };

    use ffi::{
        yah264_csp_t_YAH264_CSP_I420, yah264_csp_t_YAH264_CSP_I422, yah264_csp_t_YAH264_CSP_I444, yah264_encoder_close, yah264_encoder_encode, yah264_encoder_headers, yah264_encoder_open, yah264_encoder_t, yah264_lookahead_delay, yah264_nal_t, yah264_param_apply_preset, yah264_param_default, yah264_param_t, yah264_picture_t,
    };

    pub struct Picture {
        pic: yah264_picture_t,
        plane_size: [usize; 3],
        native: bool,
        buffer: Vec<u8>,
    }

    struct ColorspaceScale {
        w: [usize; 3],
        h: [usize; 3],
    }

    impl ColorspaceScale {
        fn iter(&self) -> impl Iterator<Item = (&usize, &usize)> {
            self.w.iter().zip(self.h.iter())
        }
    }

    fn scale_from_csp(csp: u32) -> ColorspaceScale {
        match csp {
            yah264_csp_t_YAH264_CSP_I420 => ColorspaceScale {
                w: [256, 256 / 2, 256 / 2],
                h: [256, 256 / 2, 256 / 2],
            },
            yah264_csp_t_YAH264_CSP_I422 => ColorspaceScale {
                w: [256, 256 / 2, 256 / 2],
                h: [256, 256, 256],
            },
            yah264_csp_t_YAH264_CSP_I444 => ColorspaceScale {
                w: [256, 256, 256],
                h: [256, 256, 256],
            },
            _ => unimplemented!(),
        }
    }

    impl Picture {
        pub fn from_param(param: &Param) -> Result<Picture, &'static str> {
            let width = param.par.width;
            let height = param.par.height;

            if width == 0 || height == 0 {
                return Err("Invalid encoder dimensions in Param");
            }

            let bit_depth = std::env::var("Y264_BIT_DEPTH")
                .map_err(|_| "Could not determine configured bit_depth")?
                .parse::<u8>()
                .map_err(|_| "Unsupported bit_depth")?;

            let csp = param.par.csp as u32;

            Self::picture_alloc(bit_depth, csp, width, height)
        }

        pub fn as_slice(&self, plane: usize) -> Result<&[u8], &'static str> {
            if plane >= 3 {
                return Err("Invalid plane index")
            }

            let size = self.plane_size[plane];
            Ok(unsafe {
                std::slice::from_raw_parts(self.pic.plane[plane] as *const u8, size)
            })
        }

        pub fn as_mut_slice(&self, plane: usize) -> Result<&mut [u8], &'static str> {
            if plane >= 3 {
                return Err("Invalid plane index")
            }

            let size = self.plane_size[plane];
            Ok(unsafe {
                std::slice::from_raw_parts_mut(self.pic.plane[plane] as *mut u8, size)
            })
        }

        pub fn set_timestamp(mut self, pts: i64) -> Picture {
            self.pic.pts = pts;
            self
        }

        fn picture_alloc(bit_depth: u8, colorspace: u32, width: i32, height: i32) -> Result<Picture, &'static str> {
            let w = width as usize;
            let h = height as usize;

            let (u_w, u_h) = match colorspace {
                yah264_csp_t_YAH264_CSP_I420 => (w / 2, h / 2),
                yah264_csp_t_YAH264_CSP_I422 => (w / 2, h),
                yah264_csp_t_YAH264_CSP_I444 => (w, h),
                _ => return Err("Unsupported colorspace"),
            };

            let plane_widths = [w, u_w, u_w];
            let plane_heights = [h, u_h, u_h];

            let bytes_per_pixel = if bit_depth > 8 { 2 } else { 1 };

            let mut plane_size = [0usize; 3];
            let mut total_bytes = 0;

            for i in 0..3 {
                plane_size[i] = plane_widths[i] * plane_heights[i] * bytes_per_pixel;
                total_bytes += plane_size[i];
            } 

            let mut buffer = vec![0u8; total_bytes];
            


            let mut pic: yah264_picture_t = unsafe {
                mem::zeroed()
            };

            pic.csp = colorspace as i32;
            pic.width = width;
            pic.height = height;

            let ptr = buffer.as_mut_ptr();
            let mut offset = 0;

            for i in 0..3 {
                unsafe {
                    pic.plane[i] = ptr.add(offset) as *mut c_void;
                    pic.stride[i] = plane_widths[i] as c_int;
                }

                offset += plane_size[i];
            }

            Ok(Picture { pic, plane_size, native: true, buffer })   
        } 
    }

    // TODO: Provide a builder API
    pub struct Param {
        par: yah264_param_t,
    }

    impl Default for Param {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Param {
        pub fn new() -> Param {
            let mut par: yah264_param_t = unsafe {
                std::mem::zeroed()
            };

            unsafe {
                yah264_param_default(&mut par);
            };

            let param = Param {
                par,
            };

            param
        }

        pub fn default_preset<'a, 'b, Oa, Ob>(mut self, preset: Oa, _tune: Ob) -> Result<Param, &'static str>
        where
            Oa: Into<Option<&'a str>>,
            Ob: Into<Option<&'b str>>,
        {
            let t = _tune
                .into()
                .map_or_else(|| None, |v| Some(CString::new(v).unwrap()));
            let p = preset
                .into()
                .map_or_else(|| None, |v| Some(CString::new(v).unwrap()));

            let _c_tune = t.as_ref().map_or_else(null, |v| v.as_ptr() as *const i8);
            let c_preset = p.as_ref().map_or_else(null, |v| v.as_ptr() as *const i8);

            let _ = match unsafe { yah264_param_apply_preset(&mut self.par, c_preset) } {
                -1 => Err("Invalid Argument"),
                0 => Ok(()),
                _ => Err("Unexpected"),
            };

            Ok(self)
        }

        pub fn apply_profile(mut self, _profile: &str) -> Result<Param, &'static str> {
            unimplemented!()
        }

        pub fn param_parse(mut self, _name: &str, _value: &str) -> Result<Param, &'static str> {
            unimplemented!()
        }

        pub fn set_dimension(mut self, height: usize, width: usize) -> Param {
            self.par.height = height as c_int;
            self.par.width = width as c_int;

            self
        }

        pub fn set_colorspace(mut self, colorspace: i32) -> Param {
            self.par.csp = colorspace;

            self
        }
    }

    // TODO: Expose NAL abstraction
    pub struct NalData {
        vec: Vec<u8>,
    }

    impl NalData {
        fn from_nals(c_nals: *mut yah264_nal_t, nb_nal: usize) -> NalData {
            let mut data = NalData { vec: Vec::new() };

            if c_nals.is_null() || nb_nal == 0 {
                return data;
            }

            for i in 0..nb_nal {
                unsafe {
                    let nal = &*c_nals.add(i);

                    let nal_size = nal.size;

                    if !nal.payload.is_null() && nal_size > 0 {
                        let payload = std::slice::from_raw_parts(nal.payload, nal_size);
                        data.vec.extend_from_slice(payload);
                    }
                }
            }

            data
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.vec.as_slice()
        }
    }

    pub struct Encoder {
        enc: *mut yah264_encoder_t,
    }

    impl Encoder {
        pub fn open(par: &mut Param) -> Result<Encoder, &'static str> {
            let enc = unsafe { yah264_encoder_open(&mut par.par as *mut yah264_param_t) };

            if enc.is_null() {
                Err("Out of Memory")
            } else {
                Ok(Encoder { enc })
            }
        }

        pub fn get_headers(&mut self) -> Result<NalData, &'static str> {
            let mut nb_nal: c_int = 0;
            let mut c_nals: mem::MaybeUninit<*mut yah264_nal_t> = mem::MaybeUninit::uninit();

            let bytes = unsafe {
                yah264_encoder_headers(self.enc, c_nals.as_mut_ptr(), &mut nb_nal as *mut c_int)
            };

            if bytes < 0 {
                Err("Encoding headers failed")
            } else {
                let c_nals = unsafe { c_nals.assume_init() };
                Ok(NalData::from_nals(c_nals, nb_nal as usize))
            }
        }

        pub fn delayed_frames(&self, par: &Param) -> bool {
            unsafe {
                yah264_lookahead_delay(&par.par as *const yah264_param_t) != 0
            }
        }

        pub fn encode<'a, P>(&mut self, pic: P) -> Result<Option<(NalData, i64)>, &'static str>
        where
            P: Into<Option<&'a Picture>>,
        {
            let pic_out: mem::MaybeUninit<yah264_picture_t> = mem::MaybeUninit::uninit();
            let mut c_nals: mem::MaybeUninit<*mut yah264_nal_t> = mem::MaybeUninit::uninit();
            let mut nb_nal: c_int = 0;
            let c_pic: *const yah264_picture_t = pic
                .into()
                .map_or_else(null, |v| &v.pic as *const yah264_picture_t);

            let ret = unsafe {
                yah264_encoder_encode(
                    self.enc,
                    c_nals.as_mut_ptr(),
                    &mut nb_nal as *mut c_int,
                    c_pic as *mut yah264_picture_t,
                )
            };

            if ret < 0 {
                Err("Error encoding")
            } else {
                let pic_out = unsafe { pic_out.assume_init() };
                let c_nals = unsafe { c_nals.assume_init() };

                if nb_nal > 0 {
                    let data = NalData::from_nals(c_nals, nb_nal as usize);
                    Ok(Some((data, pic_out.pts)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    impl Drop for Encoder {
        fn drop(&mut self) {
            unsafe { yah264_encoder_close(self.enc) };
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_open() {
            let mut par = Param::new().set_dimension(1920, 1080);
            let mut enc = Encoder::open(&mut par).unwrap();
            let headers = enc.get_headers().unwrap();

            println!("Headers len {}", headers.as_bytes().len());
        }

        #[test]
        fn test_picture() {
            std::env::set_var("Y264_BIT_DEPTH", "8");

            let par = Param::new().set_dimension(1920, 1080);
            {
                let pic = Picture::from_param(&par).unwrap();
                {
                    let p = pic.as_mut_slice(0).unwrap();
                    p[0] = 1;
                }
                let p = pic.as_slice(0).unwrap();

                assert_eq!(p[0], 1);
            }
        }

        #[test]
        fn test_encode() {
            std::env::set_var("Y264_BIT_DEPTH", "8");

            let mut par = Param::new().set_dimension(1920, 1080);
            let mut enc = Encoder::open(&mut par).unwrap();
            let mut pic = Picture::from_param(&par).unwrap();

            let headers = enc.get_headers().unwrap();

            println!("Headers len {}", headers.as_bytes().len());

            for pts in 0..5 {
                pic = pic.set_timestamp(pts as i64);
                let ret = enc.encode(&pic).unwrap();

                if let Some((_, pts)) = ret {
                    println!("Frame pts {}", pts);
                }
            }
        }
    }
}

#[cfg(feature = "high-level-api")]
pub mod high_level_api {}
