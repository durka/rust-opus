#[link(name = "opus")]
extern "C" {
    pub fn opus_strerror(error: ::libc::c_int) -> *const ::libc::c_char;
    pub fn opus_get_version_string() -> *const ::libc::c_char;
    pub fn opus_encoder_create(Fs: i32, channels: ::libc::c_int,
                               application: ::libc::c_int,
                               error: *mut ::libc::c_int) -> *const ::libc::c_void;
    pub fn opus_encoder_ctl(st: *mut ::libc::c_void, request: ::libc::c_int, ...)
     -> ::libc::c_int;
}
