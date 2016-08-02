/* automatically generated by rust-bindgen */

#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __clang_max_align_nonce2: ::std::os::raw::c_double,
}
impl ::std::default::Default for max_align_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type PRUint8 = ::std::os::raw::c_uchar;
pub type PRInt8 = ::std::os::raw::c_char;
pub type PRUint16 = ::std::os::raw::c_ushort;
pub type PRInt16 = ::std::os::raw::c_short;
pub type PRUint32 = ::std::os::raw::c_uint;
pub type PRInt32 = ::std::os::raw::c_int;
pub type PRInt64 = ::std::os::raw::c_long;
pub type PRUint64 = ::std::os::raw::c_ulong;
pub type PRIntn = ::std::os::raw::c_int;
pub type PRUintn = ::std::os::raw::c_uint;
pub type PRFloat64 = ::std::os::raw::c_double;
pub type PRSize = size_t;
pub type PROffset32 = PRInt32;
pub type PROffset64 = PRInt64;
pub type PRPtrdiff = ptrdiff_t;
pub type PRUptrdiff = ::std::os::raw::c_ulong;
pub type PRBool = PRIntn;
pub type PRPackedBool = PRUint8;
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum PRStatus { PR_FAILURE = -1, PR_SUCCESS = 0, }
pub type PRUnichar = PRUint16;
pub type PRWord = ::std::os::raw::c_long;
pub type PRUword = ::std::os::raw::c_ulong;
pub type uintn = PRUintn;
pub type intn = PRIntn;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_char;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
impl ::std::default::Default for __fsid_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __qaddr_t = *mut __quad_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = isize;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint_ = ::std::os::raw::c_uint;
pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulong;
pub type register_t = ::std::os::raw::c_long;
pub type __sig_atomic_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
impl ::std::default::Default for __sigset_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
impl ::std::default::Default for timespec {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
impl ::std::default::Default for timeval {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
impl ::std::default::Default for fd_set {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type fd_mask = __fd_mask;
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_attr_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl pthread_attr_t {
    pub unsafe fn __size(&mut self)
     -> *mut [::std::os::raw::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_attr_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for pthread_attr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl ::std::default::Default for __pthread_internal_list {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_mutex_t {
    pub _bindgen_data_: [u64; 5usize],
}
impl pthread_mutex_t {
    pub unsafe fn __data(&mut self) -> *mut __pthread_mutex_s {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self)
     -> *mut [::std::os::raw::c_char; 40usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_mutex_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for pthread_mutex_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
impl ::std::default::Default for __pthread_mutex_s {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pthread_mutexattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_mutexattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_mutexattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct pthread_cond_t {
    pub _bindgen_data_: [u64; 6usize],
}
impl pthread_cond_t {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed1 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self)
     -> *mut [::std::os::raw::c_char; 48usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_longlong {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_cond_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for pthread_cond_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed1 {
    pub __lock: ::std::os::raw::c_int,
    pub __futex: ::std::os::raw::c_uint,
    pub __total_seq: ::std::os::raw::c_ulonglong,
    pub __wakeup_seq: ::std::os::raw::c_ulonglong,
    pub __woken_seq: ::std::os::raw::c_ulonglong,
    pub __mutex: *mut ::std::os::raw::c_void,
    pub __nwaiters: ::std::os::raw::c_uint,
    pub __broadcast_seq: ::std::os::raw::c_uint,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pthread_condattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_condattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_condattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy)]
pub struct pthread_rwlock_t {
    pub _bindgen_data_: [u64; 7usize],
}
impl pthread_rwlock_t {
    pub unsafe fn __data(&mut self) -> *mut Struct_Unnamed2 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __size(&mut self)
     -> *mut [::std::os::raw::c_char; 56usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for pthread_rwlock_t {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for pthread_rwlock_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Struct_Unnamed2 {
    pub __lock: ::std::os::raw::c_int,
    pub __nr_readers: ::std::os::raw::c_uint,
    pub __readers_wakeup: ::std::os::raw::c_uint,
    pub __writer_wakeup: ::std::os::raw::c_uint,
    pub __nr_readers_queued: ::std::os::raw::c_uint,
    pub __nr_writers_queued: ::std::os::raw::c_uint,
    pub __writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_char,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
impl ::std::default::Default for Struct_Unnamed2 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pthread_rwlockattr_t {
    pub _bindgen_data_: [u64; 1usize],
}
impl pthread_rwlockattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 8usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_rwlockattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pthread_barrier_t {
    pub _bindgen_data_: [u64; 4usize],
}
impl pthread_barrier_t {
    pub unsafe fn __size(&mut self)
     -> *mut [::std::os::raw::c_char; 32usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_long {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_barrier_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pthread_barrierattr_t {
    pub _bindgen_data_: [u32; 1usize],
}
impl pthread_barrierattr_t {
    pub unsafe fn __size(&mut self) -> *mut [::std::os::raw::c_char; 4usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn __align(&mut self) -> *mut ::std::os::raw::c_int {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::default::Default for pthread_barrierattr_t {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type uint64 = PRUint64;
pub type uint32 = PRUint32;
pub type uint16 = PRUint16;
pub type uint8 = PRUint8;
pub type int64 = PRInt64;
pub type int32 = PRInt32;
pub type int16 = PRInt16;
pub type int8 = PRInt8;
pub type float64 = PRFloat64;
pub type uptrdiff_t = PRUptrdiff;
pub type uprword_t = PRUword;
pub type prword_t = PRWord;
pub const PR_LOG_NOTICE: PRLogModuleLevel = PRLogModuleLevel::PR_LOG_DEBUG;
pub const PR_LOG_WARN: PRLogModuleLevel = PRLogModuleLevel::PR_LOG_WARNING;
pub const PR_LOG_MIN: PRLogModuleLevel = PRLogModuleLevel::PR_LOG_DEBUG;
pub const PR_LOG_MAX: PRLogModuleLevel = PRLogModuleLevel::PR_LOG_DEBUG;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum PRLogModuleLevel {
    PR_LOG_NONE = 0,
    PR_LOG_ALWAYS = 1,
    PR_LOG_ERROR = 2,
    PR_LOG_WARNING = 3,
    PR_LOG_DEBUG = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct PRLogModuleInfo {
    pub name: *const ::std::os::raw::c_char,
    pub level: PRLogModuleLevel,
    pub next: *mut PRLogModuleInfo,
}
impl ::std::default::Default for PRLogModuleInfo {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
extern "C" {
    pub fn select(__nfds: ::std::os::raw::c_int, __readfds: *mut fd_set,
                  __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                  __timeout: *mut timeval) -> ::std::os::raw::c_int;
    pub fn pselect(__nfds: ::std::os::raw::c_int, __readfds: *mut fd_set,
                   __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                   __timeout: *const timespec, __sigmask: *const __sigset_t)
     -> ::std::os::raw::c_int;
    pub fn gnu_dev_major(__dev: ::std::os::raw::c_ulonglong)
     -> ::std::os::raw::c_uint;
    pub fn gnu_dev_minor(__dev: ::std::os::raw::c_ulonglong)
     -> ::std::os::raw::c_uint;
    pub fn gnu_dev_makedev(__major: ::std::os::raw::c_uint,
                           __minor: ::std::os::raw::c_uint)
     -> ::std::os::raw::c_ulonglong;
    pub fn PR_NewLogModule(name: *const ::std::os::raw::c_char)
     -> *mut PRLogModuleInfo;
    pub fn PR_SetLogFile(name: *const ::std::os::raw::c_char) -> PRBool;
    pub fn PR_SetLogBuffering(buffer_size: PRIntn);
    pub fn PR_LogPrint(fmt: *const ::std::os::raw::c_char, ...);
    pub fn PR_LogFlush();
    pub fn PR_Assert(s: *const ::std::os::raw::c_char,
                     file: *const ::std::os::raw::c_char, ln: PRIntn);
}
