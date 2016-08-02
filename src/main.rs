extern crate libc;
#[macro_use]
extern crate bitflags;
// include!("nss-bindings/include/nspr.rs");
// include!("nss-bindings/public/nss/nss.rs");
include!("nss-bindings/public/nss/ssl.rs");
use std::str;
use std::ffi::CString;
use std::ffi::CStr;
// use std::os::raw::c_char;
use std::ptr;
use std::process;
use std::mem;
use std::os::raw::c_void;
// use libc::size_t;

/*
 * Takes a raw *mut i8 pointer to a c-string and returns a safe rust string
 * or an emtpy string if it's a null pointer.
 */
fn get_string_mut(field: *mut i8) -> String {
    if field == 0 as *mut i8 {
        return String::new();
    }
    let f = unsafe{CStr::from_ptr(field)};
    return f.to_string_lossy().into_owned();
}
fn get_string_const(field: *const i8) -> String {
    return get_string_mut(field as *mut i8);
}

fn main() {
    /* NSS db test */
    println!("\n === access db test ===\n");
    let emtpy_string = CString::new("").unwrap().as_ptr() as *const i8;
    let db_dir = CString::new("/Users/franziskus/Code/playground/nss-rust/db").unwrap().as_ptr() as *const i8;
    let secmod_db = CString::new("secmod.db").unwrap().as_ptr() as *const i8;
    let x = unsafe{NSS_Initialize(db_dir, emtpy_string, emtpy_string, secmod_db, 0x01 /* NSS_INIT_READONLY */)};
    if x as u32 != 0 {
        println!("NSS_Initialize failed!");
        process::exit(1);
    }
    unsafe{NSS_SetDomesticPolicy()};
    let null_ptr = ptr::null_mut() as *mut c_void;
    let cert_nick = CString::new("server").unwrap().as_ptr() as *const i8;
    let cert = unsafe{PK11_FindCertFromNickname(cert_nick, null_ptr)};
    if cert != ptr::null_mut() as *mut CERTCertificateStr {
        let my_cert = unsafe{*cert};
        println!("san: {}", get_string_mut(my_cert.subjectName));
        println!("email: {}", get_string_mut(my_cert.emailAddr));
        println!("issuer: {}", get_string_mut(my_cert.issuerName));
        println!("nickname: {}", get_string_mut(my_cert.nickname));
        // c_int
        println!("series: {}", my_cert.series);
    }
    /* clean up some things */
    unsafe{CERT_DestroyCertificate(cert)};

    /* connection test */
    unsafe{PR_Init(PR_SYSTEM_THREAD, PR_PRIORITY_NORMAL, 1)};

    /* list ciphers */
    println!("\n === list ciphers test ===\n");
    let ciphers = unsafe{SSL_GetImplementedCiphers()};
    let num_ciphers = unsafe{SSL_GetNumImplementedCiphers()};
    for i in 0..2 /* num_ciphers */ {
        println!("{}", i);
        let mut info = SSLCipherSuiteInfo{..Default::default()};
        let inf = &mut info as *mut SSLCipherSuiteInfoStr;
        let s = mem::size_of::<SSLCipherSuiteInfo>() as u32;
        let c = unsafe{*ciphers.offset(i as isize)};
        let rv = unsafe{SSL_GetCipherSuiteInfo(c, inf, s)} as u32;
        if rv != 0 {
            println!("SSL_GetCipherSuiteInfo failed!");
            process::exit(1);
        }
        println!("name: {}",  get_string_const(info.cipherSuiteName));
        println!("algo: {}",  get_string_const(info.authAlgorithmName));
        println!("keyt: {}",  get_string_const(info.keaTypeName));
    }

    /* tear down */
    let done = unsafe{NSS_Shutdown()} as u32;
    if done != 0 {
        println!("NSS_Shutdown failed!");
        process::exit(1);
    }
}