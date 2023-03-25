#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/netinet.rs"));

/// in6_addr is different in GCC and MUSL
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub __in6_union: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub __u6_addr8: [u8; 16usize],
    pub __u6_addr16: [u16; 8usize],
    pub __u6_addr32: [u32; 4usize],
}
#[test]
fn bindgen_test_layout_in6_addr__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<in6_addr__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<in6_addr__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__u6_addr8) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__u6_addr16) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr16)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__u6_addr32) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr__bindgen_ty_1),
            "::",
            stringify!(__u6_addr32)
        )
    );
}
#[test]
fn bindgen_test_layout_in6_addr() {
    const UNINIT: ::std::mem::MaybeUninit<in6_addr> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<in6_addr>(),
        16usize,
        concat!("Size of: ", stringify!(in6_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<in6_addr>(),
        4usize,
        concat!("Alignment of ", stringify!(in6_addr))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__in6_union) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(in6_addr),
            "::",
            stringify!(__in6_u)
        )
    );
}
