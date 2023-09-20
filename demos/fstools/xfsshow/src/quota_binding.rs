/* automatically generated by rust-bindgen 0.68.1 */

pub const __BITS_PER_LONG: u32 = 64;
pub const __FD_SETSIZE: u32 = 1024;
pub const __DQUOT_VERSION__: &[u8; 12] = b"dquot_6.6.0\0";
pub const MAXQUOTAS: u32 = 3;
pub const USRQUOTA: u32 = 0;
pub const GRPQUOTA: u32 = 1;
pub const PRJQUOTA: u32 = 2;
pub const SUBCMDMASK: u32 = 255;
pub const SUBCMDSHIFT: u32 = 8;
pub const Q_SYNC: u32 = 8388609;
pub const Q_QUOTAON: u32 = 8388610;
pub const Q_QUOTAOFF: u32 = 8388611;
pub const Q_GETFMT: u32 = 8388612;
pub const Q_GETINFO: u32 = 8388613;
pub const Q_SETINFO: u32 = 8388614;
pub const Q_GETQUOTA: u32 = 8388615;
pub const Q_SETQUOTA: u32 = 8388616;
pub const Q_GETNEXTQUOTA: u32 = 8388617;
pub const QFMT_VFS_OLD: u32 = 1;
pub const QFMT_VFS_V0: u32 = 2;
pub const QFMT_OCFS2: u32 = 3;
pub const QFMT_VFS_V1: u32 = 4;
pub const QIF_DQBLKSIZE_BITS: u32 = 10;
pub const QIF_DQBLKSIZE: u32 = 1024;
pub const IIF_BGRACE: u32 = 1;
pub const IIF_IGRACE: u32 = 2;
pub const IIF_FLAGS: u32 = 4;
pub const IIF_ALL: u32 = 7;
pub const QUOTA_NL_NOWARN: u32 = 0;
pub const QUOTA_NL_IHARDWARN: u32 = 1;
pub const QUOTA_NL_ISOFTLONGWARN: u32 = 2;
pub const QUOTA_NL_ISOFTWARN: u32 = 3;
pub const QUOTA_NL_BHARDWARN: u32 = 4;
pub const QUOTA_NL_BSOFTLONGWARN: u32 = 5;
pub const QUOTA_NL_BSOFTWARN: u32 = 6;
pub const QUOTA_NL_IHARDBELOW: u32 = 7;
pub const QUOTA_NL_ISOFTBELOW: u32 = 8;
pub const QUOTA_NL_BHARDBELOW: u32 = 9;
pub const QUOTA_NL_BSOFTBELOW: u32 = 10;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    const UNINIT: ::std::mem::MaybeUninit<__kernel_fd_set> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fds_bits) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fd_set),
            "::",
            stringify!(fds_bits)
        )
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    const UNINIT: ::std::mem::MaybeUninit<__kernel_fsid_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__kernel_fsid_t),
            "::",
            stringify!(val)
        )
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = ::std::os::raw::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = ::std::os::raw::c_uint;
pub const QIF_BLIMITS_B: _bindgen_ty_1 = 0;
pub const QIF_SPACE_B: _bindgen_ty_1 = 1;
pub const QIF_ILIMITS_B: _bindgen_ty_1 = 2;
pub const QIF_INODES_B: _bindgen_ty_1 = 3;
pub const QIF_BTIME_B: _bindgen_ty_1 = 4;
pub const QIF_ITIME_B: _bindgen_ty_1 = 5;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct if_dqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
}
#[test]
fn bindgen_test_layout_if_dqblk() {
    const UNINIT: ::std::mem::MaybeUninit<if_dqblk> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<if_dqblk>(),
        72usize,
        concat!("Size of: ", stringify!(if_dqblk))
    );
    assert_eq!(
        ::std::mem::align_of::<if_dqblk>(),
        8usize,
        concat!("Alignment of ", stringify!(if_dqblk))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_bhardlimit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_bhardlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_bsoftlimit) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_bsoftlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_curspace) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_curspace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_ihardlimit) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_ihardlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_isoftlimit) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_isoftlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_curinodes) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_curinodes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_btime) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_btime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_itime) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_itime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_valid) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqblk),
            "::",
            stringify!(dqb_valid)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct if_nextdqblk {
    pub dqb_bhardlimit: __u64,
    pub dqb_bsoftlimit: __u64,
    pub dqb_curspace: __u64,
    pub dqb_ihardlimit: __u64,
    pub dqb_isoftlimit: __u64,
    pub dqb_curinodes: __u64,
    pub dqb_btime: __u64,
    pub dqb_itime: __u64,
    pub dqb_valid: __u32,
    pub dqb_id: __u32,
}
#[test]
fn bindgen_test_layout_if_nextdqblk() {
    const UNINIT: ::std::mem::MaybeUninit<if_nextdqblk> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<if_nextdqblk>(),
        72usize,
        concat!("Size of: ", stringify!(if_nextdqblk))
    );
    assert_eq!(
        ::std::mem::align_of::<if_nextdqblk>(),
        8usize,
        concat!("Alignment of ", stringify!(if_nextdqblk))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_bhardlimit) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_bhardlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_bsoftlimit) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_bsoftlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_curspace) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_curspace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_ihardlimit) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_ihardlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_isoftlimit) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_isoftlimit)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_curinodes) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_curinodes)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_btime) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_btime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_itime) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_itime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_valid) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_valid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqb_id) as usize - ptr as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(if_nextdqblk),
            "::",
            stringify!(dqb_id)
        )
    );
}
pub const DQF_ROOT_SQUASH_B: _bindgen_ty_2 = 0;
pub const DQF_SYS_FILE_B: _bindgen_ty_2 = 16;
pub const DQF_PRIVATE: _bindgen_ty_2 = 17;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct if_dqinfo {
    pub dqi_bgrace: __u64,
    pub dqi_igrace: __u64,
    pub dqi_flags: __u32,
    pub dqi_valid: __u32,
}
#[test]
fn bindgen_test_layout_if_dqinfo() {
    const UNINIT: ::std::mem::MaybeUninit<if_dqinfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<if_dqinfo>(),
        24usize,
        concat!("Size of: ", stringify!(if_dqinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<if_dqinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(if_dqinfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqi_bgrace) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqinfo),
            "::",
            stringify!(dqi_bgrace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqi_igrace) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqinfo),
            "::",
            stringify!(dqi_igrace)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqi_flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqinfo),
            "::",
            stringify!(dqi_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).dqi_valid) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(if_dqinfo),
            "::",
            stringify!(dqi_valid)
        )
    );
}
pub const QUOTA_NL_C_UNSPEC: _bindgen_ty_3 = 0;
pub const QUOTA_NL_C_WARNING: _bindgen_ty_3 = 1;
pub const __QUOTA_NL_C_MAX: _bindgen_ty_3 = 2;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const QUOTA_NL_A_UNSPEC: _bindgen_ty_4 = 0;
pub const QUOTA_NL_A_QTYPE: _bindgen_ty_4 = 1;
pub const QUOTA_NL_A_EXCESS_ID: _bindgen_ty_4 = 2;
pub const QUOTA_NL_A_WARNING: _bindgen_ty_4 = 3;
pub const QUOTA_NL_A_DEV_MAJOR: _bindgen_ty_4 = 4;
pub const QUOTA_NL_A_DEV_MINOR: _bindgen_ty_4 = 5;
pub const QUOTA_NL_A_CAUSED_ID: _bindgen_ty_4 = 6;
pub const QUOTA_NL_A_PAD: _bindgen_ty_4 = 7;
pub const __QUOTA_NL_A_MAX: _bindgen_ty_4 = 8;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
