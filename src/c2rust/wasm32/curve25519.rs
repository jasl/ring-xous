#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern "C" {
    fn GFp_memcmp(a: *const uint8_t, b: *const uint8_t, len: size_t) -> core::ffi::c_int;
    fn memcpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
    fn memset(
        _: *mut core::ffi::c_void,
        _: core::ffi::c_int,
        _: core::ffi::c_uint,
    ) -> *mut core::ffi::c_void;
}
pub type size_t = core::ffi::c_uint;
pub type __uint8_t = core::ffi::c_uchar;
pub type __int32_t = core::ffi::c_int;
pub type __uint32_t = core::ffi::c_uint;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type fe_limb_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fe {
    pub v: [fe_limb_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fe_loose {
    pub v: [fe_limb_t; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ge_p2 {
    pub X: fe,
    pub Y: fe,
    pub Z: fe,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ge_p3 {
    pub X: fe,
    pub Y: fe,
    pub Z: fe,
    pub T: fe,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ge_p1p1 {
    pub X: fe_loose,
    pub Y: fe_loose,
    pub Z: fe_loose,
    pub T: fe_loose,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ge_precomp {
    pub yplusx: fe_loose,
    pub yminusx: fe_loose,
    pub xy2d: fe_loose,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ge_cached {
    pub YplusX: fe_loose,
    pub YminusX: fe_loose,
    pub Z: fe_loose,
    pub T2d: fe_loose,
}
pub type fiat_25519_uint1 = core::ffi::c_uchar;
pub type fiat_25519_int1 = core::ffi::c_schar;
#[inline]
unsafe extern "C" fn value_barrier_u32(a: uint32_t) -> uint32_t {
    return a;
}
#[inline]
unsafe extern "C" fn fe_limbs_copy(r: *mut fe_limb_t, a: *const fe_limb_t) {
    let mut i: size_t = 0 as core::ffi::c_int as size_t;
    while i < 10 as core::ffi::c_int as core::ffi::c_uint {
        *r.offset(i as isize) = *a.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn GFp_memcpy(
    dst: *mut core::ffi::c_void,
    src: *const core::ffi::c_void,
    n: size_t,
) -> *mut core::ffi::c_void {
    if n == 0 as core::ffi::c_int as core::ffi::c_uint {
        return dst;
    }
    return memcpy(dst, src, n);
}
#[inline]
unsafe extern "C" fn GFp_memset(
    dst: *mut core::ffi::c_void,
    c: core::ffi::c_int,
    n: size_t,
) -> *mut core::ffi::c_void {
    if n == 0 as core::ffi::c_int as core::ffi::c_uint {
        return dst;
    }
    return memset(dst, c, n);
}
static mut d: fe = {
    let init = fe {
        v: [
            56195235 as core::ffi::c_int as fe_limb_t,
            13857412 as core::ffi::c_int as fe_limb_t,
            51736253 as core::ffi::c_int as fe_limb_t,
            6949390 as core::ffi::c_int as fe_limb_t,
            114729 as core::ffi::c_int as fe_limb_t,
            24766616 as core::ffi::c_int as fe_limb_t,
            60832955 as core::ffi::c_int as fe_limb_t,
            30306712 as core::ffi::c_int as fe_limb_t,
            48412415 as core::ffi::c_int as fe_limb_t,
            21499315 as core::ffi::c_int as fe_limb_t,
        ],
    };
    init
};
static mut sqrtm1: fe = {
    let init = fe {
        v: [
            34513072 as core::ffi::c_int as fe_limb_t,
            25610706 as core::ffi::c_int as fe_limb_t,
            9377949 as core::ffi::c_int as fe_limb_t,
            3500415 as core::ffi::c_int as fe_limb_t,
            12389472 as core::ffi::c_int as fe_limb_t,
            33281959 as core::ffi::c_int as fe_limb_t,
            41962654 as core::ffi::c_int as fe_limb_t,
            31548777 as core::ffi::c_int as fe_limb_t,
            326685 as core::ffi::c_int as fe_limb_t,
            11406482 as core::ffi::c_int as fe_limb_t,
        ],
    };
    init
};
static mut d2: fe = {
    let init = fe {
        v: [
            45281625 as core::ffi::c_int as fe_limb_t,
            27714825 as core::ffi::c_int as fe_limb_t,
            36363642 as core::ffi::c_int as fe_limb_t,
            13898781 as core::ffi::c_int as fe_limb_t,
            229458 as core::ffi::c_int as fe_limb_t,
            15978800 as core::ffi::c_int as fe_limb_t,
            54557047 as core::ffi::c_int as fe_limb_t,
            27058993 as core::ffi::c_int as fe_limb_t,
            29715967 as core::ffi::c_int as fe_limb_t,
            9444199 as core::ffi::c_int as fe_limb_t,
        ],
    };
    init
};
static mut k25519Precomp: [[ge_precomp; 8]; 32] = [
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            25967493 as core::ffi::c_int as fe_limb_t,
                            19198397 as core::ffi::c_int as fe_limb_t,
                            29566455 as core::ffi::c_int as fe_limb_t,
                            3660896 as core::ffi::c_int as fe_limb_t,
                            54414519 as core::ffi::c_int as fe_limb_t,
                            4014786 as core::ffi::c_int as fe_limb_t,
                            27544626 as core::ffi::c_int as fe_limb_t,
                            21800161 as core::ffi::c_int as fe_limb_t,
                            61029707 as core::ffi::c_int as fe_limb_t,
                            2047604 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54563134 as core::ffi::c_int as fe_limb_t,
                            934261 as core::ffi::c_int as fe_limb_t,
                            64385954 as core::ffi::c_int as fe_limb_t,
                            3049989 as core::ffi::c_int as fe_limb_t,
                            66381436 as core::ffi::c_int as fe_limb_t,
                            9406985 as core::ffi::c_int as fe_limb_t,
                            12720692 as core::ffi::c_int as fe_limb_t,
                            5043384 as core::ffi::c_int as fe_limb_t,
                            19500929 as core::ffi::c_int as fe_limb_t,
                            18085054 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            58370664 as core::ffi::c_int as fe_limb_t,
                            4489569 as core::ffi::c_int as fe_limb_t,
                            9688441 as core::ffi::c_int as fe_limb_t,
                            18769238 as core::ffi::c_int as fe_limb_t,
                            10184608 as core::ffi::c_int as fe_limb_t,
                            21191052 as core::ffi::c_int as fe_limb_t,
                            29287918 as core::ffi::c_int as fe_limb_t,
                            11864899 as core::ffi::c_int as fe_limb_t,
                            42594502 as core::ffi::c_int as fe_limb_t,
                            29115885 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54292951 as core::ffi::c_int as fe_limb_t,
                            20578084 as core::ffi::c_int as fe_limb_t,
                            45527620 as core::ffi::c_int as fe_limb_t,
                            11784319 as core::ffi::c_int as fe_limb_t,
                            41753206 as core::ffi::c_int as fe_limb_t,
                            30803714 as core::ffi::c_int as fe_limb_t,
                            55390960 as core::ffi::c_int as fe_limb_t,
                            29739860 as core::ffi::c_int as fe_limb_t,
                            66750418 as core::ffi::c_int as fe_limb_t,
                            23343128 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            45405608 as core::ffi::c_int as fe_limb_t,
                            6903824 as core::ffi::c_int as fe_limb_t,
                            27185491 as core::ffi::c_int as fe_limb_t,
                            6451973 as core::ffi::c_int as fe_limb_t,
                            37531140 as core::ffi::c_int as fe_limb_t,
                            24000426 as core::ffi::c_int as fe_limb_t,
                            51492312 as core::ffi::c_int as fe_limb_t,
                            11189267 as core::ffi::c_int as fe_limb_t,
                            40279186 as core::ffi::c_int as fe_limb_t,
                            28235350 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            26966623 as core::ffi::c_int as fe_limb_t,
                            11152617 as core::ffi::c_int as fe_limb_t,
                            32442495 as core::ffi::c_int as fe_limb_t,
                            15396054 as core::ffi::c_int as fe_limb_t,
                            14353839 as core::ffi::c_int as fe_limb_t,
                            20802097 as core::ffi::c_int as fe_limb_t,
                            63980037 as core::ffi::c_int as fe_limb_t,
                            24013313 as core::ffi::c_int as fe_limb_t,
                            51636816 as core::ffi::c_int as fe_limb_t,
                            29387734 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15636272 as core::ffi::c_int as fe_limb_t,
                            23865875 as core::ffi::c_int as fe_limb_t,
                            24204772 as core::ffi::c_int as fe_limb_t,
                            25642034 as core::ffi::c_int as fe_limb_t,
                            616976 as core::ffi::c_int as fe_limb_t,
                            16869170 as core::ffi::c_int as fe_limb_t,
                            27787599 as core::ffi::c_int as fe_limb_t,
                            18782243 as core::ffi::c_int as fe_limb_t,
                            28944399 as core::ffi::c_int as fe_limb_t,
                            32004408 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            16568933 as core::ffi::c_int as fe_limb_t,
                            4717097 as core::ffi::c_int as fe_limb_t,
                            55552716 as core::ffi::c_int as fe_limb_t,
                            32452109 as core::ffi::c_int as fe_limb_t,
                            15682895 as core::ffi::c_int as fe_limb_t,
                            21747389 as core::ffi::c_int as fe_limb_t,
                            16354576 as core::ffi::c_int as fe_limb_t,
                            21778470 as core::ffi::c_int as fe_limb_t,
                            7689661 as core::ffi::c_int as fe_limb_t,
                            11199574 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            30464137 as core::ffi::c_int as fe_limb_t,
                            27578307 as core::ffi::c_int as fe_limb_t,
                            55329429 as core::ffi::c_int as fe_limb_t,
                            17883566 as core::ffi::c_int as fe_limb_t,
                            23220364 as core::ffi::c_int as fe_limb_t,
                            15915852 as core::ffi::c_int as fe_limb_t,
                            7512774 as core::ffi::c_int as fe_limb_t,
                            10017326 as core::ffi::c_int as fe_limb_t,
                            49359771 as core::ffi::c_int as fe_limb_t,
                            23634074 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50071967 as core::ffi::c_int as fe_limb_t,
                            13921891 as core::ffi::c_int as fe_limb_t,
                            10945806 as core::ffi::c_int as fe_limb_t,
                            27521001 as core::ffi::c_int as fe_limb_t,
                            27105051 as core::ffi::c_int as fe_limb_t,
                            17470053 as core::ffi::c_int as fe_limb_t,
                            38182653 as core::ffi::c_int as fe_limb_t,
                            15006022 as core::ffi::c_int as fe_limb_t,
                            3284568 as core::ffi::c_int as fe_limb_t,
                            27277892 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            23599295 as core::ffi::c_int as fe_limb_t,
                            25248385 as core::ffi::c_int as fe_limb_t,
                            55915199 as core::ffi::c_int as fe_limb_t,
                            25867015 as core::ffi::c_int as fe_limb_t,
                            13236773 as core::ffi::c_int as fe_limb_t,
                            10506355 as core::ffi::c_int as fe_limb_t,
                            7464579 as core::ffi::c_int as fe_limb_t,
                            9656445 as core::ffi::c_int as fe_limb_t,
                            13059162 as core::ffi::c_int as fe_limb_t,
                            10374397 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            7798537 as core::ffi::c_int as fe_limb_t,
                            16710257 as core::ffi::c_int as fe_limb_t,
                            3033922 as core::ffi::c_int as fe_limb_t,
                            2874086 as core::ffi::c_int as fe_limb_t,
                            28997861 as core::ffi::c_int as fe_limb_t,
                            2835604 as core::ffi::c_int as fe_limb_t,
                            32406664 as core::ffi::c_int as fe_limb_t,
                            29715387 as core::ffi::c_int as fe_limb_t,
                            66467155 as core::ffi::c_int as fe_limb_t,
                            33453106 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            10861363 as core::ffi::c_int as fe_limb_t,
                            11473154 as core::ffi::c_int as fe_limb_t,
                            27284546 as core::ffi::c_int as fe_limb_t,
                            1981175 as core::ffi::c_int as fe_limb_t,
                            37044515 as core::ffi::c_int as fe_limb_t,
                            12577860 as core::ffi::c_int as fe_limb_t,
                            32867885 as core::ffi::c_int as fe_limb_t,
                            14515107 as core::ffi::c_int as fe_limb_t,
                            51670560 as core::ffi::c_int as fe_limb_t,
                            10819379 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            4708026 as core::ffi::c_int as fe_limb_t,
                            6336745 as core::ffi::c_int as fe_limb_t,
                            20377586 as core::ffi::c_int as fe_limb_t,
                            9066809 as core::ffi::c_int as fe_limb_t,
                            55836755 as core::ffi::c_int as fe_limb_t,
                            6594695 as core::ffi::c_int as fe_limb_t,
                            41455196 as core::ffi::c_int as fe_limb_t,
                            12483687 as core::ffi::c_int as fe_limb_t,
                            54440373 as core::ffi::c_int as fe_limb_t,
                            5581305 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19563141 as core::ffi::c_int as fe_limb_t,
                            16186464 as core::ffi::c_int as fe_limb_t,
                            37722007 as core::ffi::c_int as fe_limb_t,
                            4097518 as core::ffi::c_int as fe_limb_t,
                            10237984 as core::ffi::c_int as fe_limb_t,
                            29206317 as core::ffi::c_int as fe_limb_t,
                            28542349 as core::ffi::c_int as fe_limb_t,
                            13850243 as core::ffi::c_int as fe_limb_t,
                            43430843 as core::ffi::c_int as fe_limb_t,
                            17738489 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            51736881 as core::ffi::c_int as fe_limb_t,
                            20691677 as core::ffi::c_int as fe_limb_t,
                            32573249 as core::ffi::c_int as fe_limb_t,
                            4720197 as core::ffi::c_int as fe_limb_t,
                            40672342 as core::ffi::c_int as fe_limb_t,
                            5875510 as core::ffi::c_int as fe_limb_t,
                            47920237 as core::ffi::c_int as fe_limb_t,
                            18329612 as core::ffi::c_int as fe_limb_t,
                            57289923 as core::ffi::c_int as fe_limb_t,
                            21468654 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            58559652 as core::ffi::c_int as fe_limb_t,
                            109982 as core::ffi::c_int as fe_limb_t,
                            15149363 as core::ffi::c_int as fe_limb_t,
                            2178705 as core::ffi::c_int as fe_limb_t,
                            22900618 as core::ffi::c_int as fe_limb_t,
                            4543417 as core::ffi::c_int as fe_limb_t,
                            3044240 as core::ffi::c_int as fe_limb_t,
                            17864545 as core::ffi::c_int as fe_limb_t,
                            1762327 as core::ffi::c_int as fe_limb_t,
                            14866737 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            48909169 as core::ffi::c_int as fe_limb_t,
                            17603008 as core::ffi::c_int as fe_limb_t,
                            56635573 as core::ffi::c_int as fe_limb_t,
                            1707277 as core::ffi::c_int as fe_limb_t,
                            49922944 as core::ffi::c_int as fe_limb_t,
                            3916100 as core::ffi::c_int as fe_limb_t,
                            38872452 as core::ffi::c_int as fe_limb_t,
                            3959420 as core::ffi::c_int as fe_limb_t,
                            27914454 as core::ffi::c_int as fe_limb_t,
                            4383652 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            5153727 as core::ffi::c_int as fe_limb_t,
                            9909285 as core::ffi::c_int as fe_limb_t,
                            1723747 as core::ffi::c_int as fe_limb_t,
                            30776558 as core::ffi::c_int as fe_limb_t,
                            30523604 as core::ffi::c_int as fe_limb_t,
                            5516873 as core::ffi::c_int as fe_limb_t,
                            19480852 as core::ffi::c_int as fe_limb_t,
                            5230134 as core::ffi::c_int as fe_limb_t,
                            43156425 as core::ffi::c_int as fe_limb_t,
                            18378665 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            36839857 as core::ffi::c_int as fe_limb_t,
                            30090922 as core::ffi::c_int as fe_limb_t,
                            7665485 as core::ffi::c_int as fe_limb_t,
                            10083793 as core::ffi::c_int as fe_limb_t,
                            28475525 as core::ffi::c_int as fe_limb_t,
                            1649722 as core::ffi::c_int as fe_limb_t,
                            20654025 as core::ffi::c_int as fe_limb_t,
                            16520125 as core::ffi::c_int as fe_limb_t,
                            30598449 as core::ffi::c_int as fe_limb_t,
                            7715701 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28881826 as core::ffi::c_int as fe_limb_t,
                            14381568 as core::ffi::c_int as fe_limb_t,
                            9657904 as core::ffi::c_int as fe_limb_t,
                            3680757 as core::ffi::c_int as fe_limb_t,
                            46927229 as core::ffi::c_int as fe_limb_t,
                            7843315 as core::ffi::c_int as fe_limb_t,
                            35708204 as core::ffi::c_int as fe_limb_t,
                            1370707 as core::ffi::c_int as fe_limb_t,
                            29794553 as core::ffi::c_int as fe_limb_t,
                            32145132 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14499471 as core::ffi::c_int as fe_limb_t,
                            30824833 as core::ffi::c_int as fe_limb_t,
                            33917750 as core::ffi::c_int as fe_limb_t,
                            29299779 as core::ffi::c_int as fe_limb_t,
                            28494861 as core::ffi::c_int as fe_limb_t,
                            14271267 as core::ffi::c_int as fe_limb_t,
                            30290735 as core::ffi::c_int as fe_limb_t,
                            10876454 as core::ffi::c_int as fe_limb_t,
                            33954766 as core::ffi::c_int as fe_limb_t,
                            2381725 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            59913433 as core::ffi::c_int as fe_limb_t,
                            30899068 as core::ffi::c_int as fe_limb_t,
                            52378708 as core::ffi::c_int as fe_limb_t,
                            462250 as core::ffi::c_int as fe_limb_t,
                            39384538 as core::ffi::c_int as fe_limb_t,
                            3941371 as core::ffi::c_int as fe_limb_t,
                            60872247 as core::ffi::c_int as fe_limb_t,
                            3696004 as core::ffi::c_int as fe_limb_t,
                            34808032 as core::ffi::c_int as fe_limb_t,
                            15351954 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            27431194 as core::ffi::c_int as fe_limb_t,
                            8222322 as core::ffi::c_int as fe_limb_t,
                            16448760 as core::ffi::c_int as fe_limb_t,
                            29646437 as core::ffi::c_int as fe_limb_t,
                            48401861 as core::ffi::c_int as fe_limb_t,
                            11938354 as core::ffi::c_int as fe_limb_t,
                            34147463 as core::ffi::c_int as fe_limb_t,
                            30583916 as core::ffi::c_int as fe_limb_t,
                            29551812 as core::ffi::c_int as fe_limb_t,
                            10109425 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            53451805 as core::ffi::c_int as fe_limb_t,
                            20399000 as core::ffi::c_int as fe_limb_t,
                            35825113 as core::ffi::c_int as fe_limb_t,
                            11777097 as core::ffi::c_int as fe_limb_t,
                            21447386 as core::ffi::c_int as fe_limb_t,
                            6519384 as core::ffi::c_int as fe_limb_t,
                            64730580 as core::ffi::c_int as fe_limb_t,
                            31926875 as core::ffi::c_int as fe_limb_t,
                            10092782 as core::ffi::c_int as fe_limb_t,
                            28790261 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            27939166 as core::ffi::c_int as fe_limb_t,
                            14210322 as core::ffi::c_int as fe_limb_t,
                            4677035 as core::ffi::c_int as fe_limb_t,
                            16277044 as core::ffi::c_int as fe_limb_t,
                            44144402 as core::ffi::c_int as fe_limb_t,
                            21156292 as core::ffi::c_int as fe_limb_t,
                            34600109 as core::ffi::c_int as fe_limb_t,
                            12005537 as core::ffi::c_int as fe_limb_t,
                            49298737 as core::ffi::c_int as fe_limb_t,
                            12803509 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            17228999 as core::ffi::c_int as fe_limb_t,
                            17892808 as core::ffi::c_int as fe_limb_t,
                            65875336 as core::ffi::c_int as fe_limb_t,
                            300139 as core::ffi::c_int as fe_limb_t,
                            65883994 as core::ffi::c_int as fe_limb_t,
                            21839654 as core::ffi::c_int as fe_limb_t,
                            30364212 as core::ffi::c_int as fe_limb_t,
                            24516238 as core::ffi::c_int as fe_limb_t,
                            18016356 as core::ffi::c_int as fe_limb_t,
                            4397660 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            56150021 as core::ffi::c_int as fe_limb_t,
                            25864224 as core::ffi::c_int as fe_limb_t,
                            4776340 as core::ffi::c_int as fe_limb_t,
                            18600194 as core::ffi::c_int as fe_limb_t,
                            27850027 as core::ffi::c_int as fe_limb_t,
                            17952220 as core::ffi::c_int as fe_limb_t,
                            40489757 as core::ffi::c_int as fe_limb_t,
                            14544524 as core::ffi::c_int as fe_limb_t,
                            49631360 as core::ffi::c_int as fe_limb_t,
                            982638 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29253598 as core::ffi::c_int as fe_limb_t,
                            15796703 as core::ffi::c_int as fe_limb_t,
                            64244882 as core::ffi::c_int as fe_limb_t,
                            23645547 as core::ffi::c_int as fe_limb_t,
                            10057022 as core::ffi::c_int as fe_limb_t,
                            3163536 as core::ffi::c_int as fe_limb_t,
                            7332899 as core::ffi::c_int as fe_limb_t,
                            29434304 as core::ffi::c_int as fe_limb_t,
                            46061167 as core::ffi::c_int as fe_limb_t,
                            9934962 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            5793284 as core::ffi::c_int as fe_limb_t,
                            16271923 as core::ffi::c_int as fe_limb_t,
                            42977250 as core::ffi::c_int as fe_limb_t,
                            23438027 as core::ffi::c_int as fe_limb_t,
                            29188559 as core::ffi::c_int as fe_limb_t,
                            1206517 as core::ffi::c_int as fe_limb_t,
                            52360934 as core::ffi::c_int as fe_limb_t,
                            4559894 as core::ffi::c_int as fe_limb_t,
                            36984942 as core::ffi::c_int as fe_limb_t,
                            22656481 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            39464912 as core::ffi::c_int as fe_limb_t,
                            22061425 as core::ffi::c_int as fe_limb_t,
                            16282656 as core::ffi::c_int as fe_limb_t,
                            22517939 as core::ffi::c_int as fe_limb_t,
                            28414020 as core::ffi::c_int as fe_limb_t,
                            18542168 as core::ffi::c_int as fe_limb_t,
                            24191033 as core::ffi::c_int as fe_limb_t,
                            4541697 as core::ffi::c_int as fe_limb_t,
                            53770555 as core::ffi::c_int as fe_limb_t,
                            5500567 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            12650548 as core::ffi::c_int as fe_limb_t,
                            32057319 as core::ffi::c_int as fe_limb_t,
                            9052870 as core::ffi::c_int as fe_limb_t,
                            11355358 as core::ffi::c_int as fe_limb_t,
                            49428827 as core::ffi::c_int as fe_limb_t,
                            25154267 as core::ffi::c_int as fe_limb_t,
                            49678271 as core::ffi::c_int as fe_limb_t,
                            12264342 as core::ffi::c_int as fe_limb_t,
                            10874051 as core::ffi::c_int as fe_limb_t,
                            13524335 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            25556948 as core::ffi::c_int as fe_limb_t,
                            30508442 as core::ffi::c_int as fe_limb_t,
                            714650 as core::ffi::c_int as fe_limb_t,
                            2510400 as core::ffi::c_int as fe_limb_t,
                            23394682 as core::ffi::c_int as fe_limb_t,
                            23139102 as core::ffi::c_int as fe_limb_t,
                            33119037 as core::ffi::c_int as fe_limb_t,
                            5080568 as core::ffi::c_int as fe_limb_t,
                            44580805 as core::ffi::c_int as fe_limb_t,
                            5376627 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41020600 as core::ffi::c_int as fe_limb_t,
                            29543379 as core::ffi::c_int as fe_limb_t,
                            50095164 as core::ffi::c_int as fe_limb_t,
                            30016803 as core::ffi::c_int as fe_limb_t,
                            60382070 as core::ffi::c_int as fe_limb_t,
                            1920896 as core::ffi::c_int as fe_limb_t,
                            44787559 as core::ffi::c_int as fe_limb_t,
                            24106988 as core::ffi::c_int as fe_limb_t,
                            4535767 as core::ffi::c_int as fe_limb_t,
                            1569007 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            64853442 as core::ffi::c_int as fe_limb_t,
                            14606629 as core::ffi::c_int as fe_limb_t,
                            45416424 as core::ffi::c_int as fe_limb_t,
                            25514613 as core::ffi::c_int as fe_limb_t,
                            28430648 as core::ffi::c_int as fe_limb_t,
                            8775819 as core::ffi::c_int as fe_limb_t,
                            36614302 as core::ffi::c_int as fe_limb_t,
                            3044289 as core::ffi::c_int as fe_limb_t,
                            31848280 as core::ffi::c_int as fe_limb_t,
                            12543772 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45080285 as core::ffi::c_int as fe_limb_t,
                            2943892 as core::ffi::c_int as fe_limb_t,
                            35251351 as core::ffi::c_int as fe_limb_t,
                            6777305 as core::ffi::c_int as fe_limb_t,
                            13784462 as core::ffi::c_int as fe_limb_t,
                            29262229 as core::ffi::c_int as fe_limb_t,
                            39731668 as core::ffi::c_int as fe_limb_t,
                            31491700 as core::ffi::c_int as fe_limb_t,
                            7718481 as core::ffi::c_int as fe_limb_t,
                            14474653 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            2385296 as core::ffi::c_int as fe_limb_t,
                            2454213 as core::ffi::c_int as fe_limb_t,
                            44477544 as core::ffi::c_int as fe_limb_t,
                            46602 as core::ffi::c_int as fe_limb_t,
                            62670929 as core::ffi::c_int as fe_limb_t,
                            17874016 as core::ffi::c_int as fe_limb_t,
                            656964 as core::ffi::c_int as fe_limb_t,
                            26317767 as core::ffi::c_int as fe_limb_t,
                            24316167 as core::ffi::c_int as fe_limb_t,
                            28300865 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            13741529 as core::ffi::c_int as fe_limb_t,
                            10911568 as core::ffi::c_int as fe_limb_t,
                            33875447 as core::ffi::c_int as fe_limb_t,
                            24950694 as core::ffi::c_int as fe_limb_t,
                            46931033 as core::ffi::c_int as fe_limb_t,
                            32521134 as core::ffi::c_int as fe_limb_t,
                            33040650 as core::ffi::c_int as fe_limb_t,
                            20129900 as core::ffi::c_int as fe_limb_t,
                            46379407 as core::ffi::c_int as fe_limb_t,
                            8321685 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21060490 as core::ffi::c_int as fe_limb_t,
                            31341688 as core::ffi::c_int as fe_limb_t,
                            15712756 as core::ffi::c_int as fe_limb_t,
                            29218333 as core::ffi::c_int as fe_limb_t,
                            1639039 as core::ffi::c_int as fe_limb_t,
                            10656336 as core::ffi::c_int as fe_limb_t,
                            23845965 as core::ffi::c_int as fe_limb_t,
                            21679594 as core::ffi::c_int as fe_limb_t,
                            57124405 as core::ffi::c_int as fe_limb_t,
                            608371 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            53436132 as core::ffi::c_int as fe_limb_t,
                            18466845 as core::ffi::c_int as fe_limb_t,
                            56219170 as core::ffi::c_int as fe_limb_t,
                            25997372 as core::ffi::c_int as fe_limb_t,
                            61071954 as core::ffi::c_int as fe_limb_t,
                            11305546 as core::ffi::c_int as fe_limb_t,
                            1123968 as core::ffi::c_int as fe_limb_t,
                            26773855 as core::ffi::c_int as fe_limb_t,
                            27229398 as core::ffi::c_int as fe_limb_t,
                            23887 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            43864724 as core::ffi::c_int as fe_limb_t,
                            33260226 as core::ffi::c_int as fe_limb_t,
                            55364135 as core::ffi::c_int as fe_limb_t,
                            14712570 as core::ffi::c_int as fe_limb_t,
                            37643165 as core::ffi::c_int as fe_limb_t,
                            31524814 as core::ffi::c_int as fe_limb_t,
                            12797023 as core::ffi::c_int as fe_limb_t,
                            27114124 as core::ffi::c_int as fe_limb_t,
                            65475458 as core::ffi::c_int as fe_limb_t,
                            16678953 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            37608244 as core::ffi::c_int as fe_limb_t,
                            4770661 as core::ffi::c_int as fe_limb_t,
                            51054477 as core::ffi::c_int as fe_limb_t,
                            14001337 as core::ffi::c_int as fe_limb_t,
                            7830047 as core::ffi::c_int as fe_limb_t,
                            9564805 as core::ffi::c_int as fe_limb_t,
                            65600720 as core::ffi::c_int as fe_limb_t,
                            28759386 as core::ffi::c_int as fe_limb_t,
                            49939598 as core::ffi::c_int as fe_limb_t,
                            4904952 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            24059538 as core::ffi::c_int as fe_limb_t,
                            14617003 as core::ffi::c_int as fe_limb_t,
                            19037157 as core::ffi::c_int as fe_limb_t,
                            18514524 as core::ffi::c_int as fe_limb_t,
                            19766092 as core::ffi::c_int as fe_limb_t,
                            18648003 as core::ffi::c_int as fe_limb_t,
                            5169210 as core::ffi::c_int as fe_limb_t,
                            16191880 as core::ffi::c_int as fe_limb_t,
                            2128236 as core::ffi::c_int as fe_limb_t,
                            29227599 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            50127693 as core::ffi::c_int as fe_limb_t,
                            4124965 as core::ffi::c_int as fe_limb_t,
                            58568254 as core::ffi::c_int as fe_limb_t,
                            22900634 as core::ffi::c_int as fe_limb_t,
                            30336521 as core::ffi::c_int as fe_limb_t,
                            19449185 as core::ffi::c_int as fe_limb_t,
                            37302527 as core::ffi::c_int as fe_limb_t,
                            916032 as core::ffi::c_int as fe_limb_t,
                            60226322 as core::ffi::c_int as fe_limb_t,
                            30567899 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            44477957 as core::ffi::c_int as fe_limb_t,
                            12419371 as core::ffi::c_int as fe_limb_t,
                            59974635 as core::ffi::c_int as fe_limb_t,
                            26081060 as core::ffi::c_int as fe_limb_t,
                            50629959 as core::ffi::c_int as fe_limb_t,
                            16739174 as core::ffi::c_int as fe_limb_t,
                            285431 as core::ffi::c_int as fe_limb_t,
                            2763829 as core::ffi::c_int as fe_limb_t,
                            15736322 as core::ffi::c_int as fe_limb_t,
                            4143876 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            2379333 as core::ffi::c_int as fe_limb_t,
                            11839345 as core::ffi::c_int as fe_limb_t,
                            62998462 as core::ffi::c_int as fe_limb_t,
                            27565766 as core::ffi::c_int as fe_limb_t,
                            11274297 as core::ffi::c_int as fe_limb_t,
                            794957 as core::ffi::c_int as fe_limb_t,
                            212801 as core::ffi::c_int as fe_limb_t,
                            18959769 as core::ffi::c_int as fe_limb_t,
                            23527083 as core::ffi::c_int as fe_limb_t,
                            17096164 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            33431108 as core::ffi::c_int as fe_limb_t,
                            22423954 as core::ffi::c_int as fe_limb_t,
                            49269897 as core::ffi::c_int as fe_limb_t,
                            17927531 as core::ffi::c_int as fe_limb_t,
                            8909498 as core::ffi::c_int as fe_limb_t,
                            8376530 as core::ffi::c_int as fe_limb_t,
                            34483524 as core::ffi::c_int as fe_limb_t,
                            4087880 as core::ffi::c_int as fe_limb_t,
                            51919953 as core::ffi::c_int as fe_limb_t,
                            19138217 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            1767664 as core::ffi::c_int as fe_limb_t,
                            7197987 as core::ffi::c_int as fe_limb_t,
                            53903638 as core::ffi::c_int as fe_limb_t,
                            31531796 as core::ffi::c_int as fe_limb_t,
                            54017513 as core::ffi::c_int as fe_limb_t,
                            448825 as core::ffi::c_int as fe_limb_t,
                            5799055 as core::ffi::c_int as fe_limb_t,
                            4357868 as core::ffi::c_int as fe_limb_t,
                            62334673 as core::ffi::c_int as fe_limb_t,
                            17231393 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            6721966 as core::ffi::c_int as fe_limb_t,
                            13833823 as core::ffi::c_int as fe_limb_t,
                            43585476 as core::ffi::c_int as fe_limb_t,
                            32003117 as core::ffi::c_int as fe_limb_t,
                            26354292 as core::ffi::c_int as fe_limb_t,
                            21691111 as core::ffi::c_int as fe_limb_t,
                            23365146 as core::ffi::c_int as fe_limb_t,
                            29604700 as core::ffi::c_int as fe_limb_t,
                            7390889 as core::ffi::c_int as fe_limb_t,
                            2759800 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            4409022 as core::ffi::c_int as fe_limb_t,
                            2052381 as core::ffi::c_int as fe_limb_t,
                            23373853 as core::ffi::c_int as fe_limb_t,
                            10530217 as core::ffi::c_int as fe_limb_t,
                            7676779 as core::ffi::c_int as fe_limb_t,
                            20668478 as core::ffi::c_int as fe_limb_t,
                            21302352 as core::ffi::c_int as fe_limb_t,
                            29290375 as core::ffi::c_int as fe_limb_t,
                            1244379 as core::ffi::c_int as fe_limb_t,
                            20634787 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            62687625 as core::ffi::c_int as fe_limb_t,
                            7169618 as core::ffi::c_int as fe_limb_t,
                            4982368 as core::ffi::c_int as fe_limb_t,
                            30596842 as core::ffi::c_int as fe_limb_t,
                            30256824 as core::ffi::c_int as fe_limb_t,
                            30776892 as core::ffi::c_int as fe_limb_t,
                            14086412 as core::ffi::c_int as fe_limb_t,
                            9208236 as core::ffi::c_int as fe_limb_t,
                            15886429 as core::ffi::c_int as fe_limb_t,
                            16489664 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            1996056 as core::ffi::c_int as fe_limb_t,
                            10375649 as core::ffi::c_int as fe_limb_t,
                            14346367 as core::ffi::c_int as fe_limb_t,
                            13311202 as core::ffi::c_int as fe_limb_t,
                            60234729 as core::ffi::c_int as fe_limb_t,
                            17116020 as core::ffi::c_int as fe_limb_t,
                            53415665 as core::ffi::c_int as fe_limb_t,
                            398368 as core::ffi::c_int as fe_limb_t,
                            36502409 as core::ffi::c_int as fe_limb_t,
                            32841498 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            41801399 as core::ffi::c_int as fe_limb_t,
                            9795879 as core::ffi::c_int as fe_limb_t,
                            64331450 as core::ffi::c_int as fe_limb_t,
                            14878808 as core::ffi::c_int as fe_limb_t,
                            33577029 as core::ffi::c_int as fe_limb_t,
                            14780362 as core::ffi::c_int as fe_limb_t,
                            13348553 as core::ffi::c_int as fe_limb_t,
                            12076947 as core::ffi::c_int as fe_limb_t,
                            36272402 as core::ffi::c_int as fe_limb_t,
                            5113181 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49338080 as core::ffi::c_int as fe_limb_t,
                            11797795 as core::ffi::c_int as fe_limb_t,
                            31950843 as core::ffi::c_int as fe_limb_t,
                            13929123 as core::ffi::c_int as fe_limb_t,
                            41220562 as core::ffi::c_int as fe_limb_t,
                            12288343 as core::ffi::c_int as fe_limb_t,
                            36767763 as core::ffi::c_int as fe_limb_t,
                            26218045 as core::ffi::c_int as fe_limb_t,
                            13847710 as core::ffi::c_int as fe_limb_t,
                            5387222 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            48526701 as core::ffi::c_int as fe_limb_t,
                            30138214 as core::ffi::c_int as fe_limb_t,
                            17824842 as core::ffi::c_int as fe_limb_t,
                            31213466 as core::ffi::c_int as fe_limb_t,
                            22744342 as core::ffi::c_int as fe_limb_t,
                            23111821 as core::ffi::c_int as fe_limb_t,
                            8763060 as core::ffi::c_int as fe_limb_t,
                            3617786 as core::ffi::c_int as fe_limb_t,
                            47508202 as core::ffi::c_int as fe_limb_t,
                            10370990 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            20246567 as core::ffi::c_int as fe_limb_t,
                            19185054 as core::ffi::c_int as fe_limb_t,
                            22358228 as core::ffi::c_int as fe_limb_t,
                            33010720 as core::ffi::c_int as fe_limb_t,
                            18507282 as core::ffi::c_int as fe_limb_t,
                            23140436 as core::ffi::c_int as fe_limb_t,
                            14554436 as core::ffi::c_int as fe_limb_t,
                            24808340 as core::ffi::c_int as fe_limb_t,
                            32232923 as core::ffi::c_int as fe_limb_t,
                            16763880 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            9648486 as core::ffi::c_int as fe_limb_t,
                            10094563 as core::ffi::c_int as fe_limb_t,
                            26416693 as core::ffi::c_int as fe_limb_t,
                            14745928 as core::ffi::c_int as fe_limb_t,
                            36734546 as core::ffi::c_int as fe_limb_t,
                            27081810 as core::ffi::c_int as fe_limb_t,
                            11094160 as core::ffi::c_int as fe_limb_t,
                            15689506 as core::ffi::c_int as fe_limb_t,
                            3140038 as core::ffi::c_int as fe_limb_t,
                            17044340 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50948792 as core::ffi::c_int as fe_limb_t,
                            5472694 as core::ffi::c_int as fe_limb_t,
                            31895588 as core::ffi::c_int as fe_limb_t,
                            4744994 as core::ffi::c_int as fe_limb_t,
                            8823515 as core::ffi::c_int as fe_limb_t,
                            10365685 as core::ffi::c_int as fe_limb_t,
                            39884064 as core::ffi::c_int as fe_limb_t,
                            9448612 as core::ffi::c_int as fe_limb_t,
                            38334410 as core::ffi::c_int as fe_limb_t,
                            366294 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            19153450 as core::ffi::c_int as fe_limb_t,
                            11523972 as core::ffi::c_int as fe_limb_t,
                            56012374 as core::ffi::c_int as fe_limb_t,
                            27051289 as core::ffi::c_int as fe_limb_t,
                            42461232 as core::ffi::c_int as fe_limb_t,
                            5420646 as core::ffi::c_int as fe_limb_t,
                            28344573 as core::ffi::c_int as fe_limb_t,
                            8041113 as core::ffi::c_int as fe_limb_t,
                            719605 as core::ffi::c_int as fe_limb_t,
                            11671788 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            8678006 as core::ffi::c_int as fe_limb_t,
                            2694440 as core::ffi::c_int as fe_limb_t,
                            60300850 as core::ffi::c_int as fe_limb_t,
                            2517371 as core::ffi::c_int as fe_limb_t,
                            4964326 as core::ffi::c_int as fe_limb_t,
                            11152271 as core::ffi::c_int as fe_limb_t,
                            51675948 as core::ffi::c_int as fe_limb_t,
                            18287915 as core::ffi::c_int as fe_limb_t,
                            27000812 as core::ffi::c_int as fe_limb_t,
                            23358879 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            51950941 as core::ffi::c_int as fe_limb_t,
                            7134311 as core::ffi::c_int as fe_limb_t,
                            8639287 as core::ffi::c_int as fe_limb_t,
                            30739555 as core::ffi::c_int as fe_limb_t,
                            59873175 as core::ffi::c_int as fe_limb_t,
                            10421741 as core::ffi::c_int as fe_limb_t,
                            564065 as core::ffi::c_int as fe_limb_t,
                            5336097 as core::ffi::c_int as fe_limb_t,
                            6750977 as core::ffi::c_int as fe_limb_t,
                            19033406 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11836410 as core::ffi::c_int as fe_limb_t,
                            29574944 as core::ffi::c_int as fe_limb_t,
                            26297893 as core::ffi::c_int as fe_limb_t,
                            16080799 as core::ffi::c_int as fe_limb_t,
                            23455045 as core::ffi::c_int as fe_limb_t,
                            15735944 as core::ffi::c_int as fe_limb_t,
                            1695823 as core::ffi::c_int as fe_limb_t,
                            24735310 as core::ffi::c_int as fe_limb_t,
                            8169719 as core::ffi::c_int as fe_limb_t,
                            16220347 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            48993007 as core::ffi::c_int as fe_limb_t,
                            8653646 as core::ffi::c_int as fe_limb_t,
                            17578566 as core::ffi::c_int as fe_limb_t,
                            27461813 as core::ffi::c_int as fe_limb_t,
                            59083086 as core::ffi::c_int as fe_limb_t,
                            17541668 as core::ffi::c_int as fe_limb_t,
                            55964556 as core::ffi::c_int as fe_limb_t,
                            30926767 as core::ffi::c_int as fe_limb_t,
                            61118155 as core::ffi::c_int as fe_limb_t,
                            19388398 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43800366 as core::ffi::c_int as fe_limb_t,
                            22586119 as core::ffi::c_int as fe_limb_t,
                            15213227 as core::ffi::c_int as fe_limb_t,
                            23473218 as core::ffi::c_int as fe_limb_t,
                            36255258 as core::ffi::c_int as fe_limb_t,
                            22504427 as core::ffi::c_int as fe_limb_t,
                            27884328 as core::ffi::c_int as fe_limb_t,
                            2847284 as core::ffi::c_int as fe_limb_t,
                            2655861 as core::ffi::c_int as fe_limb_t,
                            1738395 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            39571412 as core::ffi::c_int as fe_limb_t,
                            19301410 as core::ffi::c_int as fe_limb_t,
                            41772562 as core::ffi::c_int as fe_limb_t,
                            25551651 as core::ffi::c_int as fe_limb_t,
                            57738101 as core::ffi::c_int as fe_limb_t,
                            8129820 as core::ffi::c_int as fe_limb_t,
                            21651608 as core::ffi::c_int as fe_limb_t,
                            30315096 as core::ffi::c_int as fe_limb_t,
                            48021414 as core::ffi::c_int as fe_limb_t,
                            22549153 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            1533110 as core::ffi::c_int as fe_limb_t,
                            3437855 as core::ffi::c_int as fe_limb_t,
                            23735889 as core::ffi::c_int as fe_limb_t,
                            459276 as core::ffi::c_int as fe_limb_t,
                            29970501 as core::ffi::c_int as fe_limb_t,
                            11335377 as core::ffi::c_int as fe_limb_t,
                            26030092 as core::ffi::c_int as fe_limb_t,
                            5821408 as core::ffi::c_int as fe_limb_t,
                            10478196 as core::ffi::c_int as fe_limb_t,
                            8544890 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            32173102 as core::ffi::c_int as fe_limb_t,
                            17425121 as core::ffi::c_int as fe_limb_t,
                            24896206 as core::ffi::c_int as fe_limb_t,
                            3921497 as core::ffi::c_int as fe_limb_t,
                            22579056 as core::ffi::c_int as fe_limb_t,
                            30143578 as core::ffi::c_int as fe_limb_t,
                            19270448 as core::ffi::c_int as fe_limb_t,
                            12217473 as core::ffi::c_int as fe_limb_t,
                            17789017 as core::ffi::c_int as fe_limb_t,
                            30158437 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            36555903 as core::ffi::c_int as fe_limb_t,
                            31326030 as core::ffi::c_int as fe_limb_t,
                            51530034 as core::ffi::c_int as fe_limb_t,
                            23407230 as core::ffi::c_int as fe_limb_t,
                            13243888 as core::ffi::c_int as fe_limb_t,
                            517024 as core::ffi::c_int as fe_limb_t,
                            15479401 as core::ffi::c_int as fe_limb_t,
                            29701199 as core::ffi::c_int as fe_limb_t,
                            30460519 as core::ffi::c_int as fe_limb_t,
                            1052596 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55493970 as core::ffi::c_int as fe_limb_t,
                            13323617 as core::ffi::c_int as fe_limb_t,
                            32618793 as core::ffi::c_int as fe_limb_t,
                            8175907 as core::ffi::c_int as fe_limb_t,
                            51878691 as core::ffi::c_int as fe_limb_t,
                            12596686 as core::ffi::c_int as fe_limb_t,
                            27491595 as core::ffi::c_int as fe_limb_t,
                            28942073 as core::ffi::c_int as fe_limb_t,
                            3179267 as core::ffi::c_int as fe_limb_t,
                            24075541 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31947050 as core::ffi::c_int as fe_limb_t,
                            19187781 as core::ffi::c_int as fe_limb_t,
                            62468280 as core::ffi::c_int as fe_limb_t,
                            18214510 as core::ffi::c_int as fe_limb_t,
                            51982886 as core::ffi::c_int as fe_limb_t,
                            27514722 as core::ffi::c_int as fe_limb_t,
                            52352086 as core::ffi::c_int as fe_limb_t,
                            17142691 as core::ffi::c_int as fe_limb_t,
                            19072639 as core::ffi::c_int as fe_limb_t,
                            24043372 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11685058 as core::ffi::c_int as fe_limb_t,
                            11822410 as core::ffi::c_int as fe_limb_t,
                            3158003 as core::ffi::c_int as fe_limb_t,
                            19601838 as core::ffi::c_int as fe_limb_t,
                            33402193 as core::ffi::c_int as fe_limb_t,
                            29389366 as core::ffi::c_int as fe_limb_t,
                            5977895 as core::ffi::c_int as fe_limb_t,
                            28339415 as core::ffi::c_int as fe_limb_t,
                            473098 as core::ffi::c_int as fe_limb_t,
                            5040608 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46817982 as core::ffi::c_int as fe_limb_t,
                            8198641 as core::ffi::c_int as fe_limb_t,
                            39698732 as core::ffi::c_int as fe_limb_t,
                            11602122 as core::ffi::c_int as fe_limb_t,
                            1290375 as core::ffi::c_int as fe_limb_t,
                            30754672 as core::ffi::c_int as fe_limb_t,
                            28326861 as core::ffi::c_int as fe_limb_t,
                            1721092 as core::ffi::c_int as fe_limb_t,
                            47550222 as core::ffi::c_int as fe_limb_t,
                            30422825 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            7881532 as core::ffi::c_int as fe_limb_t,
                            10687937 as core::ffi::c_int as fe_limb_t,
                            7578723 as core::ffi::c_int as fe_limb_t,
                            7738378 as core::ffi::c_int as fe_limb_t,
                            48157852 as core::ffi::c_int as fe_limb_t,
                            31000479 as core::ffi::c_int as fe_limb_t,
                            21820785 as core::ffi::c_int as fe_limb_t,
                            8076149 as core::ffi::c_int as fe_limb_t,
                            39240368 as core::ffi::c_int as fe_limb_t,
                            11538388 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47173198 as core::ffi::c_int as fe_limb_t,
                            3899860 as core::ffi::c_int as fe_limb_t,
                            18283497 as core::ffi::c_int as fe_limb_t,
                            26752864 as core::ffi::c_int as fe_limb_t,
                            51380203 as core::ffi::c_int as fe_limb_t,
                            22305220 as core::ffi::c_int as fe_limb_t,
                            8754524 as core::ffi::c_int as fe_limb_t,
                            7446702 as core::ffi::c_int as fe_limb_t,
                            61432810 as core::ffi::c_int as fe_limb_t,
                            5797015 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55813245 as core::ffi::c_int as fe_limb_t,
                            29760862 as core::ffi::c_int as fe_limb_t,
                            51326753 as core::ffi::c_int as fe_limb_t,
                            25589858 as core::ffi::c_int as fe_limb_t,
                            12708868 as core::ffi::c_int as fe_limb_t,
                            25098233 as core::ffi::c_int as fe_limb_t,
                            2014098 as core::ffi::c_int as fe_limb_t,
                            24503858 as core::ffi::c_int as fe_limb_t,
                            64739691 as core::ffi::c_int as fe_limb_t,
                            27677090 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            44636488 as core::ffi::c_int as fe_limb_t,
                            21985690 as core::ffi::c_int as fe_limb_t,
                            39426843 as core::ffi::c_int as fe_limb_t,
                            1146374 as core::ffi::c_int as fe_limb_t,
                            18956691 as core::ffi::c_int as fe_limb_t,
                            16640559 as core::ffi::c_int as fe_limb_t,
                            1192730 as core::ffi::c_int as fe_limb_t,
                            29840233 as core::ffi::c_int as fe_limb_t,
                            15123618 as core::ffi::c_int as fe_limb_t,
                            10811505 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            14352079 as core::ffi::c_int as fe_limb_t,
                            30134717 as core::ffi::c_int as fe_limb_t,
                            48166819 as core::ffi::c_int as fe_limb_t,
                            10822654 as core::ffi::c_int as fe_limb_t,
                            32750596 as core::ffi::c_int as fe_limb_t,
                            4699007 as core::ffi::c_int as fe_limb_t,
                            67038501 as core::ffi::c_int as fe_limb_t,
                            15776355 as core::ffi::c_int as fe_limb_t,
                            38222085 as core::ffi::c_int as fe_limb_t,
                            21579878 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38867681 as core::ffi::c_int as fe_limb_t,
                            25481956 as core::ffi::c_int as fe_limb_t,
                            62129901 as core::ffi::c_int as fe_limb_t,
                            28239114 as core::ffi::c_int as fe_limb_t,
                            29416930 as core::ffi::c_int as fe_limb_t,
                            1847569 as core::ffi::c_int as fe_limb_t,
                            46454691 as core::ffi::c_int as fe_limb_t,
                            17069576 as core::ffi::c_int as fe_limb_t,
                            4714546 as core::ffi::c_int as fe_limb_t,
                            23953777 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15200332 as core::ffi::c_int as fe_limb_t,
                            8368572 as core::ffi::c_int as fe_limb_t,
                            19679101 as core::ffi::c_int as fe_limb_t,
                            15970074 as core::ffi::c_int as fe_limb_t,
                            35236190 as core::ffi::c_int as fe_limb_t,
                            1959450 as core::ffi::c_int as fe_limb_t,
                            24611599 as core::ffi::c_int as fe_limb_t,
                            29010600 as core::ffi::c_int as fe_limb_t,
                            55362987 as core::ffi::c_int as fe_limb_t,
                            12340219 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            12876937 as core::ffi::c_int as fe_limb_t,
                            23074376 as core::ffi::c_int as fe_limb_t,
                            33134380 as core::ffi::c_int as fe_limb_t,
                            6590940 as core::ffi::c_int as fe_limb_t,
                            60801088 as core::ffi::c_int as fe_limb_t,
                            14872439 as core::ffi::c_int as fe_limb_t,
                            9613953 as core::ffi::c_int as fe_limb_t,
                            8241152 as core::ffi::c_int as fe_limb_t,
                            15370987 as core::ffi::c_int as fe_limb_t,
                            9608631 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            62965568 as core::ffi::c_int as fe_limb_t,
                            21540023 as core::ffi::c_int as fe_limb_t,
                            8446280 as core::ffi::c_int as fe_limb_t,
                            33162829 as core::ffi::c_int as fe_limb_t,
                            4407737 as core::ffi::c_int as fe_limb_t,
                            13629032 as core::ffi::c_int as fe_limb_t,
                            59383996 as core::ffi::c_int as fe_limb_t,
                            15866073 as core::ffi::c_int as fe_limb_t,
                            38898243 as core::ffi::c_int as fe_limb_t,
                            24740332 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            26660628 as core::ffi::c_int as fe_limb_t,
                            17876777 as core::ffi::c_int as fe_limb_t,
                            8393733 as core::ffi::c_int as fe_limb_t,
                            358047 as core::ffi::c_int as fe_limb_t,
                            59707573 as core::ffi::c_int as fe_limb_t,
                            992987 as core::ffi::c_int as fe_limb_t,
                            43204631 as core::ffi::c_int as fe_limb_t,
                            858696 as core::ffi::c_int as fe_limb_t,
                            20571223 as core::ffi::c_int as fe_limb_t,
                            8420556 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            14620696 as core::ffi::c_int as fe_limb_t,
                            13067227 as core::ffi::c_int as fe_limb_t,
                            51661590 as core::ffi::c_int as fe_limb_t,
                            8264466 as core::ffi::c_int as fe_limb_t,
                            14106269 as core::ffi::c_int as fe_limb_t,
                            15080814 as core::ffi::c_int as fe_limb_t,
                            33531827 as core::ffi::c_int as fe_limb_t,
                            12516406 as core::ffi::c_int as fe_limb_t,
                            45534429 as core::ffi::c_int as fe_limb_t,
                            21077682 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            236881 as core::ffi::c_int as fe_limb_t,
                            10476226 as core::ffi::c_int as fe_limb_t,
                            57258 as core::ffi::c_int as fe_limb_t,
                            18877408 as core::ffi::c_int as fe_limb_t,
                            6472997 as core::ffi::c_int as fe_limb_t,
                            2466984 as core::ffi::c_int as fe_limb_t,
                            17258519 as core::ffi::c_int as fe_limb_t,
                            7256740 as core::ffi::c_int as fe_limb_t,
                            8791136 as core::ffi::c_int as fe_limb_t,
                            15069930 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            1276391 as core::ffi::c_int as fe_limb_t,
                            24182514 as core::ffi::c_int as fe_limb_t,
                            22949634 as core::ffi::c_int as fe_limb_t,
                            17231625 as core::ffi::c_int as fe_limb_t,
                            43615824 as core::ffi::c_int as fe_limb_t,
                            27852245 as core::ffi::c_int as fe_limb_t,
                            14711874 as core::ffi::c_int as fe_limb_t,
                            4874229 as core::ffi::c_int as fe_limb_t,
                            36445724 as core::ffi::c_int as fe_limb_t,
                            31223040 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            5855666 as core::ffi::c_int as fe_limb_t,
                            4990204 as core::ffi::c_int as fe_limb_t,
                            53397016 as core::ffi::c_int as fe_limb_t,
                            7294283 as core::ffi::c_int as fe_limb_t,
                            59304582 as core::ffi::c_int as fe_limb_t,
                            1924646 as core::ffi::c_int as fe_limb_t,
                            65685689 as core::ffi::c_int as fe_limb_t,
                            25642053 as core::ffi::c_int as fe_limb_t,
                            34039526 as core::ffi::c_int as fe_limb_t,
                            9234252 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            20590503 as core::ffi::c_int as fe_limb_t,
                            24535444 as core::ffi::c_int as fe_limb_t,
                            31529743 as core::ffi::c_int as fe_limb_t,
                            26201766 as core::ffi::c_int as fe_limb_t,
                            64402029 as core::ffi::c_int as fe_limb_t,
                            10650547 as core::ffi::c_int as fe_limb_t,
                            31559055 as core::ffi::c_int as fe_limb_t,
                            21944845 as core::ffi::c_int as fe_limb_t,
                            18979185 as core::ffi::c_int as fe_limb_t,
                            13396066 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            24474287 as core::ffi::c_int as fe_limb_t,
                            4968103 as core::ffi::c_int as fe_limb_t,
                            22267082 as core::ffi::c_int as fe_limb_t,
                            4407354 as core::ffi::c_int as fe_limb_t,
                            24063882 as core::ffi::c_int as fe_limb_t,
                            25229252 as core::ffi::c_int as fe_limb_t,
                            48291976 as core::ffi::c_int as fe_limb_t,
                            13594781 as core::ffi::c_int as fe_limb_t,
                            33514650 as core::ffi::c_int as fe_limb_t,
                            7021958 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            55541958 as core::ffi::c_int as fe_limb_t,
                            26988926 as core::ffi::c_int as fe_limb_t,
                            45743778 as core::ffi::c_int as fe_limb_t,
                            15928891 as core::ffi::c_int as fe_limb_t,
                            40950559 as core::ffi::c_int as fe_limb_t,
                            4315420 as core::ffi::c_int as fe_limb_t,
                            41160136 as core::ffi::c_int as fe_limb_t,
                            29637754 as core::ffi::c_int as fe_limb_t,
                            45628383 as core::ffi::c_int as fe_limb_t,
                            12868081 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38473832 as core::ffi::c_int as fe_limb_t,
                            13504660 as core::ffi::c_int as fe_limb_t,
                            19988037 as core::ffi::c_int as fe_limb_t,
                            31421671 as core::ffi::c_int as fe_limb_t,
                            21078224 as core::ffi::c_int as fe_limb_t,
                            6443208 as core::ffi::c_int as fe_limb_t,
                            45662757 as core::ffi::c_int as fe_limb_t,
                            2244499 as core::ffi::c_int as fe_limb_t,
                            54653067 as core::ffi::c_int as fe_limb_t,
                            25465048 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            36513336 as core::ffi::c_int as fe_limb_t,
                            13793478 as core::ffi::c_int as fe_limb_t,
                            61256044 as core::ffi::c_int as fe_limb_t,
                            319135 as core::ffi::c_int as fe_limb_t,
                            41385692 as core::ffi::c_int as fe_limb_t,
                            27290532 as core::ffi::c_int as fe_limb_t,
                            33086545 as core::ffi::c_int as fe_limb_t,
                            8957937 as core::ffi::c_int as fe_limb_t,
                            51875216 as core::ffi::c_int as fe_limb_t,
                            5540520 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            55478669 as core::ffi::c_int as fe_limb_t,
                            22050529 as core::ffi::c_int as fe_limb_t,
                            58989363 as core::ffi::c_int as fe_limb_t,
                            25911358 as core::ffi::c_int as fe_limb_t,
                            2620055 as core::ffi::c_int as fe_limb_t,
                            1022908 as core::ffi::c_int as fe_limb_t,
                            43398120 as core::ffi::c_int as fe_limb_t,
                            31985447 as core::ffi::c_int as fe_limb_t,
                            50980335 as core::ffi::c_int as fe_limb_t,
                            18591624 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            23152952 as core::ffi::c_int as fe_limb_t,
                            775386 as core::ffi::c_int as fe_limb_t,
                            27395463 as core::ffi::c_int as fe_limb_t,
                            14006635 as core::ffi::c_int as fe_limb_t,
                            57407746 as core::ffi::c_int as fe_limb_t,
                            4649511 as core::ffi::c_int as fe_limb_t,
                            1689819 as core::ffi::c_int as fe_limb_t,
                            892185 as core::ffi::c_int as fe_limb_t,
                            55595587 as core::ffi::c_int as fe_limb_t,
                            18348483 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9770129 as core::ffi::c_int as fe_limb_t,
                            9586738 as core::ffi::c_int as fe_limb_t,
                            26496094 as core::ffi::c_int as fe_limb_t,
                            4324120 as core::ffi::c_int as fe_limb_t,
                            1556511 as core::ffi::c_int as fe_limb_t,
                            30004408 as core::ffi::c_int as fe_limb_t,
                            27453818 as core::ffi::c_int as fe_limb_t,
                            4763127 as core::ffi::c_int as fe_limb_t,
                            47929250 as core::ffi::c_int as fe_limb_t,
                            5867133 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            34343820 as core::ffi::c_int as fe_limb_t,
                            1927589 as core::ffi::c_int as fe_limb_t,
                            31726409 as core::ffi::c_int as fe_limb_t,
                            28801137 as core::ffi::c_int as fe_limb_t,
                            23962433 as core::ffi::c_int as fe_limb_t,
                            17534932 as core::ffi::c_int as fe_limb_t,
                            27846558 as core::ffi::c_int as fe_limb_t,
                            5931263 as core::ffi::c_int as fe_limb_t,
                            37359161 as core::ffi::c_int as fe_limb_t,
                            17445976 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            27461885 as core::ffi::c_int as fe_limb_t,
                            30576896 as core::ffi::c_int as fe_limb_t,
                            22380809 as core::ffi::c_int as fe_limb_t,
                            1815854 as core::ffi::c_int as fe_limb_t,
                            44075111 as core::ffi::c_int as fe_limb_t,
                            30522493 as core::ffi::c_int as fe_limb_t,
                            7283489 as core::ffi::c_int as fe_limb_t,
                            18406359 as core::ffi::c_int as fe_limb_t,
                            47582163 as core::ffi::c_int as fe_limb_t,
                            7734628 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            59098600 as core::ffi::c_int as fe_limb_t,
                            23963614 as core::ffi::c_int as fe_limb_t,
                            55988460 as core::ffi::c_int as fe_limb_t,
                            6196037 as core::ffi::c_int as fe_limb_t,
                            29344158 as core::ffi::c_int as fe_limb_t,
                            20123547 as core::ffi::c_int as fe_limb_t,
                            7585294 as core::ffi::c_int as fe_limb_t,
                            30377806 as core::ffi::c_int as fe_limb_t,
                            18549496 as core::ffi::c_int as fe_limb_t,
                            15302069 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            34450527 as core::ffi::c_int as fe_limb_t,
                            27383209 as core::ffi::c_int as fe_limb_t,
                            59436070 as core::ffi::c_int as fe_limb_t,
                            22502750 as core::ffi::c_int as fe_limb_t,
                            6258877 as core::ffi::c_int as fe_limb_t,
                            13504381 as core::ffi::c_int as fe_limb_t,
                            10458790 as core::ffi::c_int as fe_limb_t,
                            27135971 as core::ffi::c_int as fe_limb_t,
                            58236621 as core::ffi::c_int as fe_limb_t,
                            8424745 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            24687186 as core::ffi::c_int as fe_limb_t,
                            8613276 as core::ffi::c_int as fe_limb_t,
                            36441818 as core::ffi::c_int as fe_limb_t,
                            30320886 as core::ffi::c_int as fe_limb_t,
                            1863891 as core::ffi::c_int as fe_limb_t,
                            31723888 as core::ffi::c_int as fe_limb_t,
                            19206233 as core::ffi::c_int as fe_limb_t,
                            7134917 as core::ffi::c_int as fe_limb_t,
                            55824382 as core::ffi::c_int as fe_limb_t,
                            32725512 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11334899 as core::ffi::c_int as fe_limb_t,
                            24336410 as core::ffi::c_int as fe_limb_t,
                            8025292 as core::ffi::c_int as fe_limb_t,
                            12707519 as core::ffi::c_int as fe_limb_t,
                            17523892 as core::ffi::c_int as fe_limb_t,
                            23078361 as core::ffi::c_int as fe_limb_t,
                            10243737 as core::ffi::c_int as fe_limb_t,
                            18868971 as core::ffi::c_int as fe_limb_t,
                            62042829 as core::ffi::c_int as fe_limb_t,
                            16498836 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            8911542 as core::ffi::c_int as fe_limb_t,
                            6887158 as core::ffi::c_int as fe_limb_t,
                            57524604 as core::ffi::c_int as fe_limb_t,
                            26595841 as core::ffi::c_int as fe_limb_t,
                            11145640 as core::ffi::c_int as fe_limb_t,
                            24010752 as core::ffi::c_int as fe_limb_t,
                            17303924 as core::ffi::c_int as fe_limb_t,
                            19430194 as core::ffi::c_int as fe_limb_t,
                            6536640 as core::ffi::c_int as fe_limb_t,
                            10543906 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38162480 as core::ffi::c_int as fe_limb_t,
                            15479762 as core::ffi::c_int as fe_limb_t,
                            49642029 as core::ffi::c_int as fe_limb_t,
                            568875 as core::ffi::c_int as fe_limb_t,
                            65611181 as core::ffi::c_int as fe_limb_t,
                            11223453 as core::ffi::c_int as fe_limb_t,
                            64439674 as core::ffi::c_int as fe_limb_t,
                            16928857 as core::ffi::c_int as fe_limb_t,
                            39873154 as core::ffi::c_int as fe_limb_t,
                            8876770 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41365946 as core::ffi::c_int as fe_limb_t,
                            20987567 as core::ffi::c_int as fe_limb_t,
                            51458897 as core::ffi::c_int as fe_limb_t,
                            32707824 as core::ffi::c_int as fe_limb_t,
                            34082177 as core::ffi::c_int as fe_limb_t,
                            32758143 as core::ffi::c_int as fe_limb_t,
                            33627041 as core::ffi::c_int as fe_limb_t,
                            15824473 as core::ffi::c_int as fe_limb_t,
                            66504438 as core::ffi::c_int as fe_limb_t,
                            24514614 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10330056 as core::ffi::c_int as fe_limb_t,
                            70051 as core::ffi::c_int as fe_limb_t,
                            7957388 as core::ffi::c_int as fe_limb_t,
                            24551765 as core::ffi::c_int as fe_limb_t,
                            9764901 as core::ffi::c_int as fe_limb_t,
                            15609756 as core::ffi::c_int as fe_limb_t,
                            27698697 as core::ffi::c_int as fe_limb_t,
                            28664395 as core::ffi::c_int as fe_limb_t,
                            1657393 as core::ffi::c_int as fe_limb_t,
                            3084098 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10477963 as core::ffi::c_int as fe_limb_t,
                            26084172 as core::ffi::c_int as fe_limb_t,
                            12119565 as core::ffi::c_int as fe_limb_t,
                            20303627 as core::ffi::c_int as fe_limb_t,
                            29016246 as core::ffi::c_int as fe_limb_t,
                            28188843 as core::ffi::c_int as fe_limb_t,
                            31280318 as core::ffi::c_int as fe_limb_t,
                            14396151 as core::ffi::c_int as fe_limb_t,
                            36875289 as core::ffi::c_int as fe_limb_t,
                            15272408 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54820555 as core::ffi::c_int as fe_limb_t,
                            3169462 as core::ffi::c_int as fe_limb_t,
                            28813183 as core::ffi::c_int as fe_limb_t,
                            16658753 as core::ffi::c_int as fe_limb_t,
                            25116432 as core::ffi::c_int as fe_limb_t,
                            27923966 as core::ffi::c_int as fe_limb_t,
                            41934906 as core::ffi::c_int as fe_limb_t,
                            20918293 as core::ffi::c_int as fe_limb_t,
                            42094106 as core::ffi::c_int as fe_limb_t,
                            1950503 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40928506 as core::ffi::c_int as fe_limb_t,
                            9489186 as core::ffi::c_int as fe_limb_t,
                            11053416 as core::ffi::c_int as fe_limb_t,
                            18808271 as core::ffi::c_int as fe_limb_t,
                            36055143 as core::ffi::c_int as fe_limb_t,
                            5825629 as core::ffi::c_int as fe_limb_t,
                            58724558 as core::ffi::c_int as fe_limb_t,
                            24786899 as core::ffi::c_int as fe_limb_t,
                            15341278 as core::ffi::c_int as fe_limb_t,
                            8373727 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28685821 as core::ffi::c_int as fe_limb_t,
                            7759505 as core::ffi::c_int as fe_limb_t,
                            52730348 as core::ffi::c_int as fe_limb_t,
                            21551571 as core::ffi::c_int as fe_limb_t,
                            35137043 as core::ffi::c_int as fe_limb_t,
                            4079241 as core::ffi::c_int as fe_limb_t,
                            298136 as core::ffi::c_int as fe_limb_t,
                            23321830 as core::ffi::c_int as fe_limb_t,
                            64230656 as core::ffi::c_int as fe_limb_t,
                            15190419 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            34175969 as core::ffi::c_int as fe_limb_t,
                            13806335 as core::ffi::c_int as fe_limb_t,
                            52771379 as core::ffi::c_int as fe_limb_t,
                            17760000 as core::ffi::c_int as fe_limb_t,
                            43104243 as core::ffi::c_int as fe_limb_t,
                            10940927 as core::ffi::c_int as fe_limb_t,
                            8669718 as core::ffi::c_int as fe_limb_t,
                            2742393 as core::ffi::c_int as fe_limb_t,
                            41075551 as core::ffi::c_int as fe_limb_t,
                            26679428 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65528476 as core::ffi::c_int as fe_limb_t,
                            21825014 as core::ffi::c_int as fe_limb_t,
                            41129205 as core::ffi::c_int as fe_limb_t,
                            22109408 as core::ffi::c_int as fe_limb_t,
                            49696989 as core::ffi::c_int as fe_limb_t,
                            22641577 as core::ffi::c_int as fe_limb_t,
                            9291593 as core::ffi::c_int as fe_limb_t,
                            17306653 as core::ffi::c_int as fe_limb_t,
                            54954121 as core::ffi::c_int as fe_limb_t,
                            6048604 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            36803549 as core::ffi::c_int as fe_limb_t,
                            14843443 as core::ffi::c_int as fe_limb_t,
                            1539301 as core::ffi::c_int as fe_limb_t,
                            11864366 as core::ffi::c_int as fe_limb_t,
                            20201677 as core::ffi::c_int as fe_limb_t,
                            1900163 as core::ffi::c_int as fe_limb_t,
                            13934231 as core::ffi::c_int as fe_limb_t,
                            5128323 as core::ffi::c_int as fe_limb_t,
                            11213262 as core::ffi::c_int as fe_limb_t,
                            9168384 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            40828332 as core::ffi::c_int as fe_limb_t,
                            11007846 as core::ffi::c_int as fe_limb_t,
                            19408960 as core::ffi::c_int as fe_limb_t,
                            32613674 as core::ffi::c_int as fe_limb_t,
                            48515898 as core::ffi::c_int as fe_limb_t,
                            29225851 as core::ffi::c_int as fe_limb_t,
                            62020803 as core::ffi::c_int as fe_limb_t,
                            22449281 as core::ffi::c_int as fe_limb_t,
                            20470156 as core::ffi::c_int as fe_limb_t,
                            17155731 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            43972811 as core::ffi::c_int as fe_limb_t,
                            9282191 as core::ffi::c_int as fe_limb_t,
                            14855179 as core::ffi::c_int as fe_limb_t,
                            18164354 as core::ffi::c_int as fe_limb_t,
                            59746048 as core::ffi::c_int as fe_limb_t,
                            19145871 as core::ffi::c_int as fe_limb_t,
                            44324911 as core::ffi::c_int as fe_limb_t,
                            14461607 as core::ffi::c_int as fe_limb_t,
                            14042978 as core::ffi::c_int as fe_limb_t,
                            5230683 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            29969548 as core::ffi::c_int as fe_limb_t,
                            30812838 as core::ffi::c_int as fe_limb_t,
                            50396996 as core::ffi::c_int as fe_limb_t,
                            25001989 as core::ffi::c_int as fe_limb_t,
                            9175485 as core::ffi::c_int as fe_limb_t,
                            31085458 as core::ffi::c_int as fe_limb_t,
                            21556950 as core::ffi::c_int as fe_limb_t,
                            3506042 as core::ffi::c_int as fe_limb_t,
                            61174973 as core::ffi::c_int as fe_limb_t,
                            21104723 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            63964118 as core::ffi::c_int as fe_limb_t,
                            8744660 as core::ffi::c_int as fe_limb_t,
                            19704003 as core::ffi::c_int as fe_limb_t,
                            4581278 as core::ffi::c_int as fe_limb_t,
                            46678178 as core::ffi::c_int as fe_limb_t,
                            6830682 as core::ffi::c_int as fe_limb_t,
                            45824694 as core::ffi::c_int as fe_limb_t,
                            8971512 as core::ffi::c_int as fe_limb_t,
                            38569675 as core::ffi::c_int as fe_limb_t,
                            15326562 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47644235 as core::ffi::c_int as fe_limb_t,
                            10110287 as core::ffi::c_int as fe_limb_t,
                            49846336 as core::ffi::c_int as fe_limb_t,
                            30050539 as core::ffi::c_int as fe_limb_t,
                            43608476 as core::ffi::c_int as fe_limb_t,
                            1355668 as core::ffi::c_int as fe_limb_t,
                            51585814 as core::ffi::c_int as fe_limb_t,
                            15300987 as core::ffi::c_int as fe_limb_t,
                            46594746 as core::ffi::c_int as fe_limb_t,
                            9168259 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            61755510 as core::ffi::c_int as fe_limb_t,
                            4488612 as core::ffi::c_int as fe_limb_t,
                            43305616 as core::ffi::c_int as fe_limb_t,
                            16314346 as core::ffi::c_int as fe_limb_t,
                            7780487 as core::ffi::c_int as fe_limb_t,
                            17915493 as core::ffi::c_int as fe_limb_t,
                            38160505 as core::ffi::c_int as fe_limb_t,
                            9601604 as core::ffi::c_int as fe_limb_t,
                            33087103 as core::ffi::c_int as fe_limb_t,
                            24543045 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            47665694 as core::ffi::c_int as fe_limb_t,
                            18041531 as core::ffi::c_int as fe_limb_t,
                            46311396 as core::ffi::c_int as fe_limb_t,
                            21109108 as core::ffi::c_int as fe_limb_t,
                            37284416 as core::ffi::c_int as fe_limb_t,
                            10229460 as core::ffi::c_int as fe_limb_t,
                            39664535 as core::ffi::c_int as fe_limb_t,
                            18553900 as core::ffi::c_int as fe_limb_t,
                            61111993 as core::ffi::c_int as fe_limb_t,
                            15664671 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            23294591 as core::ffi::c_int as fe_limb_t,
                            16921819 as core::ffi::c_int as fe_limb_t,
                            44458082 as core::ffi::c_int as fe_limb_t,
                            25083453 as core::ffi::c_int as fe_limb_t,
                            27844203 as core::ffi::c_int as fe_limb_t,
                            11461195 as core::ffi::c_int as fe_limb_t,
                            13099750 as core::ffi::c_int as fe_limb_t,
                            31094076 as core::ffi::c_int as fe_limb_t,
                            18151675 as core::ffi::c_int as fe_limb_t,
                            13417686 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42385932 as core::ffi::c_int as fe_limb_t,
                            29377914 as core::ffi::c_int as fe_limb_t,
                            35958184 as core::ffi::c_int as fe_limb_t,
                            5988918 as core::ffi::c_int as fe_limb_t,
                            40250079 as core::ffi::c_int as fe_limb_t,
                            6685064 as core::ffi::c_int as fe_limb_t,
                            1661597 as core::ffi::c_int as fe_limb_t,
                            21002991 as core::ffi::c_int as fe_limb_t,
                            15271675 as core::ffi::c_int as fe_limb_t,
                            18101767 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11433023 as core::ffi::c_int as fe_limb_t,
                            20325767 as core::ffi::c_int as fe_limb_t,
                            8239630 as core::ffi::c_int as fe_limb_t,
                            28274915 as core::ffi::c_int as fe_limb_t,
                            65123427 as core::ffi::c_int as fe_limb_t,
                            32828713 as core::ffi::c_int as fe_limb_t,
                            48410099 as core::ffi::c_int as fe_limb_t,
                            2167543 as core::ffi::c_int as fe_limb_t,
                            60187563 as core::ffi::c_int as fe_limb_t,
                            20114249 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            35672693 as core::ffi::c_int as fe_limb_t,
                            15575145 as core::ffi::c_int as fe_limb_t,
                            30436815 as core::ffi::c_int as fe_limb_t,
                            12192228 as core::ffi::c_int as fe_limb_t,
                            44645511 as core::ffi::c_int as fe_limb_t,
                            9395378 as core::ffi::c_int as fe_limb_t,
                            57191156 as core::ffi::c_int as fe_limb_t,
                            24915434 as core::ffi::c_int as fe_limb_t,
                            12215109 as core::ffi::c_int as fe_limb_t,
                            12028277 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            14098381 as core::ffi::c_int as fe_limb_t,
                            6555944 as core::ffi::c_int as fe_limb_t,
                            23007258 as core::ffi::c_int as fe_limb_t,
                            5757252 as core::ffi::c_int as fe_limb_t,
                            51681032 as core::ffi::c_int as fe_limb_t,
                            20603929 as core::ffi::c_int as fe_limb_t,
                            30123439 as core::ffi::c_int as fe_limb_t,
                            4617780 as core::ffi::c_int as fe_limb_t,
                            50208775 as core::ffi::c_int as fe_limb_t,
                            32898803 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            63082644 as core::ffi::c_int as fe_limb_t,
                            18313596 as core::ffi::c_int as fe_limb_t,
                            11893167 as core::ffi::c_int as fe_limb_t,
                            13718664 as core::ffi::c_int as fe_limb_t,
                            52299402 as core::ffi::c_int as fe_limb_t,
                            1847384 as core::ffi::c_int as fe_limb_t,
                            51288865 as core::ffi::c_int as fe_limb_t,
                            10154008 as core::ffi::c_int as fe_limb_t,
                            23973261 as core::ffi::c_int as fe_limb_t,
                            20869958 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40577025 as core::ffi::c_int as fe_limb_t,
                            29858441 as core::ffi::c_int as fe_limb_t,
                            65199965 as core::ffi::c_int as fe_limb_t,
                            2534300 as core::ffi::c_int as fe_limb_t,
                            35238307 as core::ffi::c_int as fe_limb_t,
                            17004076 as core::ffi::c_int as fe_limb_t,
                            18341389 as core::ffi::c_int as fe_limb_t,
                            22134481 as core::ffi::c_int as fe_limb_t,
                            32013173 as core::ffi::c_int as fe_limb_t,
                            23450893 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            41629544 as core::ffi::c_int as fe_limb_t,
                            10876442 as core::ffi::c_int as fe_limb_t,
                            55337778 as core::ffi::c_int as fe_limb_t,
                            18929291 as core::ffi::c_int as fe_limb_t,
                            54739296 as core::ffi::c_int as fe_limb_t,
                            1838103 as core::ffi::c_int as fe_limb_t,
                            21911214 as core::ffi::c_int as fe_limb_t,
                            6354752 as core::ffi::c_int as fe_limb_t,
                            4425632 as core::ffi::c_int as fe_limb_t,
                            32716610 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            56675475 as core::ffi::c_int as fe_limb_t,
                            18941465 as core::ffi::c_int as fe_limb_t,
                            22229857 as core::ffi::c_int as fe_limb_t,
                            30463385 as core::ffi::c_int as fe_limb_t,
                            53917697 as core::ffi::c_int as fe_limb_t,
                            776728 as core::ffi::c_int as fe_limb_t,
                            49693489 as core::ffi::c_int as fe_limb_t,
                            21533969 as core::ffi::c_int as fe_limb_t,
                            4725004 as core::ffi::c_int as fe_limb_t,
                            14044970 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            19268631 as core::ffi::c_int as fe_limb_t,
                            26250011 as core::ffi::c_int as fe_limb_t,
                            1555348 as core::ffi::c_int as fe_limb_t,
                            8692754 as core::ffi::c_int as fe_limb_t,
                            45634805 as core::ffi::c_int as fe_limb_t,
                            23643767 as core::ffi::c_int as fe_limb_t,
                            6347389 as core::ffi::c_int as fe_limb_t,
                            32142648 as core::ffi::c_int as fe_limb_t,
                            47586572 as core::ffi::c_int as fe_limb_t,
                            17444675 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42244775 as core::ffi::c_int as fe_limb_t,
                            12986007 as core::ffi::c_int as fe_limb_t,
                            56209986 as core::ffi::c_int as fe_limb_t,
                            27995847 as core::ffi::c_int as fe_limb_t,
                            55796492 as core::ffi::c_int as fe_limb_t,
                            33405905 as core::ffi::c_int as fe_limb_t,
                            19541417 as core::ffi::c_int as fe_limb_t,
                            8180106 as core::ffi::c_int as fe_limb_t,
                            9282262 as core::ffi::c_int as fe_limb_t,
                            10282508 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            40903763 as core::ffi::c_int as fe_limb_t,
                            4428546 as core::ffi::c_int as fe_limb_t,
                            58447668 as core::ffi::c_int as fe_limb_t,
                            20360168 as core::ffi::c_int as fe_limb_t,
                            4098401 as core::ffi::c_int as fe_limb_t,
                            19389175 as core::ffi::c_int as fe_limb_t,
                            15522534 as core::ffi::c_int as fe_limb_t,
                            8372215 as core::ffi::c_int as fe_limb_t,
                            5542595 as core::ffi::c_int as fe_limb_t,
                            22851749 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56546323 as core::ffi::c_int as fe_limb_t,
                            14895632 as core::ffi::c_int as fe_limb_t,
                            26814552 as core::ffi::c_int as fe_limb_t,
                            16880582 as core::ffi::c_int as fe_limb_t,
                            49628109 as core::ffi::c_int as fe_limb_t,
                            31065071 as core::ffi::c_int as fe_limb_t,
                            64326972 as core::ffi::c_int as fe_limb_t,
                            6993760 as core::ffi::c_int as fe_limb_t,
                            49014979 as core::ffi::c_int as fe_limb_t,
                            10114654 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            47001790 as core::ffi::c_int as fe_limb_t,
                            32625013 as core::ffi::c_int as fe_limb_t,
                            31422703 as core::ffi::c_int as fe_limb_t,
                            10427861 as core::ffi::c_int as fe_limb_t,
                            59998115 as core::ffi::c_int as fe_limb_t,
                            6150668 as core::ffi::c_int as fe_limb_t,
                            38017109 as core::ffi::c_int as fe_limb_t,
                            22025285 as core::ffi::c_int as fe_limb_t,
                            25953724 as core::ffi::c_int as fe_limb_t,
                            33448274 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62874467 as core::ffi::c_int as fe_limb_t,
                            25515139 as core::ffi::c_int as fe_limb_t,
                            57989738 as core::ffi::c_int as fe_limb_t,
                            3045999 as core::ffi::c_int as fe_limb_t,
                            2101609 as core::ffi::c_int as fe_limb_t,
                            20947138 as core::ffi::c_int as fe_limb_t,
                            19390019 as core::ffi::c_int as fe_limb_t,
                            6094296 as core::ffi::c_int as fe_limb_t,
                            63793585 as core::ffi::c_int as fe_limb_t,
                            12831124 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51110167 as core::ffi::c_int as fe_limb_t,
                            7578151 as core::ffi::c_int as fe_limb_t,
                            5310217 as core::ffi::c_int as fe_limb_t,
                            14408357 as core::ffi::c_int as fe_limb_t,
                            33560244 as core::ffi::c_int as fe_limb_t,
                            33329692 as core::ffi::c_int as fe_limb_t,
                            31575953 as core::ffi::c_int as fe_limb_t,
                            6326196 as core::ffi::c_int as fe_limb_t,
                            7381791 as core::ffi::c_int as fe_limb_t,
                            31132593 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46206085 as core::ffi::c_int as fe_limb_t,
                            3296810 as core::ffi::c_int as fe_limb_t,
                            24736065 as core::ffi::c_int as fe_limb_t,
                            17226043 as core::ffi::c_int as fe_limb_t,
                            18374253 as core::ffi::c_int as fe_limb_t,
                            7318640 as core::ffi::c_int as fe_limb_t,
                            6295303 as core::ffi::c_int as fe_limb_t,
                            8082724 as core::ffi::c_int as fe_limb_t,
                            51746375 as core::ffi::c_int as fe_limb_t,
                            12339663 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            27724736 as core::ffi::c_int as fe_limb_t,
                            2291157 as core::ffi::c_int as fe_limb_t,
                            6088201 as core::ffi::c_int as fe_limb_t,
                            19369634 as core::ffi::c_int as fe_limb_t,
                            1792726 as core::ffi::c_int as fe_limb_t,
                            5857634 as core::ffi::c_int as fe_limb_t,
                            13848414 as core::ffi::c_int as fe_limb_t,
                            15768922 as core::ffi::c_int as fe_limb_t,
                            25091167 as core::ffi::c_int as fe_limb_t,
                            14856294 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            48242193 as core::ffi::c_int as fe_limb_t,
                            8331042 as core::ffi::c_int as fe_limb_t,
                            24373479 as core::ffi::c_int as fe_limb_t,
                            8541013 as core::ffi::c_int as fe_limb_t,
                            66406866 as core::ffi::c_int as fe_limb_t,
                            24284974 as core::ffi::c_int as fe_limb_t,
                            12927299 as core::ffi::c_int as fe_limb_t,
                            20858939 as core::ffi::c_int as fe_limb_t,
                            44926390 as core::ffi::c_int as fe_limb_t,
                            24541532 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55685435 as core::ffi::c_int as fe_limb_t,
                            28132841 as core::ffi::c_int as fe_limb_t,
                            11632844 as core::ffi::c_int as fe_limb_t,
                            3405020 as core::ffi::c_int as fe_limb_t,
                            30536730 as core::ffi::c_int as fe_limb_t,
                            21880393 as core::ffi::c_int as fe_limb_t,
                            39848098 as core::ffi::c_int as fe_limb_t,
                            13866389 as core::ffi::c_int as fe_limb_t,
                            30146206 as core::ffi::c_int as fe_limb_t,
                            9142070 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            3924129 as core::ffi::c_int as fe_limb_t,
                            18246916 as core::ffi::c_int as fe_limb_t,
                            53291741 as core::ffi::c_int as fe_limb_t,
                            23499471 as core::ffi::c_int as fe_limb_t,
                            12291819 as core::ffi::c_int as fe_limb_t,
                            32886066 as core::ffi::c_int as fe_limb_t,
                            39406089 as core::ffi::c_int as fe_limb_t,
                            9326383 as core::ffi::c_int as fe_limb_t,
                            58871006 as core::ffi::c_int as fe_limb_t,
                            4171293 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51186905 as core::ffi::c_int as fe_limb_t,
                            16037936 as core::ffi::c_int as fe_limb_t,
                            6713787 as core::ffi::c_int as fe_limb_t,
                            16606682 as core::ffi::c_int as fe_limb_t,
                            45496729 as core::ffi::c_int as fe_limb_t,
                            2790943 as core::ffi::c_int as fe_limb_t,
                            26396185 as core::ffi::c_int as fe_limb_t,
                            3731949 as core::ffi::c_int as fe_limb_t,
                            345228 as core::ffi::c_int as fe_limb_t,
                            28091483 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45781307 as core::ffi::c_int as fe_limb_t,
                            13448258 as core::ffi::c_int as fe_limb_t,
                            25284571 as core::ffi::c_int as fe_limb_t,
                            1143661 as core::ffi::c_int as fe_limb_t,
                            20614966 as core::ffi::c_int as fe_limb_t,
                            24705045 as core::ffi::c_int as fe_limb_t,
                            2031538 as core::ffi::c_int as fe_limb_t,
                            21163201 as core::ffi::c_int as fe_limb_t,
                            50855680 as core::ffi::c_int as fe_limb_t,
                            19972348 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31016192 as core::ffi::c_int as fe_limb_t,
                            16832003 as core::ffi::c_int as fe_limb_t,
                            26371391 as core::ffi::c_int as fe_limb_t,
                            19103199 as core::ffi::c_int as fe_limb_t,
                            62081514 as core::ffi::c_int as fe_limb_t,
                            14854136 as core::ffi::c_int as fe_limb_t,
                            17477601 as core::ffi::c_int as fe_limb_t,
                            3842657 as core::ffi::c_int as fe_limb_t,
                            28012650 as core::ffi::c_int as fe_limb_t,
                            17149012 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            62033029 as core::ffi::c_int as fe_limb_t,
                            9368965 as core::ffi::c_int as fe_limb_t,
                            58546785 as core::ffi::c_int as fe_limb_t,
                            28953529 as core::ffi::c_int as fe_limb_t,
                            51858910 as core::ffi::c_int as fe_limb_t,
                            6970559 as core::ffi::c_int as fe_limb_t,
                            57918991 as core::ffi::c_int as fe_limb_t,
                            16292056 as core::ffi::c_int as fe_limb_t,
                            58241707 as core::ffi::c_int as fe_limb_t,
                            3507939 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            29439664 as core::ffi::c_int as fe_limb_t,
                            3537914 as core::ffi::c_int as fe_limb_t,
                            23333589 as core::ffi::c_int as fe_limb_t,
                            6997794 as core::ffi::c_int as fe_limb_t,
                            49553303 as core::ffi::c_int as fe_limb_t,
                            22536363 as core::ffi::c_int as fe_limb_t,
                            51899661 as core::ffi::c_int as fe_limb_t,
                            18503164 as core::ffi::c_int as fe_limb_t,
                            57943934 as core::ffi::c_int as fe_limb_t,
                            6580395 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54923003 as core::ffi::c_int as fe_limb_t,
                            25874643 as core::ffi::c_int as fe_limb_t,
                            16438268 as core::ffi::c_int as fe_limb_t,
                            10826160 as core::ffi::c_int as fe_limb_t,
                            58412047 as core::ffi::c_int as fe_limb_t,
                            27318820 as core::ffi::c_int as fe_limb_t,
                            17860443 as core::ffi::c_int as fe_limb_t,
                            24280586 as core::ffi::c_int as fe_limb_t,
                            65013061 as core::ffi::c_int as fe_limb_t,
                            9304566 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            20714545 as core::ffi::c_int as fe_limb_t,
                            29217521 as core::ffi::c_int as fe_limb_t,
                            29088194 as core::ffi::c_int as fe_limb_t,
                            7406487 as core::ffi::c_int as fe_limb_t,
                            11426967 as core::ffi::c_int as fe_limb_t,
                            28458727 as core::ffi::c_int as fe_limb_t,
                            14792666 as core::ffi::c_int as fe_limb_t,
                            18945815 as core::ffi::c_int as fe_limb_t,
                            5289420 as core::ffi::c_int as fe_limb_t,
                            33077305 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            50443312 as core::ffi::c_int as fe_limb_t,
                            22903641 as core::ffi::c_int as fe_limb_t,
                            60948518 as core::ffi::c_int as fe_limb_t,
                            20248671 as core::ffi::c_int as fe_limb_t,
                            9192019 as core::ffi::c_int as fe_limb_t,
                            31751970 as core::ffi::c_int as fe_limb_t,
                            17271489 as core::ffi::c_int as fe_limb_t,
                            12349094 as core::ffi::c_int as fe_limb_t,
                            26939669 as core::ffi::c_int as fe_limb_t,
                            29802138 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54218966 as core::ffi::c_int as fe_limb_t,
                            9373457 as core::ffi::c_int as fe_limb_t,
                            31595848 as core::ffi::c_int as fe_limb_t,
                            16374215 as core::ffi::c_int as fe_limb_t,
                            21471720 as core::ffi::c_int as fe_limb_t,
                            13221525 as core::ffi::c_int as fe_limb_t,
                            39825369 as core::ffi::c_int as fe_limb_t,
                            21205872 as core::ffi::c_int as fe_limb_t,
                            63410057 as core::ffi::c_int as fe_limb_t,
                            117886 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            22263325 as core::ffi::c_int as fe_limb_t,
                            26994382 as core::ffi::c_int as fe_limb_t,
                            3984569 as core::ffi::c_int as fe_limb_t,
                            22379786 as core::ffi::c_int as fe_limb_t,
                            51994855 as core::ffi::c_int as fe_limb_t,
                            32987646 as core::ffi::c_int as fe_limb_t,
                            28311252 as core::ffi::c_int as fe_limb_t,
                            5358056 as core::ffi::c_int as fe_limb_t,
                            43789084 as core::ffi::c_int as fe_limb_t,
                            541963 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            16259200 as core::ffi::c_int as fe_limb_t,
                            3261970 as core::ffi::c_int as fe_limb_t,
                            2309254 as core::ffi::c_int as fe_limb_t,
                            18019958 as core::ffi::c_int as fe_limb_t,
                            50223152 as core::ffi::c_int as fe_limb_t,
                            28972515 as core::ffi::c_int as fe_limb_t,
                            24134069 as core::ffi::c_int as fe_limb_t,
                            16848603 as core::ffi::c_int as fe_limb_t,
                            53771797 as core::ffi::c_int as fe_limb_t,
                            20002236 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9378160 as core::ffi::c_int as fe_limb_t,
                            20414246 as core::ffi::c_int as fe_limb_t,
                            44262881 as core::ffi::c_int as fe_limb_t,
                            20809167 as core::ffi::c_int as fe_limb_t,
                            28198280 as core::ffi::c_int as fe_limb_t,
                            26310334 as core::ffi::c_int as fe_limb_t,
                            64709179 as core::ffi::c_int as fe_limb_t,
                            32837080 as core::ffi::c_int as fe_limb_t,
                            690425 as core::ffi::c_int as fe_limb_t,
                            14876244 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            24977353 as core::ffi::c_int as fe_limb_t,
                            33240048 as core::ffi::c_int as fe_limb_t,
                            58884894 as core::ffi::c_int as fe_limb_t,
                            20089345 as core::ffi::c_int as fe_limb_t,
                            28432342 as core::ffi::c_int as fe_limb_t,
                            32378079 as core::ffi::c_int as fe_limb_t,
                            54040059 as core::ffi::c_int as fe_limb_t,
                            21257083 as core::ffi::c_int as fe_limb_t,
                            44727879 as core::ffi::c_int as fe_limb_t,
                            6618998 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            65570671 as core::ffi::c_int as fe_limb_t,
                            11685645 as core::ffi::c_int as fe_limb_t,
                            12944378 as core::ffi::c_int as fe_limb_t,
                            13682314 as core::ffi::c_int as fe_limb_t,
                            42719353 as core::ffi::c_int as fe_limb_t,
                            19141238 as core::ffi::c_int as fe_limb_t,
                            8044828 as core::ffi::c_int as fe_limb_t,
                            19737104 as core::ffi::c_int as fe_limb_t,
                            32239828 as core::ffi::c_int as fe_limb_t,
                            27901670 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            48505798 as core::ffi::c_int as fe_limb_t,
                            4762989 as core::ffi::c_int as fe_limb_t,
                            66182614 as core::ffi::c_int as fe_limb_t,
                            8885303 as core::ffi::c_int as fe_limb_t,
                            38696384 as core::ffi::c_int as fe_limb_t,
                            30367116 as core::ffi::c_int as fe_limb_t,
                            9781646 as core::ffi::c_int as fe_limb_t,
                            23204373 as core::ffi::c_int as fe_limb_t,
                            32779358 as core::ffi::c_int as fe_limb_t,
                            5095274 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            34100715 as core::ffi::c_int as fe_limb_t,
                            28339925 as core::ffi::c_int as fe_limb_t,
                            34843976 as core::ffi::c_int as fe_limb_t,
                            29869215 as core::ffi::c_int as fe_limb_t,
                            9460460 as core::ffi::c_int as fe_limb_t,
                            24227009 as core::ffi::c_int as fe_limb_t,
                            42507207 as core::ffi::c_int as fe_limb_t,
                            14506723 as core::ffi::c_int as fe_limb_t,
                            21639561 as core::ffi::c_int as fe_limb_t,
                            30924196 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            50707921 as core::ffi::c_int as fe_limb_t,
                            20442216 as core::ffi::c_int as fe_limb_t,
                            25239337 as core::ffi::c_int as fe_limb_t,
                            15531969 as core::ffi::c_int as fe_limb_t,
                            3987758 as core::ffi::c_int as fe_limb_t,
                            29055114 as core::ffi::c_int as fe_limb_t,
                            65819361 as core::ffi::c_int as fe_limb_t,
                            26690896 as core::ffi::c_int as fe_limb_t,
                            17874573 as core::ffi::c_int as fe_limb_t,
                            558605 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            53508735 as core::ffi::c_int as fe_limb_t,
                            10240080 as core::ffi::c_int as fe_limb_t,
                            9171883 as core::ffi::c_int as fe_limb_t,
                            16131053 as core::ffi::c_int as fe_limb_t,
                            46239610 as core::ffi::c_int as fe_limb_t,
                            9599699 as core::ffi::c_int as fe_limb_t,
                            33499487 as core::ffi::c_int as fe_limb_t,
                            5080151 as core::ffi::c_int as fe_limb_t,
                            2085892 as core::ffi::c_int as fe_limb_t,
                            5119761 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            44903700 as core::ffi::c_int as fe_limb_t,
                            31034903 as core::ffi::c_int as fe_limb_t,
                            50727262 as core::ffi::c_int as fe_limb_t,
                            414690 as core::ffi::c_int as fe_limb_t,
                            42089314 as core::ffi::c_int as fe_limb_t,
                            2170429 as core::ffi::c_int as fe_limb_t,
                            30634760 as core::ffi::c_int as fe_limb_t,
                            25190818 as core::ffi::c_int as fe_limb_t,
                            35108870 as core::ffi::c_int as fe_limb_t,
                            27794547 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            60263160 as core::ffi::c_int as fe_limb_t,
                            15791201 as core::ffi::c_int as fe_limb_t,
                            8550074 as core::ffi::c_int as fe_limb_t,
                            32241778 as core::ffi::c_int as fe_limb_t,
                            29928808 as core::ffi::c_int as fe_limb_t,
                            21462176 as core::ffi::c_int as fe_limb_t,
                            27534429 as core::ffi::c_int as fe_limb_t,
                            26362287 as core::ffi::c_int as fe_limb_t,
                            44757485 as core::ffi::c_int as fe_limb_t,
                            12961481 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            42616785 as core::ffi::c_int as fe_limb_t,
                            23983660 as core::ffi::c_int as fe_limb_t,
                            10368193 as core::ffi::c_int as fe_limb_t,
                            11582341 as core::ffi::c_int as fe_limb_t,
                            43711571 as core::ffi::c_int as fe_limb_t,
                            31309144 as core::ffi::c_int as fe_limb_t,
                            16533929 as core::ffi::c_int as fe_limb_t,
                            8206996 as core::ffi::c_int as fe_limb_t,
                            36914212 as core::ffi::c_int as fe_limb_t,
                            28394793 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            55987368 as core::ffi::c_int as fe_limb_t,
                            30172197 as core::ffi::c_int as fe_limb_t,
                            2307365 as core::ffi::c_int as fe_limb_t,
                            6362031 as core::ffi::c_int as fe_limb_t,
                            66973409 as core::ffi::c_int as fe_limb_t,
                            8868176 as core::ffi::c_int as fe_limb_t,
                            50273234 as core::ffi::c_int as fe_limb_t,
                            7031274 as core::ffi::c_int as fe_limb_t,
                            7589640 as core::ffi::c_int as fe_limb_t,
                            8945490 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            34956097 as core::ffi::c_int as fe_limb_t,
                            8917966 as core::ffi::c_int as fe_limb_t,
                            6661220 as core::ffi::c_int as fe_limb_t,
                            21876816 as core::ffi::c_int as fe_limb_t,
                            65916803 as core::ffi::c_int as fe_limb_t,
                            17761038 as core::ffi::c_int as fe_limb_t,
                            7251488 as core::ffi::c_int as fe_limb_t,
                            22372252 as core::ffi::c_int as fe_limb_t,
                            24099108 as core::ffi::c_int as fe_limb_t,
                            19098262 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            5019539 as core::ffi::c_int as fe_limb_t,
                            25646962 as core::ffi::c_int as fe_limb_t,
                            4244126 as core::ffi::c_int as fe_limb_t,
                            18840076 as core::ffi::c_int as fe_limb_t,
                            40175591 as core::ffi::c_int as fe_limb_t,
                            6453164 as core::ffi::c_int as fe_limb_t,
                            47990682 as core::ffi::c_int as fe_limb_t,
                            20265406 as core::ffi::c_int as fe_limb_t,
                            60876967 as core::ffi::c_int as fe_limb_t,
                            23273695 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10853575 as core::ffi::c_int as fe_limb_t,
                            10721687 as core::ffi::c_int as fe_limb_t,
                            26480089 as core::ffi::c_int as fe_limb_t,
                            5861829 as core::ffi::c_int as fe_limb_t,
                            44113045 as core::ffi::c_int as fe_limb_t,
                            1972174 as core::ffi::c_int as fe_limb_t,
                            65242217 as core::ffi::c_int as fe_limb_t,
                            22996533 as core::ffi::c_int as fe_limb_t,
                            63745412 as core::ffi::c_int as fe_limb_t,
                            27113307 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            50106456 as core::ffi::c_int as fe_limb_t,
                            5906789 as core::ffi::c_int as fe_limb_t,
                            221599 as core::ffi::c_int as fe_limb_t,
                            26991285 as core::ffi::c_int as fe_limb_t,
                            7828207 as core::ffi::c_int as fe_limb_t,
                            20305514 as core::ffi::c_int as fe_limb_t,
                            24362660 as core::ffi::c_int as fe_limb_t,
                            31546264 as core::ffi::c_int as fe_limb_t,
                            53242455 as core::ffi::c_int as fe_limb_t,
                            7421391 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            8139908 as core::ffi::c_int as fe_limb_t,
                            27007935 as core::ffi::c_int as fe_limb_t,
                            32257645 as core::ffi::c_int as fe_limb_t,
                            27663886 as core::ffi::c_int as fe_limb_t,
                            30375718 as core::ffi::c_int as fe_limb_t,
                            1886181 as core::ffi::c_int as fe_limb_t,
                            45933756 as core::ffi::c_int as fe_limb_t,
                            15441251 as core::ffi::c_int as fe_limb_t,
                            28826358 as core::ffi::c_int as fe_limb_t,
                            29431403 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            6267067 as core::ffi::c_int as fe_limb_t,
                            9695052 as core::ffi::c_int as fe_limb_t,
                            7709135 as core::ffi::c_int as fe_limb_t,
                            16950835 as core::ffi::c_int as fe_limb_t,
                            34239795 as core::ffi::c_int as fe_limb_t,
                            31668296 as core::ffi::c_int as fe_limb_t,
                            14795159 as core::ffi::c_int as fe_limb_t,
                            25714308 as core::ffi::c_int as fe_limb_t,
                            13746020 as core::ffi::c_int as fe_limb_t,
                            31812384 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28584883 as core::ffi::c_int as fe_limb_t,
                            7787108 as core::ffi::c_int as fe_limb_t,
                            60375922 as core::ffi::c_int as fe_limb_t,
                            18503702 as core::ffi::c_int as fe_limb_t,
                            22846040 as core::ffi::c_int as fe_limb_t,
                            25983196 as core::ffi::c_int as fe_limb_t,
                            63926927 as core::ffi::c_int as fe_limb_t,
                            33190907 as core::ffi::c_int as fe_limb_t,
                            4771361 as core::ffi::c_int as fe_limb_t,
                            25134474 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            24949256 as core::ffi::c_int as fe_limb_t,
                            6376279 as core::ffi::c_int as fe_limb_t,
                            39642383 as core::ffi::c_int as fe_limb_t,
                            25379823 as core::ffi::c_int as fe_limb_t,
                            48462709 as core::ffi::c_int as fe_limb_t,
                            23623825 as core::ffi::c_int as fe_limb_t,
                            33543568 as core::ffi::c_int as fe_limb_t,
                            21412737 as core::ffi::c_int as fe_limb_t,
                            3569626 as core::ffi::c_int as fe_limb_t,
                            11342593 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            26514970 as core::ffi::c_int as fe_limb_t,
                            4740088 as core::ffi::c_int as fe_limb_t,
                            27912651 as core::ffi::c_int as fe_limb_t,
                            3697550 as core::ffi::c_int as fe_limb_t,
                            19331575 as core::ffi::c_int as fe_limb_t,
                            22082093 as core::ffi::c_int as fe_limb_t,
                            6809885 as core::ffi::c_int as fe_limb_t,
                            4608608 as core::ffi::c_int as fe_limb_t,
                            7325975 as core::ffi::c_int as fe_limb_t,
                            18753361 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55490446 as core::ffi::c_int as fe_limb_t,
                            19000001 as core::ffi::c_int as fe_limb_t,
                            42787651 as core::ffi::c_int as fe_limb_t,
                            7655127 as core::ffi::c_int as fe_limb_t,
                            65739590 as core::ffi::c_int as fe_limb_t,
                            5214311 as core::ffi::c_int as fe_limb_t,
                            39708324 as core::ffi::c_int as fe_limb_t,
                            10258389 as core::ffi::c_int as fe_limb_t,
                            49462170 as core::ffi::c_int as fe_limb_t,
                            25367739 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11431185 as core::ffi::c_int as fe_limb_t,
                            15823007 as core::ffi::c_int as fe_limb_t,
                            26570245 as core::ffi::c_int as fe_limb_t,
                            14329124 as core::ffi::c_int as fe_limb_t,
                            18029990 as core::ffi::c_int as fe_limb_t,
                            4796082 as core::ffi::c_int as fe_limb_t,
                            35662685 as core::ffi::c_int as fe_limb_t,
                            15580663 as core::ffi::c_int as fe_limb_t,
                            9280358 as core::ffi::c_int as fe_limb_t,
                            29580745 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            66948081 as core::ffi::c_int as fe_limb_t,
                            23228174 as core::ffi::c_int as fe_limb_t,
                            44253547 as core::ffi::c_int as fe_limb_t,
                            29249434 as core::ffi::c_int as fe_limb_t,
                            46247496 as core::ffi::c_int as fe_limb_t,
                            19933429 as core::ffi::c_int as fe_limb_t,
                            34297962 as core::ffi::c_int as fe_limb_t,
                            22372809 as core::ffi::c_int as fe_limb_t,
                            51563772 as core::ffi::c_int as fe_limb_t,
                            4387440 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46309467 as core::ffi::c_int as fe_limb_t,
                            12194511 as core::ffi::c_int as fe_limb_t,
                            3937617 as core::ffi::c_int as fe_limb_t,
                            27748540 as core::ffi::c_int as fe_limb_t,
                            39954043 as core::ffi::c_int as fe_limb_t,
                            9340369 as core::ffi::c_int as fe_limb_t,
                            42594872 as core::ffi::c_int as fe_limb_t,
                            8548136 as core::ffi::c_int as fe_limb_t,
                            20617071 as core::ffi::c_int as fe_limb_t,
                            26072431 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            66170039 as core::ffi::c_int as fe_limb_t,
                            29623845 as core::ffi::c_int as fe_limb_t,
                            58394552 as core::ffi::c_int as fe_limb_t,
                            16124717 as core::ffi::c_int as fe_limb_t,
                            24603125 as core::ffi::c_int as fe_limb_t,
                            27329039 as core::ffi::c_int as fe_limb_t,
                            53333511 as core::ffi::c_int as fe_limb_t,
                            21678609 as core::ffi::c_int as fe_limb_t,
                            24345682 as core::ffi::c_int as fe_limb_t,
                            10325460 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47253587 as core::ffi::c_int as fe_limb_t,
                            31985546 as core::ffi::c_int as fe_limb_t,
                            44906155 as core::ffi::c_int as fe_limb_t,
                            8714033 as core::ffi::c_int as fe_limb_t,
                            14007766 as core::ffi::c_int as fe_limb_t,
                            6928528 as core::ffi::c_int as fe_limb_t,
                            16318175 as core::ffi::c_int as fe_limb_t,
                            32543743 as core::ffi::c_int as fe_limb_t,
                            4766742 as core::ffi::c_int as fe_limb_t,
                            3552007 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45357481 as core::ffi::c_int as fe_limb_t,
                            16823515 as core::ffi::c_int as fe_limb_t,
                            1351762 as core::ffi::c_int as fe_limb_t,
                            32751011 as core::ffi::c_int as fe_limb_t,
                            63099193 as core::ffi::c_int as fe_limb_t,
                            3950934 as core::ffi::c_int as fe_limb_t,
                            3217514 as core::ffi::c_int as fe_limb_t,
                            14481909 as core::ffi::c_int as fe_limb_t,
                            10988822 as core::ffi::c_int as fe_limb_t,
                            29559670 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15564307 as core::ffi::c_int as fe_limb_t,
                            19242862 as core::ffi::c_int as fe_limb_t,
                            3101242 as core::ffi::c_int as fe_limb_t,
                            5684148 as core::ffi::c_int as fe_limb_t,
                            30446780 as core::ffi::c_int as fe_limb_t,
                            25503076 as core::ffi::c_int as fe_limb_t,
                            12677126 as core::ffi::c_int as fe_limb_t,
                            27049089 as core::ffi::c_int as fe_limb_t,
                            58813011 as core::ffi::c_int as fe_limb_t,
                            13296004 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            57666574 as core::ffi::c_int as fe_limb_t,
                            6624295 as core::ffi::c_int as fe_limb_t,
                            36809900 as core::ffi::c_int as fe_limb_t,
                            21640754 as core::ffi::c_int as fe_limb_t,
                            62437882 as core::ffi::c_int as fe_limb_t,
                            31497052 as core::ffi::c_int as fe_limb_t,
                            31521203 as core::ffi::c_int as fe_limb_t,
                            9614054 as core::ffi::c_int as fe_limb_t,
                            37108040 as core::ffi::c_int as fe_limb_t,
                            12074673 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            4771172 as core::ffi::c_int as fe_limb_t,
                            33419193 as core::ffi::c_int as fe_limb_t,
                            14290748 as core::ffi::c_int as fe_limb_t,
                            20464580 as core::ffi::c_int as fe_limb_t,
                            27992297 as core::ffi::c_int as fe_limb_t,
                            14998318 as core::ffi::c_int as fe_limb_t,
                            65694928 as core::ffi::c_int as fe_limb_t,
                            31997715 as core::ffi::c_int as fe_limb_t,
                            29832612 as core::ffi::c_int as fe_limb_t,
                            17163397 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            7064884 as core::ffi::c_int as fe_limb_t,
                            26013258 as core::ffi::c_int as fe_limb_t,
                            47946901 as core::ffi::c_int as fe_limb_t,
                            28486894 as core::ffi::c_int as fe_limb_t,
                            48217594 as core::ffi::c_int as fe_limb_t,
                            30641695 as core::ffi::c_int as fe_limb_t,
                            25825241 as core::ffi::c_int as fe_limb_t,
                            5293297 as core::ffi::c_int as fe_limb_t,
                            39986204 as core::ffi::c_int as fe_limb_t,
                            13101589 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            64810282 as core::ffi::c_int as fe_limb_t,
                            2439669 as core::ffi::c_int as fe_limb_t,
                            59642254 as core::ffi::c_int as fe_limb_t,
                            1719964 as core::ffi::c_int as fe_limb_t,
                            39841323 as core::ffi::c_int as fe_limb_t,
                            17225986 as core::ffi::c_int as fe_limb_t,
                            32512468 as core::ffi::c_int as fe_limb_t,
                            28236839 as core::ffi::c_int as fe_limb_t,
                            36752793 as core::ffi::c_int as fe_limb_t,
                            29363474 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            37102324 as core::ffi::c_int as fe_limb_t,
                            10162315 as core::ffi::c_int as fe_limb_t,
                            33928688 as core::ffi::c_int as fe_limb_t,
                            3981722 as core::ffi::c_int as fe_limb_t,
                            50626726 as core::ffi::c_int as fe_limb_t,
                            20484387 as core::ffi::c_int as fe_limb_t,
                            14413973 as core::ffi::c_int as fe_limb_t,
                            9515896 as core::ffi::c_int as fe_limb_t,
                            19568978 as core::ffi::c_int as fe_limb_t,
                            9628812 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            33053803 as core::ffi::c_int as fe_limb_t,
                            199357 as core::ffi::c_int as fe_limb_t,
                            15894591 as core::ffi::c_int as fe_limb_t,
                            1583059 as core::ffi::c_int as fe_limb_t,
                            27380243 as core::ffi::c_int as fe_limb_t,
                            28973997 as core::ffi::c_int as fe_limb_t,
                            49269969 as core::ffi::c_int as fe_limb_t,
                            27447592 as core::ffi::c_int as fe_limb_t,
                            60817077 as core::ffi::c_int as fe_limb_t,
                            3437739 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            48129987 as core::ffi::c_int as fe_limb_t,
                            3884492 as core::ffi::c_int as fe_limb_t,
                            19469877 as core::ffi::c_int as fe_limb_t,
                            12726490 as core::ffi::c_int as fe_limb_t,
                            15913552 as core::ffi::c_int as fe_limb_t,
                            13614290 as core::ffi::c_int as fe_limb_t,
                            44147131 as core::ffi::c_int as fe_limb_t,
                            70103 as core::ffi::c_int as fe_limb_t,
                            7463304 as core::ffi::c_int as fe_limb_t,
                            4176122 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            39984863 as core::ffi::c_int as fe_limb_t,
                            10659916 as core::ffi::c_int as fe_limb_t,
                            11482427 as core::ffi::c_int as fe_limb_t,
                            17484051 as core::ffi::c_int as fe_limb_t,
                            12771466 as core::ffi::c_int as fe_limb_t,
                            26919315 as core::ffi::c_int as fe_limb_t,
                            34389459 as core::ffi::c_int as fe_limb_t,
                            28231680 as core::ffi::c_int as fe_limb_t,
                            24216881 as core::ffi::c_int as fe_limb_t,
                            5944158 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            8894125 as core::ffi::c_int as fe_limb_t,
                            7450974 as core::ffi::c_int as fe_limb_t,
                            64444715 as core::ffi::c_int as fe_limb_t,
                            23788679 as core::ffi::c_int as fe_limb_t,
                            39028346 as core::ffi::c_int as fe_limb_t,
                            21165316 as core::ffi::c_int as fe_limb_t,
                            19345745 as core::ffi::c_int as fe_limb_t,
                            14680796 as core::ffi::c_int as fe_limb_t,
                            11632993 as core::ffi::c_int as fe_limb_t,
                            5847885 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            26942781 as core::ffi::c_int as fe_limb_t,
                            31239115 as core::ffi::c_int as fe_limb_t,
                            9129563 as core::ffi::c_int as fe_limb_t,
                            28647825 as core::ffi::c_int as fe_limb_t,
                            26024104 as core::ffi::c_int as fe_limb_t,
                            11769399 as core::ffi::c_int as fe_limb_t,
                            55590027 as core::ffi::c_int as fe_limb_t,
                            6367193 as core::ffi::c_int as fe_limb_t,
                            57381634 as core::ffi::c_int as fe_limb_t,
                            4782139 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19916442 as core::ffi::c_int as fe_limb_t,
                            28726022 as core::ffi::c_int as fe_limb_t,
                            44198159 as core::ffi::c_int as fe_limb_t,
                            22140040 as core::ffi::c_int as fe_limb_t,
                            25606323 as core::ffi::c_int as fe_limb_t,
                            27581991 as core::ffi::c_int as fe_limb_t,
                            33253852 as core::ffi::c_int as fe_limb_t,
                            8220911 as core::ffi::c_int as fe_limb_t,
                            6358847 as core::ffi::c_int as fe_limb_t,
                            31680575 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            801428 as core::ffi::c_int as fe_limb_t,
                            31472730 as core::ffi::c_int as fe_limb_t,
                            16569427 as core::ffi::c_int as fe_limb_t,
                            11065167 as core::ffi::c_int as fe_limb_t,
                            29875704 as core::ffi::c_int as fe_limb_t,
                            96627 as core::ffi::c_int as fe_limb_t,
                            7908388 as core::ffi::c_int as fe_limb_t,
                            29073952 as core::ffi::c_int as fe_limb_t,
                            53570360 as core::ffi::c_int as fe_limb_t,
                            1387154 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            19646058 as core::ffi::c_int as fe_limb_t,
                            5720633 as core::ffi::c_int as fe_limb_t,
                            55692158 as core::ffi::c_int as fe_limb_t,
                            12814208 as core::ffi::c_int as fe_limb_t,
                            11607948 as core::ffi::c_int as fe_limb_t,
                            12749789 as core::ffi::c_int as fe_limb_t,
                            14147075 as core::ffi::c_int as fe_limb_t,
                            15156355 as core::ffi::c_int as fe_limb_t,
                            45242033 as core::ffi::c_int as fe_limb_t,
                            11835259 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19299512 as core::ffi::c_int as fe_limb_t,
                            1155910 as core::ffi::c_int as fe_limb_t,
                            28703737 as core::ffi::c_int as fe_limb_t,
                            14890794 as core::ffi::c_int as fe_limb_t,
                            2925026 as core::ffi::c_int as fe_limb_t,
                            7269399 as core::ffi::c_int as fe_limb_t,
                            26121523 as core::ffi::c_int as fe_limb_t,
                            15467869 as core::ffi::c_int as fe_limb_t,
                            40548314 as core::ffi::c_int as fe_limb_t,
                            5052482 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            64091413 as core::ffi::c_int as fe_limb_t,
                            10058205 as core::ffi::c_int as fe_limb_t,
                            1980837 as core::ffi::c_int as fe_limb_t,
                            3964243 as core::ffi::c_int as fe_limb_t,
                            22160966 as core::ffi::c_int as fe_limb_t,
                            12322533 as core::ffi::c_int as fe_limb_t,
                            60677741 as core::ffi::c_int as fe_limb_t,
                            20936246 as core::ffi::c_int as fe_limb_t,
                            12228556 as core::ffi::c_int as fe_limb_t,
                            26550755 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            32944382 as core::ffi::c_int as fe_limb_t,
                            14922211 as core::ffi::c_int as fe_limb_t,
                            44263970 as core::ffi::c_int as fe_limb_t,
                            5188527 as core::ffi::c_int as fe_limb_t,
                            21913450 as core::ffi::c_int as fe_limb_t,
                            24834489 as core::ffi::c_int as fe_limb_t,
                            4001464 as core::ffi::c_int as fe_limb_t,
                            13238564 as core::ffi::c_int as fe_limb_t,
                            60994061 as core::ffi::c_int as fe_limb_t,
                            8653814 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            22865569 as core::ffi::c_int as fe_limb_t,
                            28901697 as core::ffi::c_int as fe_limb_t,
                            27603667 as core::ffi::c_int as fe_limb_t,
                            21009037 as core::ffi::c_int as fe_limb_t,
                            14348957 as core::ffi::c_int as fe_limb_t,
                            8234005 as core::ffi::c_int as fe_limb_t,
                            24808405 as core::ffi::c_int as fe_limb_t,
                            5719875 as core::ffi::c_int as fe_limb_t,
                            28483275 as core::ffi::c_int as fe_limb_t,
                            2841751 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50687877 as core::ffi::c_int as fe_limb_t,
                            32441126 as core::ffi::c_int as fe_limb_t,
                            66781144 as core::ffi::c_int as fe_limb_t,
                            21446575 as core::ffi::c_int as fe_limb_t,
                            21886281 as core::ffi::c_int as fe_limb_t,
                            18001658 as core::ffi::c_int as fe_limb_t,
                            65220897 as core::ffi::c_int as fe_limb_t,
                            33238773 as core::ffi::c_int as fe_limb_t,
                            19932057 as core::ffi::c_int as fe_limb_t,
                            20815229 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            55452759 as core::ffi::c_int as fe_limb_t,
                            10087520 as core::ffi::c_int as fe_limb_t,
                            58243976 as core::ffi::c_int as fe_limb_t,
                            28018288 as core::ffi::c_int as fe_limb_t,
                            47830290 as core::ffi::c_int as fe_limb_t,
                            30498519 as core::ffi::c_int as fe_limb_t,
                            3999227 as core::ffi::c_int as fe_limb_t,
                            13239134 as core::ffi::c_int as fe_limb_t,
                            62331395 as core::ffi::c_int as fe_limb_t,
                            19644223 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            1382174 as core::ffi::c_int as fe_limb_t,
                            21859713 as core::ffi::c_int as fe_limb_t,
                            17266789 as core::ffi::c_int as fe_limb_t,
                            9194690 as core::ffi::c_int as fe_limb_t,
                            53784508 as core::ffi::c_int as fe_limb_t,
                            9720080 as core::ffi::c_int as fe_limb_t,
                            20403944 as core::ffi::c_int as fe_limb_t,
                            11284705 as core::ffi::c_int as fe_limb_t,
                            53095046 as core::ffi::c_int as fe_limb_t,
                            3093229 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16650902 as core::ffi::c_int as fe_limb_t,
                            22516500 as core::ffi::c_int as fe_limb_t,
                            66044685 as core::ffi::c_int as fe_limb_t,
                            1570628 as core::ffi::c_int as fe_limb_t,
                            58779118 as core::ffi::c_int as fe_limb_t,
                            7352752 as core::ffi::c_int as fe_limb_t,
                            66806440 as core::ffi::c_int as fe_limb_t,
                            16271224 as core::ffi::c_int as fe_limb_t,
                            43059443 as core::ffi::c_int as fe_limb_t,
                            26862581 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            45197768 as core::ffi::c_int as fe_limb_t,
                            27626490 as core::ffi::c_int as fe_limb_t,
                            62497547 as core::ffi::c_int as fe_limb_t,
                            27994275 as core::ffi::c_int as fe_limb_t,
                            35364760 as core::ffi::c_int as fe_limb_t,
                            22769138 as core::ffi::c_int as fe_limb_t,
                            24123613 as core::ffi::c_int as fe_limb_t,
                            15193618 as core::ffi::c_int as fe_limb_t,
                            45456747 as core::ffi::c_int as fe_limb_t,
                            16815042 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57172930 as core::ffi::c_int as fe_limb_t,
                            29264984 as core::ffi::c_int as fe_limb_t,
                            41829040 as core::ffi::c_int as fe_limb_t,
                            4372841 as core::ffi::c_int as fe_limb_t,
                            2087473 as core::ffi::c_int as fe_limb_t,
                            10399484 as core::ffi::c_int as fe_limb_t,
                            31870908 as core::ffi::c_int as fe_limb_t,
                            14690798 as core::ffi::c_int as fe_limb_t,
                            17361620 as core::ffi::c_int as fe_limb_t,
                            11864968 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            55801235 as core::ffi::c_int as fe_limb_t,
                            6210371 as core::ffi::c_int as fe_limb_t,
                            13206574 as core::ffi::c_int as fe_limb_t,
                            5806320 as core::ffi::c_int as fe_limb_t,
                            38091172 as core::ffi::c_int as fe_limb_t,
                            19587231 as core::ffi::c_int as fe_limb_t,
                            54777658 as core::ffi::c_int as fe_limb_t,
                            26067830 as core::ffi::c_int as fe_limb_t,
                            41530403 as core::ffi::c_int as fe_limb_t,
                            17313742 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            14668443 as core::ffi::c_int as fe_limb_t,
                            21284197 as core::ffi::c_int as fe_limb_t,
                            26039038 as core::ffi::c_int as fe_limb_t,
                            15305210 as core::ffi::c_int as fe_limb_t,
                            25515617 as core::ffi::c_int as fe_limb_t,
                            4542480 as core::ffi::c_int as fe_limb_t,
                            10453892 as core::ffi::c_int as fe_limb_t,
                            6577524 as core::ffi::c_int as fe_limb_t,
                            9145645 as core::ffi::c_int as fe_limb_t,
                            27110552 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            5974855 as core::ffi::c_int as fe_limb_t,
                            3053895 as core::ffi::c_int as fe_limb_t,
                            57675815 as core::ffi::c_int as fe_limb_t,
                            23169240 as core::ffi::c_int as fe_limb_t,
                            35243739 as core::ffi::c_int as fe_limb_t,
                            3225008 as core::ffi::c_int as fe_limb_t,
                            59136222 as core::ffi::c_int as fe_limb_t,
                            3936127 as core::ffi::c_int as fe_limb_t,
                            61456591 as core::ffi::c_int as fe_limb_t,
                            30504127 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30625386 as core::ffi::c_int as fe_limb_t,
                            28825032 as core::ffi::c_int as fe_limb_t,
                            41552902 as core::ffi::c_int as fe_limb_t,
                            20761565 as core::ffi::c_int as fe_limb_t,
                            46624288 as core::ffi::c_int as fe_limb_t,
                            7695098 as core::ffi::c_int as fe_limb_t,
                            17097188 as core::ffi::c_int as fe_limb_t,
                            17250936 as core::ffi::c_int as fe_limb_t,
                            39109084 as core::ffi::c_int as fe_limb_t,
                            1803631 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            63555773 as core::ffi::c_int as fe_limb_t,
                            9865098 as core::ffi::c_int as fe_limb_t,
                            61880298 as core::ffi::c_int as fe_limb_t,
                            4272700 as core::ffi::c_int as fe_limb_t,
                            61435032 as core::ffi::c_int as fe_limb_t,
                            16864731 as core::ffi::c_int as fe_limb_t,
                            14911343 as core::ffi::c_int as fe_limb_t,
                            12196514 as core::ffi::c_int as fe_limb_t,
                            45703375 as core::ffi::c_int as fe_limb_t,
                            7047411 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            20093258 as core::ffi::c_int as fe_limb_t,
                            9920966 as core::ffi::c_int as fe_limb_t,
                            55970670 as core::ffi::c_int as fe_limb_t,
                            28210574 as core::ffi::c_int as fe_limb_t,
                            13161586 as core::ffi::c_int as fe_limb_t,
                            12044805 as core::ffi::c_int as fe_limb_t,
                            34252013 as core::ffi::c_int as fe_limb_t,
                            4124600 as core::ffi::c_int as fe_limb_t,
                            34765036 as core::ffi::c_int as fe_limb_t,
                            23296865 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            46320040 as core::ffi::c_int as fe_limb_t,
                            14084653 as core::ffi::c_int as fe_limb_t,
                            53577151 as core::ffi::c_int as fe_limb_t,
                            7842146 as core::ffi::c_int as fe_limb_t,
                            19119038 as core::ffi::c_int as fe_limb_t,
                            19731827 as core::ffi::c_int as fe_limb_t,
                            4752376 as core::ffi::c_int as fe_limb_t,
                            24839792 as core::ffi::c_int as fe_limb_t,
                            45429205 as core::ffi::c_int as fe_limb_t,
                            2288037 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40289628 as core::ffi::c_int as fe_limb_t,
                            30270716 as core::ffi::c_int as fe_limb_t,
                            29965058 as core::ffi::c_int as fe_limb_t,
                            3039786 as core::ffi::c_int as fe_limb_t,
                            52635099 as core::ffi::c_int as fe_limb_t,
                            2540456 as core::ffi::c_int as fe_limb_t,
                            29457502 as core::ffi::c_int as fe_limb_t,
                            14625692 as core::ffi::c_int as fe_limb_t,
                            42289247 as core::ffi::c_int as fe_limb_t,
                            12570231 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            66045306 as core::ffi::c_int as fe_limb_t,
                            22002608 as core::ffi::c_int as fe_limb_t,
                            16920317 as core::ffi::c_int as fe_limb_t,
                            12494842 as core::ffi::c_int as fe_limb_t,
                            1278292 as core::ffi::c_int as fe_limb_t,
                            27685323 as core::ffi::c_int as fe_limb_t,
                            45948920 as core::ffi::c_int as fe_limb_t,
                            30055751 as core::ffi::c_int as fe_limb_t,
                            55134159 as core::ffi::c_int as fe_limb_t,
                            4724942 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            17960970 as core::ffi::c_int as fe_limb_t,
                            21778898 as core::ffi::c_int as fe_limb_t,
                            62967895 as core::ffi::c_int as fe_limb_t,
                            23851901 as core::ffi::c_int as fe_limb_t,
                            58232301 as core::ffi::c_int as fe_limb_t,
                            32143814 as core::ffi::c_int as fe_limb_t,
                            54201480 as core::ffi::c_int as fe_limb_t,
                            24894499 as core::ffi::c_int as fe_limb_t,
                            37532563 as core::ffi::c_int as fe_limb_t,
                            1903855 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            23134274 as core::ffi::c_int as fe_limb_t,
                            19275300 as core::ffi::c_int as fe_limb_t,
                            56426866 as core::ffi::c_int as fe_limb_t,
                            31942495 as core::ffi::c_int as fe_limb_t,
                            20684484 as core::ffi::c_int as fe_limb_t,
                            15770816 as core::ffi::c_int as fe_limb_t,
                            54119114 as core::ffi::c_int as fe_limb_t,
                            3190295 as core::ffi::c_int as fe_limb_t,
                            26955097 as core::ffi::c_int as fe_limb_t,
                            14109738 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            15308788 as core::ffi::c_int as fe_limb_t,
                            5320727 as core::ffi::c_int as fe_limb_t,
                            36995055 as core::ffi::c_int as fe_limb_t,
                            19235554 as core::ffi::c_int as fe_limb_t,
                            22902007 as core::ffi::c_int as fe_limb_t,
                            7767164 as core::ffi::c_int as fe_limb_t,
                            29425325 as core::ffi::c_int as fe_limb_t,
                            22276870 as core::ffi::c_int as fe_limb_t,
                            31960941 as core::ffi::c_int as fe_limb_t,
                            11934971 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            39713153 as core::ffi::c_int as fe_limb_t,
                            8435795 as core::ffi::c_int as fe_limb_t,
                            4109644 as core::ffi::c_int as fe_limb_t,
                            12222639 as core::ffi::c_int as fe_limb_t,
                            42480996 as core::ffi::c_int as fe_limb_t,
                            14818668 as core::ffi::c_int as fe_limb_t,
                            20638173 as core::ffi::c_int as fe_limb_t,
                            4875028 as core::ffi::c_int as fe_limb_t,
                            10491392 as core::ffi::c_int as fe_limb_t,
                            1379718 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            53949449 as core::ffi::c_int as fe_limb_t,
                            9197840 as core::ffi::c_int as fe_limb_t,
                            3875503 as core::ffi::c_int as fe_limb_t,
                            24618324 as core::ffi::c_int as fe_limb_t,
                            65725151 as core::ffi::c_int as fe_limb_t,
                            27674630 as core::ffi::c_int as fe_limb_t,
                            33518458 as core::ffi::c_int as fe_limb_t,
                            16176658 as core::ffi::c_int as fe_limb_t,
                            21432314 as core::ffi::c_int as fe_limb_t,
                            12180697 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55321537 as core::ffi::c_int as fe_limb_t,
                            11500837 as core::ffi::c_int as fe_limb_t,
                            13787581 as core::ffi::c_int as fe_limb_t,
                            19721842 as core::ffi::c_int as fe_limb_t,
                            44678184 as core::ffi::c_int as fe_limb_t,
                            10140204 as core::ffi::c_int as fe_limb_t,
                            1465425 as core::ffi::c_int as fe_limb_t,
                            12689540 as core::ffi::c_int as fe_limb_t,
                            56807545 as core::ffi::c_int as fe_limb_t,
                            19681548 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            5414091 as core::ffi::c_int as fe_limb_t,
                            18168391 as core::ffi::c_int as fe_limb_t,
                            46101199 as core::ffi::c_int as fe_limb_t,
                            9643569 as core::ffi::c_int as fe_limb_t,
                            12834970 as core::ffi::c_int as fe_limb_t,
                            1186149 as core::ffi::c_int as fe_limb_t,
                            64485948 as core::ffi::c_int as fe_limb_t,
                            32212200 as core::ffi::c_int as fe_limb_t,
                            26128230 as core::ffi::c_int as fe_limb_t,
                            6032912 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40771450 as core::ffi::c_int as fe_limb_t,
                            19788269 as core::ffi::c_int as fe_limb_t,
                            32496024 as core::ffi::c_int as fe_limb_t,
                            19900513 as core::ffi::c_int as fe_limb_t,
                            17847800 as core::ffi::c_int as fe_limb_t,
                            20885276 as core::ffi::c_int as fe_limb_t,
                            3604024 as core::ffi::c_int as fe_limb_t,
                            8316894 as core::ffi::c_int as fe_limb_t,
                            41233830 as core::ffi::c_int as fe_limb_t,
                            23117073 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            3296484 as core::ffi::c_int as fe_limb_t,
                            6223048 as core::ffi::c_int as fe_limb_t,
                            24680646 as core::ffi::c_int as fe_limb_t,
                            21307972 as core::ffi::c_int as fe_limb_t,
                            44056843 as core::ffi::c_int as fe_limb_t,
                            5903204 as core::ffi::c_int as fe_limb_t,
                            58246567 as core::ffi::c_int as fe_limb_t,
                            28915267 as core::ffi::c_int as fe_limb_t,
                            12376616 as core::ffi::c_int as fe_limb_t,
                            3188849 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            29190469 as core::ffi::c_int as fe_limb_t,
                            18895386 as core::ffi::c_int as fe_limb_t,
                            27549112 as core::ffi::c_int as fe_limb_t,
                            32370916 as core::ffi::c_int as fe_limb_t,
                            3520065 as core::ffi::c_int as fe_limb_t,
                            22857131 as core::ffi::c_int as fe_limb_t,
                            32049514 as core::ffi::c_int as fe_limb_t,
                            26245319 as core::ffi::c_int as fe_limb_t,
                            50999629 as core::ffi::c_int as fe_limb_t,
                            23702124 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            52364359 as core::ffi::c_int as fe_limb_t,
                            24245275 as core::ffi::c_int as fe_limb_t,
                            735817 as core::ffi::c_int as fe_limb_t,
                            32955454 as core::ffi::c_int as fe_limb_t,
                            46701176 as core::ffi::c_int as fe_limb_t,
                            28496527 as core::ffi::c_int as fe_limb_t,
                            25246077 as core::ffi::c_int as fe_limb_t,
                            17758763 as core::ffi::c_int as fe_limb_t,
                            18640740 as core::ffi::c_int as fe_limb_t,
                            32593455 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            60180029 as core::ffi::c_int as fe_limb_t,
                            17123636 as core::ffi::c_int as fe_limb_t,
                            10361373 as core::ffi::c_int as fe_limb_t,
                            5642961 as core::ffi::c_int as fe_limb_t,
                            4910474 as core::ffi::c_int as fe_limb_t,
                            12345252 as core::ffi::c_int as fe_limb_t,
                            35470478 as core::ffi::c_int as fe_limb_t,
                            33060001 as core::ffi::c_int as fe_limb_t,
                            10530746 as core::ffi::c_int as fe_limb_t,
                            1053335 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            37842897 as core::ffi::c_int as fe_limb_t,
                            19367626 as core::ffi::c_int as fe_limb_t,
                            53570647 as core::ffi::c_int as fe_limb_t,
                            21437058 as core::ffi::c_int as fe_limb_t,
                            47651804 as core::ffi::c_int as fe_limb_t,
                            22899047 as core::ffi::c_int as fe_limb_t,
                            35646494 as core::ffi::c_int as fe_limb_t,
                            30605446 as core::ffi::c_int as fe_limb_t,
                            24018830 as core::ffi::c_int as fe_limb_t,
                            15026644 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            44516310 as core::ffi::c_int as fe_limb_t,
                            30409154 as core::ffi::c_int as fe_limb_t,
                            64819587 as core::ffi::c_int as fe_limb_t,
                            5953842 as core::ffi::c_int as fe_limb_t,
                            53668675 as core::ffi::c_int as fe_limb_t,
                            9425630 as core::ffi::c_int as fe_limb_t,
                            25310643 as core::ffi::c_int as fe_limb_t,
                            13003497 as core::ffi::c_int as fe_limb_t,
                            64794073 as core::ffi::c_int as fe_limb_t,
                            18408815 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            39688860 as core::ffi::c_int as fe_limb_t,
                            32951110 as core::ffi::c_int as fe_limb_t,
                            59064879 as core::ffi::c_int as fe_limb_t,
                            31885314 as core::ffi::c_int as fe_limb_t,
                            41016598 as core::ffi::c_int as fe_limb_t,
                            13987818 as core::ffi::c_int as fe_limb_t,
                            39811242 as core::ffi::c_int as fe_limb_t,
                            187898 as core::ffi::c_int as fe_limb_t,
                            43942445 as core::ffi::c_int as fe_limb_t,
                            31022696 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            45364466 as core::ffi::c_int as fe_limb_t,
                            19743956 as core::ffi::c_int as fe_limb_t,
                            1844839 as core::ffi::c_int as fe_limb_t,
                            5021428 as core::ffi::c_int as fe_limb_t,
                            56674465 as core::ffi::c_int as fe_limb_t,
                            17642958 as core::ffi::c_int as fe_limb_t,
                            9716666 as core::ffi::c_int as fe_limb_t,
                            16266922 as core::ffi::c_int as fe_limb_t,
                            62038647 as core::ffi::c_int as fe_limb_t,
                            726098 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29370903 as core::ffi::c_int as fe_limb_t,
                            27500434 as core::ffi::c_int as fe_limb_t,
                            7334070 as core::ffi::c_int as fe_limb_t,
                            18212173 as core::ffi::c_int as fe_limb_t,
                            9385286 as core::ffi::c_int as fe_limb_t,
                            2247707 as core::ffi::c_int as fe_limb_t,
                            53446902 as core::ffi::c_int as fe_limb_t,
                            28714970 as core::ffi::c_int as fe_limb_t,
                            30007387 as core::ffi::c_int as fe_limb_t,
                            17731091 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            66172485 as core::ffi::c_int as fe_limb_t,
                            16086690 as core::ffi::c_int as fe_limb_t,
                            23751945 as core::ffi::c_int as fe_limb_t,
                            33011114 as core::ffi::c_int as fe_limb_t,
                            65941325 as core::ffi::c_int as fe_limb_t,
                            28365395 as core::ffi::c_int as fe_limb_t,
                            9137108 as core::ffi::c_int as fe_limb_t,
                            730663 as core::ffi::c_int as fe_limb_t,
                            9835848 as core::ffi::c_int as fe_limb_t,
                            4555336 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43732429 as core::ffi::c_int as fe_limb_t,
                            1410445 as core::ffi::c_int as fe_limb_t,
                            44855111 as core::ffi::c_int as fe_limb_t,
                            20654817 as core::ffi::c_int as fe_limb_t,
                            30867634 as core::ffi::c_int as fe_limb_t,
                            15826977 as core::ffi::c_int as fe_limb_t,
                            17693930 as core::ffi::c_int as fe_limb_t,
                            544696 as core::ffi::c_int as fe_limb_t,
                            55123566 as core::ffi::c_int as fe_limb_t,
                            12422645 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            31117226 as core::ffi::c_int as fe_limb_t,
                            21338698 as core::ffi::c_int as fe_limb_t,
                            53606025 as core::ffi::c_int as fe_limb_t,
                            6561946 as core::ffi::c_int as fe_limb_t,
                            57231997 as core::ffi::c_int as fe_limb_t,
                            20796761 as core::ffi::c_int as fe_limb_t,
                            61990178 as core::ffi::c_int as fe_limb_t,
                            29457725 as core::ffi::c_int as fe_limb_t,
                            29120152 as core::ffi::c_int as fe_limb_t,
                            13924425 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49707966 as core::ffi::c_int as fe_limb_t,
                            19321222 as core::ffi::c_int as fe_limb_t,
                            19675798 as core::ffi::c_int as fe_limb_t,
                            30819676 as core::ffi::c_int as fe_limb_t,
                            56101901 as core::ffi::c_int as fe_limb_t,
                            27695611 as core::ffi::c_int as fe_limb_t,
                            57724924 as core::ffi::c_int as fe_limb_t,
                            22236731 as core::ffi::c_int as fe_limb_t,
                            7240930 as core::ffi::c_int as fe_limb_t,
                            33317044 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            35747106 as core::ffi::c_int as fe_limb_t,
                            22207651 as core::ffi::c_int as fe_limb_t,
                            52101416 as core::ffi::c_int as fe_limb_t,
                            27698213 as core::ffi::c_int as fe_limb_t,
                            44655523 as core::ffi::c_int as fe_limb_t,
                            21401660 as core::ffi::c_int as fe_limb_t,
                            1222335 as core::ffi::c_int as fe_limb_t,
                            4389483 as core::ffi::c_int as fe_limb_t,
                            3293637 as core::ffi::c_int as fe_limb_t,
                            18002689 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            50424044 as core::ffi::c_int as fe_limb_t,
                            19110186 as core::ffi::c_int as fe_limb_t,
                            11038543 as core::ffi::c_int as fe_limb_t,
                            11054958 as core::ffi::c_int as fe_limb_t,
                            53307689 as core::ffi::c_int as fe_limb_t,
                            30215898 as core::ffi::c_int as fe_limb_t,
                            42789283 as core::ffi::c_int as fe_limb_t,
                            7733546 as core::ffi::c_int as fe_limb_t,
                            12796905 as core::ffi::c_int as fe_limb_t,
                            27218610 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            58349431 as core::ffi::c_int as fe_limb_t,
                            22736595 as core::ffi::c_int as fe_limb_t,
                            41689999 as core::ffi::c_int as fe_limb_t,
                            10783768 as core::ffi::c_int as fe_limb_t,
                            36493307 as core::ffi::c_int as fe_limb_t,
                            23807620 as core::ffi::c_int as fe_limb_t,
                            38855524 as core::ffi::c_int as fe_limb_t,
                            3647835 as core::ffi::c_int as fe_limb_t,
                            3222231 as core::ffi::c_int as fe_limb_t,
                            22393970 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            18606113 as core::ffi::c_int as fe_limb_t,
                            1693100 as core::ffi::c_int as fe_limb_t,
                            41660478 as core::ffi::c_int as fe_limb_t,
                            18384159 as core::ffi::c_int as fe_limb_t,
                            4112352 as core::ffi::c_int as fe_limb_t,
                            10045021 as core::ffi::c_int as fe_limb_t,
                            23603893 as core::ffi::c_int as fe_limb_t,
                            31506198 as core::ffi::c_int as fe_limb_t,
                            59558087 as core::ffi::c_int as fe_limb_t,
                            2484984 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            9255298 as core::ffi::c_int as fe_limb_t,
                            30423235 as core::ffi::c_int as fe_limb_t,
                            54952701 as core::ffi::c_int as fe_limb_t,
                            32550175 as core::ffi::c_int as fe_limb_t,
                            13098012 as core::ffi::c_int as fe_limb_t,
                            24339566 as core::ffi::c_int as fe_limb_t,
                            16377219 as core::ffi::c_int as fe_limb_t,
                            31451620 as core::ffi::c_int as fe_limb_t,
                            47306788 as core::ffi::c_int as fe_limb_t,
                            30519729 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            44379556 as core::ffi::c_int as fe_limb_t,
                            7496159 as core::ffi::c_int as fe_limb_t,
                            61366665 as core::ffi::c_int as fe_limb_t,
                            11329248 as core::ffi::c_int as fe_limb_t,
                            19991973 as core::ffi::c_int as fe_limb_t,
                            30206930 as core::ffi::c_int as fe_limb_t,
                            35390715 as core::ffi::c_int as fe_limb_t,
                            9936965 as core::ffi::c_int as fe_limb_t,
                            37011176 as core::ffi::c_int as fe_limb_t,
                            22935634 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            21878571 as core::ffi::c_int as fe_limb_t,
                            28553135 as core::ffi::c_int as fe_limb_t,
                            4338335 as core::ffi::c_int as fe_limb_t,
                            13643897 as core::ffi::c_int as fe_limb_t,
                            64071999 as core::ffi::c_int as fe_limb_t,
                            13160959 as core::ffi::c_int as fe_limb_t,
                            19708896 as core::ffi::c_int as fe_limb_t,
                            5415497 as core::ffi::c_int as fe_limb_t,
                            59748361 as core::ffi::c_int as fe_limb_t,
                            29445138 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            27736842 as core::ffi::c_int as fe_limb_t,
                            10103576 as core::ffi::c_int as fe_limb_t,
                            12500508 as core::ffi::c_int as fe_limb_t,
                            8502413 as core::ffi::c_int as fe_limb_t,
                            63695848 as core::ffi::c_int as fe_limb_t,
                            23920873 as core::ffi::c_int as fe_limb_t,
                            10436917 as core::ffi::c_int as fe_limb_t,
                            32004156 as core::ffi::c_int as fe_limb_t,
                            43449720 as core::ffi::c_int as fe_limb_t,
                            25422331 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19492550 as core::ffi::c_int as fe_limb_t,
                            21450067 as core::ffi::c_int as fe_limb_t,
                            37426887 as core::ffi::c_int as fe_limb_t,
                            32701801 as core::ffi::c_int as fe_limb_t,
                            63900692 as core::ffi::c_int as fe_limb_t,
                            12403436 as core::ffi::c_int as fe_limb_t,
                            30066266 as core::ffi::c_int as fe_limb_t,
                            8367329 as core::ffi::c_int as fe_limb_t,
                            13243957 as core::ffi::c_int as fe_limb_t,
                            8709688 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12015105 as core::ffi::c_int as fe_limb_t,
                            2801261 as core::ffi::c_int as fe_limb_t,
                            28198131 as core::ffi::c_int as fe_limb_t,
                            10151021 as core::ffi::c_int as fe_limb_t,
                            24818120 as core::ffi::c_int as fe_limb_t,
                            28811299 as core::ffi::c_int as fe_limb_t,
                            55914672 as core::ffi::c_int as fe_limb_t,
                            27908697 as core::ffi::c_int as fe_limb_t,
                            5150967 as core::ffi::c_int as fe_limb_t,
                            7274186 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            2831347 as core::ffi::c_int as fe_limb_t,
                            21062286 as core::ffi::c_int as fe_limb_t,
                            1478974 as core::ffi::c_int as fe_limb_t,
                            6122054 as core::ffi::c_int as fe_limb_t,
                            23825128 as core::ffi::c_int as fe_limb_t,
                            20820846 as core::ffi::c_int as fe_limb_t,
                            31097298 as core::ffi::c_int as fe_limb_t,
                            6083058 as core::ffi::c_int as fe_limb_t,
                            31021603 as core::ffi::c_int as fe_limb_t,
                            23760822 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64578913 as core::ffi::c_int as fe_limb_t,
                            31324785 as core::ffi::c_int as fe_limb_t,
                            445612 as core::ffi::c_int as fe_limb_t,
                            10720828 as core::ffi::c_int as fe_limb_t,
                            53259337 as core::ffi::c_int as fe_limb_t,
                            22048494 as core::ffi::c_int as fe_limb_t,
                            43601132 as core::ffi::c_int as fe_limb_t,
                            16354464 as core::ffi::c_int as fe_limb_t,
                            15067285 as core::ffi::c_int as fe_limb_t,
                            19406725 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            7840923 as core::ffi::c_int as fe_limb_t,
                            14037873 as core::ffi::c_int as fe_limb_t,
                            33744001 as core::ffi::c_int as fe_limb_t,
                            15934015 as core::ffi::c_int as fe_limb_t,
                            66380651 as core::ffi::c_int as fe_limb_t,
                            29911725 as core::ffi::c_int as fe_limb_t,
                            21403987 as core::ffi::c_int as fe_limb_t,
                            1057586 as core::ffi::c_int as fe_limb_t,
                            47729402 as core::ffi::c_int as fe_limb_t,
                            21151211 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            915865 as core::ffi::c_int as fe_limb_t,
                            17085158 as core::ffi::c_int as fe_limb_t,
                            15608284 as core::ffi::c_int as fe_limb_t,
                            24765302 as core::ffi::c_int as fe_limb_t,
                            42751837 as core::ffi::c_int as fe_limb_t,
                            6060029 as core::ffi::c_int as fe_limb_t,
                            49737545 as core::ffi::c_int as fe_limb_t,
                            8410996 as core::ffi::c_int as fe_limb_t,
                            59888403 as core::ffi::c_int as fe_limb_t,
                            16527024 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            32922597 as core::ffi::c_int as fe_limb_t,
                            32997445 as core::ffi::c_int as fe_limb_t,
                            20336073 as core::ffi::c_int as fe_limb_t,
                            17369864 as core::ffi::c_int as fe_limb_t,
                            10903704 as core::ffi::c_int as fe_limb_t,
                            28169945 as core::ffi::c_int as fe_limb_t,
                            16957573 as core::ffi::c_int as fe_limb_t,
                            52992 as core::ffi::c_int as fe_limb_t,
                            23834301 as core::ffi::c_int as fe_limb_t,
                            6588044 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            32752011 as core::ffi::c_int as fe_limb_t,
                            11232950 as core::ffi::c_int as fe_limb_t,
                            3381995 as core::ffi::c_int as fe_limb_t,
                            24839566 as core::ffi::c_int as fe_limb_t,
                            22652987 as core::ffi::c_int as fe_limb_t,
                            22810329 as core::ffi::c_int as fe_limb_t,
                            17159698 as core::ffi::c_int as fe_limb_t,
                            16689107 as core::ffi::c_int as fe_limb_t,
                            46794284 as core::ffi::c_int as fe_limb_t,
                            32248439 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            62419196 as core::ffi::c_int as fe_limb_t,
                            9166775 as core::ffi::c_int as fe_limb_t,
                            41398568 as core::ffi::c_int as fe_limb_t,
                            22707125 as core::ffi::c_int as fe_limb_t,
                            11576751 as core::ffi::c_int as fe_limb_t,
                            12733943 as core::ffi::c_int as fe_limb_t,
                            7924251 as core::ffi::c_int as fe_limb_t,
                            30802151 as core::ffi::c_int as fe_limb_t,
                            1976122 as core::ffi::c_int as fe_limb_t,
                            26305405 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21251203 as core::ffi::c_int as fe_limb_t,
                            16309901 as core::ffi::c_int as fe_limb_t,
                            64125849 as core::ffi::c_int as fe_limb_t,
                            26771309 as core::ffi::c_int as fe_limb_t,
                            30810596 as core::ffi::c_int as fe_limb_t,
                            12967303 as core::ffi::c_int as fe_limb_t,
                            156041 as core::ffi::c_int as fe_limb_t,
                            30183180 as core::ffi::c_int as fe_limb_t,
                            12331344 as core::ffi::c_int as fe_limb_t,
                            25317235 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            8651595 as core::ffi::c_int as fe_limb_t,
                            29077400 as core::ffi::c_int as fe_limb_t,
                            51023227 as core::ffi::c_int as fe_limb_t,
                            28557437 as core::ffi::c_int as fe_limb_t,
                            13002506 as core::ffi::c_int as fe_limb_t,
                            2950805 as core::ffi::c_int as fe_limb_t,
                            29054427 as core::ffi::c_int as fe_limb_t,
                            28447462 as core::ffi::c_int as fe_limb_t,
                            10008135 as core::ffi::c_int as fe_limb_t,
                            28886531 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            31486061 as core::ffi::c_int as fe_limb_t,
                            15114593 as core::ffi::c_int as fe_limb_t,
                            52847614 as core::ffi::c_int as fe_limb_t,
                            12951353 as core::ffi::c_int as fe_limb_t,
                            14369431 as core::ffi::c_int as fe_limb_t,
                            26166587 as core::ffi::c_int as fe_limb_t,
                            16347320 as core::ffi::c_int as fe_limb_t,
                            19892343 as core::ffi::c_int as fe_limb_t,
                            8684154 as core::ffi::c_int as fe_limb_t,
                            23021480 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19443825 as core::ffi::c_int as fe_limb_t,
                            11385320 as core::ffi::c_int as fe_limb_t,
                            24468943 as core::ffi::c_int as fe_limb_t,
                            23895364 as core::ffi::c_int as fe_limb_t,
                            43189605 as core::ffi::c_int as fe_limb_t,
                            2187568 as core::ffi::c_int as fe_limb_t,
                            40845657 as core::ffi::c_int as fe_limb_t,
                            27467510 as core::ffi::c_int as fe_limb_t,
                            31316347 as core::ffi::c_int as fe_limb_t,
                            14219878 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            38514374 as core::ffi::c_int as fe_limb_t,
                            1193784 as core::ffi::c_int as fe_limb_t,
                            32245219 as core::ffi::c_int as fe_limb_t,
                            11392485 as core::ffi::c_int as fe_limb_t,
                            31092169 as core::ffi::c_int as fe_limb_t,
                            15722801 as core::ffi::c_int as fe_limb_t,
                            27146014 as core::ffi::c_int as fe_limb_t,
                            6992409 as core::ffi::c_int as fe_limb_t,
                            29126555 as core::ffi::c_int as fe_limb_t,
                            9207390 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            32382916 as core::ffi::c_int as fe_limb_t,
                            1110093 as core::ffi::c_int as fe_limb_t,
                            18477781 as core::ffi::c_int as fe_limb_t,
                            11028262 as core::ffi::c_int as fe_limb_t,
                            39697101 as core::ffi::c_int as fe_limb_t,
                            26006320 as core::ffi::c_int as fe_limb_t,
                            62128346 as core::ffi::c_int as fe_limb_t,
                            10843781 as core::ffi::c_int as fe_limb_t,
                            59151264 as core::ffi::c_int as fe_limb_t,
                            19118701 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            2814918 as core::ffi::c_int as fe_limb_t,
                            7836403 as core::ffi::c_int as fe_limb_t,
                            27519878 as core::ffi::c_int as fe_limb_t,
                            25686276 as core::ffi::c_int as fe_limb_t,
                            46214848 as core::ffi::c_int as fe_limb_t,
                            22000742 as core::ffi::c_int as fe_limb_t,
                            45614304 as core::ffi::c_int as fe_limb_t,
                            8550129 as core::ffi::c_int as fe_limb_t,
                            28346258 as core::ffi::c_int as fe_limb_t,
                            1994730 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            47530565 as core::ffi::c_int as fe_limb_t,
                            8085544 as core::ffi::c_int as fe_limb_t,
                            53108345 as core::ffi::c_int as fe_limb_t,
                            29605809 as core::ffi::c_int as fe_limb_t,
                            2785837 as core::ffi::c_int as fe_limb_t,
                            17323125 as core::ffi::c_int as fe_limb_t,
                            47591912 as core::ffi::c_int as fe_limb_t,
                            7174893 as core::ffi::c_int as fe_limb_t,
                            22628102 as core::ffi::c_int as fe_limb_t,
                            8115180 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            36703732 as core::ffi::c_int as fe_limb_t,
                            955510 as core::ffi::c_int as fe_limb_t,
                            55975026 as core::ffi::c_int as fe_limb_t,
                            18476362 as core::ffi::c_int as fe_limb_t,
                            34661776 as core::ffi::c_int as fe_limb_t,
                            20276352 as core::ffi::c_int as fe_limb_t,
                            41457285 as core::ffi::c_int as fe_limb_t,
                            3317159 as core::ffi::c_int as fe_limb_t,
                            57165847 as core::ffi::c_int as fe_limb_t,
                            930271 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            51805164 as core::ffi::c_int as fe_limb_t,
                            26720662 as core::ffi::c_int as fe_limb_t,
                            28856489 as core::ffi::c_int as fe_limb_t,
                            1357446 as core::ffi::c_int as fe_limb_t,
                            23421993 as core::ffi::c_int as fe_limb_t,
                            1057177 as core::ffi::c_int as fe_limb_t,
                            24091212 as core::ffi::c_int as fe_limb_t,
                            32165462 as core::ffi::c_int as fe_limb_t,
                            44343487 as core::ffi::c_int as fe_limb_t,
                            22903716 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            44357633 as core::ffi::c_int as fe_limb_t,
                            28250434 as core::ffi::c_int as fe_limb_t,
                            54201256 as core::ffi::c_int as fe_limb_t,
                            20785565 as core::ffi::c_int as fe_limb_t,
                            51297352 as core::ffi::c_int as fe_limb_t,
                            25757378 as core::ffi::c_int as fe_limb_t,
                            52269845 as core::ffi::c_int as fe_limb_t,
                            17000211 as core::ffi::c_int as fe_limb_t,
                            65241845 as core::ffi::c_int as fe_limb_t,
                            8398969 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            35139535 as core::ffi::c_int as fe_limb_t,
                            2106402 as core::ffi::c_int as fe_limb_t,
                            62372504 as core::ffi::c_int as fe_limb_t,
                            1362500 as core::ffi::c_int as fe_limb_t,
                            12813763 as core::ffi::c_int as fe_limb_t,
                            16200670 as core::ffi::c_int as fe_limb_t,
                            22981545 as core::ffi::c_int as fe_limb_t,
                            27263159 as core::ffi::c_int as fe_limb_t,
                            18009407 as core::ffi::c_int as fe_limb_t,
                            17781660 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49887941 as core::ffi::c_int as fe_limb_t,
                            24009210 as core::ffi::c_int as fe_limb_t,
                            39324209 as core::ffi::c_int as fe_limb_t,
                            14166834 as core::ffi::c_int as fe_limb_t,
                            29815394 as core::ffi::c_int as fe_limb_t,
                            7444469 as core::ffi::c_int as fe_limb_t,
                            29551787 as core::ffi::c_int as fe_limb_t,
                            29827013 as core::ffi::c_int as fe_limb_t,
                            19288548 as core::ffi::c_int as fe_limb_t,
                            1325865 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15100138 as core::ffi::c_int as fe_limb_t,
                            17718680 as core::ffi::c_int as fe_limb_t,
                            43184885 as core::ffi::c_int as fe_limb_t,
                            32549333 as core::ffi::c_int as fe_limb_t,
                            40658671 as core::ffi::c_int as fe_limb_t,
                            15509407 as core::ffi::c_int as fe_limb_t,
                            12376730 as core::ffi::c_int as fe_limb_t,
                            30075286 as core::ffi::c_int as fe_limb_t,
                            33166106 as core::ffi::c_int as fe_limb_t,
                            25511682 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            20909212 as core::ffi::c_int as fe_limb_t,
                            13023121 as core::ffi::c_int as fe_limb_t,
                            57899112 as core::ffi::c_int as fe_limb_t,
                            16251777 as core::ffi::c_int as fe_limb_t,
                            61330449 as core::ffi::c_int as fe_limb_t,
                            25459517 as core::ffi::c_int as fe_limb_t,
                            12412150 as core::ffi::c_int as fe_limb_t,
                            10018715 as core::ffi::c_int as fe_limb_t,
                            2213263 as core::ffi::c_int as fe_limb_t,
                            19676059 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            32529814 as core::ffi::c_int as fe_limb_t,
                            22479743 as core::ffi::c_int as fe_limb_t,
                            30361438 as core::ffi::c_int as fe_limb_t,
                            16864679 as core::ffi::c_int as fe_limb_t,
                            57972923 as core::ffi::c_int as fe_limb_t,
                            1513225 as core::ffi::c_int as fe_limb_t,
                            22922121 as core::ffi::c_int as fe_limb_t,
                            6382134 as core::ffi::c_int as fe_limb_t,
                            61341936 as core::ffi::c_int as fe_limb_t,
                            8371347 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9923462 as core::ffi::c_int as fe_limb_t,
                            11271500 as core::ffi::c_int as fe_limb_t,
                            12616794 as core::ffi::c_int as fe_limb_t,
                            3544722 as core::ffi::c_int as fe_limb_t,
                            37110496 as core::ffi::c_int as fe_limb_t,
                            31832805 as core::ffi::c_int as fe_limb_t,
                            12891686 as core::ffi::c_int as fe_limb_t,
                            25361300 as core::ffi::c_int as fe_limb_t,
                            40665920 as core::ffi::c_int as fe_limb_t,
                            10486143 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            44511638 as core::ffi::c_int as fe_limb_t,
                            26541766 as core::ffi::c_int as fe_limb_t,
                            8587002 as core::ffi::c_int as fe_limb_t,
                            25296571 as core::ffi::c_int as fe_limb_t,
                            4084308 as core::ffi::c_int as fe_limb_t,
                            20584370 as core::ffi::c_int as fe_limb_t,
                            361725 as core::ffi::c_int as fe_limb_t,
                            2610596 as core::ffi::c_int as fe_limb_t,
                            43187334 as core::ffi::c_int as fe_limb_t,
                            22099236 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            5408392 as core::ffi::c_int as fe_limb_t,
                            32417741 as core::ffi::c_int as fe_limb_t,
                            62139741 as core::ffi::c_int as fe_limb_t,
                            10561667 as core::ffi::c_int as fe_limb_t,
                            24145918 as core::ffi::c_int as fe_limb_t,
                            14240566 as core::ffi::c_int as fe_limb_t,
                            31319731 as core::ffi::c_int as fe_limb_t,
                            29318891 as core::ffi::c_int as fe_limb_t,
                            19985174 as core::ffi::c_int as fe_limb_t,
                            30118346 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            53114407 as core::ffi::c_int as fe_limb_t,
                            16616820 as core::ffi::c_int as fe_limb_t,
                            14549246 as core::ffi::c_int as fe_limb_t,
                            3341099 as core::ffi::c_int as fe_limb_t,
                            32155958 as core::ffi::c_int as fe_limb_t,
                            13648976 as core::ffi::c_int as fe_limb_t,
                            49531796 as core::ffi::c_int as fe_limb_t,
                            8849296 as core::ffi::c_int as fe_limb_t,
                            65030 as core::ffi::c_int as fe_limb_t,
                            8370684 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            58787919 as core::ffi::c_int as fe_limb_t,
                            21504805 as core::ffi::c_int as fe_limb_t,
                            31204562 as core::ffi::c_int as fe_limb_t,
                            5839400 as core::ffi::c_int as fe_limb_t,
                            46481576 as core::ffi::c_int as fe_limb_t,
                            32497154 as core::ffi::c_int as fe_limb_t,
                            47665921 as core::ffi::c_int as fe_limb_t,
                            6922163 as core::ffi::c_int as fe_limb_t,
                            12743482 as core::ffi::c_int as fe_limb_t,
                            23753914 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64747493 as core::ffi::c_int as fe_limb_t,
                            12678784 as core::ffi::c_int as fe_limb_t,
                            28815050 as core::ffi::c_int as fe_limb_t,
                            4759974 as core::ffi::c_int as fe_limb_t,
                            43215817 as core::ffi::c_int as fe_limb_t,
                            4884716 as core::ffi::c_int as fe_limb_t,
                            23783145 as core::ffi::c_int as fe_limb_t,
                            11038569 as core::ffi::c_int as fe_limb_t,
                            18800704 as core::ffi::c_int as fe_limb_t,
                            255233 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            61839187 as core::ffi::c_int as fe_limb_t,
                            31780545 as core::ffi::c_int as fe_limb_t,
                            13957885 as core::ffi::c_int as fe_limb_t,
                            7990715 as core::ffi::c_int as fe_limb_t,
                            23132995 as core::ffi::c_int as fe_limb_t,
                            728773 as core::ffi::c_int as fe_limb_t,
                            13393847 as core::ffi::c_int as fe_limb_t,
                            9066957 as core::ffi::c_int as fe_limb_t,
                            19258688 as core::ffi::c_int as fe_limb_t,
                            18800639 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            64172210 as core::ffi::c_int as fe_limb_t,
                            22726896 as core::ffi::c_int as fe_limb_t,
                            56676774 as core::ffi::c_int as fe_limb_t,
                            14516792 as core::ffi::c_int as fe_limb_t,
                            63468078 as core::ffi::c_int as fe_limb_t,
                            4372540 as core::ffi::c_int as fe_limb_t,
                            35173943 as core::ffi::c_int as fe_limb_t,
                            2209389 as core::ffi::c_int as fe_limb_t,
                            65584811 as core::ffi::c_int as fe_limb_t,
                            2055793 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            580882 as core::ffi::c_int as fe_limb_t,
                            16705327 as core::ffi::c_int as fe_limb_t,
                            5468415 as core::ffi::c_int as fe_limb_t,
                            30871414 as core::ffi::c_int as fe_limb_t,
                            36182444 as core::ffi::c_int as fe_limb_t,
                            18858431 as core::ffi::c_int as fe_limb_t,
                            59905517 as core::ffi::c_int as fe_limb_t,
                            24560042 as core::ffi::c_int as fe_limb_t,
                            37087844 as core::ffi::c_int as fe_limb_t,
                            7394434 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            23838809 as core::ffi::c_int as fe_limb_t,
                            1822728 as core::ffi::c_int as fe_limb_t,
                            51370421 as core::ffi::c_int as fe_limb_t,
                            15242726 as core::ffi::c_int as fe_limb_t,
                            8318092 as core::ffi::c_int as fe_limb_t,
                            29821328 as core::ffi::c_int as fe_limb_t,
                            45436683 as core::ffi::c_int as fe_limb_t,
                            30062226 as core::ffi::c_int as fe_limb_t,
                            62287122 as core::ffi::c_int as fe_limb_t,
                            14799920 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            13345610 as core::ffi::c_int as fe_limb_t,
                            9759151 as core::ffi::c_int as fe_limb_t,
                            3371034 as core::ffi::c_int as fe_limb_t,
                            17416641 as core::ffi::c_int as fe_limb_t,
                            16353038 as core::ffi::c_int as fe_limb_t,
                            8577942 as core::ffi::c_int as fe_limb_t,
                            31129804 as core::ffi::c_int as fe_limb_t,
                            13496856 as core::ffi::c_int as fe_limb_t,
                            58052846 as core::ffi::c_int as fe_limb_t,
                            7402517 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            2286874 as core::ffi::c_int as fe_limb_t,
                            29118501 as core::ffi::c_int as fe_limb_t,
                            47066405 as core::ffi::c_int as fe_limb_t,
                            31546095 as core::ffi::c_int as fe_limb_t,
                            53412636 as core::ffi::c_int as fe_limb_t,
                            5038121 as core::ffi::c_int as fe_limb_t,
                            11006906 as core::ffi::c_int as fe_limb_t,
                            17794080 as core::ffi::c_int as fe_limb_t,
                            8205060 as core::ffi::c_int as fe_limb_t,
                            1607563 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14414067 as core::ffi::c_int as fe_limb_t,
                            25552300 as core::ffi::c_int as fe_limb_t,
                            3331829 as core::ffi::c_int as fe_limb_t,
                            30346215 as core::ffi::c_int as fe_limb_t,
                            22249150 as core::ffi::c_int as fe_limb_t,
                            27960244 as core::ffi::c_int as fe_limb_t,
                            18364660 as core::ffi::c_int as fe_limb_t,
                            30647474 as core::ffi::c_int as fe_limb_t,
                            30019586 as core::ffi::c_int as fe_limb_t,
                            24525154 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            39420813 as core::ffi::c_int as fe_limb_t,
                            1585952 as core::ffi::c_int as fe_limb_t,
                            56333811 as core::ffi::c_int as fe_limb_t,
                            931068 as core::ffi::c_int as fe_limb_t,
                            37988643 as core::ffi::c_int as fe_limb_t,
                            22552112 as core::ffi::c_int as fe_limb_t,
                            52698034 as core::ffi::c_int as fe_limb_t,
                            12029092 as core::ffi::c_int as fe_limb_t,
                            9944378 as core::ffi::c_int as fe_limb_t,
                            8024 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            4368715 as core::ffi::c_int as fe_limb_t,
                            29844802 as core::ffi::c_int as fe_limb_t,
                            29874199 as core::ffi::c_int as fe_limb_t,
                            18531449 as core::ffi::c_int as fe_limb_t,
                            46878477 as core::ffi::c_int as fe_limb_t,
                            22143727 as core::ffi::c_int as fe_limb_t,
                            50994269 as core::ffi::c_int as fe_limb_t,
                            32555346 as core::ffi::c_int as fe_limb_t,
                            58966475 as core::ffi::c_int as fe_limb_t,
                            5640029 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            10299591 as core::ffi::c_int as fe_limb_t,
                            13746483 as core::ffi::c_int as fe_limb_t,
                            11661824 as core::ffi::c_int as fe_limb_t,
                            16234854 as core::ffi::c_int as fe_limb_t,
                            7630238 as core::ffi::c_int as fe_limb_t,
                            5998374 as core::ffi::c_int as fe_limb_t,
                            9809887 as core::ffi::c_int as fe_limb_t,
                            16859868 as core::ffi::c_int as fe_limb_t,
                            15219797 as core::ffi::c_int as fe_limb_t,
                            19226649 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            27425505 as core::ffi::c_int as fe_limb_t,
                            27835351 as core::ffi::c_int as fe_limb_t,
                            3055005 as core::ffi::c_int as fe_limb_t,
                            10660664 as core::ffi::c_int as fe_limb_t,
                            23458024 as core::ffi::c_int as fe_limb_t,
                            595578 as core::ffi::c_int as fe_limb_t,
                            51710259 as core::ffi::c_int as fe_limb_t,
                            32381236 as core::ffi::c_int as fe_limb_t,
                            48766680 as core::ffi::c_int as fe_limb_t,
                            9742716 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            6744077 as core::ffi::c_int as fe_limb_t,
                            2427284 as core::ffi::c_int as fe_limb_t,
                            26042789 as core::ffi::c_int as fe_limb_t,
                            2720740 as core::ffi::c_int as fe_limb_t,
                            66260958 as core::ffi::c_int as fe_limb_t,
                            1118973 as core::ffi::c_int as fe_limb_t,
                            32324614 as core::ffi::c_int as fe_limb_t,
                            7406442 as core::ffi::c_int as fe_limb_t,
                            12420155 as core::ffi::c_int as fe_limb_t,
                            1994844 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14012502 as core::ffi::c_int as fe_limb_t,
                            28529712 as core::ffi::c_int as fe_limb_t,
                            48724410 as core::ffi::c_int as fe_limb_t,
                            23975962 as core::ffi::c_int as fe_limb_t,
                            40623521 as core::ffi::c_int as fe_limb_t,
                            29617992 as core::ffi::c_int as fe_limb_t,
                            54075385 as core::ffi::c_int as fe_limb_t,
                            22644628 as core::ffi::c_int as fe_limb_t,
                            24319928 as core::ffi::c_int as fe_limb_t,
                            27108099 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            16412671 as core::ffi::c_int as fe_limb_t,
                            29047065 as core::ffi::c_int as fe_limb_t,
                            10772640 as core::ffi::c_int as fe_limb_t,
                            15929391 as core::ffi::c_int as fe_limb_t,
                            50040076 as core::ffi::c_int as fe_limb_t,
                            28895810 as core::ffi::c_int as fe_limb_t,
                            10555944 as core::ffi::c_int as fe_limb_t,
                            23070383 as core::ffi::c_int as fe_limb_t,
                            37006495 as core::ffi::c_int as fe_limb_t,
                            28815383 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            22397363 as core::ffi::c_int as fe_limb_t,
                            25786748 as core::ffi::c_int as fe_limb_t,
                            57815702 as core::ffi::c_int as fe_limb_t,
                            20761563 as core::ffi::c_int as fe_limb_t,
                            17166286 as core::ffi::c_int as fe_limb_t,
                            23799296 as core::ffi::c_int as fe_limb_t,
                            39775798 as core::ffi::c_int as fe_limb_t,
                            6199365 as core::ffi::c_int as fe_limb_t,
                            21880021 as core::ffi::c_int as fe_limb_t,
                            21303672 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62825557 as core::ffi::c_int as fe_limb_t,
                            5368522 as core::ffi::c_int as fe_limb_t,
                            35991846 as core::ffi::c_int as fe_limb_t,
                            8163388 as core::ffi::c_int as fe_limb_t,
                            36785801 as core::ffi::c_int as fe_limb_t,
                            3209127 as core::ffi::c_int as fe_limb_t,
                            16557151 as core::ffi::c_int as fe_limb_t,
                            8890729 as core::ffi::c_int as fe_limb_t,
                            8840445 as core::ffi::c_int as fe_limb_t,
                            4957760 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51661137 as core::ffi::c_int as fe_limb_t,
                            709326 as core::ffi::c_int as fe_limb_t,
                            60189418 as core::ffi::c_int as fe_limb_t,
                            22684253 as core::ffi::c_int as fe_limb_t,
                            37330941 as core::ffi::c_int as fe_limb_t,
                            6522331 as core::ffi::c_int as fe_limb_t,
                            45388683 as core::ffi::c_int as fe_limb_t,
                            12130071 as core::ffi::c_int as fe_limb_t,
                            52312361 as core::ffi::c_int as fe_limb_t,
                            5005756 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64994094 as core::ffi::c_int as fe_limb_t,
                            19246303 as core::ffi::c_int as fe_limb_t,
                            23019041 as core::ffi::c_int as fe_limb_t,
                            15765735 as core::ffi::c_int as fe_limb_t,
                            41839181 as core::ffi::c_int as fe_limb_t,
                            6002751 as core::ffi::c_int as fe_limb_t,
                            10183197 as core::ffi::c_int as fe_limb_t,
                            20315106 as core::ffi::c_int as fe_limb_t,
                            50713577 as core::ffi::c_int as fe_limb_t,
                            31378319 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            48083108 as core::ffi::c_int as fe_limb_t,
                            1632004 as core::ffi::c_int as fe_limb_t,
                            13466291 as core::ffi::c_int as fe_limb_t,
                            25559332 as core::ffi::c_int as fe_limb_t,
                            43468412 as core::ffi::c_int as fe_limb_t,
                            16573536 as core::ffi::c_int as fe_limb_t,
                            35094956 as core::ffi::c_int as fe_limb_t,
                            30497327 as core::ffi::c_int as fe_limb_t,
                            22208661 as core::ffi::c_int as fe_limb_t,
                            2000468 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            3065054 as core::ffi::c_int as fe_limb_t,
                            32141671 as core::ffi::c_int as fe_limb_t,
                            41510189 as core::ffi::c_int as fe_limb_t,
                            33192999 as core::ffi::c_int as fe_limb_t,
                            49425798 as core::ffi::c_int as fe_limb_t,
                            27851016 as core::ffi::c_int as fe_limb_t,
                            58944651 as core::ffi::c_int as fe_limb_t,
                            11248526 as core::ffi::c_int as fe_limb_t,
                            63417650 as core::ffi::c_int as fe_limb_t,
                            26140247 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10379208 as core::ffi::c_int as fe_limb_t,
                            27508878 as core::ffi::c_int as fe_limb_t,
                            8877318 as core::ffi::c_int as fe_limb_t,
                            1473647 as core::ffi::c_int as fe_limb_t,
                            37817580 as core::ffi::c_int as fe_limb_t,
                            21046851 as core::ffi::c_int as fe_limb_t,
                            16690914 as core::ffi::c_int as fe_limb_t,
                            2553332 as core::ffi::c_int as fe_limb_t,
                            63976176 as core::ffi::c_int as fe_limb_t,
                            16400288 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15716668 as core::ffi::c_int as fe_limb_t,
                            1254266 as core::ffi::c_int as fe_limb_t,
                            48636174 as core::ffi::c_int as fe_limb_t,
                            7446273 as core::ffi::c_int as fe_limb_t,
                            58659946 as core::ffi::c_int as fe_limb_t,
                            6344163 as core::ffi::c_int as fe_limb_t,
                            45011593 as core::ffi::c_int as fe_limb_t,
                            26268851 as core::ffi::c_int as fe_limb_t,
                            26894936 as core::ffi::c_int as fe_limb_t,
                            9132066 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            24158868 as core::ffi::c_int as fe_limb_t,
                            12938817 as core::ffi::c_int as fe_limb_t,
                            11085297 as core::ffi::c_int as fe_limb_t,
                            25376834 as core::ffi::c_int as fe_limb_t,
                            39045385 as core::ffi::c_int as fe_limb_t,
                            29097348 as core::ffi::c_int as fe_limb_t,
                            36532400 as core::ffi::c_int as fe_limb_t,
                            64451 as core::ffi::c_int as fe_limb_t,
                            60291780 as core::ffi::c_int as fe_limb_t,
                            30861549 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            13488534 as core::ffi::c_int as fe_limb_t,
                            7794716 as core::ffi::c_int as fe_limb_t,
                            22236231 as core::ffi::c_int as fe_limb_t,
                            5989356 as core::ffi::c_int as fe_limb_t,
                            25426474 as core::ffi::c_int as fe_limb_t,
                            20976224 as core::ffi::c_int as fe_limb_t,
                            2350709 as core::ffi::c_int as fe_limb_t,
                            30135921 as core::ffi::c_int as fe_limb_t,
                            62420857 as core::ffi::c_int as fe_limb_t,
                            2364225 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16335033 as core::ffi::c_int as fe_limb_t,
                            9132434 as core::ffi::c_int as fe_limb_t,
                            25640582 as core::ffi::c_int as fe_limb_t,
                            6678888 as core::ffi::c_int as fe_limb_t,
                            1725628 as core::ffi::c_int as fe_limb_t,
                            8517937 as core::ffi::c_int as fe_limb_t,
                            55301840 as core::ffi::c_int as fe_limb_t,
                            21856974 as core::ffi::c_int as fe_limb_t,
                            15445874 as core::ffi::c_int as fe_limb_t,
                            25756331 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29004188 as core::ffi::c_int as fe_limb_t,
                            25687351 as core::ffi::c_int as fe_limb_t,
                            28661401 as core::ffi::c_int as fe_limb_t,
                            32914020 as core::ffi::c_int as fe_limb_t,
                            54314860 as core::ffi::c_int as fe_limb_t,
                            25611345 as core::ffi::c_int as fe_limb_t,
                            31863254 as core::ffi::c_int as fe_limb_t,
                            29418892 as core::ffi::c_int as fe_limb_t,
                            66830813 as core::ffi::c_int as fe_limb_t,
                            17795152 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            60986784 as core::ffi::c_int as fe_limb_t,
                            18687766 as core::ffi::c_int as fe_limb_t,
                            38493958 as core::ffi::c_int as fe_limb_t,
                            14569918 as core::ffi::c_int as fe_limb_t,
                            56250865 as core::ffi::c_int as fe_limb_t,
                            29962602 as core::ffi::c_int as fe_limb_t,
                            10343411 as core::ffi::c_int as fe_limb_t,
                            26578142 as core::ffi::c_int as fe_limb_t,
                            37280576 as core::ffi::c_int as fe_limb_t,
                            22738620 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            27081650 as core::ffi::c_int as fe_limb_t,
                            3463984 as core::ffi::c_int as fe_limb_t,
                            14099042 as core::ffi::c_int as fe_limb_t,
                            29036828 as core::ffi::c_int as fe_limb_t,
                            1616302 as core::ffi::c_int as fe_limb_t,
                            27348828 as core::ffi::c_int as fe_limb_t,
                            29542635 as core::ffi::c_int as fe_limb_t,
                            15372179 as core::ffi::c_int as fe_limb_t,
                            17293797 as core::ffi::c_int as fe_limb_t,
                            960709 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            20263915 as core::ffi::c_int as fe_limb_t,
                            11434237 as core::ffi::c_int as fe_limb_t,
                            61343429 as core::ffi::c_int as fe_limb_t,
                            11236809 as core::ffi::c_int as fe_limb_t,
                            13505955 as core::ffi::c_int as fe_limb_t,
                            22697330 as core::ffi::c_int as fe_limb_t,
                            50997518 as core::ffi::c_int as fe_limb_t,
                            6493121 as core::ffi::c_int as fe_limb_t,
                            47724353 as core::ffi::c_int as fe_limb_t,
                            7639713 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64278047 as core::ffi::c_int as fe_limb_t,
                            18715199 as core::ffi::c_int as fe_limb_t,
                            25403037 as core::ffi::c_int as fe_limb_t,
                            25339236 as core::ffi::c_int as fe_limb_t,
                            58791851 as core::ffi::c_int as fe_limb_t,
                            17380732 as core::ffi::c_int as fe_limb_t,
                            18006286 as core::ffi::c_int as fe_limb_t,
                            17510682 as core::ffi::c_int as fe_limb_t,
                            29994676 as core::ffi::c_int as fe_limb_t,
                            17746311 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9769828 as core::ffi::c_int as fe_limb_t,
                            5202651 as core::ffi::c_int as fe_limb_t,
                            42951466 as core::ffi::c_int as fe_limb_t,
                            19923039 as core::ffi::c_int as fe_limb_t,
                            39057860 as core::ffi::c_int as fe_limb_t,
                            21992807 as core::ffi::c_int as fe_limb_t,
                            42495722 as core::ffi::c_int as fe_limb_t,
                            19693649 as core::ffi::c_int as fe_limb_t,
                            35924288 as core::ffi::c_int as fe_limb_t,
                            709463 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            12286395 as core::ffi::c_int as fe_limb_t,
                            13076066 as core::ffi::c_int as fe_limb_t,
                            45333675 as core::ffi::c_int as fe_limb_t,
                            32377809 as core::ffi::c_int as fe_limb_t,
                            42105665 as core::ffi::c_int as fe_limb_t,
                            4057651 as core::ffi::c_int as fe_limb_t,
                            35090736 as core::ffi::c_int as fe_limb_t,
                            24663557 as core::ffi::c_int as fe_limb_t,
                            16102006 as core::ffi::c_int as fe_limb_t,
                            13205847 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            13733362 as core::ffi::c_int as fe_limb_t,
                            5599946 as core::ffi::c_int as fe_limb_t,
                            10557076 as core::ffi::c_int as fe_limb_t,
                            3195751 as core::ffi::c_int as fe_limb_t,
                            61550873 as core::ffi::c_int as fe_limb_t,
                            8536969 as core::ffi::c_int as fe_limb_t,
                            41568694 as core::ffi::c_int as fe_limb_t,
                            8525971 as core::ffi::c_int as fe_limb_t,
                            10151379 as core::ffi::c_int as fe_limb_t,
                            10394400 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            4024660 as core::ffi::c_int as fe_limb_t,
                            17416881 as core::ffi::c_int as fe_limb_t,
                            22436261 as core::ffi::c_int as fe_limb_t,
                            12276534 as core::ffi::c_int as fe_limb_t,
                            58009849 as core::ffi::c_int as fe_limb_t,
                            30868332 as core::ffi::c_int as fe_limb_t,
                            19698228 as core::ffi::c_int as fe_limb_t,
                            11743039 as core::ffi::c_int as fe_limb_t,
                            33806530 as core::ffi::c_int as fe_limb_t,
                            8934413 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51229064 as core::ffi::c_int as fe_limb_t,
                            29029191 as core::ffi::c_int as fe_limb_t,
                            58528116 as core::ffi::c_int as fe_limb_t,
                            30620370 as core::ffi::c_int as fe_limb_t,
                            14634844 as core::ffi::c_int as fe_limb_t,
                            32856154 as core::ffi::c_int as fe_limb_t,
                            57659786 as core::ffi::c_int as fe_limb_t,
                            3137093 as core::ffi::c_int as fe_limb_t,
                            55571978 as core::ffi::c_int as fe_limb_t,
                            11721157 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            17555920 as core::ffi::c_int as fe_limb_t,
                            28540494 as core::ffi::c_int as fe_limb_t,
                            8268605 as core::ffi::c_int as fe_limb_t,
                            2331751 as core::ffi::c_int as fe_limb_t,
                            44370049 as core::ffi::c_int as fe_limb_t,
                            9761012 as core::ffi::c_int as fe_limb_t,
                            9319229 as core::ffi::c_int as fe_limb_t,
                            8835153 as core::ffi::c_int as fe_limb_t,
                            57903375 as core::ffi::c_int as fe_limb_t,
                            32274386 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            66647436 as core::ffi::c_int as fe_limb_t,
                            25724417 as core::ffi::c_int as fe_limb_t,
                            20614117 as core::ffi::c_int as fe_limb_t,
                            16688288 as core::ffi::c_int as fe_limb_t,
                            59594098 as core::ffi::c_int as fe_limb_t,
                            28747312 as core::ffi::c_int as fe_limb_t,
                            22300303 as core::ffi::c_int as fe_limb_t,
                            505429 as core::ffi::c_int as fe_limb_t,
                            6108462 as core::ffi::c_int as fe_limb_t,
                            27371017 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            62038564 as core::ffi::c_int as fe_limb_t,
                            12367916 as core::ffi::c_int as fe_limb_t,
                            36445330 as core::ffi::c_int as fe_limb_t,
                            3234472 as core::ffi::c_int as fe_limb_t,
                            32617080 as core::ffi::c_int as fe_limb_t,
                            25131790 as core::ffi::c_int as fe_limb_t,
                            29880582 as core::ffi::c_int as fe_limb_t,
                            20071101 as core::ffi::c_int as fe_limb_t,
                            40210373 as core::ffi::c_int as fe_limb_t,
                            25686972 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            35133562 as core::ffi::c_int as fe_limb_t,
                            5726538 as core::ffi::c_int as fe_limb_t,
                            26934134 as core::ffi::c_int as fe_limb_t,
                            10237677 as core::ffi::c_int as fe_limb_t,
                            63935147 as core::ffi::c_int as fe_limb_t,
                            32949378 as core::ffi::c_int as fe_limb_t,
                            24199303 as core::ffi::c_int as fe_limb_t,
                            3795095 as core::ffi::c_int as fe_limb_t,
                            7592688 as core::ffi::c_int as fe_limb_t,
                            18562353 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            21594432 as core::ffi::c_int as fe_limb_t,
                            18590204 as core::ffi::c_int as fe_limb_t,
                            17466407 as core::ffi::c_int as fe_limb_t,
                            29477210 as core::ffi::c_int as fe_limb_t,
                            32537083 as core::ffi::c_int as fe_limb_t,
                            2739898 as core::ffi::c_int as fe_limb_t,
                            6407723 as core::ffi::c_int as fe_limb_t,
                            12018833 as core::ffi::c_int as fe_limb_t,
                            38852812 as core::ffi::c_int as fe_limb_t,
                            4298411 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            46458361 as core::ffi::c_int as fe_limb_t,
                            21592935 as core::ffi::c_int as fe_limb_t,
                            39872588 as core::ffi::c_int as fe_limb_t,
                            570497 as core::ffi::c_int as fe_limb_t,
                            3767144 as core::ffi::c_int as fe_limb_t,
                            31836892 as core::ffi::c_int as fe_limb_t,
                            13891941 as core::ffi::c_int as fe_limb_t,
                            31985238 as core::ffi::c_int as fe_limb_t,
                            13717173 as core::ffi::c_int as fe_limb_t,
                            10805743 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            52432215 as core::ffi::c_int as fe_limb_t,
                            17910135 as core::ffi::c_int as fe_limb_t,
                            15287173 as core::ffi::c_int as fe_limb_t,
                            11927123 as core::ffi::c_int as fe_limb_t,
                            24177847 as core::ffi::c_int as fe_limb_t,
                            25378864 as core::ffi::c_int as fe_limb_t,
                            66312432 as core::ffi::c_int as fe_limb_t,
                            14860608 as core::ffi::c_int as fe_limb_t,
                            40169934 as core::ffi::c_int as fe_limb_t,
                            27690595 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12962541 as core::ffi::c_int as fe_limb_t,
                            5311799 as core::ffi::c_int as fe_limb_t,
                            57048096 as core::ffi::c_int as fe_limb_t,
                            11658279 as core::ffi::c_int as fe_limb_t,
                            18855286 as core::ffi::c_int as fe_limb_t,
                            25600231 as core::ffi::c_int as fe_limb_t,
                            13286262 as core::ffi::c_int as fe_limb_t,
                            20745728 as core::ffi::c_int as fe_limb_t,
                            62727807 as core::ffi::c_int as fe_limb_t,
                            9882021 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18512060 as core::ffi::c_int as fe_limb_t,
                            11319350 as core::ffi::c_int as fe_limb_t,
                            46985740 as core::ffi::c_int as fe_limb_t,
                            15090308 as core::ffi::c_int as fe_limb_t,
                            18818594 as core::ffi::c_int as fe_limb_t,
                            5271736 as core::ffi::c_int as fe_limb_t,
                            44380960 as core::ffi::c_int as fe_limb_t,
                            3666878 as core::ffi::c_int as fe_limb_t,
                            43141434 as core::ffi::c_int as fe_limb_t,
                            30255002 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            60319844 as core::ffi::c_int as fe_limb_t,
                            30408388 as core::ffi::c_int as fe_limb_t,
                            16192428 as core::ffi::c_int as fe_limb_t,
                            13241070 as core::ffi::c_int as fe_limb_t,
                            15898607 as core::ffi::c_int as fe_limb_t,
                            19348318 as core::ffi::c_int as fe_limb_t,
                            57023983 as core::ffi::c_int as fe_limb_t,
                            26893321 as core::ffi::c_int as fe_limb_t,
                            64705764 as core::ffi::c_int as fe_limb_t,
                            5276064 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30169808 as core::ffi::c_int as fe_limb_t,
                            28236784 as core::ffi::c_int as fe_limb_t,
                            26306205 as core::ffi::c_int as fe_limb_t,
                            21803573 as core::ffi::c_int as fe_limb_t,
                            27814963 as core::ffi::c_int as fe_limb_t,
                            7069267 as core::ffi::c_int as fe_limb_t,
                            7152851 as core::ffi::c_int as fe_limb_t,
                            3684982 as core::ffi::c_int as fe_limb_t,
                            1449224 as core::ffi::c_int as fe_limb_t,
                            13082861 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10342807 as core::ffi::c_int as fe_limb_t,
                            3098505 as core::ffi::c_int as fe_limb_t,
                            2119311 as core::ffi::c_int as fe_limb_t,
                            193222 as core::ffi::c_int as fe_limb_t,
                            25702612 as core::ffi::c_int as fe_limb_t,
                            12233820 as core::ffi::c_int as fe_limb_t,
                            23697382 as core::ffi::c_int as fe_limb_t,
                            15056736 as core::ffi::c_int as fe_limb_t,
                            46092426 as core::ffi::c_int as fe_limb_t,
                            25352431 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            33958735 as core::ffi::c_int as fe_limb_t,
                            3261607 as core::ffi::c_int as fe_limb_t,
                            22745853 as core::ffi::c_int as fe_limb_t,
                            7948688 as core::ffi::c_int as fe_limb_t,
                            19370557 as core::ffi::c_int as fe_limb_t,
                            18376767 as core::ffi::c_int as fe_limb_t,
                            40936887 as core::ffi::c_int as fe_limb_t,
                            6482813 as core::ffi::c_int as fe_limb_t,
                            56808784 as core::ffi::c_int as fe_limb_t,
                            22494330 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            32869458 as core::ffi::c_int as fe_limb_t,
                            28145887 as core::ffi::c_int as fe_limb_t,
                            25609742 as core::ffi::c_int as fe_limb_t,
                            15678670 as core::ffi::c_int as fe_limb_t,
                            56421095 as core::ffi::c_int as fe_limb_t,
                            18083360 as core::ffi::c_int as fe_limb_t,
                            26112420 as core::ffi::c_int as fe_limb_t,
                            2521008 as core::ffi::c_int as fe_limb_t,
                            44444576 as core::ffi::c_int as fe_limb_t,
                            6904814 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29506904 as core::ffi::c_int as fe_limb_t,
                            4457497 as core::ffi::c_int as fe_limb_t,
                            3377935 as core::ffi::c_int as fe_limb_t,
                            23757988 as core::ffi::c_int as fe_limb_t,
                            36598817 as core::ffi::c_int as fe_limb_t,
                            12935079 as core::ffi::c_int as fe_limb_t,
                            1561737 as core::ffi::c_int as fe_limb_t,
                            3841096 as core::ffi::c_int as fe_limb_t,
                            38105225 as core::ffi::c_int as fe_limb_t,
                            26896789 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10340844 as core::ffi::c_int as fe_limb_t,
                            26924055 as core::ffi::c_int as fe_limb_t,
                            48452231 as core::ffi::c_int as fe_limb_t,
                            31276001 as core::ffi::c_int as fe_limb_t,
                            12621150 as core::ffi::c_int as fe_limb_t,
                            20215377 as core::ffi::c_int as fe_limb_t,
                            30878496 as core::ffi::c_int as fe_limb_t,
                            21730062 as core::ffi::c_int as fe_limb_t,
                            41524312 as core::ffi::c_int as fe_limb_t,
                            5181965 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            25940096 as core::ffi::c_int as fe_limb_t,
                            20896407 as core::ffi::c_int as fe_limb_t,
                            17324187 as core::ffi::c_int as fe_limb_t,
                            23247058 as core::ffi::c_int as fe_limb_t,
                            58437395 as core::ffi::c_int as fe_limb_t,
                            15029093 as core::ffi::c_int as fe_limb_t,
                            24396252 as core::ffi::c_int as fe_limb_t,
                            17103510 as core::ffi::c_int as fe_limb_t,
                            64786011 as core::ffi::c_int as fe_limb_t,
                            21165857 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            45343161 as core::ffi::c_int as fe_limb_t,
                            9916822 as core::ffi::c_int as fe_limb_t,
                            65808455 as core::ffi::c_int as fe_limb_t,
                            4079497 as core::ffi::c_int as fe_limb_t,
                            66080518 as core::ffi::c_int as fe_limb_t,
                            11909558 as core::ffi::c_int as fe_limb_t,
                            1782390 as core::ffi::c_int as fe_limb_t,
                            12641087 as core::ffi::c_int as fe_limb_t,
                            20603771 as core::ffi::c_int as fe_limb_t,
                            26992690 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            48226577 as core::ffi::c_int as fe_limb_t,
                            21881051 as core::ffi::c_int as fe_limb_t,
                            24849421 as core::ffi::c_int as fe_limb_t,
                            11501709 as core::ffi::c_int as fe_limb_t,
                            13161720 as core::ffi::c_int as fe_limb_t,
                            28785558 as core::ffi::c_int as fe_limb_t,
                            1925522 as core::ffi::c_int as fe_limb_t,
                            11914390 as core::ffi::c_int as fe_limb_t,
                            4662781 as core::ffi::c_int as fe_limb_t,
                            7820689 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12241050 as core::ffi::c_int as fe_limb_t,
                            33128450 as core::ffi::c_int as fe_limb_t,
                            8132690 as core::ffi::c_int as fe_limb_t,
                            9393934 as core::ffi::c_int as fe_limb_t,
                            32846760 as core::ffi::c_int as fe_limb_t,
                            31954812 as core::ffi::c_int as fe_limb_t,
                            29749455 as core::ffi::c_int as fe_limb_t,
                            12172924 as core::ffi::c_int as fe_limb_t,
                            16136752 as core::ffi::c_int as fe_limb_t,
                            15264020 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56758909 as core::ffi::c_int as fe_limb_t,
                            18873868 as core::ffi::c_int as fe_limb_t,
                            58896884 as core::ffi::c_int as fe_limb_t,
                            2330219 as core::ffi::c_int as fe_limb_t,
                            49446315 as core::ffi::c_int as fe_limb_t,
                            19008651 as core::ffi::c_int as fe_limb_t,
                            10658212 as core::ffi::c_int as fe_limb_t,
                            6671822 as core::ffi::c_int as fe_limb_t,
                            19012087 as core::ffi::c_int as fe_limb_t,
                            3772772 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            3753511 as core::ffi::c_int as fe_limb_t,
                            30133366 as core::ffi::c_int as fe_limb_t,
                            10617073 as core::ffi::c_int as fe_limb_t,
                            2028709 as core::ffi::c_int as fe_limb_t,
                            14841030 as core::ffi::c_int as fe_limb_t,
                            26832768 as core::ffi::c_int as fe_limb_t,
                            28718731 as core::ffi::c_int as fe_limb_t,
                            17791548 as core::ffi::c_int as fe_limb_t,
                            20527770 as core::ffi::c_int as fe_limb_t,
                            12988982 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            52286360 as core::ffi::c_int as fe_limb_t,
                            27757162 as core::ffi::c_int as fe_limb_t,
                            63400876 as core::ffi::c_int as fe_limb_t,
                            12689772 as core::ffi::c_int as fe_limb_t,
                            66209881 as core::ffi::c_int as fe_limb_t,
                            22639565 as core::ffi::c_int as fe_limb_t,
                            42925817 as core::ffi::c_int as fe_limb_t,
                            22989488 as core::ffi::c_int as fe_limb_t,
                            3299664 as core::ffi::c_int as fe_limb_t,
                            21129479 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            50331161 as core::ffi::c_int as fe_limb_t,
                            18301130 as core::ffi::c_int as fe_limb_t,
                            57466446 as core::ffi::c_int as fe_limb_t,
                            4978982 as core::ffi::c_int as fe_limb_t,
                            3308785 as core::ffi::c_int as fe_limb_t,
                            8755439 as core::ffi::c_int as fe_limb_t,
                            6943197 as core::ffi::c_int as fe_limb_t,
                            6461331 as core::ffi::c_int as fe_limb_t,
                            41525717 as core::ffi::c_int as fe_limb_t,
                            8991217 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49882601 as core::ffi::c_int as fe_limb_t,
                            1816361 as core::ffi::c_int as fe_limb_t,
                            65435576 as core::ffi::c_int as fe_limb_t,
                            27467992 as core::ffi::c_int as fe_limb_t,
                            31783887 as core::ffi::c_int as fe_limb_t,
                            25378441 as core::ffi::c_int as fe_limb_t,
                            34160718 as core::ffi::c_int as fe_limb_t,
                            7417949 as core::ffi::c_int as fe_limb_t,
                            36866577 as core::ffi::c_int as fe_limb_t,
                            1507264 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            29692644 as core::ffi::c_int as fe_limb_t,
                            6829891 as core::ffi::c_int as fe_limb_t,
                            56610064 as core::ffi::c_int as fe_limb_t,
                            4334895 as core::ffi::c_int as fe_limb_t,
                            20945975 as core::ffi::c_int as fe_limb_t,
                            21647936 as core::ffi::c_int as fe_limb_t,
                            38221255 as core::ffi::c_int as fe_limb_t,
                            8209390 as core::ffi::c_int as fe_limb_t,
                            14606362 as core::ffi::c_int as fe_limb_t,
                            22907359 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            63627275 as core::ffi::c_int as fe_limb_t,
                            8707080 as core::ffi::c_int as fe_limb_t,
                            32188102 as core::ffi::c_int as fe_limb_t,
                            5672294 as core::ffi::c_int as fe_limb_t,
                            22096700 as core::ffi::c_int as fe_limb_t,
                            1711240 as core::ffi::c_int as fe_limb_t,
                            34088169 as core::ffi::c_int as fe_limb_t,
                            9761486 as core::ffi::c_int as fe_limb_t,
                            4170404 as core::ffi::c_int as fe_limb_t,
                            31469107 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55521375 as core::ffi::c_int as fe_limb_t,
                            14855944 as core::ffi::c_int as fe_limb_t,
                            62981086 as core::ffi::c_int as fe_limb_t,
                            32022574 as core::ffi::c_int as fe_limb_t,
                            40459774 as core::ffi::c_int as fe_limb_t,
                            15084045 as core::ffi::c_int as fe_limb_t,
                            22186522 as core::ffi::c_int as fe_limb_t,
                            16002000 as core::ffi::c_int as fe_limb_t,
                            52832027 as core::ffi::c_int as fe_limb_t,
                            25153633 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62297408 as core::ffi::c_int as fe_limb_t,
                            13761028 as core::ffi::c_int as fe_limb_t,
                            35404987 as core::ffi::c_int as fe_limb_t,
                            31070512 as core::ffi::c_int as fe_limb_t,
                            63796392 as core::ffi::c_int as fe_limb_t,
                            7869046 as core::ffi::c_int as fe_limb_t,
                            59995292 as core::ffi::c_int as fe_limb_t,
                            23934339 as core::ffi::c_int as fe_limb_t,
                            13240844 as core::ffi::c_int as fe_limb_t,
                            10965870 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            59366301 as core::ffi::c_int as fe_limb_t,
                            25297669 as core::ffi::c_int as fe_limb_t,
                            52340529 as core::ffi::c_int as fe_limb_t,
                            19898171 as core::ffi::c_int as fe_limb_t,
                            43876480 as core::ffi::c_int as fe_limb_t,
                            12387165 as core::ffi::c_int as fe_limb_t,
                            4498947 as core::ffi::c_int as fe_limb_t,
                            14147411 as core::ffi::c_int as fe_limb_t,
                            29514390 as core::ffi::c_int as fe_limb_t,
                            4302863 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            53695440 as core::ffi::c_int as fe_limb_t,
                            21146572 as core::ffi::c_int as fe_limb_t,
                            20757301 as core::ffi::c_int as fe_limb_t,
                            19752600 as core::ffi::c_int as fe_limb_t,
                            14785142 as core::ffi::c_int as fe_limb_t,
                            8976368 as core::ffi::c_int as fe_limb_t,
                            62047588 as core::ffi::c_int as fe_limb_t,
                            31410058 as core::ffi::c_int as fe_limb_t,
                            17846987 as core::ffi::c_int as fe_limb_t,
                            19582505 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            64864412 as core::ffi::c_int as fe_limb_t,
                            32799703 as core::ffi::c_int as fe_limb_t,
                            62511833 as core::ffi::c_int as fe_limb_t,
                            32488122 as core::ffi::c_int as fe_limb_t,
                            60861691 as core::ffi::c_int as fe_limb_t,
                            1455298 as core::ffi::c_int as fe_limb_t,
                            45461136 as core::ffi::c_int as fe_limb_t,
                            24339642 as core::ffi::c_int as fe_limb_t,
                            61886162 as core::ffi::c_int as fe_limb_t,
                            12650266 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            57202067 as core::ffi::c_int as fe_limb_t,
                            17484121 as core::ffi::c_int as fe_limb_t,
                            21134159 as core::ffi::c_int as fe_limb_t,
                            12198166 as core::ffi::c_int as fe_limb_t,
                            40044289 as core::ffi::c_int as fe_limb_t,
                            708125 as core::ffi::c_int as fe_limb_t,
                            387813 as core::ffi::c_int as fe_limb_t,
                            13770293 as core::ffi::c_int as fe_limb_t,
                            47974538 as core::ffi::c_int as fe_limb_t,
                            10958662 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            22470984 as core::ffi::c_int as fe_limb_t,
                            12369526 as core::ffi::c_int as fe_limb_t,
                            23446014 as core::ffi::c_int as fe_limb_t,
                            28113323 as core::ffi::c_int as fe_limb_t,
                            45588061 as core::ffi::c_int as fe_limb_t,
                            23855708 as core::ffi::c_int as fe_limb_t,
                            55336367 as core::ffi::c_int as fe_limb_t,
                            21979976 as core::ffi::c_int as fe_limb_t,
                            42025033 as core::ffi::c_int as fe_limb_t,
                            4271861 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41939299 as core::ffi::c_int as fe_limb_t,
                            23500789 as core::ffi::c_int as fe_limb_t,
                            47199531 as core::ffi::c_int as fe_limb_t,
                            15361594 as core::ffi::c_int as fe_limb_t,
                            61124506 as core::ffi::c_int as fe_limb_t,
                            2159191 as core::ffi::c_int as fe_limb_t,
                            75375 as core::ffi::c_int as fe_limb_t,
                            29275903 as core::ffi::c_int as fe_limb_t,
                            34582642 as core::ffi::c_int as fe_limb_t,
                            8469672 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            15854951 as core::ffi::c_int as fe_limb_t,
                            4148314 as core::ffi::c_int as fe_limb_t,
                            58214974 as core::ffi::c_int as fe_limb_t,
                            7259001 as core::ffi::c_int as fe_limb_t,
                            11666551 as core::ffi::c_int as fe_limb_t,
                            13824734 as core::ffi::c_int as fe_limb_t,
                            36577666 as core::ffi::c_int as fe_limb_t,
                            2697371 as core::ffi::c_int as fe_limb_t,
                            24154791 as core::ffi::c_int as fe_limb_t,
                            24093489 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            15446137 as core::ffi::c_int as fe_limb_t,
                            17747788 as core::ffi::c_int as fe_limb_t,
                            29759746 as core::ffi::c_int as fe_limb_t,
                            14019369 as core::ffi::c_int as fe_limb_t,
                            30811221 as core::ffi::c_int as fe_limb_t,
                            23944241 as core::ffi::c_int as fe_limb_t,
                            35526855 as core::ffi::c_int as fe_limb_t,
                            12840103 as core::ffi::c_int as fe_limb_t,
                            24913809 as core::ffi::c_int as fe_limb_t,
                            9815020 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62399578 as core::ffi::c_int as fe_limb_t,
                            27940162 as core::ffi::c_int as fe_limb_t,
                            35267365 as core::ffi::c_int as fe_limb_t,
                            21265538 as core::ffi::c_int as fe_limb_t,
                            52665326 as core::ffi::c_int as fe_limb_t,
                            10799413 as core::ffi::c_int as fe_limb_t,
                            58005188 as core::ffi::c_int as fe_limb_t,
                            13438768 as core::ffi::c_int as fe_limb_t,
                            18735128 as core::ffi::c_int as fe_limb_t,
                            9466238 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11933045 as core::ffi::c_int as fe_limb_t,
                            9281483 as core::ffi::c_int as fe_limb_t,
                            5081055 as core::ffi::c_int as fe_limb_t,
                            28370608 as core::ffi::c_int as fe_limb_t,
                            64480701 as core::ffi::c_int as fe_limb_t,
                            28648802 as core::ffi::c_int as fe_limb_t,
                            59381042 as core::ffi::c_int as fe_limb_t,
                            22658328 as core::ffi::c_int as fe_limb_t,
                            44380208 as core::ffi::c_int as fe_limb_t,
                            16199063 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            14576810 as core::ffi::c_int as fe_limb_t,
                            379472 as core::ffi::c_int as fe_limb_t,
                            40322331 as core::ffi::c_int as fe_limb_t,
                            25237195 as core::ffi::c_int as fe_limb_t,
                            37682355 as core::ffi::c_int as fe_limb_t,
                            22741457 as core::ffi::c_int as fe_limb_t,
                            67006097 as core::ffi::c_int as fe_limb_t,
                            1876698 as core::ffi::c_int as fe_limb_t,
                            30801119 as core::ffi::c_int as fe_limb_t,
                            2164795 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15995086 as core::ffi::c_int as fe_limb_t,
                            3199873 as core::ffi::c_int as fe_limb_t,
                            13672555 as core::ffi::c_int as fe_limb_t,
                            13712240 as core::ffi::c_int as fe_limb_t,
                            47730029 as core::ffi::c_int as fe_limb_t,
                            28906785 as core::ffi::c_int as fe_limb_t,
                            54027253 as core::ffi::c_int as fe_limb_t,
                            18058162 as core::ffi::c_int as fe_limb_t,
                            53616056 as core::ffi::c_int as fe_limb_t,
                            1268051 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56818250 as core::ffi::c_int as fe_limb_t,
                            29895392 as core::ffi::c_int as fe_limb_t,
                            63822271 as core::ffi::c_int as fe_limb_t,
                            10948817 as core::ffi::c_int as fe_limb_t,
                            23037027 as core::ffi::c_int as fe_limb_t,
                            3794475 as core::ffi::c_int as fe_limb_t,
                            63638526 as core::ffi::c_int as fe_limb_t,
                            20954210 as core::ffi::c_int as fe_limb_t,
                            50053494 as core::ffi::c_int as fe_limb_t,
                            3565903 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            29210069 as core::ffi::c_int as fe_limb_t,
                            24135095 as core::ffi::c_int as fe_limb_t,
                            61189071 as core::ffi::c_int as fe_limb_t,
                            28601646 as core::ffi::c_int as fe_limb_t,
                            10834810 as core::ffi::c_int as fe_limb_t,
                            20226706 as core::ffi::c_int as fe_limb_t,
                            50596761 as core::ffi::c_int as fe_limb_t,
                            22733718 as core::ffi::c_int as fe_limb_t,
                            39946641 as core::ffi::c_int as fe_limb_t,
                            19523900 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            53946955 as core::ffi::c_int as fe_limb_t,
                            15508587 as core::ffi::c_int as fe_limb_t,
                            16663704 as core::ffi::c_int as fe_limb_t,
                            25398282 as core::ffi::c_int as fe_limb_t,
                            38758921 as core::ffi::c_int as fe_limb_t,
                            9019122 as core::ffi::c_int as fe_limb_t,
                            37925443 as core::ffi::c_int as fe_limb_t,
                            29785008 as core::ffi::c_int as fe_limb_t,
                            2244110 as core::ffi::c_int as fe_limb_t,
                            19552453 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            61955989 as core::ffi::c_int as fe_limb_t,
                            29753495 as core::ffi::c_int as fe_limb_t,
                            57802388 as core::ffi::c_int as fe_limb_t,
                            27482848 as core::ffi::c_int as fe_limb_t,
                            16243068 as core::ffi::c_int as fe_limb_t,
                            14684434 as core::ffi::c_int as fe_limb_t,
                            41435776 as core::ffi::c_int as fe_limb_t,
                            17373631 as core::ffi::c_int as fe_limb_t,
                            13491505 as core::ffi::c_int as fe_limb_t,
                            4641841 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10813398 as core::ffi::c_int as fe_limb_t,
                            643330 as core::ffi::c_int as fe_limb_t,
                            47920349 as core::ffi::c_int as fe_limb_t,
                            32825515 as core::ffi::c_int as fe_limb_t,
                            30292061 as core::ffi::c_int as fe_limb_t,
                            16954354 as core::ffi::c_int as fe_limb_t,
                            27548446 as core::ffi::c_int as fe_limb_t,
                            25833190 as core::ffi::c_int as fe_limb_t,
                            14476988 as core::ffi::c_int as fe_limb_t,
                            20787001 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            10292079 as core::ffi::c_int as fe_limb_t,
                            9984945 as core::ffi::c_int as fe_limb_t,
                            6481436 as core::ffi::c_int as fe_limb_t,
                            8279905 as core::ffi::c_int as fe_limb_t,
                            59857350 as core::ffi::c_int as fe_limb_t,
                            7032742 as core::ffi::c_int as fe_limb_t,
                            27282937 as core::ffi::c_int as fe_limb_t,
                            31910173 as core::ffi::c_int as fe_limb_t,
                            39196053 as core::ffi::c_int as fe_limb_t,
                            12651323 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            35923332 as core::ffi::c_int as fe_limb_t,
                            32741048 as core::ffi::c_int as fe_limb_t,
                            22271203 as core::ffi::c_int as fe_limb_t,
                            11835308 as core::ffi::c_int as fe_limb_t,
                            10201545 as core::ffi::c_int as fe_limb_t,
                            15351028 as core::ffi::c_int as fe_limb_t,
                            17099662 as core::ffi::c_int as fe_limb_t,
                            3988035 as core::ffi::c_int as fe_limb_t,
                            21721536 as core::ffi::c_int as fe_limb_t,
                            30405492 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10202177 as core::ffi::c_int as fe_limb_t,
                            27008593 as core::ffi::c_int as fe_limb_t,
                            35735631 as core::ffi::c_int as fe_limb_t,
                            23979793 as core::ffi::c_int as fe_limb_t,
                            34958221 as core::ffi::c_int as fe_limb_t,
                            25434748 as core::ffi::c_int as fe_limb_t,
                            54202543 as core::ffi::c_int as fe_limb_t,
                            3852693 as core::ffi::c_int as fe_limb_t,
                            13216206 as core::ffi::c_int as fe_limb_t,
                            14842320 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            51293224 as core::ffi::c_int as fe_limb_t,
                            22953365 as core::ffi::c_int as fe_limb_t,
                            60569911 as core::ffi::c_int as fe_limb_t,
                            26295436 as core::ffi::c_int as fe_limb_t,
                            60124204 as core::ffi::c_int as fe_limb_t,
                            26972653 as core::ffi::c_int as fe_limb_t,
                            35608016 as core::ffi::c_int as fe_limb_t,
                            13765823 as core::ffi::c_int as fe_limb_t,
                            39674467 as core::ffi::c_int as fe_limb_t,
                            9900183 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            14465486 as core::ffi::c_int as fe_limb_t,
                            19721101 as core::ffi::c_int as fe_limb_t,
                            34974879 as core::ffi::c_int as fe_limb_t,
                            18815558 as core::ffi::c_int as fe_limb_t,
                            39665676 as core::ffi::c_int as fe_limb_t,
                            12990491 as core::ffi::c_int as fe_limb_t,
                            33046193 as core::ffi::c_int as fe_limb_t,
                            15796406 as core::ffi::c_int as fe_limb_t,
                            60056998 as core::ffi::c_int as fe_limb_t,
                            25514317 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            30924398 as core::ffi::c_int as fe_limb_t,
                            25274812 as core::ffi::c_int as fe_limb_t,
                            6359015 as core::ffi::c_int as fe_limb_t,
                            20738097 as core::ffi::c_int as fe_limb_t,
                            16508376 as core::ffi::c_int as fe_limb_t,
                            9071735 as core::ffi::c_int as fe_limb_t,
                            41620263 as core::ffi::c_int as fe_limb_t,
                            15413634 as core::ffi::c_int as fe_limb_t,
                            9524356 as core::ffi::c_int as fe_limb_t,
                            26535554 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12274201 as core::ffi::c_int as fe_limb_t,
                            20378885 as core::ffi::c_int as fe_limb_t,
                            32627640 as core::ffi::c_int as fe_limb_t,
                            31769106 as core::ffi::c_int as fe_limb_t,
                            6736624 as core::ffi::c_int as fe_limb_t,
                            13267305 as core::ffi::c_int as fe_limb_t,
                            5237659 as core::ffi::c_int as fe_limb_t,
                            28444949 as core::ffi::c_int as fe_limb_t,
                            15663515 as core::ffi::c_int as fe_limb_t,
                            4035784 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            64157555 as core::ffi::c_int as fe_limb_t,
                            8903984 as core::ffi::c_int as fe_limb_t,
                            17349946 as core::ffi::c_int as fe_limb_t,
                            601635 as core::ffi::c_int as fe_limb_t,
                            50676049 as core::ffi::c_int as fe_limb_t,
                            28941875 as core::ffi::c_int as fe_limb_t,
                            53376124 as core::ffi::c_int as fe_limb_t,
                            17665097 as core::ffi::c_int as fe_limb_t,
                            44850385 as core::ffi::c_int as fe_limb_t,
                            4659090 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            50192582 as core::ffi::c_int as fe_limb_t,
                            28601458 as core::ffi::c_int as fe_limb_t,
                            36715152 as core::ffi::c_int as fe_limb_t,
                            18395610 as core::ffi::c_int as fe_limb_t,
                            20774811 as core::ffi::c_int as fe_limb_t,
                            15897498 as core::ffi::c_int as fe_limb_t,
                            5736189 as core::ffi::c_int as fe_limb_t,
                            15026997 as core::ffi::c_int as fe_limb_t,
                            64930608 as core::ffi::c_int as fe_limb_t,
                            20098846 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            58249865 as core::ffi::c_int as fe_limb_t,
                            31335375 as core::ffi::c_int as fe_limb_t,
                            28571665 as core::ffi::c_int as fe_limb_t,
                            23398914 as core::ffi::c_int as fe_limb_t,
                            66634396 as core::ffi::c_int as fe_limb_t,
                            23448733 as core::ffi::c_int as fe_limb_t,
                            63307367 as core::ffi::c_int as fe_limb_t,
                            278094 as core::ffi::c_int as fe_limb_t,
                            23440562 as core::ffi::c_int as fe_limb_t,
                            33264224 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10226222 as core::ffi::c_int as fe_limb_t,
                            27625730 as core::ffi::c_int as fe_limb_t,
                            15139955 as core::ffi::c_int as fe_limb_t,
                            120818 as core::ffi::c_int as fe_limb_t,
                            52241171 as core::ffi::c_int as fe_limb_t,
                            5218602 as core::ffi::c_int as fe_limb_t,
                            32937275 as core::ffi::c_int as fe_limb_t,
                            11551483 as core::ffi::c_int as fe_limb_t,
                            50536904 as core::ffi::c_int as fe_limb_t,
                            26111567 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            17932739 as core::ffi::c_int as fe_limb_t,
                            21117156 as core::ffi::c_int as fe_limb_t,
                            43069306 as core::ffi::c_int as fe_limb_t,
                            10749059 as core::ffi::c_int as fe_limb_t,
                            11316803 as core::ffi::c_int as fe_limb_t,
                            7535897 as core::ffi::c_int as fe_limb_t,
                            22503767 as core::ffi::c_int as fe_limb_t,
                            5561594 as core::ffi::c_int as fe_limb_t,
                            63462240 as core::ffi::c_int as fe_limb_t,
                            3898660 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            7749907 as core::ffi::c_int as fe_limb_t,
                            32584865 as core::ffi::c_int as fe_limb_t,
                            50769132 as core::ffi::c_int as fe_limb_t,
                            33537967 as core::ffi::c_int as fe_limb_t,
                            42090752 as core::ffi::c_int as fe_limb_t,
                            15122142 as core::ffi::c_int as fe_limb_t,
                            65535333 as core::ffi::c_int as fe_limb_t,
                            7152529 as core::ffi::c_int as fe_limb_t,
                            21831162 as core::ffi::c_int as fe_limb_t,
                            1245233 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            26958440 as core::ffi::c_int as fe_limb_t,
                            18896406 as core::ffi::c_int as fe_limb_t,
                            4314585 as core::ffi::c_int as fe_limb_t,
                            8346991 as core::ffi::c_int as fe_limb_t,
                            61431100 as core::ffi::c_int as fe_limb_t,
                            11960071 as core::ffi::c_int as fe_limb_t,
                            34519569 as core::ffi::c_int as fe_limb_t,
                            32934396 as core::ffi::c_int as fe_limb_t,
                            36706772 as core::ffi::c_int as fe_limb_t,
                            16838219 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            54942968 as core::ffi::c_int as fe_limb_t,
                            9166946 as core::ffi::c_int as fe_limb_t,
                            33491384 as core::ffi::c_int as fe_limb_t,
                            13673479 as core::ffi::c_int as fe_limb_t,
                            29787085 as core::ffi::c_int as fe_limb_t,
                            13096535 as core::ffi::c_int as fe_limb_t,
                            6280834 as core::ffi::c_int as fe_limb_t,
                            14587357 as core::ffi::c_int as fe_limb_t,
                            44770839 as core::ffi::c_int as fe_limb_t,
                            13987524 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            42758936 as core::ffi::c_int as fe_limb_t,
                            7778774 as core::ffi::c_int as fe_limb_t,
                            21116000 as core::ffi::c_int as fe_limb_t,
                            15572597 as core::ffi::c_int as fe_limb_t,
                            62275598 as core::ffi::c_int as fe_limb_t,
                            28196653 as core::ffi::c_int as fe_limb_t,
                            62807965 as core::ffi::c_int as fe_limb_t,
                            28429792 as core::ffi::c_int as fe_limb_t,
                            59639082 as core::ffi::c_int as fe_limb_t,
                            30696363 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            9681908 as core::ffi::c_int as fe_limb_t,
                            26817309 as core::ffi::c_int as fe_limb_t,
                            35157219 as core::ffi::c_int as fe_limb_t,
                            13591837 as core::ffi::c_int as fe_limb_t,
                            60225043 as core::ffi::c_int as fe_limb_t,
                            386949 as core::ffi::c_int as fe_limb_t,
                            31622781 as core::ffi::c_int as fe_limb_t,
                            6439245 as core::ffi::c_int as fe_limb_t,
                            52527852 as core::ffi::c_int as fe_limb_t,
                            4091396 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            58682418 as core::ffi::c_int as fe_limb_t,
                            1470726 as core::ffi::c_int as fe_limb_t,
                            38999185 as core::ffi::c_int as fe_limb_t,
                            31957441 as core::ffi::c_int as fe_limb_t,
                            3978626 as core::ffi::c_int as fe_limb_t,
                            28430809 as core::ffi::c_int as fe_limb_t,
                            47486180 as core::ffi::c_int as fe_limb_t,
                            12092162 as core::ffi::c_int as fe_limb_t,
                            29077877 as core::ffi::c_int as fe_limb_t,
                            18812444 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            5269168 as core::ffi::c_int as fe_limb_t,
                            26694706 as core::ffi::c_int as fe_limb_t,
                            53878652 as core::ffi::c_int as fe_limb_t,
                            25533716 as core::ffi::c_int as fe_limb_t,
                            25932562 as core::ffi::c_int as fe_limb_t,
                            1763552 as core::ffi::c_int as fe_limb_t,
                            61502754 as core::ffi::c_int as fe_limb_t,
                            28048550 as core::ffi::c_int as fe_limb_t,
                            47091016 as core::ffi::c_int as fe_limb_t,
                            2357888 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            32264008 as core::ffi::c_int as fe_limb_t,
                            18146780 as core::ffi::c_int as fe_limb_t,
                            61721128 as core::ffi::c_int as fe_limb_t,
                            32394338 as core::ffi::c_int as fe_limb_t,
                            65017541 as core::ffi::c_int as fe_limb_t,
                            29607531 as core::ffi::c_int as fe_limb_t,
                            23104803 as core::ffi::c_int as fe_limb_t,
                            20684524 as core::ffi::c_int as fe_limb_t,
                            5727337 as core::ffi::c_int as fe_limb_t,
                            189038 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            14609104 as core::ffi::c_int as fe_limb_t,
                            24599962 as core::ffi::c_int as fe_limb_t,
                            61108297 as core::ffi::c_int as fe_limb_t,
                            16931650 as core::ffi::c_int as fe_limb_t,
                            52531476 as core::ffi::c_int as fe_limb_t,
                            25810533 as core::ffi::c_int as fe_limb_t,
                            40363694 as core::ffi::c_int as fe_limb_t,
                            10942114 as core::ffi::c_int as fe_limb_t,
                            41219933 as core::ffi::c_int as fe_limb_t,
                            18669734 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            20513481 as core::ffi::c_int as fe_limb_t,
                            5557931 as core::ffi::c_int as fe_limb_t,
                            51504251 as core::ffi::c_int as fe_limb_t,
                            7829530 as core::ffi::c_int as fe_limb_t,
                            26413943 as core::ffi::c_int as fe_limb_t,
                            31535028 as core::ffi::c_int as fe_limb_t,
                            45729895 as core::ffi::c_int as fe_limb_t,
                            7471780 as core::ffi::c_int as fe_limb_t,
                            13913677 as core::ffi::c_int as fe_limb_t,
                            28416557 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            41534488 as core::ffi::c_int as fe_limb_t,
                            11967825 as core::ffi::c_int as fe_limb_t,
                            29233242 as core::ffi::c_int as fe_limb_t,
                            12948236 as core::ffi::c_int as fe_limb_t,
                            60354399 as core::ffi::c_int as fe_limb_t,
                            4713226 as core::ffi::c_int as fe_limb_t,
                            58167894 as core::ffi::c_int as fe_limb_t,
                            14059179 as core::ffi::c_int as fe_limb_t,
                            12878652 as core::ffi::c_int as fe_limb_t,
                            8511905 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            41452044 as core::ffi::c_int as fe_limb_t,
                            3393630 as core::ffi::c_int as fe_limb_t,
                            64153449 as core::ffi::c_int as fe_limb_t,
                            26478905 as core::ffi::c_int as fe_limb_t,
                            64858154 as core::ffi::c_int as fe_limb_t,
                            9366907 as core::ffi::c_int as fe_limb_t,
                            36885446 as core::ffi::c_int as fe_limb_t,
                            6812973 as core::ffi::c_int as fe_limb_t,
                            5568676 as core::ffi::c_int as fe_limb_t,
                            30426776 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11630004 as core::ffi::c_int as fe_limb_t,
                            12144454 as core::ffi::c_int as fe_limb_t,
                            2116339 as core::ffi::c_int as fe_limb_t,
                            13606037 as core::ffi::c_int as fe_limb_t,
                            27378885 as core::ffi::c_int as fe_limb_t,
                            15676917 as core::ffi::c_int as fe_limb_t,
                            49700111 as core::ffi::c_int as fe_limb_t,
                            20050058 as core::ffi::c_int as fe_limb_t,
                            52713667 as core::ffi::c_int as fe_limb_t,
                            8070817 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            27117677 as core::ffi::c_int as fe_limb_t,
                            23547054 as core::ffi::c_int as fe_limb_t,
                            35826092 as core::ffi::c_int as fe_limb_t,
                            27984343 as core::ffi::c_int as fe_limb_t,
                            1127281 as core::ffi::c_int as fe_limb_t,
                            12772488 as core::ffi::c_int as fe_limb_t,
                            37262958 as core::ffi::c_int as fe_limb_t,
                            10483305 as core::ffi::c_int as fe_limb_t,
                            55556115 as core::ffi::c_int as fe_limb_t,
                            32525717 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            10637467 as core::ffi::c_int as fe_limb_t,
                            27866368 as core::ffi::c_int as fe_limb_t,
                            5674780 as core::ffi::c_int as fe_limb_t,
                            1072708 as core::ffi::c_int as fe_limb_t,
                            40765276 as core::ffi::c_int as fe_limb_t,
                            26572129 as core::ffi::c_int as fe_limb_t,
                            65424888 as core::ffi::c_int as fe_limb_t,
                            9177852 as core::ffi::c_int as fe_limb_t,
                            39615702 as core::ffi::c_int as fe_limb_t,
                            15431202 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            20525126 as core::ffi::c_int as fe_limb_t,
                            10892566 as core::ffi::c_int as fe_limb_t,
                            54366392 as core::ffi::c_int as fe_limb_t,
                            12779442 as core::ffi::c_int as fe_limb_t,
                            37615830 as core::ffi::c_int as fe_limb_t,
                            16150074 as core::ffi::c_int as fe_limb_t,
                            38868345 as core::ffi::c_int as fe_limb_t,
                            14943141 as core::ffi::c_int as fe_limb_t,
                            52052074 as core::ffi::c_int as fe_limb_t,
                            25618500 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            37084402 as core::ffi::c_int as fe_limb_t,
                            5626925 as core::ffi::c_int as fe_limb_t,
                            66557297 as core::ffi::c_int as fe_limb_t,
                            23573344 as core::ffi::c_int as fe_limb_t,
                            753597 as core::ffi::c_int as fe_limb_t,
                            11981191 as core::ffi::c_int as fe_limb_t,
                            25244767 as core::ffi::c_int as fe_limb_t,
                            30314666 as core::ffi::c_int as fe_limb_t,
                            63752313 as core::ffi::c_int as fe_limb_t,
                            9594023 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43356201 as core::ffi::c_int as fe_limb_t,
                            2636869 as core::ffi::c_int as fe_limb_t,
                            61944954 as core::ffi::c_int as fe_limb_t,
                            23450613 as core::ffi::c_int as fe_limb_t,
                            585133 as core::ffi::c_int as fe_limb_t,
                            7877383 as core::ffi::c_int as fe_limb_t,
                            11345683 as core::ffi::c_int as fe_limb_t,
                            27062142 as core::ffi::c_int as fe_limb_t,
                            13352334 as core::ffi::c_int as fe_limb_t,
                            22577348 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            65177046 as core::ffi::c_int as fe_limb_t,
                            28146973 as core::ffi::c_int as fe_limb_t,
                            3304648 as core::ffi::c_int as fe_limb_t,
                            20669563 as core::ffi::c_int as fe_limb_t,
                            17015805 as core::ffi::c_int as fe_limb_t,
                            28677341 as core::ffi::c_int as fe_limb_t,
                            37325013 as core::ffi::c_int as fe_limb_t,
                            25801949 as core::ffi::c_int as fe_limb_t,
                            53893326 as core::ffi::c_int as fe_limb_t,
                            33235227 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            20239939 as core::ffi::c_int as fe_limb_t,
                            6607058 as core::ffi::c_int as fe_limb_t,
                            6203985 as core::ffi::c_int as fe_limb_t,
                            3483793 as core::ffi::c_int as fe_limb_t,
                            48721888 as core::ffi::c_int as fe_limb_t,
                            32775202 as core::ffi::c_int as fe_limb_t,
                            46385121 as core::ffi::c_int as fe_limb_t,
                            15077869 as core::ffi::c_int as fe_limb_t,
                            44358105 as core::ffi::c_int as fe_limb_t,
                            14523816 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            27406023 as core::ffi::c_int as fe_limb_t,
                            27512775 as core::ffi::c_int as fe_limb_t,
                            27423595 as core::ffi::c_int as fe_limb_t,
                            29057038 as core::ffi::c_int as fe_limb_t,
                            4996213 as core::ffi::c_int as fe_limb_t,
                            10002360 as core::ffi::c_int as fe_limb_t,
                            38266833 as core::ffi::c_int as fe_limb_t,
                            29008937 as core::ffi::c_int as fe_limb_t,
                            36936121 as core::ffi::c_int as fe_limb_t,
                            28748764 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11374242 as core::ffi::c_int as fe_limb_t,
                            12660715 as core::ffi::c_int as fe_limb_t,
                            17861383 as core::ffi::c_int as fe_limb_t,
                            21013599 as core::ffi::c_int as fe_limb_t,
                            10935567 as core::ffi::c_int as fe_limb_t,
                            1099227 as core::ffi::c_int as fe_limb_t,
                            53222788 as core::ffi::c_int as fe_limb_t,
                            24462691 as core::ffi::c_int as fe_limb_t,
                            39381819 as core::ffi::c_int as fe_limb_t,
                            11358503 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54378055 as core::ffi::c_int as fe_limb_t,
                            10311866 as core::ffi::c_int as fe_limb_t,
                            1510375 as core::ffi::c_int as fe_limb_t,
                            10778093 as core::ffi::c_int as fe_limb_t,
                            64989409 as core::ffi::c_int as fe_limb_t,
                            24408729 as core::ffi::c_int as fe_limb_t,
                            32676002 as core::ffi::c_int as fe_limb_t,
                            11149336 as core::ffi::c_int as fe_limb_t,
                            40985213 as core::ffi::c_int as fe_limb_t,
                            4985767 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            48012542 as core::ffi::c_int as fe_limb_t,
                            341146 as core::ffi::c_int as fe_limb_t,
                            60911379 as core::ffi::c_int as fe_limb_t,
                            33315398 as core::ffi::c_int as fe_limb_t,
                            15756972 as core::ffi::c_int as fe_limb_t,
                            24757770 as core::ffi::c_int as fe_limb_t,
                            66125820 as core::ffi::c_int as fe_limb_t,
                            13794113 as core::ffi::c_int as fe_limb_t,
                            47694557 as core::ffi::c_int as fe_limb_t,
                            17933176 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            6490062 as core::ffi::c_int as fe_limb_t,
                            11940286 as core::ffi::c_int as fe_limb_t,
                            25495923 as core::ffi::c_int as fe_limb_t,
                            25828072 as core::ffi::c_int as fe_limb_t,
                            8668372 as core::ffi::c_int as fe_limb_t,
                            24803116 as core::ffi::c_int as fe_limb_t,
                            3367602 as core::ffi::c_int as fe_limb_t,
                            6970005 as core::ffi::c_int as fe_limb_t,
                            65417799 as core::ffi::c_int as fe_limb_t,
                            24549641 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            1656478 as core::ffi::c_int as fe_limb_t,
                            13457317 as core::ffi::c_int as fe_limb_t,
                            15370807 as core::ffi::c_int as fe_limb_t,
                            6364910 as core::ffi::c_int as fe_limb_t,
                            13605745 as core::ffi::c_int as fe_limb_t,
                            8362338 as core::ffi::c_int as fe_limb_t,
                            47934242 as core::ffi::c_int as fe_limb_t,
                            28078708 as core::ffi::c_int as fe_limb_t,
                            50312267 as core::ffi::c_int as fe_limb_t,
                            28522993 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            44835530 as core::ffi::c_int as fe_limb_t,
                            20030007 as core::ffi::c_int as fe_limb_t,
                            67044178 as core::ffi::c_int as fe_limb_t,
                            29220208 as core::ffi::c_int as fe_limb_t,
                            48503227 as core::ffi::c_int as fe_limb_t,
                            22632463 as core::ffi::c_int as fe_limb_t,
                            46537798 as core::ffi::c_int as fe_limb_t,
                            26546453 as core::ffi::c_int as fe_limb_t,
                            67009010 as core::ffi::c_int as fe_limb_t,
                            23317098 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            17747446 as core::ffi::c_int as fe_limb_t,
                            10039260 as core::ffi::c_int as fe_limb_t,
                            19368299 as core::ffi::c_int as fe_limb_t,
                            29503841 as core::ffi::c_int as fe_limb_t,
                            46478228 as core::ffi::c_int as fe_limb_t,
                            17513145 as core::ffi::c_int as fe_limb_t,
                            31992682 as core::ffi::c_int as fe_limb_t,
                            17696456 as core::ffi::c_int as fe_limb_t,
                            37848500 as core::ffi::c_int as fe_limb_t,
                            28042460 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            31932008 as core::ffi::c_int as fe_limb_t,
                            28568291 as core::ffi::c_int as fe_limb_t,
                            47496481 as core::ffi::c_int as fe_limb_t,
                            16366579 as core::ffi::c_int as fe_limb_t,
                            22023614 as core::ffi::c_int as fe_limb_t,
                            88450 as core::ffi::c_int as fe_limb_t,
                            11371999 as core::ffi::c_int as fe_limb_t,
                            29810185 as core::ffi::c_int as fe_limb_t,
                            4882241 as core::ffi::c_int as fe_limb_t,
                            22927527 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            29796488 as core::ffi::c_int as fe_limb_t,
                            37186 as core::ffi::c_int as fe_limb_t,
                            19818052 as core::ffi::c_int as fe_limb_t,
                            10115756 as core::ffi::c_int as fe_limb_t,
                            55279832 as core::ffi::c_int as fe_limb_t,
                            3352735 as core::ffi::c_int as fe_limb_t,
                            18551198 as core::ffi::c_int as fe_limb_t,
                            3272828 as core::ffi::c_int as fe_limb_t,
                            61917932 as core::ffi::c_int as fe_limb_t,
                            29392022 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12501267 as core::ffi::c_int as fe_limb_t,
                            4044383 as core::ffi::c_int as fe_limb_t,
                            58495907 as core::ffi::c_int as fe_limb_t,
                            20162046 as core::ffi::c_int as fe_limb_t,
                            34678811 as core::ffi::c_int as fe_limb_t,
                            5136598 as core::ffi::c_int as fe_limb_t,
                            47878486 as core::ffi::c_int as fe_limb_t,
                            30024734 as core::ffi::c_int as fe_limb_t,
                            330069 as core::ffi::c_int as fe_limb_t,
                            29895023 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            6384877 as core::ffi::c_int as fe_limb_t,
                            2899513 as core::ffi::c_int as fe_limb_t,
                            17807477 as core::ffi::c_int as fe_limb_t,
                            7663917 as core::ffi::c_int as fe_limb_t,
                            64749976 as core::ffi::c_int as fe_limb_t,
                            12363164 as core::ffi::c_int as fe_limb_t,
                            25366522 as core::ffi::c_int as fe_limb_t,
                            24980540 as core::ffi::c_int as fe_limb_t,
                            66837568 as core::ffi::c_int as fe_limb_t,
                            12071498 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            58743349 as core::ffi::c_int as fe_limb_t,
                            29511910 as core::ffi::c_int as fe_limb_t,
                            25133447 as core::ffi::c_int as fe_limb_t,
                            29037077 as core::ffi::c_int as fe_limb_t,
                            60897836 as core::ffi::c_int as fe_limb_t,
                            2265926 as core::ffi::c_int as fe_limb_t,
                            34339246 as core::ffi::c_int as fe_limb_t,
                            1936674 as core::ffi::c_int as fe_limb_t,
                            61949167 as core::ffi::c_int as fe_limb_t,
                            3829362 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            28425966 as core::ffi::c_int as fe_limb_t,
                            27718999 as core::ffi::c_int as fe_limb_t,
                            66531773 as core::ffi::c_int as fe_limb_t,
                            28857233 as core::ffi::c_int as fe_limb_t,
                            52891308 as core::ffi::c_int as fe_limb_t,
                            6870929 as core::ffi::c_int as fe_limb_t,
                            7921550 as core::ffi::c_int as fe_limb_t,
                            26986645 as core::ffi::c_int as fe_limb_t,
                            26333139 as core::ffi::c_int as fe_limb_t,
                            14267664 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56041645 as core::ffi::c_int as fe_limb_t,
                            11871230 as core::ffi::c_int as fe_limb_t,
                            27385719 as core::ffi::c_int as fe_limb_t,
                            22994888 as core::ffi::c_int as fe_limb_t,
                            62522949 as core::ffi::c_int as fe_limb_t,
                            22365119 as core::ffi::c_int as fe_limb_t,
                            10004785 as core::ffi::c_int as fe_limb_t,
                            24844944 as core::ffi::c_int as fe_limb_t,
                            45347639 as core::ffi::c_int as fe_limb_t,
                            8930323 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45911060 as core::ffi::c_int as fe_limb_t,
                            17158396 as core::ffi::c_int as fe_limb_t,
                            25654215 as core::ffi::c_int as fe_limb_t,
                            31829035 as core::ffi::c_int as fe_limb_t,
                            12282011 as core::ffi::c_int as fe_limb_t,
                            11008919 as core::ffi::c_int as fe_limb_t,
                            1541940 as core::ffi::c_int as fe_limb_t,
                            4757911 as core::ffi::c_int as fe_limb_t,
                            40617363 as core::ffi::c_int as fe_limb_t,
                            17145491 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            13537262 as core::ffi::c_int as fe_limb_t,
                            25794942 as core::ffi::c_int as fe_limb_t,
                            46504023 as core::ffi::c_int as fe_limb_t,
                            10961926 as core::ffi::c_int as fe_limb_t,
                            61186044 as core::ffi::c_int as fe_limb_t,
                            20336366 as core::ffi::c_int as fe_limb_t,
                            53952279 as core::ffi::c_int as fe_limb_t,
                            6217253 as core::ffi::c_int as fe_limb_t,
                            51165165 as core::ffi::c_int as fe_limb_t,
                            13814989 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            49686272 as core::ffi::c_int as fe_limb_t,
                            15157789 as core::ffi::c_int as fe_limb_t,
                            18705543 as core::ffi::c_int as fe_limb_t,
                            29619 as core::ffi::c_int as fe_limb_t,
                            24409717 as core::ffi::c_int as fe_limb_t,
                            33293956 as core::ffi::c_int as fe_limb_t,
                            27361680 as core::ffi::c_int as fe_limb_t,
                            9257833 as core::ffi::c_int as fe_limb_t,
                            65152338 as core::ffi::c_int as fe_limb_t,
                            31777517 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42063564 as core::ffi::c_int as fe_limb_t,
                            23362465 as core::ffi::c_int as fe_limb_t,
                            15366584 as core::ffi::c_int as fe_limb_t,
                            15166509 as core::ffi::c_int as fe_limb_t,
                            54003778 as core::ffi::c_int as fe_limb_t,
                            8423555 as core::ffi::c_int as fe_limb_t,
                            37937324 as core::ffi::c_int as fe_limb_t,
                            12361134 as core::ffi::c_int as fe_limb_t,
                            48422886 as core::ffi::c_int as fe_limb_t,
                            4578289 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            24579768 as core::ffi::c_int as fe_limb_t,
                            3711570 as core::ffi::c_int as fe_limb_t,
                            1342322 as core::ffi::c_int as fe_limb_t,
                            22374306 as core::ffi::c_int as fe_limb_t,
                            40103728 as core::ffi::c_int as fe_limb_t,
                            14124955 as core::ffi::c_int as fe_limb_t,
                            44564335 as core::ffi::c_int as fe_limb_t,
                            14074918 as core::ffi::c_int as fe_limb_t,
                            21964432 as core::ffi::c_int as fe_limb_t,
                            8235257 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            60580251 as core::ffi::c_int as fe_limb_t,
                            31142934 as core::ffi::c_int as fe_limb_t,
                            9442965 as core::ffi::c_int as fe_limb_t,
                            27628844 as core::ffi::c_int as fe_limb_t,
                            12025639 as core::ffi::c_int as fe_limb_t,
                            32067012 as core::ffi::c_int as fe_limb_t,
                            64127349 as core::ffi::c_int as fe_limb_t,
                            31885225 as core::ffi::c_int as fe_limb_t,
                            13006805 as core::ffi::c_int as fe_limb_t,
                            2355433 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            50803946 as core::ffi::c_int as fe_limb_t,
                            19949172 as core::ffi::c_int as fe_limb_t,
                            60476436 as core::ffi::c_int as fe_limb_t,
                            28412082 as core::ffi::c_int as fe_limb_t,
                            16974358 as core::ffi::c_int as fe_limb_t,
                            22643349 as core::ffi::c_int as fe_limb_t,
                            27202043 as core::ffi::c_int as fe_limb_t,
                            1719366 as core::ffi::c_int as fe_limb_t,
                            1141648 as core::ffi::c_int as fe_limb_t,
                            20758196 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54244920 as core::ffi::c_int as fe_limb_t,
                            20334445 as core::ffi::c_int as fe_limb_t,
                            58790597 as core::ffi::c_int as fe_limb_t,
                            22536340 as core::ffi::c_int as fe_limb_t,
                            60298718 as core::ffi::c_int as fe_limb_t,
                            28710537 as core::ffi::c_int as fe_limb_t,
                            13475065 as core::ffi::c_int as fe_limb_t,
                            30420460 as core::ffi::c_int as fe_limb_t,
                            32674894 as core::ffi::c_int as fe_limb_t,
                            13715045 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11423316 as core::ffi::c_int as fe_limb_t,
                            28086373 as core::ffi::c_int as fe_limb_t,
                            32344215 as core::ffi::c_int as fe_limb_t,
                            8962751 as core::ffi::c_int as fe_limb_t,
                            24989809 as core::ffi::c_int as fe_limb_t,
                            9241752 as core::ffi::c_int as fe_limb_t,
                            53843611 as core::ffi::c_int as fe_limb_t,
                            16086211 as core::ffi::c_int as fe_limb_t,
                            38367983 as core::ffi::c_int as fe_limb_t,
                            17912338 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            65699196 as core::ffi::c_int as fe_limb_t,
                            12530727 as core::ffi::c_int as fe_limb_t,
                            60740138 as core::ffi::c_int as fe_limb_t,
                            10847386 as core::ffi::c_int as fe_limb_t,
                            19531186 as core::ffi::c_int as fe_limb_t,
                            19422272 as core::ffi::c_int as fe_limb_t,
                            55399715 as core::ffi::c_int as fe_limb_t,
                            7791793 as core::ffi::c_int as fe_limb_t,
                            39862921 as core::ffi::c_int as fe_limb_t,
                            4383346 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            38137966 as core::ffi::c_int as fe_limb_t,
                            5271446 as core::ffi::c_int as fe_limb_t,
                            65842855 as core::ffi::c_int as fe_limb_t,
                            23817442 as core::ffi::c_int as fe_limb_t,
                            54653627 as core::ffi::c_int as fe_limb_t,
                            16732598 as core::ffi::c_int as fe_limb_t,
                            62246457 as core::ffi::c_int as fe_limb_t,
                            28647982 as core::ffi::c_int as fe_limb_t,
                            27193556 as core::ffi::c_int as fe_limb_t,
                            6245191 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51914908 as core::ffi::c_int as fe_limb_t,
                            5362277 as core::ffi::c_int as fe_limb_t,
                            65324971 as core::ffi::c_int as fe_limb_t,
                            2695833 as core::ffi::c_int as fe_limb_t,
                            4960227 as core::ffi::c_int as fe_limb_t,
                            12840725 as core::ffi::c_int as fe_limb_t,
                            23061898 as core::ffi::c_int as fe_limb_t,
                            3260492 as core::ffi::c_int as fe_limb_t,
                            22510453 as core::ffi::c_int as fe_limb_t,
                            8577507 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            54476394 as core::ffi::c_int as fe_limb_t,
                            11257345 as core::ffi::c_int as fe_limb_t,
                            34415870 as core::ffi::c_int as fe_limb_t,
                            13548176 as core::ffi::c_int as fe_limb_t,
                            66387860 as core::ffi::c_int as fe_limb_t,
                            10879010 as core::ffi::c_int as fe_limb_t,
                            31168030 as core::ffi::c_int as fe_limb_t,
                            13952092 as core::ffi::c_int as fe_limb_t,
                            37537372 as core::ffi::c_int as fe_limb_t,
                            29918525 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            3877321 as core::ffi::c_int as fe_limb_t,
                            23981693 as core::ffi::c_int as fe_limb_t,
                            32416691 as core::ffi::c_int as fe_limb_t,
                            5405324 as core::ffi::c_int as fe_limb_t,
                            56104457 as core::ffi::c_int as fe_limb_t,
                            19897796 as core::ffi::c_int as fe_limb_t,
                            3759768 as core::ffi::c_int as fe_limb_t,
                            11935320 as core::ffi::c_int as fe_limb_t,
                            5611860 as core::ffi::c_int as fe_limb_t,
                            8164018 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            50833043 as core::ffi::c_int as fe_limb_t,
                            14667796 as core::ffi::c_int as fe_limb_t,
                            15906460 as core::ffi::c_int as fe_limb_t,
                            12155291 as core::ffi::c_int as fe_limb_t,
                            44997715 as core::ffi::c_int as fe_limb_t,
                            24514713 as core::ffi::c_int as fe_limb_t,
                            32003001 as core::ffi::c_int as fe_limb_t,
                            24722143 as core::ffi::c_int as fe_limb_t,
                            5773084 as core::ffi::c_int as fe_limb_t,
                            25132323 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43320746 as core::ffi::c_int as fe_limb_t,
                            25300131 as core::ffi::c_int as fe_limb_t,
                            1950874 as core::ffi::c_int as fe_limb_t,
                            8937633 as core::ffi::c_int as fe_limb_t,
                            18686727 as core::ffi::c_int as fe_limb_t,
                            16459170 as core::ffi::c_int as fe_limb_t,
                            66203139 as core::ffi::c_int as fe_limb_t,
                            12376319 as core::ffi::c_int as fe_limb_t,
                            31632953 as core::ffi::c_int as fe_limb_t,
                            190926 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            42515238 as core::ffi::c_int as fe_limb_t,
                            17415546 as core::ffi::c_int as fe_limb_t,
                            58684872 as core::ffi::c_int as fe_limb_t,
                            13378745 as core::ffi::c_int as fe_limb_t,
                            14162407 as core::ffi::c_int as fe_limb_t,
                            6901328 as core::ffi::c_int as fe_limb_t,
                            58820115 as core::ffi::c_int as fe_limb_t,
                            4508563 as core::ffi::c_int as fe_limb_t,
                            41767309 as core::ffi::c_int as fe_limb_t,
                            29926903 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            8884438 as core::ffi::c_int as fe_limb_t,
                            27670423 as core::ffi::c_int as fe_limb_t,
                            6023973 as core::ffi::c_int as fe_limb_t,
                            10104341 as core::ffi::c_int as fe_limb_t,
                            60227295 as core::ffi::c_int as fe_limb_t,
                            28612898 as core::ffi::c_int as fe_limb_t,
                            18722940 as core::ffi::c_int as fe_limb_t,
                            18768427 as core::ffi::c_int as fe_limb_t,
                            65436375 as core::ffi::c_int as fe_limb_t,
                            827624 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            34388281 as core::ffi::c_int as fe_limb_t,
                            17265135 as core::ffi::c_int as fe_limb_t,
                            34605316 as core::ffi::c_int as fe_limb_t,
                            7101209 as core::ffi::c_int as fe_limb_t,
                            13354605 as core::ffi::c_int as fe_limb_t,
                            2659080 as core::ffi::c_int as fe_limb_t,
                            65308289 as core::ffi::c_int as fe_limb_t,
                            19446395 as core::ffi::c_int as fe_limb_t,
                            42230385 as core::ffi::c_int as fe_limb_t,
                            1541285 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            2901328 as core::ffi::c_int as fe_limb_t,
                            32436745 as core::ffi::c_int as fe_limb_t,
                            3880375 as core::ffi::c_int as fe_limb_t,
                            23495044 as core::ffi::c_int as fe_limb_t,
                            49487923 as core::ffi::c_int as fe_limb_t,
                            29941650 as core::ffi::c_int as fe_limb_t,
                            45306746 as core::ffi::c_int as fe_limb_t,
                            29986950 as core::ffi::c_int as fe_limb_t,
                            20456844 as core::ffi::c_int as fe_limb_t,
                            31669399 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            27019610 as core::ffi::c_int as fe_limb_t,
                            12299467 as core::ffi::c_int as fe_limb_t,
                            53450576 as core::ffi::c_int as fe_limb_t,
                            31951197 as core::ffi::c_int as fe_limb_t,
                            54247203 as core::ffi::c_int as fe_limb_t,
                            28692960 as core::ffi::c_int as fe_limb_t,
                            47568713 as core::ffi::c_int as fe_limb_t,
                            28538373 as core::ffi::c_int as fe_limb_t,
                            29439640 as core::ffi::c_int as fe_limb_t,
                            15138866 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21536104 as core::ffi::c_int as fe_limb_t,
                            26928012 as core::ffi::c_int as fe_limb_t,
                            34661045 as core::ffi::c_int as fe_limb_t,
                            22864223 as core::ffi::c_int as fe_limb_t,
                            44700786 as core::ffi::c_int as fe_limb_t,
                            5175813 as core::ffi::c_int as fe_limb_t,
                            61688824 as core::ffi::c_int as fe_limb_t,
                            17193268 as core::ffi::c_int as fe_limb_t,
                            7779327 as core::ffi::c_int as fe_limb_t,
                            109896 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30279725 as core::ffi::c_int as fe_limb_t,
                            14648750 as core::ffi::c_int as fe_limb_t,
                            59063993 as core::ffi::c_int as fe_limb_t,
                            6425557 as core::ffi::c_int as fe_limb_t,
                            13639621 as core::ffi::c_int as fe_limb_t,
                            32810923 as core::ffi::c_int as fe_limb_t,
                            28698389 as core::ffi::c_int as fe_limb_t,
                            12180118 as core::ffi::c_int as fe_limb_t,
                            23177719 as core::ffi::c_int as fe_limb_t,
                            33000357 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            26572828 as core::ffi::c_int as fe_limb_t,
                            3405927 as core::ffi::c_int as fe_limb_t,
                            35407164 as core::ffi::c_int as fe_limb_t,
                            12890904 as core::ffi::c_int as fe_limb_t,
                            47843196 as core::ffi::c_int as fe_limb_t,
                            5335865 as core::ffi::c_int as fe_limb_t,
                            60615096 as core::ffi::c_int as fe_limb_t,
                            2378491 as core::ffi::c_int as fe_limb_t,
                            4439158 as core::ffi::c_int as fe_limb_t,
                            20275085 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            44392139 as core::ffi::c_int as fe_limb_t,
                            3489069 as core::ffi::c_int as fe_limb_t,
                            57883598 as core::ffi::c_int as fe_limb_t,
                            33221678 as core::ffi::c_int as fe_limb_t,
                            18875721 as core::ffi::c_int as fe_limb_t,
                            32414337 as core::ffi::c_int as fe_limb_t,
                            14819433 as core::ffi::c_int as fe_limb_t,
                            20822905 as core::ffi::c_int as fe_limb_t,
                            49391106 as core::ffi::c_int as fe_limb_t,
                            28092994 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62052362 as core::ffi::c_int as fe_limb_t,
                            16566550 as core::ffi::c_int as fe_limb_t,
                            15953661 as core::ffi::c_int as fe_limb_t,
                            3767752 as core::ffi::c_int as fe_limb_t,
                            56672365 as core::ffi::c_int as fe_limb_t,
                            15627059 as core::ffi::c_int as fe_limb_t,
                            66287910 as core::ffi::c_int as fe_limb_t,
                            2177224 as core::ffi::c_int as fe_limb_t,
                            8550082 as core::ffi::c_int as fe_limb_t,
                            18440267 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            48635543 as core::ffi::c_int as fe_limb_t,
                            16596774 as core::ffi::c_int as fe_limb_t,
                            66727204 as core::ffi::c_int as fe_limb_t,
                            15663610 as core::ffi::c_int as fe_limb_t,
                            22860960 as core::ffi::c_int as fe_limb_t,
                            15585581 as core::ffi::c_int as fe_limb_t,
                            39264755 as core::ffi::c_int as fe_limb_t,
                            29971692 as core::ffi::c_int as fe_limb_t,
                            43848403 as core::ffi::c_int as fe_limb_t,
                            25125843 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            34628313 as core::ffi::c_int as fe_limb_t,
                            15707274 as core::ffi::c_int as fe_limb_t,
                            58902952 as core::ffi::c_int as fe_limb_t,
                            27902350 as core::ffi::c_int as fe_limb_t,
                            29464557 as core::ffi::c_int as fe_limb_t,
                            2713815 as core::ffi::c_int as fe_limb_t,
                            44383727 as core::ffi::c_int as fe_limb_t,
                            15860481 as core::ffi::c_int as fe_limb_t,
                            45206294 as core::ffi::c_int as fe_limb_t,
                            1494192 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            47546773 as core::ffi::c_int as fe_limb_t,
                            19467038 as core::ffi::c_int as fe_limb_t,
                            41524991 as core::ffi::c_int as fe_limb_t,
                            24254879 as core::ffi::c_int as fe_limb_t,
                            13127841 as core::ffi::c_int as fe_limb_t,
                            759709 as core::ffi::c_int as fe_limb_t,
                            21923482 as core::ffi::c_int as fe_limb_t,
                            16529112 as core::ffi::c_int as fe_limb_t,
                            8742704 as core::ffi::c_int as fe_limb_t,
                            12967017 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            38643965 as core::ffi::c_int as fe_limb_t,
                            1553204 as core::ffi::c_int as fe_limb_t,
                            32536856 as core::ffi::c_int as fe_limb_t,
                            23080703 as core::ffi::c_int as fe_limb_t,
                            42417258 as core::ffi::c_int as fe_limb_t,
                            33148257 as core::ffi::c_int as fe_limb_t,
                            58194238 as core::ffi::c_int as fe_limb_t,
                            30620535 as core::ffi::c_int as fe_limb_t,
                            37205105 as core::ffi::c_int as fe_limb_t,
                            15553882 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21877890 as core::ffi::c_int as fe_limb_t,
                            3230008 as core::ffi::c_int as fe_limb_t,
                            9881174 as core::ffi::c_int as fe_limb_t,
                            10539357 as core::ffi::c_int as fe_limb_t,
                            62311749 as core::ffi::c_int as fe_limb_t,
                            2841331 as core::ffi::c_int as fe_limb_t,
                            11543572 as core::ffi::c_int as fe_limb_t,
                            14513274 as core::ffi::c_int as fe_limb_t,
                            19375923 as core::ffi::c_int as fe_limb_t,
                            20906471 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            8832269 as core::ffi::c_int as fe_limb_t,
                            19058947 as core::ffi::c_int as fe_limb_t,
                            13253510 as core::ffi::c_int as fe_limb_t,
                            5137575 as core::ffi::c_int as fe_limb_t,
                            5037871 as core::ffi::c_int as fe_limb_t,
                            4078777 as core::ffi::c_int as fe_limb_t,
                            24880818 as core::ffi::c_int as fe_limb_t,
                            27331716 as core::ffi::c_int as fe_limb_t,
                            2862652 as core::ffi::c_int as fe_limb_t,
                            9455043 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29306751 as core::ffi::c_int as fe_limb_t,
                            5123106 as core::ffi::c_int as fe_limb_t,
                            20245049 as core::ffi::c_int as fe_limb_t,
                            19404543 as core::ffi::c_int as fe_limb_t,
                            9592565 as core::ffi::c_int as fe_limb_t,
                            8447059 as core::ffi::c_int as fe_limb_t,
                            65031740 as core::ffi::c_int as fe_limb_t,
                            30564351 as core::ffi::c_int as fe_limb_t,
                            15511448 as core::ffi::c_int as fe_limb_t,
                            4789663 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46429108 as core::ffi::c_int as fe_limb_t,
                            7004546 as core::ffi::c_int as fe_limb_t,
                            8824831 as core::ffi::c_int as fe_limb_t,
                            24119455 as core::ffi::c_int as fe_limb_t,
                            63063159 as core::ffi::c_int as fe_limb_t,
                            29803695 as core::ffi::c_int as fe_limb_t,
                            61354101 as core::ffi::c_int as fe_limb_t,
                            108892 as core::ffi::c_int as fe_limb_t,
                            23513200 as core::ffi::c_int as fe_limb_t,
                            16652362 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            33852691 as core::ffi::c_int as fe_limb_t,
                            4144781 as core::ffi::c_int as fe_limb_t,
                            62632835 as core::ffi::c_int as fe_limb_t,
                            26975308 as core::ffi::c_int as fe_limb_t,
                            10770038 as core::ffi::c_int as fe_limb_t,
                            26398890 as core::ffi::c_int as fe_limb_t,
                            60458447 as core::ffi::c_int as fe_limb_t,
                            20618131 as core::ffi::c_int as fe_limb_t,
                            48789665 as core::ffi::c_int as fe_limb_t,
                            10212859 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            2756062 as core::ffi::c_int as fe_limb_t,
                            8598110 as core::ffi::c_int as fe_limb_t,
                            7383731 as core::ffi::c_int as fe_limb_t,
                            26694540 as core::ffi::c_int as fe_limb_t,
                            22312758 as core::ffi::c_int as fe_limb_t,
                            32449420 as core::ffi::c_int as fe_limb_t,
                            21179800 as core::ffi::c_int as fe_limb_t,
                            2600940 as core::ffi::c_int as fe_limb_t,
                            57120566 as core::ffi::c_int as fe_limb_t,
                            21047965 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42463153 as core::ffi::c_int as fe_limb_t,
                            13317461 as core::ffi::c_int as fe_limb_t,
                            36659605 as core::ffi::c_int as fe_limb_t,
                            17900503 as core::ffi::c_int as fe_limb_t,
                            21365573 as core::ffi::c_int as fe_limb_t,
                            22684775 as core::ffi::c_int as fe_limb_t,
                            11344423 as core::ffi::c_int as fe_limb_t,
                            864440 as core::ffi::c_int as fe_limb_t,
                            64609187 as core::ffi::c_int as fe_limb_t,
                            16844368 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            40676061 as core::ffi::c_int as fe_limb_t,
                            6148328 as core::ffi::c_int as fe_limb_t,
                            49924452 as core::ffi::c_int as fe_limb_t,
                            19080277 as core::ffi::c_int as fe_limb_t,
                            18782928 as core::ffi::c_int as fe_limb_t,
                            33278435 as core::ffi::c_int as fe_limb_t,
                            44547329 as core::ffi::c_int as fe_limb_t,
                            211299 as core::ffi::c_int as fe_limb_t,
                            2719757 as core::ffi::c_int as fe_limb_t,
                            4940997 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65784982 as core::ffi::c_int as fe_limb_t,
                            3911312 as core::ffi::c_int as fe_limb_t,
                            60160120 as core::ffi::c_int as fe_limb_t,
                            14759764 as core::ffi::c_int as fe_limb_t,
                            37081714 as core::ffi::c_int as fe_limb_t,
                            7851206 as core::ffi::c_int as fe_limb_t,
                            21690126 as core::ffi::c_int as fe_limb_t,
                            8518463 as core::ffi::c_int as fe_limb_t,
                            26699843 as core::ffi::c_int as fe_limb_t,
                            5276295 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            53958991 as core::ffi::c_int as fe_limb_t,
                            27125364 as core::ffi::c_int as fe_limb_t,
                            9396248 as core::ffi::c_int as fe_limb_t,
                            365013 as core::ffi::c_int as fe_limb_t,
                            24703301 as core::ffi::c_int as fe_limb_t,
                            23065493 as core::ffi::c_int as fe_limb_t,
                            1321585 as core::ffi::c_int as fe_limb_t,
                            149635 as core::ffi::c_int as fe_limb_t,
                            51656090 as core::ffi::c_int as fe_limb_t,
                            7159368 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9987761 as core::ffi::c_int as fe_limb_t,
                            30149673 as core::ffi::c_int as fe_limb_t,
                            17507961 as core::ffi::c_int as fe_limb_t,
                            9505530 as core::ffi::c_int as fe_limb_t,
                            9731535 as core::ffi::c_int as fe_limb_t,
                            31388918 as core::ffi::c_int as fe_limb_t,
                            22356008 as core::ffi::c_int as fe_limb_t,
                            8312176 as core::ffi::c_int as fe_limb_t,
                            22477218 as core::ffi::c_int as fe_limb_t,
                            25151047 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18155857 as core::ffi::c_int as fe_limb_t,
                            17049442 as core::ffi::c_int as fe_limb_t,
                            19744715 as core::ffi::c_int as fe_limb_t,
                            9006923 as core::ffi::c_int as fe_limb_t,
                            15154154 as core::ffi::c_int as fe_limb_t,
                            23015456 as core::ffi::c_int as fe_limb_t,
                            24256459 as core::ffi::c_int as fe_limb_t,
                            28689437 as core::ffi::c_int as fe_limb_t,
                            44560690 as core::ffi::c_int as fe_limb_t,
                            9334108 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            2986088 as core::ffi::c_int as fe_limb_t,
                            28642539 as core::ffi::c_int as fe_limb_t,
                            10776627 as core::ffi::c_int as fe_limb_t,
                            30080588 as core::ffi::c_int as fe_limb_t,
                            10620589 as core::ffi::c_int as fe_limb_t,
                            26471229 as core::ffi::c_int as fe_limb_t,
                            45695018 as core::ffi::c_int as fe_limb_t,
                            14253544 as core::ffi::c_int as fe_limb_t,
                            44521715 as core::ffi::c_int as fe_limb_t,
                            536905 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            4377737 as core::ffi::c_int as fe_limb_t,
                            8115836 as core::ffi::c_int as fe_limb_t,
                            24567078 as core::ffi::c_int as fe_limb_t,
                            15495314 as core::ffi::c_int as fe_limb_t,
                            11625074 as core::ffi::c_int as fe_limb_t,
                            13064599 as core::ffi::c_int as fe_limb_t,
                            7390551 as core::ffi::c_int as fe_limb_t,
                            10589625 as core::ffi::c_int as fe_limb_t,
                            10838060 as core::ffi::c_int as fe_limb_t,
                            18134008 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47766460 as core::ffi::c_int as fe_limb_t,
                            867879 as core::ffi::c_int as fe_limb_t,
                            9277171 as core::ffi::c_int as fe_limb_t,
                            30335973 as core::ffi::c_int as fe_limb_t,
                            52677291 as core::ffi::c_int as fe_limb_t,
                            31567988 as core::ffi::c_int as fe_limb_t,
                            19295825 as core::ffi::c_int as fe_limb_t,
                            17757482 as core::ffi::c_int as fe_limb_t,
                            6378259 as core::ffi::c_int as fe_limb_t,
                            699185 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            7895007 as core::ffi::c_int as fe_limb_t,
                            4057113 as core::ffi::c_int as fe_limb_t,
                            60027092 as core::ffi::c_int as fe_limb_t,
                            20476675 as core::ffi::c_int as fe_limb_t,
                            49222032 as core::ffi::c_int as fe_limb_t,
                            33231305 as core::ffi::c_int as fe_limb_t,
                            66392824 as core::ffi::c_int as fe_limb_t,
                            15693154 as core::ffi::c_int as fe_limb_t,
                            62063800 as core::ffi::c_int as fe_limb_t,
                            20180469 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            59371282 as core::ffi::c_int as fe_limb_t,
                            27685029 as core::ffi::c_int as fe_limb_t,
                            52542544 as core::ffi::c_int as fe_limb_t,
                            26147512 as core::ffi::c_int as fe_limb_t,
                            11385653 as core::ffi::c_int as fe_limb_t,
                            13201616 as core::ffi::c_int as fe_limb_t,
                            31730678 as core::ffi::c_int as fe_limb_t,
                            22591592 as core::ffi::c_int as fe_limb_t,
                            63190227 as core::ffi::c_int as fe_limb_t,
                            23885106 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10188286 as core::ffi::c_int as fe_limb_t,
                            17783598 as core::ffi::c_int as fe_limb_t,
                            59772502 as core::ffi::c_int as fe_limb_t,
                            13427542 as core::ffi::c_int as fe_limb_t,
                            22223443 as core::ffi::c_int as fe_limb_t,
                            14896287 as core::ffi::c_int as fe_limb_t,
                            30743455 as core::ffi::c_int as fe_limb_t,
                            7116568 as core::ffi::c_int as fe_limb_t,
                            45322357 as core::ffi::c_int as fe_limb_t,
                            5427592 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            696102 as core::ffi::c_int as fe_limb_t,
                            13206899 as core::ffi::c_int as fe_limb_t,
                            27047647 as core::ffi::c_int as fe_limb_t,
                            22922350 as core::ffi::c_int as fe_limb_t,
                            15285304 as core::ffi::c_int as fe_limb_t,
                            23701253 as core::ffi::c_int as fe_limb_t,
                            10798489 as core::ffi::c_int as fe_limb_t,
                            28975712 as core::ffi::c_int as fe_limb_t,
                            19236242 as core::ffi::c_int as fe_limb_t,
                            12477404 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            55879425 as core::ffi::c_int as fe_limb_t,
                            11243795 as core::ffi::c_int as fe_limb_t,
                            50054594 as core::ffi::c_int as fe_limb_t,
                            25513566 as core::ffi::c_int as fe_limb_t,
                            66320635 as core::ffi::c_int as fe_limb_t,
                            25386464 as core::ffi::c_int as fe_limb_t,
                            63211194 as core::ffi::c_int as fe_limb_t,
                            11180503 as core::ffi::c_int as fe_limb_t,
                            43939348 as core::ffi::c_int as fe_limb_t,
                            7733643 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            17800790 as core::ffi::c_int as fe_limb_t,
                            19518253 as core::ffi::c_int as fe_limb_t,
                            40108434 as core::ffi::c_int as fe_limb_t,
                            21787760 as core::ffi::c_int as fe_limb_t,
                            23887826 as core::ffi::c_int as fe_limb_t,
                            3149671 as core::ffi::c_int as fe_limb_t,
                            23466177 as core::ffi::c_int as fe_limb_t,
                            23016261 as core::ffi::c_int as fe_limb_t,
                            10322026 as core::ffi::c_int as fe_limb_t,
                            15313801 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            26246234 as core::ffi::c_int as fe_limb_t,
                            11968874 as core::ffi::c_int as fe_limb_t,
                            32263343 as core::ffi::c_int as fe_limb_t,
                            28085704 as core::ffi::c_int as fe_limb_t,
                            6830754 as core::ffi::c_int as fe_limb_t,
                            20231401 as core::ffi::c_int as fe_limb_t,
                            51314159 as core::ffi::c_int as fe_limb_t,
                            33452449 as core::ffi::c_int as fe_limb_t,
                            42659621 as core::ffi::c_int as fe_limb_t,
                            10890803 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            35743198 as core::ffi::c_int as fe_limb_t,
                            10271362 as core::ffi::c_int as fe_limb_t,
                            54448239 as core::ffi::c_int as fe_limb_t,
                            27287163 as core::ffi::c_int as fe_limb_t,
                            16690206 as core::ffi::c_int as fe_limb_t,
                            20491888 as core::ffi::c_int as fe_limb_t,
                            52126651 as core::ffi::c_int as fe_limb_t,
                            16484930 as core::ffi::c_int as fe_limb_t,
                            25180797 as core::ffi::c_int as fe_limb_t,
                            28219548 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            66522290 as core::ffi::c_int as fe_limb_t,
                            10376443 as core::ffi::c_int as fe_limb_t,
                            34522450 as core::ffi::c_int as fe_limb_t,
                            22268075 as core::ffi::c_int as fe_limb_t,
                            19801892 as core::ffi::c_int as fe_limb_t,
                            10997610 as core::ffi::c_int as fe_limb_t,
                            2276632 as core::ffi::c_int as fe_limb_t,
                            9482883 as core::ffi::c_int as fe_limb_t,
                            316878 as core::ffi::c_int as fe_limb_t,
                            13820577 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57226037 as core::ffi::c_int as fe_limb_t,
                            29044064 as core::ffi::c_int as fe_limb_t,
                            64993357 as core::ffi::c_int as fe_limb_t,
                            16457135 as core::ffi::c_int as fe_limb_t,
                            56008783 as core::ffi::c_int as fe_limb_t,
                            11674995 as core::ffi::c_int as fe_limb_t,
                            30756178 as core::ffi::c_int as fe_limb_t,
                            26039378 as core::ffi::c_int as fe_limb_t,
                            30696929 as core::ffi::c_int as fe_limb_t,
                            29841583 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            32988917 as core::ffi::c_int as fe_limb_t,
                            23951020 as core::ffi::c_int as fe_limb_t,
                            12499365 as core::ffi::c_int as fe_limb_t,
                            7910787 as core::ffi::c_int as fe_limb_t,
                            56491607 as core::ffi::c_int as fe_limb_t,
                            21622917 as core::ffi::c_int as fe_limb_t,
                            59766047 as core::ffi::c_int as fe_limb_t,
                            23569034 as core::ffi::c_int as fe_limb_t,
                            34759346 as core::ffi::c_int as fe_limb_t,
                            7392472 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            58253184 as core::ffi::c_int as fe_limb_t,
                            15927860 as core::ffi::c_int as fe_limb_t,
                            9866406 as core::ffi::c_int as fe_limb_t,
                            29905021 as core::ffi::c_int as fe_limb_t,
                            64711949 as core::ffi::c_int as fe_limb_t,
                            16898650 as core::ffi::c_int as fe_limb_t,
                            36699387 as core::ffi::c_int as fe_limb_t,
                            24419436 as core::ffi::c_int as fe_limb_t,
                            25112946 as core::ffi::c_int as fe_limb_t,
                            30627788 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64604801 as core::ffi::c_int as fe_limb_t,
                            33117465 as core::ffi::c_int as fe_limb_t,
                            25621773 as core::ffi::c_int as fe_limb_t,
                            27875660 as core::ffi::c_int as fe_limb_t,
                            15085041 as core::ffi::c_int as fe_limb_t,
                            28074555 as core::ffi::c_int as fe_limb_t,
                            42223985 as core::ffi::c_int as fe_limb_t,
                            20028237 as core::ffi::c_int as fe_limb_t,
                            5537437 as core::ffi::c_int as fe_limb_t,
                            19640113 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            55883280 as core::ffi::c_int as fe_limb_t,
                            2320284 as core::ffi::c_int as fe_limb_t,
                            57524584 as core::ffi::c_int as fe_limb_t,
                            10149186 as core::ffi::c_int as fe_limb_t,
                            33664201 as core::ffi::c_int as fe_limb_t,
                            5808647 as core::ffi::c_int as fe_limb_t,
                            52232613 as core::ffi::c_int as fe_limb_t,
                            31824764 as core::ffi::c_int as fe_limb_t,
                            31234589 as core::ffi::c_int as fe_limb_t,
                            6090599 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            57475529 as core::ffi::c_int as fe_limb_t,
                            116425 as core::ffi::c_int as fe_limb_t,
                            26083934 as core::ffi::c_int as fe_limb_t,
                            2897444 as core::ffi::c_int as fe_limb_t,
                            60744427 as core::ffi::c_int as fe_limb_t,
                            30866345 as core::ffi::c_int as fe_limb_t,
                            609720 as core::ffi::c_int as fe_limb_t,
                            15878753 as core::ffi::c_int as fe_limb_t,
                            60138459 as core::ffi::c_int as fe_limb_t,
                            24519663 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            39351007 as core::ffi::c_int as fe_limb_t,
                            247743 as core::ffi::c_int as fe_limb_t,
                            51914090 as core::ffi::c_int as fe_limb_t,
                            24551880 as core::ffi::c_int as fe_limb_t,
                            23288160 as core::ffi::c_int as fe_limb_t,
                            23542496 as core::ffi::c_int as fe_limb_t,
                            43239268 as core::ffi::c_int as fe_limb_t,
                            6503645 as core::ffi::c_int as fe_limb_t,
                            20650474 as core::ffi::c_int as fe_limb_t,
                            1804084 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            39519059 as core::ffi::c_int as fe_limb_t,
                            15456423 as core::ffi::c_int as fe_limb_t,
                            8972517 as core::ffi::c_int as fe_limb_t,
                            8469608 as core::ffi::c_int as fe_limb_t,
                            15640622 as core::ffi::c_int as fe_limb_t,
                            4439847 as core::ffi::c_int as fe_limb_t,
                            3121995 as core::ffi::c_int as fe_limb_t,
                            23224719 as core::ffi::c_int as fe_limb_t,
                            27842615 as core::ffi::c_int as fe_limb_t,
                            33352104 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51801891 as core::ffi::c_int as fe_limb_t,
                            2839643 as core::ffi::c_int as fe_limb_t,
                            22530074 as core::ffi::c_int as fe_limb_t,
                            10026331 as core::ffi::c_int as fe_limb_t,
                            4602058 as core::ffi::c_int as fe_limb_t,
                            5048462 as core::ffi::c_int as fe_limb_t,
                            28248656 as core::ffi::c_int as fe_limb_t,
                            5031932 as core::ffi::c_int as fe_limb_t,
                            55733782 as core::ffi::c_int as fe_limb_t,
                            12714368 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            20807691 as core::ffi::c_int as fe_limb_t,
                            26283607 as core::ffi::c_int as fe_limb_t,
                            29286140 as core::ffi::c_int as fe_limb_t,
                            11421711 as core::ffi::c_int as fe_limb_t,
                            39232341 as core::ffi::c_int as fe_limb_t,
                            19686201 as core::ffi::c_int as fe_limb_t,
                            45881388 as core::ffi::c_int as fe_limb_t,
                            1035545 as core::ffi::c_int as fe_limb_t,
                            47375635 as core::ffi::c_int as fe_limb_t,
                            12796919 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            12076880 as core::ffi::c_int as fe_limb_t,
                            19253146 as core::ffi::c_int as fe_limb_t,
                            58323862 as core::ffi::c_int as fe_limb_t,
                            21705509 as core::ffi::c_int as fe_limb_t,
                            42096072 as core::ffi::c_int as fe_limb_t,
                            16400683 as core::ffi::c_int as fe_limb_t,
                            49517369 as core::ffi::c_int as fe_limb_t,
                            20654993 as core::ffi::c_int as fe_limb_t,
                            3480664 as core::ffi::c_int as fe_limb_t,
                            18371617 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            34747315 as core::ffi::c_int as fe_limb_t,
                            5457596 as core::ffi::c_int as fe_limb_t,
                            28548107 as core::ffi::c_int as fe_limb_t,
                            7833186 as core::ffi::c_int as fe_limb_t,
                            7303070 as core::ffi::c_int as fe_limb_t,
                            21600887 as core::ffi::c_int as fe_limb_t,
                            42745799 as core::ffi::c_int as fe_limb_t,
                            17632556 as core::ffi::c_int as fe_limb_t,
                            33734809 as core::ffi::c_int as fe_limb_t,
                            2771024 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45719598 as core::ffi::c_int as fe_limb_t,
                            421931 as core::ffi::c_int as fe_limb_t,
                            26597266 as core::ffi::c_int as fe_limb_t,
                            6860826 as core::ffi::c_int as fe_limb_t,
                            22486084 as core::ffi::c_int as fe_limb_t,
                            26817260 as core::ffi::c_int as fe_limb_t,
                            49971378 as core::ffi::c_int as fe_limb_t,
                            29344205 as core::ffi::c_int as fe_limb_t,
                            42556581 as core::ffi::c_int as fe_limb_t,
                            15673396 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            46924223 as core::ffi::c_int as fe_limb_t,
                            2338215 as core::ffi::c_int as fe_limb_t,
                            19788685 as core::ffi::c_int as fe_limb_t,
                            23933476 as core::ffi::c_int as fe_limb_t,
                            63107598 as core::ffi::c_int as fe_limb_t,
                            24813538 as core::ffi::c_int as fe_limb_t,
                            46837679 as core::ffi::c_int as fe_limb_t,
                            4733253 as core::ffi::c_int as fe_limb_t,
                            3727144 as core::ffi::c_int as fe_limb_t,
                            20619984 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            6120100 as core::ffi::c_int as fe_limb_t,
                            814863 as core::ffi::c_int as fe_limb_t,
                            55314462 as core::ffi::c_int as fe_limb_t,
                            32931715 as core::ffi::c_int as fe_limb_t,
                            6812204 as core::ffi::c_int as fe_limb_t,
                            17806661 as core::ffi::c_int as fe_limb_t,
                            2019593 as core::ffi::c_int as fe_limb_t,
                            7975683 as core::ffi::c_int as fe_limb_t,
                            31123697 as core::ffi::c_int as fe_limb_t,
                            22595451 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            30069250 as core::ffi::c_int as fe_limb_t,
                            22119100 as core::ffi::c_int as fe_limb_t,
                            30434653 as core::ffi::c_int as fe_limb_t,
                            2958439 as core::ffi::c_int as fe_limb_t,
                            18399564 as core::ffi::c_int as fe_limb_t,
                            32578143 as core::ffi::c_int as fe_limb_t,
                            12296868 as core::ffi::c_int as fe_limb_t,
                            9204260 as core::ffi::c_int as fe_limb_t,
                            50676426 as core::ffi::c_int as fe_limb_t,
                            9648164 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            32705413 as core::ffi::c_int as fe_limb_t,
                            32003455 as core::ffi::c_int as fe_limb_t,
                            30705657 as core::ffi::c_int as fe_limb_t,
                            7451065 as core::ffi::c_int as fe_limb_t,
                            55303258 as core::ffi::c_int as fe_limb_t,
                            9631812 as core::ffi::c_int as fe_limb_t,
                            3305266 as core::ffi::c_int as fe_limb_t,
                            5248604 as core::ffi::c_int as fe_limb_t,
                            41100532 as core::ffi::c_int as fe_limb_t,
                            22176930 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            17219846 as core::ffi::c_int as fe_limb_t,
                            2375039 as core::ffi::c_int as fe_limb_t,
                            35537917 as core::ffi::c_int as fe_limb_t,
                            27978816 as core::ffi::c_int as fe_limb_t,
                            47649184 as core::ffi::c_int as fe_limb_t,
                            9219902 as core::ffi::c_int as fe_limb_t,
                            294711 as core::ffi::c_int as fe_limb_t,
                            15298639 as core::ffi::c_int as fe_limb_t,
                            2662509 as core::ffi::c_int as fe_limb_t,
                            17257359 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            65935918 as core::ffi::c_int as fe_limb_t,
                            25995736 as core::ffi::c_int as fe_limb_t,
                            62742093 as core::ffi::c_int as fe_limb_t,
                            29266687 as core::ffi::c_int as fe_limb_t,
                            45762450 as core::ffi::c_int as fe_limb_t,
                            25120105 as core::ffi::c_int as fe_limb_t,
                            32087528 as core::ffi::c_int as fe_limb_t,
                            32331655 as core::ffi::c_int as fe_limb_t,
                            32247247 as core::ffi::c_int as fe_limb_t,
                            19164571 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14312609 as core::ffi::c_int as fe_limb_t,
                            1221556 as core::ffi::c_int as fe_limb_t,
                            17395390 as core::ffi::c_int as fe_limb_t,
                            24854289 as core::ffi::c_int as fe_limb_t,
                            62163122 as core::ffi::c_int as fe_limb_t,
                            24869796 as core::ffi::c_int as fe_limb_t,
                            38911119 as core::ffi::c_int as fe_limb_t,
                            23916614 as core::ffi::c_int as fe_limb_t,
                            51081240 as core::ffi::c_int as fe_limb_t,
                            20175586 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65680039 as core::ffi::c_int as fe_limb_t,
                            23875441 as core::ffi::c_int as fe_limb_t,
                            57873182 as core::ffi::c_int as fe_limb_t,
                            6549686 as core::ffi::c_int as fe_limb_t,
                            59725795 as core::ffi::c_int as fe_limb_t,
                            33085767 as core::ffi::c_int as fe_limb_t,
                            23046501 as core::ffi::c_int as fe_limb_t,
                            9803137 as core::ffi::c_int as fe_limb_t,
                            17597934 as core::ffi::c_int as fe_limb_t,
                            2346211 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            18510781 as core::ffi::c_int as fe_limb_t,
                            15337574 as core::ffi::c_int as fe_limb_t,
                            26171504 as core::ffi::c_int as fe_limb_t,
                            981392 as core::ffi::c_int as fe_limb_t,
                            44867312 as core::ffi::c_int as fe_limb_t,
                            7827555 as core::ffi::c_int as fe_limb_t,
                            43617730 as core::ffi::c_int as fe_limb_t,
                            22231079 as core::ffi::c_int as fe_limb_t,
                            3059832 as core::ffi::c_int as fe_limb_t,
                            21771562 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            10141598 as core::ffi::c_int as fe_limb_t,
                            6082907 as core::ffi::c_int as fe_limb_t,
                            17829293 as core::ffi::c_int as fe_limb_t,
                            31606789 as core::ffi::c_int as fe_limb_t,
                            9830091 as core::ffi::c_int as fe_limb_t,
                            13613136 as core::ffi::c_int as fe_limb_t,
                            41552228 as core::ffi::c_int as fe_limb_t,
                            28009845 as core::ffi::c_int as fe_limb_t,
                            33606651 as core::ffi::c_int as fe_limb_t,
                            3592095 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            33114149 as core::ffi::c_int as fe_limb_t,
                            17665080 as core::ffi::c_int as fe_limb_t,
                            40583177 as core::ffi::c_int as fe_limb_t,
                            20211034 as core::ffi::c_int as fe_limb_t,
                            33076704 as core::ffi::c_int as fe_limb_t,
                            8716171 as core::ffi::c_int as fe_limb_t,
                            1151462 as core::ffi::c_int as fe_limb_t,
                            1521897 as core::ffi::c_int as fe_limb_t,
                            66126199 as core::ffi::c_int as fe_limb_t,
                            26716628 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            34169699 as core::ffi::c_int as fe_limb_t,
                            29298616 as core::ffi::c_int as fe_limb_t,
                            23947180 as core::ffi::c_int as fe_limb_t,
                            33230254 as core::ffi::c_int as fe_limb_t,
                            34035889 as core::ffi::c_int as fe_limb_t,
                            21248794 as core::ffi::c_int as fe_limb_t,
                            50471177 as core::ffi::c_int as fe_limb_t,
                            3891703 as core::ffi::c_int as fe_limb_t,
                            26353178 as core::ffi::c_int as fe_limb_t,
                            693168 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30374239 as core::ffi::c_int as fe_limb_t,
                            1595580 as core::ffi::c_int as fe_limb_t,
                            50224825 as core::ffi::c_int as fe_limb_t,
                            13186930 as core::ffi::c_int as fe_limb_t,
                            4600344 as core::ffi::c_int as fe_limb_t,
                            406904 as core::ffi::c_int as fe_limb_t,
                            9585294 as core::ffi::c_int as fe_limb_t,
                            33153764 as core::ffi::c_int as fe_limb_t,
                            31375463 as core::ffi::c_int as fe_limb_t,
                            14369965 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            52738210 as core::ffi::c_int as fe_limb_t,
                            25781902 as core::ffi::c_int as fe_limb_t,
                            1510300 as core::ffi::c_int as fe_limb_t,
                            6434173 as core::ffi::c_int as fe_limb_t,
                            48324075 as core::ffi::c_int as fe_limb_t,
                            27291703 as core::ffi::c_int as fe_limb_t,
                            32732229 as core::ffi::c_int as fe_limb_t,
                            20445593 as core::ffi::c_int as fe_limb_t,
                            17901440 as core::ffi::c_int as fe_limb_t,
                            16011505 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            18171223 as core::ffi::c_int as fe_limb_t,
                            21619806 as core::ffi::c_int as fe_limb_t,
                            54608461 as core::ffi::c_int as fe_limb_t,
                            15197121 as core::ffi::c_int as fe_limb_t,
                            56070717 as core::ffi::c_int as fe_limb_t,
                            18324396 as core::ffi::c_int as fe_limb_t,
                            47936623 as core::ffi::c_int as fe_limb_t,
                            17508055 as core::ffi::c_int as fe_limb_t,
                            8764034 as core::ffi::c_int as fe_limb_t,
                            12309598 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            5975889 as core::ffi::c_int as fe_limb_t,
                            28311244 as core::ffi::c_int as fe_limb_t,
                            47649501 as core::ffi::c_int as fe_limb_t,
                            23872684 as core::ffi::c_int as fe_limb_t,
                            55567586 as core::ffi::c_int as fe_limb_t,
                            14015781 as core::ffi::c_int as fe_limb_t,
                            43443107 as core::ffi::c_int as fe_limb_t,
                            1228318 as core::ffi::c_int as fe_limb_t,
                            17544096 as core::ffi::c_int as fe_limb_t,
                            22960650 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            5811932 as core::ffi::c_int as fe_limb_t,
                            31839139 as core::ffi::c_int as fe_limb_t,
                            3442886 as core::ffi::c_int as fe_limb_t,
                            31285122 as core::ffi::c_int as fe_limb_t,
                            48741515 as core::ffi::c_int as fe_limb_t,
                            25194890 as core::ffi::c_int as fe_limb_t,
                            49064820 as core::ffi::c_int as fe_limb_t,
                            18144304 as core::ffi::c_int as fe_limb_t,
                            61543482 as core::ffi::c_int as fe_limb_t,
                            12348899 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            35709185 as core::ffi::c_int as fe_limb_t,
                            11407554 as core::ffi::c_int as fe_limb_t,
                            25755363 as core::ffi::c_int as fe_limb_t,
                            6891399 as core::ffi::c_int as fe_limb_t,
                            63851926 as core::ffi::c_int as fe_limb_t,
                            14872273 as core::ffi::c_int as fe_limb_t,
                            42259511 as core::ffi::c_int as fe_limb_t,
                            8141294 as core::ffi::c_int as fe_limb_t,
                            56476330 as core::ffi::c_int as fe_limb_t,
                            32968952 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54433560 as core::ffi::c_int as fe_limb_t,
                            694025 as core::ffi::c_int as fe_limb_t,
                            62032719 as core::ffi::c_int as fe_limb_t,
                            13300343 as core::ffi::c_int as fe_limb_t,
                            14015258 as core::ffi::c_int as fe_limb_t,
                            19103038 as core::ffi::c_int as fe_limb_t,
                            57410191 as core::ffi::c_int as fe_limb_t,
                            22225381 as core::ffi::c_int as fe_limb_t,
                            30944592 as core::ffi::c_int as fe_limb_t,
                            1130208 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            8247747 as core::ffi::c_int as fe_limb_t,
                            26843490 as core::ffi::c_int as fe_limb_t,
                            40546482 as core::ffi::c_int as fe_limb_t,
                            25845122 as core::ffi::c_int as fe_limb_t,
                            52706924 as core::ffi::c_int as fe_limb_t,
                            18905521 as core::ffi::c_int as fe_limb_t,
                            4652151 as core::ffi::c_int as fe_limb_t,
                            2488540 as core::ffi::c_int as fe_limb_t,
                            23550156 as core::ffi::c_int as fe_limb_t,
                            33283200 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            17294297 as core::ffi::c_int as fe_limb_t,
                            29765994 as core::ffi::c_int as fe_limb_t,
                            7026747 as core::ffi::c_int as fe_limb_t,
                            15626851 as core::ffi::c_int as fe_limb_t,
                            22990044 as core::ffi::c_int as fe_limb_t,
                            113481 as core::ffi::c_int as fe_limb_t,
                            2267737 as core::ffi::c_int as fe_limb_t,
                            27646286 as core::ffi::c_int as fe_limb_t,
                            66700045 as core::ffi::c_int as fe_limb_t,
                            33416712 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16091066 as core::ffi::c_int as fe_limb_t,
                            17300506 as core::ffi::c_int as fe_limb_t,
                            18599251 as core::ffi::c_int as fe_limb_t,
                            7340678 as core::ffi::c_int as fe_limb_t,
                            2137637 as core::ffi::c_int as fe_limb_t,
                            32332775 as core::ffi::c_int as fe_limb_t,
                            63744702 as core::ffi::c_int as fe_limb_t,
                            14550935 as core::ffi::c_int as fe_limb_t,
                            3260525 as core::ffi::c_int as fe_limb_t,
                            26388161 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            62198760 as core::ffi::c_int as fe_limb_t,
                            20221544 as core::ffi::c_int as fe_limb_t,
                            18550886 as core::ffi::c_int as fe_limb_t,
                            10864893 as core::ffi::c_int as fe_limb_t,
                            50649539 as core::ffi::c_int as fe_limb_t,
                            26262835 as core::ffi::c_int as fe_limb_t,
                            44079994 as core::ffi::c_int as fe_limb_t,
                            20349526 as core::ffi::c_int as fe_limb_t,
                            54360141 as core::ffi::c_int as fe_limb_t,
                            2701325 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            58534169 as core::ffi::c_int as fe_limb_t,
                            16099414 as core::ffi::c_int as fe_limb_t,
                            4629974 as core::ffi::c_int as fe_limb_t,
                            17213908 as core::ffi::c_int as fe_limb_t,
                            46322650 as core::ffi::c_int as fe_limb_t,
                            27548999 as core::ffi::c_int as fe_limb_t,
                            57090500 as core::ffi::c_int as fe_limb_t,
                            9276970 as core::ffi::c_int as fe_limb_t,
                            11329923 as core::ffi::c_int as fe_limb_t,
                            1862132 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14763057 as core::ffi::c_int as fe_limb_t,
                            17650824 as core::ffi::c_int as fe_limb_t,
                            36190593 as core::ffi::c_int as fe_limb_t,
                            3689866 as core::ffi::c_int as fe_limb_t,
                            3511892 as core::ffi::c_int as fe_limb_t,
                            10313526 as core::ffi::c_int as fe_limb_t,
                            45157776 as core::ffi::c_int as fe_limb_t,
                            12219230 as core::ffi::c_int as fe_limb_t,
                            58070901 as core::ffi::c_int as fe_limb_t,
                            32614131 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            8894987 as core::ffi::c_int as fe_limb_t,
                            30108338 as core::ffi::c_int as fe_limb_t,
                            6150752 as core::ffi::c_int as fe_limb_t,
                            3013931 as core::ffi::c_int as fe_limb_t,
                            301220 as core::ffi::c_int as fe_limb_t,
                            15693451 as core::ffi::c_int as fe_limb_t,
                            35127648 as core::ffi::c_int as fe_limb_t,
                            30644714 as core::ffi::c_int as fe_limb_t,
                            51670695 as core::ffi::c_int as fe_limb_t,
                            11595569 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            15214943 as core::ffi::c_int as fe_limb_t,
                            3537601 as core::ffi::c_int as fe_limb_t,
                            40870142 as core::ffi::c_int as fe_limb_t,
                            19495559 as core::ffi::c_int as fe_limb_t,
                            4418656 as core::ffi::c_int as fe_limb_t,
                            18323671 as core::ffi::c_int as fe_limb_t,
                            13947275 as core::ffi::c_int as fe_limb_t,
                            10730794 as core::ffi::c_int as fe_limb_t,
                            53619402 as core::ffi::c_int as fe_limb_t,
                            29190761 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            64570558 as core::ffi::c_int as fe_limb_t,
                            7682792 as core::ffi::c_int as fe_limb_t,
                            32759013 as core::ffi::c_int as fe_limb_t,
                            263109 as core::ffi::c_int as fe_limb_t,
                            37124133 as core::ffi::c_int as fe_limb_t,
                            25598979 as core::ffi::c_int as fe_limb_t,
                            44776739 as core::ffi::c_int as fe_limb_t,
                            23365796 as core::ffi::c_int as fe_limb_t,
                            977107 as core::ffi::c_int as fe_limb_t,
                            699994 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54642373 as core::ffi::c_int as fe_limb_t,
                            4195083 as core::ffi::c_int as fe_limb_t,
                            57897332 as core::ffi::c_int as fe_limb_t,
                            550903 as core::ffi::c_int as fe_limb_t,
                            51543527 as core::ffi::c_int as fe_limb_t,
                            12917919 as core::ffi::c_int as fe_limb_t,
                            19118110 as core::ffi::c_int as fe_limb_t,
                            33114591 as core::ffi::c_int as fe_limb_t,
                            36574330 as core::ffi::c_int as fe_limb_t,
                            19216518 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            31788442 as core::ffi::c_int as fe_limb_t,
                            19046775 as core::ffi::c_int as fe_limb_t,
                            4799988 as core::ffi::c_int as fe_limb_t,
                            7372237 as core::ffi::c_int as fe_limb_t,
                            8808585 as core::ffi::c_int as fe_limb_t,
                            18806489 as core::ffi::c_int as fe_limb_t,
                            9408236 as core::ffi::c_int as fe_limb_t,
                            23502657 as core::ffi::c_int as fe_limb_t,
                            12493931 as core::ffi::c_int as fe_limb_t,
                            28145115 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41428258 as core::ffi::c_int as fe_limb_t,
                            5260743 as core::ffi::c_int as fe_limb_t,
                            47873055 as core::ffi::c_int as fe_limb_t,
                            27269961 as core::ffi::c_int as fe_limb_t,
                            63412921 as core::ffi::c_int as fe_limb_t,
                            16566086 as core::ffi::c_int as fe_limb_t,
                            27218280 as core::ffi::c_int as fe_limb_t,
                            2607121 as core::ffi::c_int as fe_limb_t,
                            29375955 as core::ffi::c_int as fe_limb_t,
                            6024730 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            842132 as core::ffi::c_int as fe_limb_t,
                            30759739 as core::ffi::c_int as fe_limb_t,
                            62345482 as core::ffi::c_int as fe_limb_t,
                            24831616 as core::ffi::c_int as fe_limb_t,
                            26332017 as core::ffi::c_int as fe_limb_t,
                            21148791 as core::ffi::c_int as fe_limb_t,
                            11831879 as core::ffi::c_int as fe_limb_t,
                            6985184 as core::ffi::c_int as fe_limb_t,
                            57168503 as core::ffi::c_int as fe_limb_t,
                            2854095 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            62261602 as core::ffi::c_int as fe_limb_t,
                            25585100 as core::ffi::c_int as fe_limb_t,
                            2516241 as core::ffi::c_int as fe_limb_t,
                            27706719 as core::ffi::c_int as fe_limb_t,
                            9695690 as core::ffi::c_int as fe_limb_t,
                            26333246 as core::ffi::c_int as fe_limb_t,
                            16512644 as core::ffi::c_int as fe_limb_t,
                            960770 as core::ffi::c_int as fe_limb_t,
                            12121869 as core::ffi::c_int as fe_limb_t,
                            16648078 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            51890212 as core::ffi::c_int as fe_limb_t,
                            14667095 as core::ffi::c_int as fe_limb_t,
                            53772635 as core::ffi::c_int as fe_limb_t,
                            2013716 as core::ffi::c_int as fe_limb_t,
                            30598287 as core::ffi::c_int as fe_limb_t,
                            33090295 as core::ffi::c_int as fe_limb_t,
                            35603941 as core::ffi::c_int as fe_limb_t,
                            25672367 as core::ffi::c_int as fe_limb_t,
                            20237805 as core::ffi::c_int as fe_limb_t,
                            2838411 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47820798 as core::ffi::c_int as fe_limb_t,
                            4453151 as core::ffi::c_int as fe_limb_t,
                            15298546 as core::ffi::c_int as fe_limb_t,
                            17376044 as core::ffi::c_int as fe_limb_t,
                            22115042 as core::ffi::c_int as fe_limb_t,
                            17581828 as core::ffi::c_int as fe_limb_t,
                            12544293 as core::ffi::c_int as fe_limb_t,
                            20083975 as core::ffi::c_int as fe_limb_t,
                            1068880 as core::ffi::c_int as fe_limb_t,
                            21054527 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57549981 as core::ffi::c_int as fe_limb_t,
                            17035596 as core::ffi::c_int as fe_limb_t,
                            33238497 as core::ffi::c_int as fe_limb_t,
                            13506958 as core::ffi::c_int as fe_limb_t,
                            30505848 as core::ffi::c_int as fe_limb_t,
                            32439836 as core::ffi::c_int as fe_limb_t,
                            58621956 as core::ffi::c_int as fe_limb_t,
                            30924378 as core::ffi::c_int as fe_limb_t,
                            12521377 as core::ffi::c_int as fe_limb_t,
                            4845654 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            38910324 as core::ffi::c_int as fe_limb_t,
                            10744107 as core::ffi::c_int as fe_limb_t,
                            64150484 as core::ffi::c_int as fe_limb_t,
                            10199663 as core::ffi::c_int as fe_limb_t,
                            7759311 as core::ffi::c_int as fe_limb_t,
                            20465832 as core::ffi::c_int as fe_limb_t,
                            3409347 as core::ffi::c_int as fe_limb_t,
                            32681032 as core::ffi::c_int as fe_limb_t,
                            60626557 as core::ffi::c_int as fe_limb_t,
                            20668561 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            43547042 as core::ffi::c_int as fe_limb_t,
                            6230155 as core::ffi::c_int as fe_limb_t,
                            46726851 as core::ffi::c_int as fe_limb_t,
                            10655313 as core::ffi::c_int as fe_limb_t,
                            43068279 as core::ffi::c_int as fe_limb_t,
                            21933259 as core::ffi::c_int as fe_limb_t,
                            10477733 as core::ffi::c_int as fe_limb_t,
                            32314216 as core::ffi::c_int as fe_limb_t,
                            63995636 as core::ffi::c_int as fe_limb_t,
                            13974497 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            12966261 as core::ffi::c_int as fe_limb_t,
                            15550616 as core::ffi::c_int as fe_limb_t,
                            35069916 as core::ffi::c_int as fe_limb_t,
                            31939085 as core::ffi::c_int as fe_limb_t,
                            21025979 as core::ffi::c_int as fe_limb_t,
                            32924988 as core::ffi::c_int as fe_limb_t,
                            5642324 as core::ffi::c_int as fe_limb_t,
                            7188737 as core::ffi::c_int as fe_limb_t,
                            18895762 as core::ffi::c_int as fe_limb_t,
                            12629579 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            14741879 as core::ffi::c_int as fe_limb_t,
                            18607545 as core::ffi::c_int as fe_limb_t,
                            22177207 as core::ffi::c_int as fe_limb_t,
                            21833195 as core::ffi::c_int as fe_limb_t,
                            1279740 as core::ffi::c_int as fe_limb_t,
                            8058600 as core::ffi::c_int as fe_limb_t,
                            11758140 as core::ffi::c_int as fe_limb_t,
                            789443 as core::ffi::c_int as fe_limb_t,
                            32195181 as core::ffi::c_int as fe_limb_t,
                            3895677 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10758205 as core::ffi::c_int as fe_limb_t,
                            15755439 as core::ffi::c_int as fe_limb_t,
                            62598914 as core::ffi::c_int as fe_limb_t,
                            9243697 as core::ffi::c_int as fe_limb_t,
                            62229442 as core::ffi::c_int as fe_limb_t,
                            6879878 as core::ffi::c_int as fe_limb_t,
                            64904289 as core::ffi::c_int as fe_limb_t,
                            29988312 as core::ffi::c_int as fe_limb_t,
                            58126794 as core::ffi::c_int as fe_limb_t,
                            4429646 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64654951 as core::ffi::c_int as fe_limb_t,
                            15725972 as core::ffi::c_int as fe_limb_t,
                            46672522 as core::ffi::c_int as fe_limb_t,
                            23143759 as core::ffi::c_int as fe_limb_t,
                            61304955 as core::ffi::c_int as fe_limb_t,
                            22514211 as core::ffi::c_int as fe_limb_t,
                            59972993 as core::ffi::c_int as fe_limb_t,
                            21911536 as core::ffi::c_int as fe_limb_t,
                            18047435 as core::ffi::c_int as fe_limb_t,
                            18272689 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41935844 as core::ffi::c_int as fe_limb_t,
                            22247266 as core::ffi::c_int as fe_limb_t,
                            29759955 as core::ffi::c_int as fe_limb_t,
                            11776784 as core::ffi::c_int as fe_limb_t,
                            44846481 as core::ffi::c_int as fe_limb_t,
                            17733976 as core::ffi::c_int as fe_limb_t,
                            10993113 as core::ffi::c_int as fe_limb_t,
                            20703595 as core::ffi::c_int as fe_limb_t,
                            49488162 as core::ffi::c_int as fe_limb_t,
                            24145963 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            21987233 as core::ffi::c_int as fe_limb_t,
                            700364 as core::ffi::c_int as fe_limb_t,
                            42603816 as core::ffi::c_int as fe_limb_t,
                            14972007 as core::ffi::c_int as fe_limb_t,
                            59334599 as core::ffi::c_int as fe_limb_t,
                            27836036 as core::ffi::c_int as fe_limb_t,
                            32155025 as core::ffi::c_int as fe_limb_t,
                            2581431 as core::ffi::c_int as fe_limb_t,
                            37149879 as core::ffi::c_int as fe_limb_t,
                            8773374 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            41540495 as core::ffi::c_int as fe_limb_t,
                            454462 as core::ffi::c_int as fe_limb_t,
                            53896929 as core::ffi::c_int as fe_limb_t,
                            16126714 as core::ffi::c_int as fe_limb_t,
                            25240068 as core::ffi::c_int as fe_limb_t,
                            8594567 as core::ffi::c_int as fe_limb_t,
                            20656846 as core::ffi::c_int as fe_limb_t,
                            12017935 as core::ffi::c_int as fe_limb_t,
                            59234475 as core::ffi::c_int as fe_limb_t,
                            19634276 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            6028163 as core::ffi::c_int as fe_limb_t,
                            6263078 as core::ffi::c_int as fe_limb_t,
                            36097058 as core::ffi::c_int as fe_limb_t,
                            22252721 as core::ffi::c_int as fe_limb_t,
                            66289944 as core::ffi::c_int as fe_limb_t,
                            2461771 as core::ffi::c_int as fe_limb_t,
                            35267690 as core::ffi::c_int as fe_limb_t,
                            28086389 as core::ffi::c_int as fe_limb_t,
                            65387075 as core::ffi::c_int as fe_limb_t,
                            30777706 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54829870 as core::ffi::c_int as fe_limb_t,
                            16624276 as core::ffi::c_int as fe_limb_t,
                            987579 as core::ffi::c_int as fe_limb_t,
                            27631834 as core::ffi::c_int as fe_limb_t,
                            32908202 as core::ffi::c_int as fe_limb_t,
                            1248608 as core::ffi::c_int as fe_limb_t,
                            7719845 as core::ffi::c_int as fe_limb_t,
                            29387734 as core::ffi::c_int as fe_limb_t,
                            28408819 as core::ffi::c_int as fe_limb_t,
                            6816612 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            56750770 as core::ffi::c_int as fe_limb_t,
                            25316602 as core::ffi::c_int as fe_limb_t,
                            19549650 as core::ffi::c_int as fe_limb_t,
                            21385210 as core::ffi::c_int as fe_limb_t,
                            22082622 as core::ffi::c_int as fe_limb_t,
                            16147817 as core::ffi::c_int as fe_limb_t,
                            20613181 as core::ffi::c_int as fe_limb_t,
                            13982702 as core::ffi::c_int as fe_limb_t,
                            56769294 as core::ffi::c_int as fe_limb_t,
                            5067942 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            36602878 as core::ffi::c_int as fe_limb_t,
                            29732664 as core::ffi::c_int as fe_limb_t,
                            12074680 as core::ffi::c_int as fe_limb_t,
                            13582412 as core::ffi::c_int as fe_limb_t,
                            47230892 as core::ffi::c_int as fe_limb_t,
                            2443950 as core::ffi::c_int as fe_limb_t,
                            47389578 as core::ffi::c_int as fe_limb_t,
                            12746131 as core::ffi::c_int as fe_limb_t,
                            5331210 as core::ffi::c_int as fe_limb_t,
                            23448488 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            30528792 as core::ffi::c_int as fe_limb_t,
                            3601899 as core::ffi::c_int as fe_limb_t,
                            65151774 as core::ffi::c_int as fe_limb_t,
                            4619784 as core::ffi::c_int as fe_limb_t,
                            39747042 as core::ffi::c_int as fe_limb_t,
                            18118043 as core::ffi::c_int as fe_limb_t,
                            24180792 as core::ffi::c_int as fe_limb_t,
                            20984038 as core::ffi::c_int as fe_limb_t,
                            27679907 as core::ffi::c_int as fe_limb_t,
                            31905504 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            9402385 as core::ffi::c_int as fe_limb_t,
                            19597367 as core::ffi::c_int as fe_limb_t,
                            32834042 as core::ffi::c_int as fe_limb_t,
                            10838634 as core::ffi::c_int as fe_limb_t,
                            40528714 as core::ffi::c_int as fe_limb_t,
                            20317236 as core::ffi::c_int as fe_limb_t,
                            26653273 as core::ffi::c_int as fe_limb_t,
                            24868867 as core::ffi::c_int as fe_limb_t,
                            22611443 as core::ffi::c_int as fe_limb_t,
                            20839026 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            22190590 as core::ffi::c_int as fe_limb_t,
                            1118029 as core::ffi::c_int as fe_limb_t,
                            22736441 as core::ffi::c_int as fe_limb_t,
                            15130463 as core::ffi::c_int as fe_limb_t,
                            36648172 as core::ffi::c_int as fe_limb_t,
                            27563110 as core::ffi::c_int as fe_limb_t,
                            19189624 as core::ffi::c_int as fe_limb_t,
                            28905490 as core::ffi::c_int as fe_limb_t,
                            4854858 as core::ffi::c_int as fe_limb_t,
                            6622139 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            58798126 as core::ffi::c_int as fe_limb_t,
                            30600981 as core::ffi::c_int as fe_limb_t,
                            58846284 as core::ffi::c_int as fe_limb_t,
                            30166382 as core::ffi::c_int as fe_limb_t,
                            56707132 as core::ffi::c_int as fe_limb_t,
                            33282502 as core::ffi::c_int as fe_limb_t,
                            13424425 as core::ffi::c_int as fe_limb_t,
                            29987205 as core::ffi::c_int as fe_limb_t,
                            26404408 as core::ffi::c_int as fe_limb_t,
                            13001963 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            35867026 as core::ffi::c_int as fe_limb_t,
                            18138731 as core::ffi::c_int as fe_limb_t,
                            64114613 as core::ffi::c_int as fe_limb_t,
                            8939345 as core::ffi::c_int as fe_limb_t,
                            11562230 as core::ffi::c_int as fe_limb_t,
                            20713762 as core::ffi::c_int as fe_limb_t,
                            41044498 as core::ffi::c_int as fe_limb_t,
                            21932711 as core::ffi::c_int as fe_limb_t,
                            51703708 as core::ffi::c_int as fe_limb_t,
                            11020692 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            1866042 as core::ffi::c_int as fe_limb_t,
                            25604943 as core::ffi::c_int as fe_limb_t,
                            59210214 as core::ffi::c_int as fe_limb_t,
                            23253421 as core::ffi::c_int as fe_limb_t,
                            12483314 as core::ffi::c_int as fe_limb_t,
                            13477547 as core::ffi::c_int as fe_limb_t,
                            3175636 as core::ffi::c_int as fe_limb_t,
                            21130269 as core::ffi::c_int as fe_limb_t,
                            28761761 as core::ffi::c_int as fe_limb_t,
                            1406734 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            66660290 as core::ffi::c_int as fe_limb_t,
                            31776765 as core::ffi::c_int as fe_limb_t,
                            13018550 as core::ffi::c_int as fe_limb_t,
                            3194501 as core::ffi::c_int as fe_limb_t,
                            57528444 as core::ffi::c_int as fe_limb_t,
                            22392694 as core::ffi::c_int as fe_limb_t,
                            24760584 as core::ffi::c_int as fe_limb_t,
                            29207344 as core::ffi::c_int as fe_limb_t,
                            25577410 as core::ffi::c_int as fe_limb_t,
                            20175752 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42818486 as core::ffi::c_int as fe_limb_t,
                            4759344 as core::ffi::c_int as fe_limb_t,
                            66418211 as core::ffi::c_int as fe_limb_t,
                            31701615 as core::ffi::c_int as fe_limb_t,
                            2066746 as core::ffi::c_int as fe_limb_t,
                            10693769 as core::ffi::c_int as fe_limb_t,
                            37513074 as core::ffi::c_int as fe_limb_t,
                            9884935 as core::ffi::c_int as fe_limb_t,
                            57739938 as core::ffi::c_int as fe_limb_t,
                            4745409 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            57967561 as core::ffi::c_int as fe_limb_t,
                            6049713 as core::ffi::c_int as fe_limb_t,
                            47577803 as core::ffi::c_int as fe_limb_t,
                            29213020 as core::ffi::c_int as fe_limb_t,
                            35848065 as core::ffi::c_int as fe_limb_t,
                            9944275 as core::ffi::c_int as fe_limb_t,
                            51646856 as core::ffi::c_int as fe_limb_t,
                            22242579 as core::ffi::c_int as fe_limb_t,
                            10931923 as core::ffi::c_int as fe_limb_t,
                            21622501 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            50547351 as core::ffi::c_int as fe_limb_t,
                            14112679 as core::ffi::c_int as fe_limb_t,
                            59096219 as core::ffi::c_int as fe_limb_t,
                            4817317 as core::ffi::c_int as fe_limb_t,
                            59068400 as core::ffi::c_int as fe_limb_t,
                            22139825 as core::ffi::c_int as fe_limb_t,
                            44255434 as core::ffi::c_int as fe_limb_t,
                            10856640 as core::ffi::c_int as fe_limb_t,
                            46638094 as core::ffi::c_int as fe_limb_t,
                            13434653 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            22759470 as core::ffi::c_int as fe_limb_t,
                            23480998 as core::ffi::c_int as fe_limb_t,
                            50342599 as core::ffi::c_int as fe_limb_t,
                            31683009 as core::ffi::c_int as fe_limb_t,
                            13637441 as core::ffi::c_int as fe_limb_t,
                            23386341 as core::ffi::c_int as fe_limb_t,
                            1765143 as core::ffi::c_int as fe_limb_t,
                            20900106 as core::ffi::c_int as fe_limb_t,
                            28445306 as core::ffi::c_int as fe_limb_t,
                            28189722 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            29875063 as core::ffi::c_int as fe_limb_t,
                            12493613 as core::ffi::c_int as fe_limb_t,
                            2795536 as core::ffi::c_int as fe_limb_t,
                            29768102 as core::ffi::c_int as fe_limb_t,
                            1710619 as core::ffi::c_int as fe_limb_t,
                            15181182 as core::ffi::c_int as fe_limb_t,
                            56913147 as core::ffi::c_int as fe_limb_t,
                            24765756 as core::ffi::c_int as fe_limb_t,
                            9074233 as core::ffi::c_int as fe_limb_t,
                            1167180 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40903181 as core::ffi::c_int as fe_limb_t,
                            11014232 as core::ffi::c_int as fe_limb_t,
                            57266213 as core::ffi::c_int as fe_limb_t,
                            30918946 as core::ffi::c_int as fe_limb_t,
                            40200743 as core::ffi::c_int as fe_limb_t,
                            7532293 as core::ffi::c_int as fe_limb_t,
                            48391976 as core::ffi::c_int as fe_limb_t,
                            24018933 as core::ffi::c_int as fe_limb_t,
                            3843902 as core::ffi::c_int as fe_limb_t,
                            9367684 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            56139269 as core::ffi::c_int as fe_limb_t,
                            27150720 as core::ffi::c_int as fe_limb_t,
                            9591133 as core::ffi::c_int as fe_limb_t,
                            9582310 as core::ffi::c_int as fe_limb_t,
                            11349256 as core::ffi::c_int as fe_limb_t,
                            108879 as core::ffi::c_int as fe_limb_t,
                            16235123 as core::ffi::c_int as fe_limb_t,
                            8601684 as core::ffi::c_int as fe_limb_t,
                            66969667 as core::ffi::c_int as fe_limb_t,
                            4242894 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            22092954 as core::ffi::c_int as fe_limb_t,
                            20363309 as core::ffi::c_int as fe_limb_t,
                            65066070 as core::ffi::c_int as fe_limb_t,
                            21585919 as core::ffi::c_int as fe_limb_t,
                            32186752 as core::ffi::c_int as fe_limb_t,
                            22037044 as core::ffi::c_int as fe_limb_t,
                            60534522 as core::ffi::c_int as fe_limb_t,
                            2470659 as core::ffi::c_int as fe_limb_t,
                            39691498 as core::ffi::c_int as fe_limb_t,
                            16625500 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56051142 as core::ffi::c_int as fe_limb_t,
                            3042015 as core::ffi::c_int as fe_limb_t,
                            13770083 as core::ffi::c_int as fe_limb_t,
                            24296510 as core::ffi::c_int as fe_limb_t,
                            584235 as core::ffi::c_int as fe_limb_t,
                            33009577 as core::ffi::c_int as fe_limb_t,
                            59338006 as core::ffi::c_int as fe_limb_t,
                            2602724 as core::ffi::c_int as fe_limb_t,
                            39757248 as core::ffi::c_int as fe_limb_t,
                            14247412 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            6314156 as core::ffi::c_int as fe_limb_t,
                            23289540 as core::ffi::c_int as fe_limb_t,
                            34336361 as core::ffi::c_int as fe_limb_t,
                            15957556 as core::ffi::c_int as fe_limb_t,
                            56951134 as core::ffi::c_int as fe_limb_t,
                            168749 as core::ffi::c_int as fe_limb_t,
                            58490057 as core::ffi::c_int as fe_limb_t,
                            14290060 as core::ffi::c_int as fe_limb_t,
                            27108877 as core::ffi::c_int as fe_limb_t,
                            32373552 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            58522267 as core::ffi::c_int as fe_limb_t,
                            26383465 as core::ffi::c_int as fe_limb_t,
                            13241781 as core::ffi::c_int as fe_limb_t,
                            10960156 as core::ffi::c_int as fe_limb_t,
                            34117849 as core::ffi::c_int as fe_limb_t,
                            19759835 as core::ffi::c_int as fe_limb_t,
                            33547975 as core::ffi::c_int as fe_limb_t,
                            22495543 as core::ffi::c_int as fe_limb_t,
                            39960412 as core::ffi::c_int as fe_limb_t,
                            981873 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            22833421 as core::ffi::c_int as fe_limb_t,
                            9293594 as core::ffi::c_int as fe_limb_t,
                            34459416 as core::ffi::c_int as fe_limb_t,
                            19935764 as core::ffi::c_int as fe_limb_t,
                            57971897 as core::ffi::c_int as fe_limb_t,
                            14756818 as core::ffi::c_int as fe_limb_t,
                            44180005 as core::ffi::c_int as fe_limb_t,
                            19583651 as core::ffi::c_int as fe_limb_t,
                            56629059 as core::ffi::c_int as fe_limb_t,
                            17356469 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            59340277 as core::ffi::c_int as fe_limb_t,
                            3326785 as core::ffi::c_int as fe_limb_t,
                            38997067 as core::ffi::c_int as fe_limb_t,
                            10783823 as core::ffi::c_int as fe_limb_t,
                            19178761 as core::ffi::c_int as fe_limb_t,
                            14905060 as core::ffi::c_int as fe_limb_t,
                            22680049 as core::ffi::c_int as fe_limb_t,
                            13906969 as core::ffi::c_int as fe_limb_t,
                            51175174 as core::ffi::c_int as fe_limb_t,
                            3797898 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            21721337 as core::ffi::c_int as fe_limb_t,
                            29341686 as core::ffi::c_int as fe_limb_t,
                            54902740 as core::ffi::c_int as fe_limb_t,
                            9310181 as core::ffi::c_int as fe_limb_t,
                            63226625 as core::ffi::c_int as fe_limb_t,
                            19901321 as core::ffi::c_int as fe_limb_t,
                            23740223 as core::ffi::c_int as fe_limb_t,
                            30845200 as core::ffi::c_int as fe_limb_t,
                            20491982 as core::ffi::c_int as fe_limb_t,
                            25512280 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            9209251 as core::ffi::c_int as fe_limb_t,
                            18419377 as core::ffi::c_int as fe_limb_t,
                            53852306 as core::ffi::c_int as fe_limb_t,
                            27386633 as core::ffi::c_int as fe_limb_t,
                            66377847 as core::ffi::c_int as fe_limb_t,
                            15289672 as core::ffi::c_int as fe_limb_t,
                            25947805 as core::ffi::c_int as fe_limb_t,
                            15286587 as core::ffi::c_int as fe_limb_t,
                            30997318 as core::ffi::c_int as fe_limb_t,
                            26851369 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            7392013 as core::ffi::c_int as fe_limb_t,
                            16618386 as core::ffi::c_int as fe_limb_t,
                            23946583 as core::ffi::c_int as fe_limb_t,
                            25514540 as core::ffi::c_int as fe_limb_t,
                            53843699 as core::ffi::c_int as fe_limb_t,
                            32020573 as core::ffi::c_int as fe_limb_t,
                            52911418 as core::ffi::c_int as fe_limb_t,
                            31232855 as core::ffi::c_int as fe_limb_t,
                            17649997 as core::ffi::c_int as fe_limb_t,
                            33304352 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            57807776 as core::ffi::c_int as fe_limb_t,
                            19360604 as core::ffi::c_int as fe_limb_t,
                            30609525 as core::ffi::c_int as fe_limb_t,
                            30504889 as core::ffi::c_int as fe_limb_t,
                            41933794 as core::ffi::c_int as fe_limb_t,
                            32270679 as core::ffi::c_int as fe_limb_t,
                            51867297 as core::ffi::c_int as fe_limb_t,
                            24028707 as core::ffi::c_int as fe_limb_t,
                            64875610 as core::ffi::c_int as fe_limb_t,
                            7662145 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            49550191 as core::ffi::c_int as fe_limb_t,
                            1763593 as core::ffi::c_int as fe_limb_t,
                            33994528 as core::ffi::c_int as fe_limb_t,
                            15908609 as core::ffi::c_int as fe_limb_t,
                            37067994 as core::ffi::c_int as fe_limb_t,
                            21380136 as core::ffi::c_int as fe_limb_t,
                            7335079 as core::ffi::c_int as fe_limb_t,
                            25082233 as core::ffi::c_int as fe_limb_t,
                            63934189 as core::ffi::c_int as fe_limb_t,
                            3440182 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            47219164 as core::ffi::c_int as fe_limb_t,
                            27577423 as core::ffi::c_int as fe_limb_t,
                            42997570 as core::ffi::c_int as fe_limb_t,
                            23865561 as core::ffi::c_int as fe_limb_t,
                            10799742 as core::ffi::c_int as fe_limb_t,
                            16982475 as core::ffi::c_int as fe_limb_t,
                            40449 as core::ffi::c_int as fe_limb_t,
                            29122597 as core::ffi::c_int as fe_limb_t,
                            4862399 as core::ffi::c_int as fe_limb_t,
                            1133 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            34252636 as core::ffi::c_int as fe_limb_t,
                            25680474 as core::ffi::c_int as fe_limb_t,
                            61686474 as core::ffi::c_int as fe_limb_t,
                            14860949 as core::ffi::c_int as fe_limb_t,
                            50789833 as core::ffi::c_int as fe_limb_t,
                            7956141 as core::ffi::c_int as fe_limb_t,
                            7258061 as core::ffi::c_int as fe_limb_t,
                            311861 as core::ffi::c_int as fe_limb_t,
                            36513873 as core::ffi::c_int as fe_limb_t,
                            26175010 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            63335436 as core::ffi::c_int as fe_limb_t,
                            31988495 as core::ffi::c_int as fe_limb_t,
                            28985339 as core::ffi::c_int as fe_limb_t,
                            7499440 as core::ffi::c_int as fe_limb_t,
                            24445838 as core::ffi::c_int as fe_limb_t,
                            9325937 as core::ffi::c_int as fe_limb_t,
                            29727763 as core::ffi::c_int as fe_limb_t,
                            16527196 as core::ffi::c_int as fe_limb_t,
                            18278453 as core::ffi::c_int as fe_limb_t,
                            15405622 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            62726958 as core::ffi::c_int as fe_limb_t,
                            8508651 as core::ffi::c_int as fe_limb_t,
                            47210498 as core::ffi::c_int as fe_limb_t,
                            29880007 as core::ffi::c_int as fe_limb_t,
                            61124410 as core::ffi::c_int as fe_limb_t,
                            15149969 as core::ffi::c_int as fe_limb_t,
                            53795266 as core::ffi::c_int as fe_limb_t,
                            843522 as core::ffi::c_int as fe_limb_t,
                            45233802 as core::ffi::c_int as fe_limb_t,
                            13626196 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            2281448 as core::ffi::c_int as fe_limb_t,
                            20067377 as core::ffi::c_int as fe_limb_t,
                            56193445 as core::ffi::c_int as fe_limb_t,
                            30944521 as core::ffi::c_int as fe_limb_t,
                            1879357 as core::ffi::c_int as fe_limb_t,
                            16164207 as core::ffi::c_int as fe_limb_t,
                            56324982 as core::ffi::c_int as fe_limb_t,
                            3953791 as core::ffi::c_int as fe_limb_t,
                            13340839 as core::ffi::c_int as fe_limb_t,
                            15928663 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            31727126 as core::ffi::c_int as fe_limb_t,
                            26374577 as core::ffi::c_int as fe_limb_t,
                            48671360 as core::ffi::c_int as fe_limb_t,
                            25270779 as core::ffi::c_int as fe_limb_t,
                            2875792 as core::ffi::c_int as fe_limb_t,
                            17164102 as core::ffi::c_int as fe_limb_t,
                            41838969 as core::ffi::c_int as fe_limb_t,
                            26539605 as core::ffi::c_int as fe_limb_t,
                            43656557 as core::ffi::c_int as fe_limb_t,
                            5964752 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            4100401 as core::ffi::c_int as fe_limb_t,
                            27594980 as core::ffi::c_int as fe_limb_t,
                            49929526 as core::ffi::c_int as fe_limb_t,
                            6017713 as core::ffi::c_int as fe_limb_t,
                            48403027 as core::ffi::c_int as fe_limb_t,
                            12227140 as core::ffi::c_int as fe_limb_t,
                            40424029 as core::ffi::c_int as fe_limb_t,
                            11344143 as core::ffi::c_int as fe_limb_t,
                            2538215 as core::ffi::c_int as fe_limb_t,
                            25983677 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            57675240 as core::ffi::c_int as fe_limb_t,
                            6123112 as core::ffi::c_int as fe_limb_t,
                            11159803 as core::ffi::c_int as fe_limb_t,
                            31397824 as core::ffi::c_int as fe_limb_t,
                            30016279 as core::ffi::c_int as fe_limb_t,
                            14966241 as core::ffi::c_int as fe_limb_t,
                            46633881 as core::ffi::c_int as fe_limb_t,
                            1485420 as core::ffi::c_int as fe_limb_t,
                            66479608 as core::ffi::c_int as fe_limb_t,
                            17595569 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40304287 as core::ffi::c_int as fe_limb_t,
                            4260918 as core::ffi::c_int as fe_limb_t,
                            11851389 as core::ffi::c_int as fe_limb_t,
                            9658551 as core::ffi::c_int as fe_limb_t,
                            35091757 as core::ffi::c_int as fe_limb_t,
                            16367491 as core::ffi::c_int as fe_limb_t,
                            46903439 as core::ffi::c_int as fe_limb_t,
                            20363143 as core::ffi::c_int as fe_limb_t,
                            11659921 as core::ffi::c_int as fe_limb_t,
                            22439314 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            26180377 as core::ffi::c_int as fe_limb_t,
                            10015009 as core::ffi::c_int as fe_limb_t,
                            36264640 as core::ffi::c_int as fe_limb_t,
                            24973138 as core::ffi::c_int as fe_limb_t,
                            5418196 as core::ffi::c_int as fe_limb_t,
                            9480663 as core::ffi::c_int as fe_limb_t,
                            2231568 as core::ffi::c_int as fe_limb_t,
                            23384352 as core::ffi::c_int as fe_limb_t,
                            33100371 as core::ffi::c_int as fe_limb_t,
                            32248261 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15121094 as core::ffi::c_int as fe_limb_t,
                            28352561 as core::ffi::c_int as fe_limb_t,
                            56718958 as core::ffi::c_int as fe_limb_t,
                            15427820 as core::ffi::c_int as fe_limb_t,
                            39598927 as core::ffi::c_int as fe_limb_t,
                            17561924 as core::ffi::c_int as fe_limb_t,
                            21670946 as core::ffi::c_int as fe_limb_t,
                            4486675 as core::ffi::c_int as fe_limb_t,
                            61177054 as core::ffi::c_int as fe_limb_t,
                            19088051 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            16166467 as core::ffi::c_int as fe_limb_t,
                            24070699 as core::ffi::c_int as fe_limb_t,
                            56004733 as core::ffi::c_int as fe_limb_t,
                            6023907 as core::ffi::c_int as fe_limb_t,
                            35182066 as core::ffi::c_int as fe_limb_t,
                            32189508 as core::ffi::c_int as fe_limb_t,
                            2340059 as core::ffi::c_int as fe_limb_t,
                            17299464 as core::ffi::c_int as fe_limb_t,
                            56373093 as core::ffi::c_int as fe_limb_t,
                            23514607 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28042865 as core::ffi::c_int as fe_limb_t,
                            29997343 as core::ffi::c_int as fe_limb_t,
                            54982337 as core::ffi::c_int as fe_limb_t,
                            12259705 as core::ffi::c_int as fe_limb_t,
                            63391366 as core::ffi::c_int as fe_limb_t,
                            26608532 as core::ffi::c_int as fe_limb_t,
                            6766452 as core::ffi::c_int as fe_limb_t,
                            24864833 as core::ffi::c_int as fe_limb_t,
                            18036435 as core::ffi::c_int as fe_limb_t,
                            5803270 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            66291264 as core::ffi::c_int as fe_limb_t,
                            6763911 as core::ffi::c_int as fe_limb_t,
                            11803561 as core::ffi::c_int as fe_limb_t,
                            1585585 as core::ffi::c_int as fe_limb_t,
                            10958447 as core::ffi::c_int as fe_limb_t,
                            30883267 as core::ffi::c_int as fe_limb_t,
                            23855390 as core::ffi::c_int as fe_limb_t,
                            4598332 as core::ffi::c_int as fe_limb_t,
                            60949433 as core::ffi::c_int as fe_limb_t,
                            19436993 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            36077558 as core::ffi::c_int as fe_limb_t,
                            19298237 as core::ffi::c_int as fe_limb_t,
                            17332028 as core::ffi::c_int as fe_limb_t,
                            31170912 as core::ffi::c_int as fe_limb_t,
                            31312681 as core::ffi::c_int as fe_limb_t,
                            27587249 as core::ffi::c_int as fe_limb_t,
                            696308 as core::ffi::c_int as fe_limb_t,
                            50292 as core::ffi::c_int as fe_limb_t,
                            47013125 as core::ffi::c_int as fe_limb_t,
                            11763583 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            66514282 as core::ffi::c_int as fe_limb_t,
                            31040148 as core::ffi::c_int as fe_limb_t,
                            34874710 as core::ffi::c_int as fe_limb_t,
                            12643979 as core::ffi::c_int as fe_limb_t,
                            12650761 as core::ffi::c_int as fe_limb_t,
                            14811489 as core::ffi::c_int as fe_limb_t,
                            665117 as core::ffi::c_int as fe_limb_t,
                            20940800 as core::ffi::c_int as fe_limb_t,
                            47335652 as core::ffi::c_int as fe_limb_t,
                            22840869 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30464590 as core::ffi::c_int as fe_limb_t,
                            22291560 as core::ffi::c_int as fe_limb_t,
                            62981387 as core::ffi::c_int as fe_limb_t,
                            20819953 as core::ffi::c_int as fe_limb_t,
                            19835326 as core::ffi::c_int as fe_limb_t,
                            26448819 as core::ffi::c_int as fe_limb_t,
                            42712688 as core::ffi::c_int as fe_limb_t,
                            2075772 as core::ffi::c_int as fe_limb_t,
                            50088707 as core::ffi::c_int as fe_limb_t,
                            992470 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18357166 as core::ffi::c_int as fe_limb_t,
                            26559999 as core::ffi::c_int as fe_limb_t,
                            7766381 as core::ffi::c_int as fe_limb_t,
                            16342475 as core::ffi::c_int as fe_limb_t,
                            37783946 as core::ffi::c_int as fe_limb_t,
                            411173 as core::ffi::c_int as fe_limb_t,
                            14578841 as core::ffi::c_int as fe_limb_t,
                            8080033 as core::ffi::c_int as fe_limb_t,
                            55534529 as core::ffi::c_int as fe_limb_t,
                            22952821 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19598397 as core::ffi::c_int as fe_limb_t,
                            10334610 as core::ffi::c_int as fe_limb_t,
                            12555054 as core::ffi::c_int as fe_limb_t,
                            2555664 as core::ffi::c_int as fe_limb_t,
                            18821899 as core::ffi::c_int as fe_limb_t,
                            23214652 as core::ffi::c_int as fe_limb_t,
                            21873262 as core::ffi::c_int as fe_limb_t,
                            16014234 as core::ffi::c_int as fe_limb_t,
                            26224780 as core::ffi::c_int as fe_limb_t,
                            16452269 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            36884939 as core::ffi::c_int as fe_limb_t,
                            5145195 as core::ffi::c_int as fe_limb_t,
                            5944548 as core::ffi::c_int as fe_limb_t,
                            16385966 as core::ffi::c_int as fe_limb_t,
                            3976735 as core::ffi::c_int as fe_limb_t,
                            2009897 as core::ffi::c_int as fe_limb_t,
                            55731060 as core::ffi::c_int as fe_limb_t,
                            25936245 as core::ffi::c_int as fe_limb_t,
                            46575034 as core::ffi::c_int as fe_limb_t,
                            3698649 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            14187449 as core::ffi::c_int as fe_limb_t,
                            3448569 as core::ffi::c_int as fe_limb_t,
                            56472628 as core::ffi::c_int as fe_limb_t,
                            22743496 as core::ffi::c_int as fe_limb_t,
                            44444983 as core::ffi::c_int as fe_limb_t,
                            30120835 as core::ffi::c_int as fe_limb_t,
                            7268409 as core::ffi::c_int as fe_limb_t,
                            22663988 as core::ffi::c_int as fe_limb_t,
                            27394300 as core::ffi::c_int as fe_limb_t,
                            12015369 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            19695742 as core::ffi::c_int as fe_limb_t,
                            16087646 as core::ffi::c_int as fe_limb_t,
                            28032085 as core::ffi::c_int as fe_limb_t,
                            12999827 as core::ffi::c_int as fe_limb_t,
                            6817792 as core::ffi::c_int as fe_limb_t,
                            11427614 as core::ffi::c_int as fe_limb_t,
                            20244189 as core::ffi::c_int as fe_limb_t,
                            32241655 as core::ffi::c_int as fe_limb_t,
                            53849736 as core::ffi::c_int as fe_limb_t,
                            30151970 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30860084 as core::ffi::c_int as fe_limb_t,
                            12735208 as core::ffi::c_int as fe_limb_t,
                            65220619 as core::ffi::c_int as fe_limb_t,
                            28854697 as core::ffi::c_int as fe_limb_t,
                            50133957 as core::ffi::c_int as fe_limb_t,
                            2256939 as core::ffi::c_int as fe_limb_t,
                            58942851 as core::ffi::c_int as fe_limb_t,
                            12298311 as core::ffi::c_int as fe_limb_t,
                            58558340 as core::ffi::c_int as fe_limb_t,
                            23160969 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            61389038 as core::ffi::c_int as fe_limb_t,
                            22309106 as core::ffi::c_int as fe_limb_t,
                            65198214 as core::ffi::c_int as fe_limb_t,
                            15569034 as core::ffi::c_int as fe_limb_t,
                            26642876 as core::ffi::c_int as fe_limb_t,
                            25966672 as core::ffi::c_int as fe_limb_t,
                            61319509 as core::ffi::c_int as fe_limb_t,
                            18435777 as core::ffi::c_int as fe_limb_t,
                            62132699 as core::ffi::c_int as fe_limb_t,
                            12651792 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64260450 as core::ffi::c_int as fe_limb_t,
                            9953420 as core::ffi::c_int as fe_limb_t,
                            11531313 as core::ffi::c_int as fe_limb_t,
                            28271553 as core::ffi::c_int as fe_limb_t,
                            26895122 as core::ffi::c_int as fe_limb_t,
                            20857343 as core::ffi::c_int as fe_limb_t,
                            53990043 as core::ffi::c_int as fe_limb_t,
                            17036529 as core::ffi::c_int as fe_limb_t,
                            9768697 as core::ffi::c_int as fe_limb_t,
                            31021214 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            42389405 as core::ffi::c_int as fe_limb_t,
                            1894650 as core::ffi::c_int as fe_limb_t,
                            66821166 as core::ffi::c_int as fe_limb_t,
                            28850346 as core::ffi::c_int as fe_limb_t,
                            15348718 as core::ffi::c_int as fe_limb_t,
                            25397902 as core::ffi::c_int as fe_limb_t,
                            32767512 as core::ffi::c_int as fe_limb_t,
                            12765450 as core::ffi::c_int as fe_limb_t,
                            4940095 as core::ffi::c_int as fe_limb_t,
                            10678226 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18860224 as core::ffi::c_int as fe_limb_t,
                            15980149 as core::ffi::c_int as fe_limb_t,
                            48121624 as core::ffi::c_int as fe_limb_t,
                            31991861 as core::ffi::c_int as fe_limb_t,
                            40875851 as core::ffi::c_int as fe_limb_t,
                            22482575 as core::ffi::c_int as fe_limb_t,
                            59264981 as core::ffi::c_int as fe_limb_t,
                            13944023 as core::ffi::c_int as fe_limb_t,
                            42736516 as core::ffi::c_int as fe_limb_t,
                            16582018 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            51604604 as core::ffi::c_int as fe_limb_t,
                            4970267 as core::ffi::c_int as fe_limb_t,
                            37215820 as core::ffi::c_int as fe_limb_t,
                            4175592 as core::ffi::c_int as fe_limb_t,
                            46115652 as core::ffi::c_int as fe_limb_t,
                            31354675 as core::ffi::c_int as fe_limb_t,
                            55404809 as core::ffi::c_int as fe_limb_t,
                            15444559 as core::ffi::c_int as fe_limb_t,
                            56105103 as core::ffi::c_int as fe_limb_t,
                            7989036 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31490433 as core::ffi::c_int as fe_limb_t,
                            5568061 as core::ffi::c_int as fe_limb_t,
                            64696061 as core::ffi::c_int as fe_limb_t,
                            2182382 as core::ffi::c_int as fe_limb_t,
                            34772017 as core::ffi::c_int as fe_limb_t,
                            4531685 as core::ffi::c_int as fe_limb_t,
                            35030595 as core::ffi::c_int as fe_limb_t,
                            6200205 as core::ffi::c_int as fe_limb_t,
                            47422751 as core::ffi::c_int as fe_limb_t,
                            18754260 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            49800177 as core::ffi::c_int as fe_limb_t,
                            17674491 as core::ffi::c_int as fe_limb_t,
                            35586086 as core::ffi::c_int as fe_limb_t,
                            33551600 as core::ffi::c_int as fe_limb_t,
                            34221481 as core::ffi::c_int as fe_limb_t,
                            16375548 as core::ffi::c_int as fe_limb_t,
                            8680158 as core::ffi::c_int as fe_limb_t,
                            17182719 as core::ffi::c_int as fe_limb_t,
                            28550067 as core::ffi::c_int as fe_limb_t,
                            26697300 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38981977 as core::ffi::c_int as fe_limb_t,
                            27866340 as core::ffi::c_int as fe_limb_t,
                            16837844 as core::ffi::c_int as fe_limb_t,
                            31733974 as core::ffi::c_int as fe_limb_t,
                            60258182 as core::ffi::c_int as fe_limb_t,
                            12700015 as core::ffi::c_int as fe_limb_t,
                            37068883 as core::ffi::c_int as fe_limb_t,
                            4364037 as core::ffi::c_int as fe_limb_t,
                            1155602 as core::ffi::c_int as fe_limb_t,
                            5988841 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            21890435 as core::ffi::c_int as fe_limb_t,
                            20281525 as core::ffi::c_int as fe_limb_t,
                            54484852 as core::ffi::c_int as fe_limb_t,
                            12154348 as core::ffi::c_int as fe_limb_t,
                            59276991 as core::ffi::c_int as fe_limb_t,
                            15300495 as core::ffi::c_int as fe_limb_t,
                            23148983 as core::ffi::c_int as fe_limb_t,
                            29083951 as core::ffi::c_int as fe_limb_t,
                            24618406 as core::ffi::c_int as fe_limb_t,
                            8283181 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            33972757 as core::ffi::c_int as fe_limb_t,
                            23041680 as core::ffi::c_int as fe_limb_t,
                            9975415 as core::ffi::c_int as fe_limb_t,
                            6841041 as core::ffi::c_int as fe_limb_t,
                            35549071 as core::ffi::c_int as fe_limb_t,
                            16356535 as core::ffi::c_int as fe_limb_t,
                            3070187 as core::ffi::c_int as fe_limb_t,
                            26528504 as core::ffi::c_int as fe_limb_t,
                            1466168 as core::ffi::c_int as fe_limb_t,
                            10740210 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            65599446 as core::ffi::c_int as fe_limb_t,
                            18066246 as core::ffi::c_int as fe_limb_t,
                            53605478 as core::ffi::c_int as fe_limb_t,
                            22898515 as core::ffi::c_int as fe_limb_t,
                            32799043 as core::ffi::c_int as fe_limb_t,
                            909394 as core::ffi::c_int as fe_limb_t,
                            53169961 as core::ffi::c_int as fe_limb_t,
                            27774712 as core::ffi::c_int as fe_limb_t,
                            34944214 as core::ffi::c_int as fe_limb_t,
                            18227391 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            3960804 as core::ffi::c_int as fe_limb_t,
                            19286629 as core::ffi::c_int as fe_limb_t,
                            39082773 as core::ffi::c_int as fe_limb_t,
                            17636380 as core::ffi::c_int as fe_limb_t,
                            47704005 as core::ffi::c_int as fe_limb_t,
                            13146867 as core::ffi::c_int as fe_limb_t,
                            15567327 as core::ffi::c_int as fe_limb_t,
                            951507 as core::ffi::c_int as fe_limb_t,
                            63848543 as core::ffi::c_int as fe_limb_t,
                            32980496 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            24740822 as core::ffi::c_int as fe_limb_t,
                            5052253 as core::ffi::c_int as fe_limb_t,
                            37014733 as core::ffi::c_int as fe_limb_t,
                            8961360 as core::ffi::c_int as fe_limb_t,
                            25877428 as core::ffi::c_int as fe_limb_t,
                            6165135 as core::ffi::c_int as fe_limb_t,
                            42740684 as core::ffi::c_int as fe_limb_t,
                            14397371 as core::ffi::c_int as fe_limb_t,
                            59728495 as core::ffi::c_int as fe_limb_t,
                            27410326 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38220480 as core::ffi::c_int as fe_limb_t,
                            3510802 as core::ffi::c_int as fe_limb_t,
                            39005586 as core::ffi::c_int as fe_limb_t,
                            32395953 as core::ffi::c_int as fe_limb_t,
                            55870735 as core::ffi::c_int as fe_limb_t,
                            22922977 as core::ffi::c_int as fe_limb_t,
                            51667400 as core::ffi::c_int as fe_limb_t,
                            19101303 as core::ffi::c_int as fe_limb_t,
                            65483377 as core::ffi::c_int as fe_limb_t,
                            27059617 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            793280 as core::ffi::c_int as fe_limb_t,
                            24323954 as core::ffi::c_int as fe_limb_t,
                            8836301 as core::ffi::c_int as fe_limb_t,
                            27318725 as core::ffi::c_int as fe_limb_t,
                            39747955 as core::ffi::c_int as fe_limb_t,
                            31184838 as core::ffi::c_int as fe_limb_t,
                            33152842 as core::ffi::c_int as fe_limb_t,
                            28669181 as core::ffi::c_int as fe_limb_t,
                            57202663 as core::ffi::c_int as fe_limb_t,
                            32932579 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            5666214 as core::ffi::c_int as fe_limb_t,
                            525582 as core::ffi::c_int as fe_limb_t,
                            20782575 as core::ffi::c_int as fe_limb_t,
                            25516013 as core::ffi::c_int as fe_limb_t,
                            42570364 as core::ffi::c_int as fe_limb_t,
                            14657739 as core::ffi::c_int as fe_limb_t,
                            16099374 as core::ffi::c_int as fe_limb_t,
                            1468826 as core::ffi::c_int as fe_limb_t,
                            60937436 as core::ffi::c_int as fe_limb_t,
                            18367850 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            62249590 as core::ffi::c_int as fe_limb_t,
                            29775088 as core::ffi::c_int as fe_limb_t,
                            64191105 as core::ffi::c_int as fe_limb_t,
                            26806412 as core::ffi::c_int as fe_limb_t,
                            7778749 as core::ffi::c_int as fe_limb_t,
                            11688288 as core::ffi::c_int as fe_limb_t,
                            36704511 as core::ffi::c_int as fe_limb_t,
                            23683193 as core::ffi::c_int as fe_limb_t,
                            65549940 as core::ffi::c_int as fe_limb_t,
                            23690785 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            10896313 as core::ffi::c_int as fe_limb_t,
                            25834728 as core::ffi::c_int as fe_limb_t,
                            824274 as core::ffi::c_int as fe_limb_t,
                            472601 as core::ffi::c_int as fe_limb_t,
                            47648556 as core::ffi::c_int as fe_limb_t,
                            3009586 as core::ffi::c_int as fe_limb_t,
                            25248958 as core::ffi::c_int as fe_limb_t,
                            14783338 as core::ffi::c_int as fe_limb_t,
                            36527388 as core::ffi::c_int as fe_limb_t,
                            17796587 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10566929 as core::ffi::c_int as fe_limb_t,
                            12612572 as core::ffi::c_int as fe_limb_t,
                            35164652 as core::ffi::c_int as fe_limb_t,
                            11118702 as core::ffi::c_int as fe_limb_t,
                            54475488 as core::ffi::c_int as fe_limb_t,
                            12362878 as core::ffi::c_int as fe_limb_t,
                            21752402 as core::ffi::c_int as fe_limb_t,
                            8822496 as core::ffi::c_int as fe_limb_t,
                            24003793 as core::ffi::c_int as fe_limb_t,
                            14264025 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            27713843 as core::ffi::c_int as fe_limb_t,
                            26198459 as core::ffi::c_int as fe_limb_t,
                            56100623 as core::ffi::c_int as fe_limb_t,
                            9227529 as core::ffi::c_int as fe_limb_t,
                            27050101 as core::ffi::c_int as fe_limb_t,
                            2504721 as core::ffi::c_int as fe_limb_t,
                            23886875 as core::ffi::c_int as fe_limb_t,
                            20436907 as core::ffi::c_int as fe_limb_t,
                            13958494 as core::ffi::c_int as fe_limb_t,
                            27821979 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43627235 as core::ffi::c_int as fe_limb_t,
                            4867225 as core::ffi::c_int as fe_limb_t,
                            39861736 as core::ffi::c_int as fe_limb_t,
                            3900520 as core::ffi::c_int as fe_limb_t,
                            29838369 as core::ffi::c_int as fe_limb_t,
                            25342141 as core::ffi::c_int as fe_limb_t,
                            35219464 as core::ffi::c_int as fe_limb_t,
                            23512650 as core::ffi::c_int as fe_limb_t,
                            7340520 as core::ffi::c_int as fe_limb_t,
                            18144364 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            4646495 as core::ffi::c_int as fe_limb_t,
                            25543308 as core::ffi::c_int as fe_limb_t,
                            44342840 as core::ffi::c_int as fe_limb_t,
                            22021777 as core::ffi::c_int as fe_limb_t,
                            23184552 as core::ffi::c_int as fe_limb_t,
                            8566613 as core::ffi::c_int as fe_limb_t,
                            31366726 as core::ffi::c_int as fe_limb_t,
                            32173371 as core::ffi::c_int as fe_limb_t,
                            52042079 as core::ffi::c_int as fe_limb_t,
                            23179239 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49838347 as core::ffi::c_int as fe_limb_t,
                            12723031 as core::ffi::c_int as fe_limb_t,
                            50115803 as core::ffi::c_int as fe_limb_t,
                            14878793 as core::ffi::c_int as fe_limb_t,
                            21619651 as core::ffi::c_int as fe_limb_t,
                            27356856 as core::ffi::c_int as fe_limb_t,
                            27584816 as core::ffi::c_int as fe_limb_t,
                            3093888 as core::ffi::c_int as fe_limb_t,
                            58265170 as core::ffi::c_int as fe_limb_t,
                            3849920 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            58043933 as core::ffi::c_int as fe_limb_t,
                            2103171 as core::ffi::c_int as fe_limb_t,
                            25561640 as core::ffi::c_int as fe_limb_t,
                            18428694 as core::ffi::c_int as fe_limb_t,
                            61869039 as core::ffi::c_int as fe_limb_t,
                            9582957 as core::ffi::c_int as fe_limb_t,
                            32477045 as core::ffi::c_int as fe_limb_t,
                            24536477 as core::ffi::c_int as fe_limb_t,
                            5002293 as core::ffi::c_int as fe_limb_t,
                            18004173 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            55051311 as core::ffi::c_int as fe_limb_t,
                            22376525 as core::ffi::c_int as fe_limb_t,
                            21115584 as core::ffi::c_int as fe_limb_t,
                            20189277 as core::ffi::c_int as fe_limb_t,
                            8808711 as core::ffi::c_int as fe_limb_t,
                            21523724 as core::ffi::c_int as fe_limb_t,
                            16489529 as core::ffi::c_int as fe_limb_t,
                            13378448 as core::ffi::c_int as fe_limb_t,
                            41263148 as core::ffi::c_int as fe_limb_t,
                            12741425 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            61162478 as core::ffi::c_int as fe_limb_t,
                            10645102 as core::ffi::c_int as fe_limb_t,
                            36197278 as core::ffi::c_int as fe_limb_t,
                            15390283 as core::ffi::c_int as fe_limb_t,
                            63821882 as core::ffi::c_int as fe_limb_t,
                            26435754 as core::ffi::c_int as fe_limb_t,
                            24306471 as core::ffi::c_int as fe_limb_t,
                            15852464 as core::ffi::c_int as fe_limb_t,
                            28834118 as core::ffi::c_int as fe_limb_t,
                            25908360 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            49773116 as core::ffi::c_int as fe_limb_t,
                            24447374 as core::ffi::c_int as fe_limb_t,
                            42577584 as core::ffi::c_int as fe_limb_t,
                            9434952 as core::ffi::c_int as fe_limb_t,
                            58636780 as core::ffi::c_int as fe_limb_t,
                            32971069 as core::ffi::c_int as fe_limb_t,
                            54018092 as core::ffi::c_int as fe_limb_t,
                            455840 as core::ffi::c_int as fe_limb_t,
                            20461858 as core::ffi::c_int as fe_limb_t,
                            5491305 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            13669229 as core::ffi::c_int as fe_limb_t,
                            17458950 as core::ffi::c_int as fe_limb_t,
                            54626889 as core::ffi::c_int as fe_limb_t,
                            23351392 as core::ffi::c_int as fe_limb_t,
                            52539093 as core::ffi::c_int as fe_limb_t,
                            21661233 as core::ffi::c_int as fe_limb_t,
                            42112877 as core::ffi::c_int as fe_limb_t,
                            11293806 as core::ffi::c_int as fe_limb_t,
                            38520660 as core::ffi::c_int as fe_limb_t,
                            24132599 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28497909 as core::ffi::c_int as fe_limb_t,
                            6272777 as core::ffi::c_int as fe_limb_t,
                            34085870 as core::ffi::c_int as fe_limb_t,
                            14470569 as core::ffi::c_int as fe_limb_t,
                            8906179 as core::ffi::c_int as fe_limb_t,
                            32328802 as core::ffi::c_int as fe_limb_t,
                            18504673 as core::ffi::c_int as fe_limb_t,
                            19389266 as core::ffi::c_int as fe_limb_t,
                            29867744 as core::ffi::c_int as fe_limb_t,
                            24758489 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50901822 as core::ffi::c_int as fe_limb_t,
                            13517195 as core::ffi::c_int as fe_limb_t,
                            39309234 as core::ffi::c_int as fe_limb_t,
                            19856633 as core::ffi::c_int as fe_limb_t,
                            24009063 as core::ffi::c_int as fe_limb_t,
                            27180541 as core::ffi::c_int as fe_limb_t,
                            60741263 as core::ffi::c_int as fe_limb_t,
                            20379039 as core::ffi::c_int as fe_limb_t,
                            22853428 as core::ffi::c_int as fe_limb_t,
                            29542421 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            24191359 as core::ffi::c_int as fe_limb_t,
                            16712145 as core::ffi::c_int as fe_limb_t,
                            53177067 as core::ffi::c_int as fe_limb_t,
                            15217830 as core::ffi::c_int as fe_limb_t,
                            14542237 as core::ffi::c_int as fe_limb_t,
                            1646131 as core::ffi::c_int as fe_limb_t,
                            18603514 as core::ffi::c_int as fe_limb_t,
                            22516545 as core::ffi::c_int as fe_limb_t,
                            12876622 as core::ffi::c_int as fe_limb_t,
                            31441985 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            17902668 as core::ffi::c_int as fe_limb_t,
                            4518229 as core::ffi::c_int as fe_limb_t,
                            66697162 as core::ffi::c_int as fe_limb_t,
                            30725184 as core::ffi::c_int as fe_limb_t,
                            26878216 as core::ffi::c_int as fe_limb_t,
                            5258055 as core::ffi::c_int as fe_limb_t,
                            54248111 as core::ffi::c_int as fe_limb_t,
                            608396 as core::ffi::c_int as fe_limb_t,
                            16031844 as core::ffi::c_int as fe_limb_t,
                            3723494 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            38476072 as core::ffi::c_int as fe_limb_t,
                            12763727 as core::ffi::c_int as fe_limb_t,
                            46662418 as core::ffi::c_int as fe_limb_t,
                            7577503 as core::ffi::c_int as fe_limb_t,
                            33001348 as core::ffi::c_int as fe_limb_t,
                            20536687 as core::ffi::c_int as fe_limb_t,
                            17558841 as core::ffi::c_int as fe_limb_t,
                            25681542 as core::ffi::c_int as fe_limb_t,
                            23896953 as core::ffi::c_int as fe_limb_t,
                            29240187 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            47103464 as core::ffi::c_int as fe_limb_t,
                            21542479 as core::ffi::c_int as fe_limb_t,
                            31520463 as core::ffi::c_int as fe_limb_t,
                            605201 as core::ffi::c_int as fe_limb_t,
                            2543521 as core::ffi::c_int as fe_limb_t,
                            5991821 as core::ffi::c_int as fe_limb_t,
                            64163800 as core::ffi::c_int as fe_limb_t,
                            7229063 as core::ffi::c_int as fe_limb_t,
                            57189218 as core::ffi::c_int as fe_limb_t,
                            24727572 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            28816026 as core::ffi::c_int as fe_limb_t,
                            298879 as core::ffi::c_int as fe_limb_t,
                            38943848 as core::ffi::c_int as fe_limb_t,
                            17633493 as core::ffi::c_int as fe_limb_t,
                            19000927 as core::ffi::c_int as fe_limb_t,
                            31888542 as core::ffi::c_int as fe_limb_t,
                            54428030 as core::ffi::c_int as fe_limb_t,
                            30605106 as core::ffi::c_int as fe_limb_t,
                            49057085 as core::ffi::c_int as fe_limb_t,
                            31471516 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16000882 as core::ffi::c_int as fe_limb_t,
                            33209536 as core::ffi::c_int as fe_limb_t,
                            3493091 as core::ffi::c_int as fe_limb_t,
                            22107234 as core::ffi::c_int as fe_limb_t,
                            37604268 as core::ffi::c_int as fe_limb_t,
                            20394642 as core::ffi::c_int as fe_limb_t,
                            12577739 as core::ffi::c_int as fe_limb_t,
                            16041268 as core::ffi::c_int as fe_limb_t,
                            47393624 as core::ffi::c_int as fe_limb_t,
                            7847706 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            10151868 as core::ffi::c_int as fe_limb_t,
                            10572098 as core::ffi::c_int as fe_limb_t,
                            27312476 as core::ffi::c_int as fe_limb_t,
                            7922682 as core::ffi::c_int as fe_limb_t,
                            14825339 as core::ffi::c_int as fe_limb_t,
                            4723128 as core::ffi::c_int as fe_limb_t,
                            34252933 as core::ffi::c_int as fe_limb_t,
                            27035413 as core::ffi::c_int as fe_limb_t,
                            57088296 as core::ffi::c_int as fe_limb_t,
                            3852847 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            55678375 as core::ffi::c_int as fe_limb_t,
                            15697595 as core::ffi::c_int as fe_limb_t,
                            45987307 as core::ffi::c_int as fe_limb_t,
                            29133784 as core::ffi::c_int as fe_limb_t,
                            5386313 as core::ffi::c_int as fe_limb_t,
                            15063598 as core::ffi::c_int as fe_limb_t,
                            16514493 as core::ffi::c_int as fe_limb_t,
                            17622322 as core::ffi::c_int as fe_limb_t,
                            29330898 as core::ffi::c_int as fe_limb_t,
                            18478208 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41609129 as core::ffi::c_int as fe_limb_t,
                            29175637 as core::ffi::c_int as fe_limb_t,
                            51885955 as core::ffi::c_int as fe_limb_t,
                            26653220 as core::ffi::c_int as fe_limb_t,
                            16615730 as core::ffi::c_int as fe_limb_t,
                            2051784 as core::ffi::c_int as fe_limb_t,
                            3303702 as core::ffi::c_int as fe_limb_t,
                            15490 as core::ffi::c_int as fe_limb_t,
                            39560068 as core::ffi::c_int as fe_limb_t,
                            12314390 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            15683501 as core::ffi::c_int as fe_limb_t,
                            27551389 as core::ffi::c_int as fe_limb_t,
                            18109119 as core::ffi::c_int as fe_limb_t,
                            23573784 as core::ffi::c_int as fe_limb_t,
                            15337967 as core::ffi::c_int as fe_limb_t,
                            27556609 as core::ffi::c_int as fe_limb_t,
                            50391428 as core::ffi::c_int as fe_limb_t,
                            15921865 as core::ffi::c_int as fe_limb_t,
                            16103996 as core::ffi::c_int as fe_limb_t,
                            29823217 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43939021 as core::ffi::c_int as fe_limb_t,
                            22773182 as core::ffi::c_int as fe_limb_t,
                            13588191 as core::ffi::c_int as fe_limb_t,
                            31925625 as core::ffi::c_int as fe_limb_t,
                            63310306 as core::ffi::c_int as fe_limb_t,
                            32479502 as core::ffi::c_int as fe_limb_t,
                            47835256 as core::ffi::c_int as fe_limb_t,
                            5402698 as core::ffi::c_int as fe_limb_t,
                            37293151 as core::ffi::c_int as fe_limb_t,
                            23713330 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            23190676 as core::ffi::c_int as fe_limb_t,
                            2384583 as core::ffi::c_int as fe_limb_t,
                            34394524 as core::ffi::c_int as fe_limb_t,
                            3462153 as core::ffi::c_int as fe_limb_t,
                            37205209 as core::ffi::c_int as fe_limb_t,
                            32025299 as core::ffi::c_int as fe_limb_t,
                            55842007 as core::ffi::c_int as fe_limb_t,
                            8911516 as core::ffi::c_int as fe_limb_t,
                            41903005 as core::ffi::c_int as fe_limb_t,
                            2739712 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            21374101 as core::ffi::c_int as fe_limb_t,
                            30000182 as core::ffi::c_int as fe_limb_t,
                            33584214 as core::ffi::c_int as fe_limb_t,
                            9874410 as core::ffi::c_int as fe_limb_t,
                            15377179 as core::ffi::c_int as fe_limb_t,
                            11831242 as core::ffi::c_int as fe_limb_t,
                            33578960 as core::ffi::c_int as fe_limb_t,
                            6134906 as core::ffi::c_int as fe_limb_t,
                            4931255 as core::ffi::c_int as fe_limb_t,
                            11987849 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            67101132 as core::ffi::c_int as fe_limb_t,
                            30575573 as core::ffi::c_int as fe_limb_t,
                            50885377 as core::ffi::c_int as fe_limb_t,
                            7277596 as core::ffi::c_int as fe_limb_t,
                            105524 as core::ffi::c_int as fe_limb_t,
                            33232381 as core::ffi::c_int as fe_limb_t,
                            35628324 as core::ffi::c_int as fe_limb_t,
                            13861387 as core::ffi::c_int as fe_limb_t,
                            37032554 as core::ffi::c_int as fe_limb_t,
                            10117929 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            37607694 as core::ffi::c_int as fe_limb_t,
                            22809559 as core::ffi::c_int as fe_limb_t,
                            40945095 as core::ffi::c_int as fe_limb_t,
                            13051538 as core::ffi::c_int as fe_limb_t,
                            41483300 as core::ffi::c_int as fe_limb_t,
                            5089642 as core::ffi::c_int as fe_limb_t,
                            60783361 as core::ffi::c_int as fe_limb_t,
                            6704078 as core::ffi::c_int as fe_limb_t,
                            12890019 as core::ffi::c_int as fe_limb_t,
                            15728940 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            45136504 as core::ffi::c_int as fe_limb_t,
                            21783052 as core::ffi::c_int as fe_limb_t,
                            66157804 as core::ffi::c_int as fe_limb_t,
                            29135591 as core::ffi::c_int as fe_limb_t,
                            14704839 as core::ffi::c_int as fe_limb_t,
                            2695116 as core::ffi::c_int as fe_limb_t,
                            903376 as core::ffi::c_int as fe_limb_t,
                            23126293 as core::ffi::c_int as fe_limb_t,
                            12885166 as core::ffi::c_int as fe_limb_t,
                            8311031 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            49592363 as core::ffi::c_int as fe_limb_t,
                            5352193 as core::ffi::c_int as fe_limb_t,
                            10384213 as core::ffi::c_int as fe_limb_t,
                            19742774 as core::ffi::c_int as fe_limb_t,
                            7506450 as core::ffi::c_int as fe_limb_t,
                            13453191 as core::ffi::c_int as fe_limb_t,
                            26423267 as core::ffi::c_int as fe_limb_t,
                            4384730 as core::ffi::c_int as fe_limb_t,
                            1888765 as core::ffi::c_int as fe_limb_t,
                            28119028 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41291507 as core::ffi::c_int as fe_limb_t,
                            30447119 as core::ffi::c_int as fe_limb_t,
                            53614264 as core::ffi::c_int as fe_limb_t,
                            30371925 as core::ffi::c_int as fe_limb_t,
                            30896458 as core::ffi::c_int as fe_limb_t,
                            19632703 as core::ffi::c_int as fe_limb_t,
                            34857219 as core::ffi::c_int as fe_limb_t,
                            20846562 as core::ffi::c_int as fe_limb_t,
                            47644429 as core::ffi::c_int as fe_limb_t,
                            30214188 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            43500868 as core::ffi::c_int as fe_limb_t,
                            30888657 as core::ffi::c_int as fe_limb_t,
                            66582772 as core::ffi::c_int as fe_limb_t,
                            4651135 as core::ffi::c_int as fe_limb_t,
                            5765089 as core::ffi::c_int as fe_limb_t,
                            4618330 as core::ffi::c_int as fe_limb_t,
                            6092245 as core::ffi::c_int as fe_limb_t,
                            14845197 as core::ffi::c_int as fe_limb_t,
                            17151279 as core::ffi::c_int as fe_limb_t,
                            23700316 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42278406 as core::ffi::c_int as fe_limb_t,
                            20820711 as core::ffi::c_int as fe_limb_t,
                            51942885 as core::ffi::c_int as fe_limb_t,
                            10367249 as core::ffi::c_int as fe_limb_t,
                            37577956 as core::ffi::c_int as fe_limb_t,
                            33289075 as core::ffi::c_int as fe_limb_t,
                            22825804 as core::ffi::c_int as fe_limb_t,
                            26467153 as core::ffi::c_int as fe_limb_t,
                            50242379 as core::ffi::c_int as fe_limb_t,
                            16176524 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43525589 as core::ffi::c_int as fe_limb_t,
                            6564960 as core::ffi::c_int as fe_limb_t,
                            20063689 as core::ffi::c_int as fe_limb_t,
                            3798228 as core::ffi::c_int as fe_limb_t,
                            62368686 as core::ffi::c_int as fe_limb_t,
                            7359224 as core::ffi::c_int as fe_limb_t,
                            2006182 as core::ffi::c_int as fe_limb_t,
                            23191006 as core::ffi::c_int as fe_limb_t,
                            38362610 as core::ffi::c_int as fe_limb_t,
                            23356922 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56482264 as core::ffi::c_int as fe_limb_t,
                            29068029 as core::ffi::c_int as fe_limb_t,
                            53788301 as core::ffi::c_int as fe_limb_t,
                            28429114 as core::ffi::c_int as fe_limb_t,
                            3432135 as core::ffi::c_int as fe_limb_t,
                            27161203 as core::ffi::c_int as fe_limb_t,
                            23632036 as core::ffi::c_int as fe_limb_t,
                            31613822 as core::ffi::c_int as fe_limb_t,
                            32808309 as core::ffi::c_int as fe_limb_t,
                            1099883 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            15030958 as core::ffi::c_int as fe_limb_t,
                            5768825 as core::ffi::c_int as fe_limb_t,
                            39657628 as core::ffi::c_int as fe_limb_t,
                            30667132 as core::ffi::c_int as fe_limb_t,
                            60681485 as core::ffi::c_int as fe_limb_t,
                            18193060 as core::ffi::c_int as fe_limb_t,
                            51830967 as core::ffi::c_int as fe_limb_t,
                            26745081 as core::ffi::c_int as fe_limb_t,
                            2051440 as core::ffi::c_int as fe_limb_t,
                            18328567 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            63746541 as core::ffi::c_int as fe_limb_t,
                            26315059 as core::ffi::c_int as fe_limb_t,
                            7517889 as core::ffi::c_int as fe_limb_t,
                            9824992 as core::ffi::c_int as fe_limb_t,
                            23555850 as core::ffi::c_int as fe_limb_t,
                            295369 as core::ffi::c_int as fe_limb_t,
                            5148398 as core::ffi::c_int as fe_limb_t,
                            19400244 as core::ffi::c_int as fe_limb_t,
                            44422509 as core::ffi::c_int as fe_limb_t,
                            16633659 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            4577067 as core::ffi::c_int as fe_limb_t,
                            16802144 as core::ffi::c_int as fe_limb_t,
                            13249840 as core::ffi::c_int as fe_limb_t,
                            18250104 as core::ffi::c_int as fe_limb_t,
                            19958762 as core::ffi::c_int as fe_limb_t,
                            19017158 as core::ffi::c_int as fe_limb_t,
                            18559669 as core::ffi::c_int as fe_limb_t,
                            22794883 as core::ffi::c_int as fe_limb_t,
                            8402477 as core::ffi::c_int as fe_limb_t,
                            23690159 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            38702534 as core::ffi::c_int as fe_limb_t,
                            32502850 as core::ffi::c_int as fe_limb_t,
                            40318708 as core::ffi::c_int as fe_limb_t,
                            32646733 as core::ffi::c_int as fe_limb_t,
                            49896449 as core::ffi::c_int as fe_limb_t,
                            22523642 as core::ffi::c_int as fe_limb_t,
                            9453450 as core::ffi::c_int as fe_limb_t,
                            18574360 as core::ffi::c_int as fe_limb_t,
                            17983009 as core::ffi::c_int as fe_limb_t,
                            9967138 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41346370 as core::ffi::c_int as fe_limb_t,
                            6524721 as core::ffi::c_int as fe_limb_t,
                            26585488 as core::ffi::c_int as fe_limb_t,
                            9969270 as core::ffi::c_int as fe_limb_t,
                            24709298 as core::ffi::c_int as fe_limb_t,
                            1220360 as core::ffi::c_int as fe_limb_t,
                            65430874 as core::ffi::c_int as fe_limb_t,
                            7806336 as core::ffi::c_int as fe_limb_t,
                            17507396 as core::ffi::c_int as fe_limb_t,
                            3651560 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            56688388 as core::ffi::c_int as fe_limb_t,
                            29436320 as core::ffi::c_int as fe_limb_t,
                            14584638 as core::ffi::c_int as fe_limb_t,
                            15971087 as core::ffi::c_int as fe_limb_t,
                            51340543 as core::ffi::c_int as fe_limb_t,
                            8861009 as core::ffi::c_int as fe_limb_t,
                            26556809 as core::ffi::c_int as fe_limb_t,
                            27979875 as core::ffi::c_int as fe_limb_t,
                            48555541 as core::ffi::c_int as fe_limb_t,
                            22197296 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            2839082 as core::ffi::c_int as fe_limb_t,
                            14284142 as core::ffi::c_int as fe_limb_t,
                            4029895 as core::ffi::c_int as fe_limb_t,
                            3472686 as core::ffi::c_int as fe_limb_t,
                            14402957 as core::ffi::c_int as fe_limb_t,
                            12689363 as core::ffi::c_int as fe_limb_t,
                            40466743 as core::ffi::c_int as fe_limb_t,
                            8459446 as core::ffi::c_int as fe_limb_t,
                            61503401 as core::ffi::c_int as fe_limb_t,
                            25932490 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62269556 as core::ffi::c_int as fe_limb_t,
                            30018987 as core::ffi::c_int as fe_limb_t,
                            9744960 as core::ffi::c_int as fe_limb_t,
                            2871048 as core::ffi::c_int as fe_limb_t,
                            25113978 as core::ffi::c_int as fe_limb_t,
                            3187018 as core::ffi::c_int as fe_limb_t,
                            41998051 as core::ffi::c_int as fe_limb_t,
                            32705365 as core::ffi::c_int as fe_limb_t,
                            17258083 as core::ffi::c_int as fe_limb_t,
                            25576693 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18164541 as core::ffi::c_int as fe_limb_t,
                            22959256 as core::ffi::c_int as fe_limb_t,
                            49953981 as core::ffi::c_int as fe_limb_t,
                            32012014 as core::ffi::c_int as fe_limb_t,
                            19237077 as core::ffi::c_int as fe_limb_t,
                            23809137 as core::ffi::c_int as fe_limb_t,
                            23357532 as core::ffi::c_int as fe_limb_t,
                            18337424 as core::ffi::c_int as fe_limb_t,
                            26908269 as core::ffi::c_int as fe_limb_t,
                            12150756 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            36843994 as core::ffi::c_int as fe_limb_t,
                            25906566 as core::ffi::c_int as fe_limb_t,
                            5112248 as core::ffi::c_int as fe_limb_t,
                            26517760 as core::ffi::c_int as fe_limb_t,
                            65609056 as core::ffi::c_int as fe_limb_t,
                            26580174 as core::ffi::c_int as fe_limb_t,
                            43167 as core::ffi::c_int as fe_limb_t,
                            28016731 as core::ffi::c_int as fe_limb_t,
                            34806789 as core::ffi::c_int as fe_limb_t,
                            16215818 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            60209940 as core::ffi::c_int as fe_limb_t,
                            9824393 as core::ffi::c_int as fe_limb_t,
                            54804085 as core::ffi::c_int as fe_limb_t,
                            29153342 as core::ffi::c_int as fe_limb_t,
                            35711722 as core::ffi::c_int as fe_limb_t,
                            27277596 as core::ffi::c_int as fe_limb_t,
                            32574488 as core::ffi::c_int as fe_limb_t,
                            12532905 as core::ffi::c_int as fe_limb_t,
                            59605792 as core::ffi::c_int as fe_limb_t,
                            24879084 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            39765323 as core::ffi::c_int as fe_limb_t,
                            17038963 as core::ffi::c_int as fe_limb_t,
                            39957339 as core::ffi::c_int as fe_limb_t,
                            22831480 as core::ffi::c_int as fe_limb_t,
                            946345 as core::ffi::c_int as fe_limb_t,
                            16291093 as core::ffi::c_int as fe_limb_t,
                            254968 as core::ffi::c_int as fe_limb_t,
                            7168080 as core::ffi::c_int as fe_limb_t,
                            21676107 as core::ffi::c_int as fe_limb_t,
                            31611404 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21260942 as core::ffi::c_int as fe_limb_t,
                            25129680 as core::ffi::c_int as fe_limb_t,
                            50276977 as core::ffi::c_int as fe_limb_t,
                            21633609 as core::ffi::c_int as fe_limb_t,
                            43430902 as core::ffi::c_int as fe_limb_t,
                            3968120 as core::ffi::c_int as fe_limb_t,
                            63456915 as core::ffi::c_int as fe_limb_t,
                            27338965 as core::ffi::c_int as fe_limb_t,
                            63552672 as core::ffi::c_int as fe_limb_t,
                            25641356 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16544735 as core::ffi::c_int as fe_limb_t,
                            13250366 as core::ffi::c_int as fe_limb_t,
                            50304436 as core::ffi::c_int as fe_limb_t,
                            15546241 as core::ffi::c_int as fe_limb_t,
                            62525861 as core::ffi::c_int as fe_limb_t,
                            12757257 as core::ffi::c_int as fe_limb_t,
                            64646556 as core::ffi::c_int as fe_limb_t,
                            24874095 as core::ffi::c_int as fe_limb_t,
                            48201831 as core::ffi::c_int as fe_limb_t,
                            23891632 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            64693606 as core::ffi::c_int as fe_limb_t,
                            17976703 as core::ffi::c_int as fe_limb_t,
                            18312302 as core::ffi::c_int as fe_limb_t,
                            4964443 as core::ffi::c_int as fe_limb_t,
                            51836334 as core::ffi::c_int as fe_limb_t,
                            20900867 as core::ffi::c_int as fe_limb_t,
                            26820650 as core::ffi::c_int as fe_limb_t,
                            16690659 as core::ffi::c_int as fe_limb_t,
                            25459437 as core::ffi::c_int as fe_limb_t,
                            28989823 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            41964155 as core::ffi::c_int as fe_limb_t,
                            11425019 as core::ffi::c_int as fe_limb_t,
                            28423002 as core::ffi::c_int as fe_limb_t,
                            22533875 as core::ffi::c_int as fe_limb_t,
                            60963942 as core::ffi::c_int as fe_limb_t,
                            17728207 as core::ffi::c_int as fe_limb_t,
                            9142794 as core::ffi::c_int as fe_limb_t,
                            31162830 as core::ffi::c_int as fe_limb_t,
                            60676445 as core::ffi::c_int as fe_limb_t,
                            31909614 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            44004212 as core::ffi::c_int as fe_limb_t,
                            6253475 as core::ffi::c_int as fe_limb_t,
                            16964147 as core::ffi::c_int as fe_limb_t,
                            29785560 as core::ffi::c_int as fe_limb_t,
                            41994891 as core::ffi::c_int as fe_limb_t,
                            21257994 as core::ffi::c_int as fe_limb_t,
                            39651638 as core::ffi::c_int as fe_limb_t,
                            17209773 as core::ffi::c_int as fe_limb_t,
                            6335691 as core::ffi::c_int as fe_limb_t,
                            7249989 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            36775618 as core::ffi::c_int as fe_limb_t,
                            13979674 as core::ffi::c_int as fe_limb_t,
                            7503222 as core::ffi::c_int as fe_limb_t,
                            21186118 as core::ffi::c_int as fe_limb_t,
                            55152142 as core::ffi::c_int as fe_limb_t,
                            28932738 as core::ffi::c_int as fe_limb_t,
                            36836594 as core::ffi::c_int as fe_limb_t,
                            2682241 as core::ffi::c_int as fe_limb_t,
                            25993170 as core::ffi::c_int as fe_limb_t,
                            21075909 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            4364628 as core::ffi::c_int as fe_limb_t,
                            5930691 as core::ffi::c_int as fe_limb_t,
                            32304656 as core::ffi::c_int as fe_limb_t,
                            23509878 as core::ffi::c_int as fe_limb_t,
                            59054082 as core::ffi::c_int as fe_limb_t,
                            15091130 as core::ffi::c_int as fe_limb_t,
                            22857016 as core::ffi::c_int as fe_limb_t,
                            22955477 as core::ffi::c_int as fe_limb_t,
                            31820367 as core::ffi::c_int as fe_limb_t,
                            15075278 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31879134 as core::ffi::c_int as fe_limb_t,
                            24635739 as core::ffi::c_int as fe_limb_t,
                            17258760 as core::ffi::c_int as fe_limb_t,
                            90626 as core::ffi::c_int as fe_limb_t,
                            59067028 as core::ffi::c_int as fe_limb_t,
                            28636722 as core::ffi::c_int as fe_limb_t,
                            24162787 as core::ffi::c_int as fe_limb_t,
                            23903546 as core::ffi::c_int as fe_limb_t,
                            49138625 as core::ffi::c_int as fe_limb_t,
                            12833044 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            19073683 as core::ffi::c_int as fe_limb_t,
                            14851414 as core::ffi::c_int as fe_limb_t,
                            42705695 as core::ffi::c_int as fe_limb_t,
                            21694263 as core::ffi::c_int as fe_limb_t,
                            7625277 as core::ffi::c_int as fe_limb_t,
                            11091125 as core::ffi::c_int as fe_limb_t,
                            47489674 as core::ffi::c_int as fe_limb_t,
                            2074448 as core::ffi::c_int as fe_limb_t,
                            57694925 as core::ffi::c_int as fe_limb_t,
                            14905376 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            24483648 as core::ffi::c_int as fe_limb_t,
                            21618865 as core::ffi::c_int as fe_limb_t,
                            64589997 as core::ffi::c_int as fe_limb_t,
                            22007013 as core::ffi::c_int as fe_limb_t,
                            65555733 as core::ffi::c_int as fe_limb_t,
                            15355505 as core::ffi::c_int as fe_limb_t,
                            41826784 as core::ffi::c_int as fe_limb_t,
                            9253128 as core::ffi::c_int as fe_limb_t,
                            27628530 as core::ffi::c_int as fe_limb_t,
                            25998952 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            17597607 as core::ffi::c_int as fe_limb_t,
                            8340603 as core::ffi::c_int as fe_limb_t,
                            19355617 as core::ffi::c_int as fe_limb_t,
                            552187 as core::ffi::c_int as fe_limb_t,
                            26198470 as core::ffi::c_int as fe_limb_t,
                            30377849 as core::ffi::c_int as fe_limb_t,
                            4593323 as core::ffi::c_int as fe_limb_t,
                            24396850 as core::ffi::c_int as fe_limb_t,
                            52997988 as core::ffi::c_int as fe_limb_t,
                            15297015 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            510886 as core::ffi::c_int as fe_limb_t,
                            14337390 as core::ffi::c_int as fe_limb_t,
                            35323607 as core::ffi::c_int as fe_limb_t,
                            16638631 as core::ffi::c_int as fe_limb_t,
                            6328095 as core::ffi::c_int as fe_limb_t,
                            2713355 as core::ffi::c_int as fe_limb_t,
                            46891447 as core::ffi::c_int as fe_limb_t,
                            21690211 as core::ffi::c_int as fe_limb_t,
                            8683220 as core::ffi::c_int as fe_limb_t,
                            2921426 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            18606791 as core::ffi::c_int as fe_limb_t,
                            11874196 as core::ffi::c_int as fe_limb_t,
                            27155355 as core::ffi::c_int as fe_limb_t,
                            28272950 as core::ffi::c_int as fe_limb_t,
                            43077121 as core::ffi::c_int as fe_limb_t,
                            6265445 as core::ffi::c_int as fe_limb_t,
                            41930624 as core::ffi::c_int as fe_limb_t,
                            32275507 as core::ffi::c_int as fe_limb_t,
                            4674689 as core::ffi::c_int as fe_limb_t,
                            13890525 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            13609624 as core::ffi::c_int as fe_limb_t,
                            13069022 as core::ffi::c_int as fe_limb_t,
                            39736503 as core::ffi::c_int as fe_limb_t,
                            20498523 as core::ffi::c_int as fe_limb_t,
                            24360585 as core::ffi::c_int as fe_limb_t,
                            9592974 as core::ffi::c_int as fe_limb_t,
                            14977157 as core::ffi::c_int as fe_limb_t,
                            9835105 as core::ffi::c_int as fe_limb_t,
                            4389687 as core::ffi::c_int as fe_limb_t,
                            288396 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            9922506 as core::ffi::c_int as fe_limb_t,
                            33035038 as core::ffi::c_int as fe_limb_t,
                            13613106 as core::ffi::c_int as fe_limb_t,
                            5883594 as core::ffi::c_int as fe_limb_t,
                            48350519 as core::ffi::c_int as fe_limb_t,
                            33120168 as core::ffi::c_int as fe_limb_t,
                            54804801 as core::ffi::c_int as fe_limb_t,
                            8317627 as core::ffi::c_int as fe_limb_t,
                            23388070 as core::ffi::c_int as fe_limb_t,
                            16052080 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            12719997 as core::ffi::c_int as fe_limb_t,
                            11937594 as core::ffi::c_int as fe_limb_t,
                            35138804 as core::ffi::c_int as fe_limb_t,
                            28525742 as core::ffi::c_int as fe_limb_t,
                            26900119 as core::ffi::c_int as fe_limb_t,
                            8561328 as core::ffi::c_int as fe_limb_t,
                            46953177 as core::ffi::c_int as fe_limb_t,
                            21921452 as core::ffi::c_int as fe_limb_t,
                            52354592 as core::ffi::c_int as fe_limb_t,
                            22741539 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            15961858 as core::ffi::c_int as fe_limb_t,
                            14150409 as core::ffi::c_int as fe_limb_t,
                            26716931 as core::ffi::c_int as fe_limb_t,
                            32888600 as core::ffi::c_int as fe_limb_t,
                            44314535 as core::ffi::c_int as fe_limb_t,
                            13603568 as core::ffi::c_int as fe_limb_t,
                            11829573 as core::ffi::c_int as fe_limb_t,
                            7467844 as core::ffi::c_int as fe_limb_t,
                            38286736 as core::ffi::c_int as fe_limb_t,
                            929274 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11038231 as core::ffi::c_int as fe_limb_t,
                            21972036 as core::ffi::c_int as fe_limb_t,
                            39798381 as core::ffi::c_int as fe_limb_t,
                            26237869 as core::ffi::c_int as fe_limb_t,
                            56610336 as core::ffi::c_int as fe_limb_t,
                            17246600 as core::ffi::c_int as fe_limb_t,
                            43629330 as core::ffi::c_int as fe_limb_t,
                            24182562 as core::ffi::c_int as fe_limb_t,
                            45715720 as core::ffi::c_int as fe_limb_t,
                            2465073 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            20017144 as core::ffi::c_int as fe_limb_t,
                            29231206 as core::ffi::c_int as fe_limb_t,
                            27915241 as core::ffi::c_int as fe_limb_t,
                            1529148 as core::ffi::c_int as fe_limb_t,
                            12396362 as core::ffi::c_int as fe_limb_t,
                            15675764 as core::ffi::c_int as fe_limb_t,
                            13817261 as core::ffi::c_int as fe_limb_t,
                            23896366 as core::ffi::c_int as fe_limb_t,
                            2463390 as core::ffi::c_int as fe_limb_t,
                            28932292 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50749986 as core::ffi::c_int as fe_limb_t,
                            20890520 as core::ffi::c_int as fe_limb_t,
                            55043680 as core::ffi::c_int as fe_limb_t,
                            4996453 as core::ffi::c_int as fe_limb_t,
                            65852442 as core::ffi::c_int as fe_limb_t,
                            1073571 as core::ffi::c_int as fe_limb_t,
                            9583558 as core::ffi::c_int as fe_limb_t,
                            12851107 as core::ffi::c_int as fe_limb_t,
                            4003896 as core::ffi::c_int as fe_limb_t,
                            12673717 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65377275 as core::ffi::c_int as fe_limb_t,
                            18398561 as core::ffi::c_int as fe_limb_t,
                            63845933 as core::ffi::c_int as fe_limb_t,
                            16143081 as core::ffi::c_int as fe_limb_t,
                            19294135 as core::ffi::c_int as fe_limb_t,
                            13385325 as core::ffi::c_int as fe_limb_t,
                            14741514 as core::ffi::c_int as fe_limb_t,
                            24450706 as core::ffi::c_int as fe_limb_t,
                            7903885 as core::ffi::c_int as fe_limb_t,
                            2348101 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            24536016 as core::ffi::c_int as fe_limb_t,
                            17039225 as core::ffi::c_int as fe_limb_t,
                            12715591 as core::ffi::c_int as fe_limb_t,
                            29692277 as core::ffi::c_int as fe_limb_t,
                            1511292 as core::ffi::c_int as fe_limb_t,
                            10047386 as core::ffi::c_int as fe_limb_t,
                            63266518 as core::ffi::c_int as fe_limb_t,
                            26425272 as core::ffi::c_int as fe_limb_t,
                            38731325 as core::ffi::c_int as fe_limb_t,
                            10048126 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            54486638 as core::ffi::c_int as fe_limb_t,
                            27349611 as core::ffi::c_int as fe_limb_t,
                            30718824 as core::ffi::c_int as fe_limb_t,
                            2591312 as core::ffi::c_int as fe_limb_t,
                            56491836 as core::ffi::c_int as fe_limb_t,
                            12192839 as core::ffi::c_int as fe_limb_t,
                            18873298 as core::ffi::c_int as fe_limb_t,
                            26257342 as core::ffi::c_int as fe_limb_t,
                            34811107 as core::ffi::c_int as fe_limb_t,
                            15221631 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            40630742 as core::ffi::c_int as fe_limb_t,
                            22450567 as core::ffi::c_int as fe_limb_t,
                            11546243 as core::ffi::c_int as fe_limb_t,
                            31701949 as core::ffi::c_int as fe_limb_t,
                            9180879 as core::ffi::c_int as fe_limb_t,
                            7656409 as core::ffi::c_int as fe_limb_t,
                            45764914 as core::ffi::c_int as fe_limb_t,
                            2095754 as core::ffi::c_int as fe_limb_t,
                            29769758 as core::ffi::c_int as fe_limb_t,
                            6593415 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            35114656 as core::ffi::c_int as fe_limb_t,
                            30646970 as core::ffi::c_int as fe_limb_t,
                            4176911 as core::ffi::c_int as fe_limb_t,
                            3264766 as core::ffi::c_int as fe_limb_t,
                            12538965 as core::ffi::c_int as fe_limb_t,
                            32686321 as core::ffi::c_int as fe_limb_t,
                            26312344 as core::ffi::c_int as fe_limb_t,
                            27435754 as core::ffi::c_int as fe_limb_t,
                            30958053 as core::ffi::c_int as fe_limb_t,
                            8292160 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31429803 as core::ffi::c_int as fe_limb_t,
                            19595316 as core::ffi::c_int as fe_limb_t,
                            29173531 as core::ffi::c_int as fe_limb_t,
                            15632448 as core::ffi::c_int as fe_limb_t,
                            12174511 as core::ffi::c_int as fe_limb_t,
                            30794338 as core::ffi::c_int as fe_limb_t,
                            32808830 as core::ffi::c_int as fe_limb_t,
                            3977186 as core::ffi::c_int as fe_limb_t,
                            26143136 as core::ffi::c_int as fe_limb_t,
                            30405556 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            22648882 as core::ffi::c_int as fe_limb_t,
                            1402143 as core::ffi::c_int as fe_limb_t,
                            44308880 as core::ffi::c_int as fe_limb_t,
                            13746058 as core::ffi::c_int as fe_limb_t,
                            7936347 as core::ffi::c_int as fe_limb_t,
                            365344 as core::ffi::c_int as fe_limb_t,
                            58440231 as core::ffi::c_int as fe_limb_t,
                            31879998 as core::ffi::c_int as fe_limb_t,
                            63350620 as core::ffi::c_int as fe_limb_t,
                            31249806 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            51616947 as core::ffi::c_int as fe_limb_t,
                            8012312 as core::ffi::c_int as fe_limb_t,
                            64594134 as core::ffi::c_int as fe_limb_t,
                            20851969 as core::ffi::c_int as fe_limb_t,
                            43143017 as core::ffi::c_int as fe_limb_t,
                            23300402 as core::ffi::c_int as fe_limb_t,
                            65496150 as core::ffi::c_int as fe_limb_t,
                            32018862 as core::ffi::c_int as fe_limb_t,
                            50444388 as core::ffi::c_int as fe_limb_t,
                            8194477 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            27338066 as core::ffi::c_int as fe_limb_t,
                            26047012 as core::ffi::c_int as fe_limb_t,
                            59694639 as core::ffi::c_int as fe_limb_t,
                            10140404 as core::ffi::c_int as fe_limb_t,
                            48082437 as core::ffi::c_int as fe_limb_t,
                            26964542 as core::ffi::c_int as fe_limb_t,
                            27277190 as core::ffi::c_int as fe_limb_t,
                            8855376 as core::ffi::c_int as fe_limb_t,
                            28572286 as core::ffi::c_int as fe_limb_t,
                            3005164 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            26287105 as core::ffi::c_int as fe_limb_t,
                            4821776 as core::ffi::c_int as fe_limb_t,
                            25476601 as core::ffi::c_int as fe_limb_t,
                            29408529 as core::ffi::c_int as fe_limb_t,
                            63344350 as core::ffi::c_int as fe_limb_t,
                            17765447 as core::ffi::c_int as fe_limb_t,
                            49100281 as core::ffi::c_int as fe_limb_t,
                            1182478 as core::ffi::c_int as fe_limb_t,
                            41014043 as core::ffi::c_int as fe_limb_t,
                            20474836 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            59937691 as core::ffi::c_int as fe_limb_t,
                            3178079 as core::ffi::c_int as fe_limb_t,
                            23970071 as core::ffi::c_int as fe_limb_t,
                            6201893 as core::ffi::c_int as fe_limb_t,
                            49913287 as core::ffi::c_int as fe_limb_t,
                            29065239 as core::ffi::c_int as fe_limb_t,
                            45232588 as core::ffi::c_int as fe_limb_t,
                            19571804 as core::ffi::c_int as fe_limb_t,
                            32208682 as core::ffi::c_int as fe_limb_t,
                            32356184 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            50451143 as core::ffi::c_int as fe_limb_t,
                            2817642 as core::ffi::c_int as fe_limb_t,
                            56822502 as core::ffi::c_int as fe_limb_t,
                            14811297 as core::ffi::c_int as fe_limb_t,
                            6024667 as core::ffi::c_int as fe_limb_t,
                            13349505 as core::ffi::c_int as fe_limb_t,
                            39793360 as core::ffi::c_int as fe_limb_t,
                            23056589 as core::ffi::c_int as fe_limb_t,
                            39436278 as core::ffi::c_int as fe_limb_t,
                            22014573 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            15941010 as core::ffi::c_int as fe_limb_t,
                            24148500 as core::ffi::c_int as fe_limb_t,
                            45741813 as core::ffi::c_int as fe_limb_t,
                            8062054 as core::ffi::c_int as fe_limb_t,
                            31876073 as core::ffi::c_int as fe_limb_t,
                            33315803 as core::ffi::c_int as fe_limb_t,
                            51830470 as core::ffi::c_int as fe_limb_t,
                            32110002 as core::ffi::c_int as fe_limb_t,
                            15397330 as core::ffi::c_int as fe_limb_t,
                            29424239 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            8934485 as core::ffi::c_int as fe_limb_t,
                            20068965 as core::ffi::c_int as fe_limb_t,
                            43822466 as core::ffi::c_int as fe_limb_t,
                            20131190 as core::ffi::c_int as fe_limb_t,
                            34662773 as core::ffi::c_int as fe_limb_t,
                            14047985 as core::ffi::c_int as fe_limb_t,
                            31170398 as core::ffi::c_int as fe_limb_t,
                            32113411 as core::ffi::c_int as fe_limb_t,
                            39603297 as core::ffi::c_int as fe_limb_t,
                            15087183 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            48751602 as core::ffi::c_int as fe_limb_t,
                            31397940 as core::ffi::c_int as fe_limb_t,
                            24524912 as core::ffi::c_int as fe_limb_t,
                            16876564 as core::ffi::c_int as fe_limb_t,
                            15520426 as core::ffi::c_int as fe_limb_t,
                            27193656 as core::ffi::c_int as fe_limb_t,
                            51606457 as core::ffi::c_int as fe_limb_t,
                            11461895 as core::ffi::c_int as fe_limb_t,
                            16788528 as core::ffi::c_int as fe_limb_t,
                            27685490 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65161459 as core::ffi::c_int as fe_limb_t,
                            16013772 as core::ffi::c_int as fe_limb_t,
                            21750665 as core::ffi::c_int as fe_limb_t,
                            3714552 as core::ffi::c_int as fe_limb_t,
                            49707082 as core::ffi::c_int as fe_limb_t,
                            17498998 as core::ffi::c_int as fe_limb_t,
                            63338576 as core::ffi::c_int as fe_limb_t,
                            23231111 as core::ffi::c_int as fe_limb_t,
                            31322513 as core::ffi::c_int as fe_limb_t,
                            21938797 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            21426636 as core::ffi::c_int as fe_limb_t,
                            27904214 as core::ffi::c_int as fe_limb_t,
                            53460576 as core::ffi::c_int as fe_limb_t,
                            28206894 as core::ffi::c_int as fe_limb_t,
                            38296674 as core::ffi::c_int as fe_limb_t,
                            28633461 as core::ffi::c_int as fe_limb_t,
                            48833472 as core::ffi::c_int as fe_limb_t,
                            18933017 as core::ffi::c_int as fe_limb_t,
                            13040861 as core::ffi::c_int as fe_limb_t,
                            21441484 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            11293895 as core::ffi::c_int as fe_limb_t,
                            12478086 as core::ffi::c_int as fe_limb_t,
                            39972463 as core::ffi::c_int as fe_limb_t,
                            15083749 as core::ffi::c_int as fe_limb_t,
                            37801443 as core::ffi::c_int as fe_limb_t,
                            14748871 as core::ffi::c_int as fe_limb_t,
                            14555558 as core::ffi::c_int as fe_limb_t,
                            20137329 as core::ffi::c_int as fe_limb_t,
                            1613710 as core::ffi::c_int as fe_limb_t,
                            4896935 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            41213962 as core::ffi::c_int as fe_limb_t,
                            15323293 as core::ffi::c_int as fe_limb_t,
                            58619073 as core::ffi::c_int as fe_limb_t,
                            25496531 as core::ffi::c_int as fe_limb_t,
                            25967125 as core::ffi::c_int as fe_limb_t,
                            20128972 as core::ffi::c_int as fe_limb_t,
                            2825959 as core::ffi::c_int as fe_limb_t,
                            28657387 as core::ffi::c_int as fe_limb_t,
                            43137087 as core::ffi::c_int as fe_limb_t,
                            22287016 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            51184079 as core::ffi::c_int as fe_limb_t,
                            28324551 as core::ffi::c_int as fe_limb_t,
                            49665331 as core::ffi::c_int as fe_limb_t,
                            6410663 as core::ffi::c_int as fe_limb_t,
                            3622847 as core::ffi::c_int as fe_limb_t,
                            10243618 as core::ffi::c_int as fe_limb_t,
                            20615400 as core::ffi::c_int as fe_limb_t,
                            12405433 as core::ffi::c_int as fe_limb_t,
                            43355834 as core::ffi::c_int as fe_limb_t,
                            25118015 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            60017550 as core::ffi::c_int as fe_limb_t,
                            12556207 as core::ffi::c_int as fe_limb_t,
                            46917512 as core::ffi::c_int as fe_limb_t,
                            9025186 as core::ffi::c_int as fe_limb_t,
                            50036385 as core::ffi::c_int as fe_limb_t,
                            4333800 as core::ffi::c_int as fe_limb_t,
                            4378436 as core::ffi::c_int as fe_limb_t,
                            2432030 as core::ffi::c_int as fe_limb_t,
                            23097949 as core::ffi::c_int as fe_limb_t,
                            32988414 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            4565804 as core::ffi::c_int as fe_limb_t,
                            17528778 as core::ffi::c_int as fe_limb_t,
                            20084411 as core::ffi::c_int as fe_limb_t,
                            25711615 as core::ffi::c_int as fe_limb_t,
                            1724998 as core::ffi::c_int as fe_limb_t,
                            189254 as core::ffi::c_int as fe_limb_t,
                            24767264 as core::ffi::c_int as fe_limb_t,
                            10103221 as core::ffi::c_int as fe_limb_t,
                            48596551 as core::ffi::c_int as fe_limb_t,
                            2424777 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            366633 as core::ffi::c_int as fe_limb_t,
                            21577626 as core::ffi::c_int as fe_limb_t,
                            8173089 as core::ffi::c_int as fe_limb_t,
                            26664313 as core::ffi::c_int as fe_limb_t,
                            30788633 as core::ffi::c_int as fe_limb_t,
                            5745705 as core::ffi::c_int as fe_limb_t,
                            59940186 as core::ffi::c_int as fe_limb_t,
                            1344108 as core::ffi::c_int as fe_limb_t,
                            63466311 as core::ffi::c_int as fe_limb_t,
                            12412658 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43107073 as core::ffi::c_int as fe_limb_t,
                            7690285 as core::ffi::c_int as fe_limb_t,
                            14929416 as core::ffi::c_int as fe_limb_t,
                            33386175 as core::ffi::c_int as fe_limb_t,
                            34898028 as core::ffi::c_int as fe_limb_t,
                            20141445 as core::ffi::c_int as fe_limb_t,
                            24162696 as core::ffi::c_int as fe_limb_t,
                            18227928 as core::ffi::c_int as fe_limb_t,
                            63967362 as core::ffi::c_int as fe_limb_t,
                            11179384 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            18289503 as core::ffi::c_int as fe_limb_t,
                            18829478 as core::ffi::c_int as fe_limb_t,
                            8056944 as core::ffi::c_int as fe_limb_t,
                            16430056 as core::ffi::c_int as fe_limb_t,
                            45379140 as core::ffi::c_int as fe_limb_t,
                            7842513 as core::ffi::c_int as fe_limb_t,
                            61107423 as core::ffi::c_int as fe_limb_t,
                            32067534 as core::ffi::c_int as fe_limb_t,
                            48424218 as core::ffi::c_int as fe_limb_t,
                            22110928 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            476239 as core::ffi::c_int as fe_limb_t,
                            6601091 as core::ffi::c_int as fe_limb_t,
                            60956074 as core::ffi::c_int as fe_limb_t,
                            23831056 as core::ffi::c_int as fe_limb_t,
                            17503544 as core::ffi::c_int as fe_limb_t,
                            28690532 as core::ffi::c_int as fe_limb_t,
                            27672958 as core::ffi::c_int as fe_limb_t,
                            13403813 as core::ffi::c_int as fe_limb_t,
                            11052904 as core::ffi::c_int as fe_limb_t,
                            5219329 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            20678527 as core::ffi::c_int as fe_limb_t,
                            25178694 as core::ffi::c_int as fe_limb_t,
                            34436965 as core::ffi::c_int as fe_limb_t,
                            8849122 as core::ffi::c_int as fe_limb_t,
                            62099106 as core::ffi::c_int as fe_limb_t,
                            14574751 as core::ffi::c_int as fe_limb_t,
                            31186971 as core::ffi::c_int as fe_limb_t,
                            29580702 as core::ffi::c_int as fe_limb_t,
                            9014761 as core::ffi::c_int as fe_limb_t,
                            24975376 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            53464795 as core::ffi::c_int as fe_limb_t,
                            23204192 as core::ffi::c_int as fe_limb_t,
                            51146355 as core::ffi::c_int as fe_limb_t,
                            5075807 as core::ffi::c_int as fe_limb_t,
                            65594203 as core::ffi::c_int as fe_limb_t,
                            22019831 as core::ffi::c_int as fe_limb_t,
                            34006363 as core::ffi::c_int as fe_limb_t,
                            9160279 as core::ffi::c_int as fe_limb_t,
                            8473550 as core::ffi::c_int as fe_limb_t,
                            30297594 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            24900749 as core::ffi::c_int as fe_limb_t,
                            14435722 as core::ffi::c_int as fe_limb_t,
                            17209120 as core::ffi::c_int as fe_limb_t,
                            18261891 as core::ffi::c_int as fe_limb_t,
                            44516588 as core::ffi::c_int as fe_limb_t,
                            9878982 as core::ffi::c_int as fe_limb_t,
                            59419555 as core::ffi::c_int as fe_limb_t,
                            17218610 as core::ffi::c_int as fe_limb_t,
                            42540382 as core::ffi::c_int as fe_limb_t,
                            11788947 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            63990690 as core::ffi::c_int as fe_limb_t,
                            22159237 as core::ffi::c_int as fe_limb_t,
                            53306774 as core::ffi::c_int as fe_limb_t,
                            14797440 as core::ffi::c_int as fe_limb_t,
                            9652448 as core::ffi::c_int as fe_limb_t,
                            26708528 as core::ffi::c_int as fe_limb_t,
                            47071426 as core::ffi::c_int as fe_limb_t,
                            10410732 as core::ffi::c_int as fe_limb_t,
                            42540394 as core::ffi::c_int as fe_limb_t,
                            32095740 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51449703 as core::ffi::c_int as fe_limb_t,
                            16736705 as core::ffi::c_int as fe_limb_t,
                            44641714 as core::ffi::c_int as fe_limb_t,
                            10215877 as core::ffi::c_int as fe_limb_t,
                            58011687 as core::ffi::c_int as fe_limb_t,
                            7563910 as core::ffi::c_int as fe_limb_t,
                            11871841 as core::ffi::c_int as fe_limb_t,
                            21049238 as core::ffi::c_int as fe_limb_t,
                            48595538 as core::ffi::c_int as fe_limb_t,
                            8464117 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43708233 as core::ffi::c_int as fe_limb_t,
                            8348506 as core::ffi::c_int as fe_limb_t,
                            52522913 as core::ffi::c_int as fe_limb_t,
                            32692717 as core::ffi::c_int as fe_limb_t,
                            63158658 as core::ffi::c_int as fe_limb_t,
                            27181012 as core::ffi::c_int as fe_limb_t,
                            14325288 as core::ffi::c_int as fe_limb_t,
                            8628612 as core::ffi::c_int as fe_limb_t,
                            33313881 as core::ffi::c_int as fe_limb_t,
                            25183915 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            46921872 as core::ffi::c_int as fe_limb_t,
                            28586496 as core::ffi::c_int as fe_limb_t,
                            22367355 as core::ffi::c_int as fe_limb_t,
                            5271547 as core::ffi::c_int as fe_limb_t,
                            66011747 as core::ffi::c_int as fe_limb_t,
                            28765593 as core::ffi::c_int as fe_limb_t,
                            42303196 as core::ffi::c_int as fe_limb_t,
                            23317577 as core::ffi::c_int as fe_limb_t,
                            58168128 as core::ffi::c_int as fe_limb_t,
                            27736162 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            60160060 as core::ffi::c_int as fe_limb_t,
                            31759219 as core::ffi::c_int as fe_limb_t,
                            34483180 as core::ffi::c_int as fe_limb_t,
                            17533252 as core::ffi::c_int as fe_limb_t,
                            32635413 as core::ffi::c_int as fe_limb_t,
                            26180187 as core::ffi::c_int as fe_limb_t,
                            15989196 as core::ffi::c_int as fe_limb_t,
                            20716244 as core::ffi::c_int as fe_limb_t,
                            28358191 as core::ffi::c_int as fe_limb_t,
                            29300528 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43547083 as core::ffi::c_int as fe_limb_t,
                            30755372 as core::ffi::c_int as fe_limb_t,
                            34757181 as core::ffi::c_int as fe_limb_t,
                            31892468 as core::ffi::c_int as fe_limb_t,
                            57961144 as core::ffi::c_int as fe_limb_t,
                            10429266 as core::ffi::c_int as fe_limb_t,
                            50471180 as core::ffi::c_int as fe_limb_t,
                            4072015 as core::ffi::c_int as fe_limb_t,
                            61757200 as core::ffi::c_int as fe_limb_t,
                            5596588 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            38872266 as core::ffi::c_int as fe_limb_t,
                            30164383 as core::ffi::c_int as fe_limb_t,
                            12312895 as core::ffi::c_int as fe_limb_t,
                            6213178 as core::ffi::c_int as fe_limb_t,
                            3117142 as core::ffi::c_int as fe_limb_t,
                            16078565 as core::ffi::c_int as fe_limb_t,
                            29266239 as core::ffi::c_int as fe_limb_t,
                            2557221 as core::ffi::c_int as fe_limb_t,
                            1768301 as core::ffi::c_int as fe_limb_t,
                            15373193 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            59865506 as core::ffi::c_int as fe_limb_t,
                            30307471 as core::ffi::c_int as fe_limb_t,
                            62515396 as core::ffi::c_int as fe_limb_t,
                            26001078 as core::ffi::c_int as fe_limb_t,
                            66980936 as core::ffi::c_int as fe_limb_t,
                            32642186 as core::ffi::c_int as fe_limb_t,
                            66017961 as core::ffi::c_int as fe_limb_t,
                            29049440 as core::ffi::c_int as fe_limb_t,
                            42448372 as core::ffi::c_int as fe_limb_t,
                            3442909 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            36898293 as core::ffi::c_int as fe_limb_t,
                            5124042 as core::ffi::c_int as fe_limb_t,
                            14181784 as core::ffi::c_int as fe_limb_t,
                            8197961 as core::ffi::c_int as fe_limb_t,
                            18964734 as core::ffi::c_int as fe_limb_t,
                            21615339 as core::ffi::c_int as fe_limb_t,
                            22597930 as core::ffi::c_int as fe_limb_t,
                            7176455 as core::ffi::c_int as fe_limb_t,
                            48523386 as core::ffi::c_int as fe_limb_t,
                            13365929 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            59231455 as core::ffi::c_int as fe_limb_t,
                            32054473 as core::ffi::c_int as fe_limb_t,
                            8324672 as core::ffi::c_int as fe_limb_t,
                            4690079 as core::ffi::c_int as fe_limb_t,
                            6261860 as core::ffi::c_int as fe_limb_t,
                            890446 as core::ffi::c_int as fe_limb_t,
                            24538107 as core::ffi::c_int as fe_limb_t,
                            24984246 as core::ffi::c_int as fe_limb_t,
                            57419264 as core::ffi::c_int as fe_limb_t,
                            30522764 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            25008885 as core::ffi::c_int as fe_limb_t,
                            22782833 as core::ffi::c_int as fe_limb_t,
                            62803832 as core::ffi::c_int as fe_limb_t,
                            23916421 as core::ffi::c_int as fe_limb_t,
                            16265035 as core::ffi::c_int as fe_limb_t,
                            15721635 as core::ffi::c_int as fe_limb_t,
                            683793 as core::ffi::c_int as fe_limb_t,
                            21730648 as core::ffi::c_int as fe_limb_t,
                            15723478 as core::ffi::c_int as fe_limb_t,
                            18390951 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57448220 as core::ffi::c_int as fe_limb_t,
                            12374378 as core::ffi::c_int as fe_limb_t,
                            40101865 as core::ffi::c_int as fe_limb_t,
                            26528283 as core::ffi::c_int as fe_limb_t,
                            59384749 as core::ffi::c_int as fe_limb_t,
                            21239917 as core::ffi::c_int as fe_limb_t,
                            11879681 as core::ffi::c_int as fe_limb_t,
                            5400171 as core::ffi::c_int as fe_limb_t,
                            519526 as core::ffi::c_int as fe_limb_t,
                            32318556 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            22258397 as core::ffi::c_int as fe_limb_t,
                            17222199 as core::ffi::c_int as fe_limb_t,
                            59239046 as core::ffi::c_int as fe_limb_t,
                            14613015 as core::ffi::c_int as fe_limb_t,
                            44588609 as core::ffi::c_int as fe_limb_t,
                            30603508 as core::ffi::c_int as fe_limb_t,
                            46754982 as core::ffi::c_int as fe_limb_t,
                            7315966 as core::ffi::c_int as fe_limb_t,
                            16648397 as core::ffi::c_int as fe_limb_t,
                            7605640 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            59027556 as core::ffi::c_int as fe_limb_t,
                            25089834 as core::ffi::c_int as fe_limb_t,
                            58885552 as core::ffi::c_int as fe_limb_t,
                            9719709 as core::ffi::c_int as fe_limb_t,
                            19259459 as core::ffi::c_int as fe_limb_t,
                            18206220 as core::ffi::c_int as fe_limb_t,
                            23994941 as core::ffi::c_int as fe_limb_t,
                            28272877 as core::ffi::c_int as fe_limb_t,
                            57640015 as core::ffi::c_int as fe_limb_t,
                            4763277 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            45409620 as core::ffi::c_int as fe_limb_t,
                            9220968 as core::ffi::c_int as fe_limb_t,
                            51378240 as core::ffi::c_int as fe_limb_t,
                            1084136 as core::ffi::c_int as fe_limb_t,
                            41632757 as core::ffi::c_int as fe_limb_t,
                            30702041 as core::ffi::c_int as fe_limb_t,
                            31088446 as core::ffi::c_int as fe_limb_t,
                            25789909 as core::ffi::c_int as fe_limb_t,
                            55752334 as core::ffi::c_int as fe_limb_t,
                            728111 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            26047201 as core::ffi::c_int as fe_limb_t,
                            21802961 as core::ffi::c_int as fe_limb_t,
                            60208540 as core::ffi::c_int as fe_limb_t,
                            17032633 as core::ffi::c_int as fe_limb_t,
                            24092067 as core::ffi::c_int as fe_limb_t,
                            9158119 as core::ffi::c_int as fe_limb_t,
                            62835319 as core::ffi::c_int as fe_limb_t,
                            20998873 as core::ffi::c_int as fe_limb_t,
                            37743427 as core::ffi::c_int as fe_limb_t,
                            28056159 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            17510331 as core::ffi::c_int as fe_limb_t,
                            33231575 as core::ffi::c_int as fe_limb_t,
                            5854288 as core::ffi::c_int as fe_limb_t,
                            8403524 as core::ffi::c_int as fe_limb_t,
                            17133918 as core::ffi::c_int as fe_limb_t,
                            30441820 as core::ffi::c_int as fe_limb_t,
                            38997856 as core::ffi::c_int as fe_limb_t,
                            12327944 as core::ffi::c_int as fe_limb_t,
                            10750447 as core::ffi::c_int as fe_limb_t,
                            10014012 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            56796096 as core::ffi::c_int as fe_limb_t,
                            3936951 as core::ffi::c_int as fe_limb_t,
                            9156313 as core::ffi::c_int as fe_limb_t,
                            24656749 as core::ffi::c_int as fe_limb_t,
                            16498691 as core::ffi::c_int as fe_limb_t,
                            32559785 as core::ffi::c_int as fe_limb_t,
                            39627812 as core::ffi::c_int as fe_limb_t,
                            32887699 as core::ffi::c_int as fe_limb_t,
                            3424690 as core::ffi::c_int as fe_limb_t,
                            7540221 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30322361 as core::ffi::c_int as fe_limb_t,
                            26590322 as core::ffi::c_int as fe_limb_t,
                            11361004 as core::ffi::c_int as fe_limb_t,
                            29411115 as core::ffi::c_int as fe_limb_t,
                            7433303 as core::ffi::c_int as fe_limb_t,
                            4989748 as core::ffi::c_int as fe_limb_t,
                            60037442 as core::ffi::c_int as fe_limb_t,
                            17237212 as core::ffi::c_int as fe_limb_t,
                            57864598 as core::ffi::c_int as fe_limb_t,
                            15258045 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            13054543 as core::ffi::c_int as fe_limb_t,
                            30774935 as core::ffi::c_int as fe_limb_t,
                            19155473 as core::ffi::c_int as fe_limb_t,
                            469045 as core::ffi::c_int as fe_limb_t,
                            54626067 as core::ffi::c_int as fe_limb_t,
                            4566041 as core::ffi::c_int as fe_limb_t,
                            5631406 as core::ffi::c_int as fe_limb_t,
                            2711395 as core::ffi::c_int as fe_limb_t,
                            1062915 as core::ffi::c_int as fe_limb_t,
                            28418087 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            47868616 as core::ffi::c_int as fe_limb_t,
                            22299832 as core::ffi::c_int as fe_limb_t,
                            37599834 as core::ffi::c_int as fe_limb_t,
                            26054466 as core::ffi::c_int as fe_limb_t,
                            61273100 as core::ffi::c_int as fe_limb_t,
                            13005410 as core::ffi::c_int as fe_limb_t,
                            61042375 as core::ffi::c_int as fe_limb_t,
                            12194496 as core::ffi::c_int as fe_limb_t,
                            32960380 as core::ffi::c_int as fe_limb_t,
                            1459310 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            19852015 as core::ffi::c_int as fe_limb_t,
                            7027924 as core::ffi::c_int as fe_limb_t,
                            23669353 as core::ffi::c_int as fe_limb_t,
                            10020366 as core::ffi::c_int as fe_limb_t,
                            8586503 as core::ffi::c_int as fe_limb_t,
                            26896525 as core::ffi::c_int as fe_limb_t,
                            394196 as core::ffi::c_int as fe_limb_t,
                            27452547 as core::ffi::c_int as fe_limb_t,
                            18638002 as core::ffi::c_int as fe_limb_t,
                            22379495 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            31395515 as core::ffi::c_int as fe_limb_t,
                            15098109 as core::ffi::c_int as fe_limb_t,
                            26581030 as core::ffi::c_int as fe_limb_t,
                            8030562 as core::ffi::c_int as fe_limb_t,
                            50580950 as core::ffi::c_int as fe_limb_t,
                            28547297 as core::ffi::c_int as fe_limb_t,
                            9012485 as core::ffi::c_int as fe_limb_t,
                            25970078 as core::ffi::c_int as fe_limb_t,
                            60465776 as core::ffi::c_int as fe_limb_t,
                            28111795 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57916680 as core::ffi::c_int as fe_limb_t,
                            31207054 as core::ffi::c_int as fe_limb_t,
                            65111764 as core::ffi::c_int as fe_limb_t,
                            4529533 as core::ffi::c_int as fe_limb_t,
                            25766844 as core::ffi::c_int as fe_limb_t,
                            607986 as core::ffi::c_int as fe_limb_t,
                            67095642 as core::ffi::c_int as fe_limb_t,
                            9677542 as core::ffi::c_int as fe_limb_t,
                            34813975 as core::ffi::c_int as fe_limb_t,
                            27098423 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            64664349 as core::ffi::c_int as fe_limb_t,
                            33404494 as core::ffi::c_int as fe_limb_t,
                            29348901 as core::ffi::c_int as fe_limb_t,
                            8186665 as core::ffi::c_int as fe_limb_t,
                            1873760 as core::ffi::c_int as fe_limb_t,
                            12489863 as core::ffi::c_int as fe_limb_t,
                            36174285 as core::ffi::c_int as fe_limb_t,
                            25714739 as core::ffi::c_int as fe_limb_t,
                            59256019 as core::ffi::c_int as fe_limb_t,
                            25416002 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            51872508 as core::ffi::c_int as fe_limb_t,
                            18120922 as core::ffi::c_int as fe_limb_t,
                            7766469 as core::ffi::c_int as fe_limb_t,
                            746860 as core::ffi::c_int as fe_limb_t,
                            26346930 as core::ffi::c_int as fe_limb_t,
                            23332670 as core::ffi::c_int as fe_limb_t,
                            39775412 as core::ffi::c_int as fe_limb_t,
                            10754587 as core::ffi::c_int as fe_limb_t,
                            57677388 as core::ffi::c_int as fe_limb_t,
                            5203575 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            31834314 as core::ffi::c_int as fe_limb_t,
                            14135496 as core::ffi::c_int as fe_limb_t,
                            66338857 as core::ffi::c_int as fe_limb_t,
                            5159117 as core::ffi::c_int as fe_limb_t,
                            20917671 as core::ffi::c_int as fe_limb_t,
                            16786336 as core::ffi::c_int as fe_limb_t,
                            59640890 as core::ffi::c_int as fe_limb_t,
                            26216907 as core::ffi::c_int as fe_limb_t,
                            31809242 as core::ffi::c_int as fe_limb_t,
                            7347066 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            57502122 as core::ffi::c_int as fe_limb_t,
                            21680191 as core::ffi::c_int as fe_limb_t,
                            20414458 as core::ffi::c_int as fe_limb_t,
                            13033986 as core::ffi::c_int as fe_limb_t,
                            13716524 as core::ffi::c_int as fe_limb_t,
                            21862551 as core::ffi::c_int as fe_limb_t,
                            19797969 as core::ffi::c_int as fe_limb_t,
                            21343177 as core::ffi::c_int as fe_limb_t,
                            15192875 as core::ffi::c_int as fe_limb_t,
                            31466942 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54445282 as core::ffi::c_int as fe_limb_t,
                            31372712 as core::ffi::c_int as fe_limb_t,
                            1168161 as core::ffi::c_int as fe_limb_t,
                            29749623 as core::ffi::c_int as fe_limb_t,
                            26747876 as core::ffi::c_int as fe_limb_t,
                            19416341 as core::ffi::c_int as fe_limb_t,
                            10609329 as core::ffi::c_int as fe_limb_t,
                            12694420 as core::ffi::c_int as fe_limb_t,
                            33473243 as core::ffi::c_int as fe_limb_t,
                            20172328 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            33184999 as core::ffi::c_int as fe_limb_t,
                            11180355 as core::ffi::c_int as fe_limb_t,
                            15832085 as core::ffi::c_int as fe_limb_t,
                            22169002 as core::ffi::c_int as fe_limb_t,
                            65475192 as core::ffi::c_int as fe_limb_t,
                            225883 as core::ffi::c_int as fe_limb_t,
                            15089336 as core::ffi::c_int as fe_limb_t,
                            22530529 as core::ffi::c_int as fe_limb_t,
                            60973201 as core::ffi::c_int as fe_limb_t,
                            14480052 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            31308717 as core::ffi::c_int as fe_limb_t,
                            27934434 as core::ffi::c_int as fe_limb_t,
                            31030839 as core::ffi::c_int as fe_limb_t,
                            31657333 as core::ffi::c_int as fe_limb_t,
                            15674546 as core::ffi::c_int as fe_limb_t,
                            26971549 as core::ffi::c_int as fe_limb_t,
                            5496207 as core::ffi::c_int as fe_limb_t,
                            13685227 as core::ffi::c_int as fe_limb_t,
                            27595050 as core::ffi::c_int as fe_limb_t,
                            8737275 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            46790012 as core::ffi::c_int as fe_limb_t,
                            18404192 as core::ffi::c_int as fe_limb_t,
                            10933842 as core::ffi::c_int as fe_limb_t,
                            17376410 as core::ffi::c_int as fe_limb_t,
                            8335351 as core::ffi::c_int as fe_limb_t,
                            26008410 as core::ffi::c_int as fe_limb_t,
                            36100512 as core::ffi::c_int as fe_limb_t,
                            20943827 as core::ffi::c_int as fe_limb_t,
                            26498113 as core::ffi::c_int as fe_limb_t,
                            66511 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            22644435 as core::ffi::c_int as fe_limb_t,
                            24792703 as core::ffi::c_int as fe_limb_t,
                            50437087 as core::ffi::c_int as fe_limb_t,
                            4884561 as core::ffi::c_int as fe_limb_t,
                            64003250 as core::ffi::c_int as fe_limb_t,
                            19995065 as core::ffi::c_int as fe_limb_t,
                            30540765 as core::ffi::c_int as fe_limb_t,
                            29267685 as core::ffi::c_int as fe_limb_t,
                            53781076 as core::ffi::c_int as fe_limb_t,
                            26039336 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            39091017 as core::ffi::c_int as fe_limb_t,
                            9834844 as core::ffi::c_int as fe_limb_t,
                            18617207 as core::ffi::c_int as fe_limb_t,
                            30873120 as core::ffi::c_int as fe_limb_t,
                            63706907 as core::ffi::c_int as fe_limb_t,
                            20246925 as core::ffi::c_int as fe_limb_t,
                            8205539 as core::ffi::c_int as fe_limb_t,
                            13585437 as core::ffi::c_int as fe_limb_t,
                            49981399 as core::ffi::c_int as fe_limb_t,
                            15115438 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            23711543 as core::ffi::c_int as fe_limb_t,
                            32881517 as core::ffi::c_int as fe_limb_t,
                            31206560 as core::ffi::c_int as fe_limb_t,
                            25191721 as core::ffi::c_int as fe_limb_t,
                            6164646 as core::ffi::c_int as fe_limb_t,
                            23844445 as core::ffi::c_int as fe_limb_t,
                            33572981 as core::ffi::c_int as fe_limb_t,
                            32128335 as core::ffi::c_int as fe_limb_t,
                            8236920 as core::ffi::c_int as fe_limb_t,
                            16492939 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            43198286 as core::ffi::c_int as fe_limb_t,
                            20038905 as core::ffi::c_int as fe_limb_t,
                            40809380 as core::ffi::c_int as fe_limb_t,
                            29050590 as core::ffi::c_int as fe_limb_t,
                            25005589 as core::ffi::c_int as fe_limb_t,
                            25867162 as core::ffi::c_int as fe_limb_t,
                            19574901 as core::ffi::c_int as fe_limb_t,
                            10071562 as core::ffi::c_int as fe_limb_t,
                            6708380 as core::ffi::c_int as fe_limb_t,
                            27332008 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            2101372 as core::ffi::c_int as fe_limb_t,
                            28624378 as core::ffi::c_int as fe_limb_t,
                            19702730 as core::ffi::c_int as fe_limb_t,
                            2367575 as core::ffi::c_int as fe_limb_t,
                            51681697 as core::ffi::c_int as fe_limb_t,
                            1047674 as core::ffi::c_int as fe_limb_t,
                            5301017 as core::ffi::c_int as fe_limb_t,
                            9328700 as core::ffi::c_int as fe_limb_t,
                            29955601 as core::ffi::c_int as fe_limb_t,
                            21876122 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            3096359 as core::ffi::c_int as fe_limb_t,
                            9271816 as core::ffi::c_int as fe_limb_t,
                            45488000 as core::ffi::c_int as fe_limb_t,
                            18032587 as core::ffi::c_int as fe_limb_t,
                            52260867 as core::ffi::c_int as fe_limb_t,
                            25961494 as core::ffi::c_int as fe_limb_t,
                            41216721 as core::ffi::c_int as fe_limb_t,
                            20918836 as core::ffi::c_int as fe_limb_t,
                            57191288 as core::ffi::c_int as fe_limb_t,
                            6216607 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            34493015 as core::ffi::c_int as fe_limb_t,
                            338662 as core::ffi::c_int as fe_limb_t,
                            41913253 as core::ffi::c_int as fe_limb_t,
                            2510421 as core::ffi::c_int as fe_limb_t,
                            37895298 as core::ffi::c_int as fe_limb_t,
                            19734218 as core::ffi::c_int as fe_limb_t,
                            24822829 as core::ffi::c_int as fe_limb_t,
                            27407865 as core::ffi::c_int as fe_limb_t,
                            40341383 as core::ffi::c_int as fe_limb_t,
                            7525078 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            44042215 as core::ffi::c_int as fe_limb_t,
                            19568808 as core::ffi::c_int as fe_limb_t,
                            16133486 as core::ffi::c_int as fe_limb_t,
                            25658254 as core::ffi::c_int as fe_limb_t,
                            63719298 as core::ffi::c_int as fe_limb_t,
                            778787 as core::ffi::c_int as fe_limb_t,
                            66198528 as core::ffi::c_int as fe_limb_t,
                            30771936 as core::ffi::c_int as fe_limb_t,
                            47722230 as core::ffi::c_int as fe_limb_t,
                            11994100 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            21691500 as core::ffi::c_int as fe_limb_t,
                            19929806 as core::ffi::c_int as fe_limb_t,
                            66467532 as core::ffi::c_int as fe_limb_t,
                            19187410 as core::ffi::c_int as fe_limb_t,
                            3285880 as core::ffi::c_int as fe_limb_t,
                            30070836 as core::ffi::c_int as fe_limb_t,
                            42044197 as core::ffi::c_int as fe_limb_t,
                            9718257 as core::ffi::c_int as fe_limb_t,
                            59631427 as core::ffi::c_int as fe_limb_t,
                            13381417 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            18445390 as core::ffi::c_int as fe_limb_t,
                            29352196 as core::ffi::c_int as fe_limb_t,
                            14979845 as core::ffi::c_int as fe_limb_t,
                            11622458 as core::ffi::c_int as fe_limb_t,
                            65381754 as core::ffi::c_int as fe_limb_t,
                            29971451 as core::ffi::c_int as fe_limb_t,
                            23111647 as core::ffi::c_int as fe_limb_t,
                            27179185 as core::ffi::c_int as fe_limb_t,
                            28535281 as core::ffi::c_int as fe_limb_t,
                            15779576 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30098034 as core::ffi::c_int as fe_limb_t,
                            3089662 as core::ffi::c_int as fe_limb_t,
                            57874477 as core::ffi::c_int as fe_limb_t,
                            16662134 as core::ffi::c_int as fe_limb_t,
                            45801924 as core::ffi::c_int as fe_limb_t,
                            11308410 as core::ffi::c_int as fe_limb_t,
                            53040410 as core::ffi::c_int as fe_limb_t,
                            12021729 as core::ffi::c_int as fe_limb_t,
                            9955285 as core::ffi::c_int as fe_limb_t,
                            17251076 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            9734894 as core::ffi::c_int as fe_limb_t,
                            18977602 as core::ffi::c_int as fe_limb_t,
                            59635230 as core::ffi::c_int as fe_limb_t,
                            24415696 as core::ffi::c_int as fe_limb_t,
                            2060391 as core::ffi::c_int as fe_limb_t,
                            11313496 as core::ffi::c_int as fe_limb_t,
                            48682835 as core::ffi::c_int as fe_limb_t,
                            9924398 as core::ffi::c_int as fe_limb_t,
                            20194861 as core::ffi::c_int as fe_limb_t,
                            13380996 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            40730762 as core::ffi::c_int as fe_limb_t,
                            25589224 as core::ffi::c_int as fe_limb_t,
                            44941042 as core::ffi::c_int as fe_limb_t,
                            15789296 as core::ffi::c_int as fe_limb_t,
                            49053522 as core::ffi::c_int as fe_limb_t,
                            27385639 as core::ffi::c_int as fe_limb_t,
                            65123949 as core::ffi::c_int as fe_limb_t,
                            15707770 as core::ffi::c_int as fe_limb_t,
                            26342023 as core::ffi::c_int as fe_limb_t,
                            10146099 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41091971 as core::ffi::c_int as fe_limb_t,
                            33334488 as core::ffi::c_int as fe_limb_t,
                            21339190 as core::ffi::c_int as fe_limb_t,
                            33513044 as core::ffi::c_int as fe_limb_t,
                            19745255 as core::ffi::c_int as fe_limb_t,
                            30675732 as core::ffi::c_int as fe_limb_t,
                            37471583 as core::ffi::c_int as fe_limb_t,
                            2227039 as core::ffi::c_int as fe_limb_t,
                            21612326 as core::ffi::c_int as fe_limb_t,
                            33008704 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            54031477 as core::ffi::c_int as fe_limb_t,
                            1184227 as core::ffi::c_int as fe_limb_t,
                            23562814 as core::ffi::c_int as fe_limb_t,
                            27583990 as core::ffi::c_int as fe_limb_t,
                            46757619 as core::ffi::c_int as fe_limb_t,
                            27205717 as core::ffi::c_int as fe_limb_t,
                            25764460 as core::ffi::c_int as fe_limb_t,
                            12243797 as core::ffi::c_int as fe_limb_t,
                            46252298 as core::ffi::c_int as fe_limb_t,
                            11649657 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            57077370 as core::ffi::c_int as fe_limb_t,
                            11262625 as core::ffi::c_int as fe_limb_t,
                            27384172 as core::ffi::c_int as fe_limb_t,
                            2271902 as core::ffi::c_int as fe_limb_t,
                            26947504 as core::ffi::c_int as fe_limb_t,
                            17556661 as core::ffi::c_int as fe_limb_t,
                            39943 as core::ffi::c_int as fe_limb_t,
                            6114064 as core::ffi::c_int as fe_limb_t,
                            33514190 as core::ffi::c_int as fe_limb_t,
                            2333242 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            45675257 as core::ffi::c_int as fe_limb_t,
                            21132610 as core::ffi::c_int as fe_limb_t,
                            8119781 as core::ffi::c_int as fe_limb_t,
                            7219913 as core::ffi::c_int as fe_limb_t,
                            45278342 as core::ffi::c_int as fe_limb_t,
                            24538297 as core::ffi::c_int as fe_limb_t,
                            60429113 as core::ffi::c_int as fe_limb_t,
                            20883793 as core::ffi::c_int as fe_limb_t,
                            24350577 as core::ffi::c_int as fe_limb_t,
                            20104431 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            62992557 as core::ffi::c_int as fe_limb_t,
                            22282898 as core::ffi::c_int as fe_limb_t,
                            43222677 as core::ffi::c_int as fe_limb_t,
                            4843614 as core::ffi::c_int as fe_limb_t,
                            37020525 as core::ffi::c_int as fe_limb_t,
                            690622 as core::ffi::c_int as fe_limb_t,
                            35572776 as core::ffi::c_int as fe_limb_t,
                            23147595 as core::ffi::c_int as fe_limb_t,
                            8317859 as core::ffi::c_int as fe_limb_t,
                            12352766 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            18200138 as core::ffi::c_int as fe_limb_t,
                            19078521 as core::ffi::c_int as fe_limb_t,
                            34021104 as core::ffi::c_int as fe_limb_t,
                            30857812 as core::ffi::c_int as fe_limb_t,
                            43406342 as core::ffi::c_int as fe_limb_t,
                            24451920 as core::ffi::c_int as fe_limb_t,
                            43556767 as core::ffi::c_int as fe_limb_t,
                            31266881 as core::ffi::c_int as fe_limb_t,
                            20712162 as core::ffi::c_int as fe_limb_t,
                            6719373 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            26656189 as core::ffi::c_int as fe_limb_t,
                            6075253 as core::ffi::c_int as fe_limb_t,
                            59250308 as core::ffi::c_int as fe_limb_t,
                            1886071 as core::ffi::c_int as fe_limb_t,
                            38764821 as core::ffi::c_int as fe_limb_t,
                            4262325 as core::ffi::c_int as fe_limb_t,
                            11117530 as core::ffi::c_int as fe_limb_t,
                            29791222 as core::ffi::c_int as fe_limb_t,
                            26224234 as core::ffi::c_int as fe_limb_t,
                            30256974 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            49939907 as core::ffi::c_int as fe_limb_t,
                            18700334 as core::ffi::c_int as fe_limb_t,
                            63713187 as core::ffi::c_int as fe_limb_t,
                            17184554 as core::ffi::c_int as fe_limb_t,
                            47154818 as core::ffi::c_int as fe_limb_t,
                            14050419 as core::ffi::c_int as fe_limb_t,
                            21728352 as core::ffi::c_int as fe_limb_t,
                            9493610 as core::ffi::c_int as fe_limb_t,
                            18620611 as core::ffi::c_int as fe_limb_t,
                            17125804 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            53785524 as core::ffi::c_int as fe_limb_t,
                            13325348 as core::ffi::c_int as fe_limb_t,
                            11432106 as core::ffi::c_int as fe_limb_t,
                            5964811 as core::ffi::c_int as fe_limb_t,
                            18609221 as core::ffi::c_int as fe_limb_t,
                            6062965 as core::ffi::c_int as fe_limb_t,
                            61839393 as core::ffi::c_int as fe_limb_t,
                            23828875 as core::ffi::c_int as fe_limb_t,
                            36407290 as core::ffi::c_int as fe_limb_t,
                            17074774 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            43248326 as core::ffi::c_int as fe_limb_t,
                            22321272 as core::ffi::c_int as fe_limb_t,
                            26961356 as core::ffi::c_int as fe_limb_t,
                            1640861 as core::ffi::c_int as fe_limb_t,
                            34695752 as core::ffi::c_int as fe_limb_t,
                            16816491 as core::ffi::c_int as fe_limb_t,
                            12248508 as core::ffi::c_int as fe_limb_t,
                            28313793 as core::ffi::c_int as fe_limb_t,
                            13735341 as core::ffi::c_int as fe_limb_t,
                            1934062 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            25089769 as core::ffi::c_int as fe_limb_t,
                            6742589 as core::ffi::c_int as fe_limb_t,
                            17081145 as core::ffi::c_int as fe_limb_t,
                            20148166 as core::ffi::c_int as fe_limb_t,
                            21909292 as core::ffi::c_int as fe_limb_t,
                            17486451 as core::ffi::c_int as fe_limb_t,
                            51972569 as core::ffi::c_int as fe_limb_t,
                            29789085 as core::ffi::c_int as fe_limb_t,
                            45830866 as core::ffi::c_int as fe_limb_t,
                            5473615 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            31883658 as core::ffi::c_int as fe_limb_t,
                            25593331 as core::ffi::c_int as fe_limb_t,
                            1083431 as core::ffi::c_int as fe_limb_t,
                            21982029 as core::ffi::c_int as fe_limb_t,
                            22828470 as core::ffi::c_int as fe_limb_t,
                            13290673 as core::ffi::c_int as fe_limb_t,
                            59983779 as core::ffi::c_int as fe_limb_t,
                            12469655 as core::ffi::c_int as fe_limb_t,
                            29111212 as core::ffi::c_int as fe_limb_t,
                            28103418 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            24244947 as core::ffi::c_int as fe_limb_t,
                            18504025 as core::ffi::c_int as fe_limb_t,
                            40845887 as core::ffi::c_int as fe_limb_t,
                            2791539 as core::ffi::c_int as fe_limb_t,
                            52111265 as core::ffi::c_int as fe_limb_t,
                            16666677 as core::ffi::c_int as fe_limb_t,
                            24367466 as core::ffi::c_int as fe_limb_t,
                            6388839 as core::ffi::c_int as fe_limb_t,
                            56813277 as core::ffi::c_int as fe_limb_t,
                            452382 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            41468082 as core::ffi::c_int as fe_limb_t,
                            30136590 as core::ffi::c_int as fe_limb_t,
                            5217915 as core::ffi::c_int as fe_limb_t,
                            16224624 as core::ffi::c_int as fe_limb_t,
                            19987036 as core::ffi::c_int as fe_limb_t,
                            29472163 as core::ffi::c_int as fe_limb_t,
                            42872612 as core::ffi::c_int as fe_limb_t,
                            27639183 as core::ffi::c_int as fe_limb_t,
                            15766061 as core::ffi::c_int as fe_limb_t,
                            8407814 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46701865 as core::ffi::c_int as fe_limb_t,
                            13990230 as core::ffi::c_int as fe_limb_t,
                            15495425 as core::ffi::c_int as fe_limb_t,
                            16395525 as core::ffi::c_int as fe_limb_t,
                            5377168 as core::ffi::c_int as fe_limb_t,
                            15166495 as core::ffi::c_int as fe_limb_t,
                            58191841 as core::ffi::c_int as fe_limb_t,
                            29165478 as core::ffi::c_int as fe_limb_t,
                            59040954 as core::ffi::c_int as fe_limb_t,
                            2276717 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            30157899 as core::ffi::c_int as fe_limb_t,
                            12924066 as core::ffi::c_int as fe_limb_t,
                            49396814 as core::ffi::c_int as fe_limb_t,
                            9245752 as core::ffi::c_int as fe_limb_t,
                            19895028 as core::ffi::c_int as fe_limb_t,
                            3368142 as core::ffi::c_int as fe_limb_t,
                            43281277 as core::ffi::c_int as fe_limb_t,
                            5096218 as core::ffi::c_int as fe_limb_t,
                            22740376 as core::ffi::c_int as fe_limb_t,
                            26251015 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            2041139 as core::ffi::c_int as fe_limb_t,
                            19298082 as core::ffi::c_int as fe_limb_t,
                            7783686 as core::ffi::c_int as fe_limb_t,
                            13876377 as core::ffi::c_int as fe_limb_t,
                            41161879 as core::ffi::c_int as fe_limb_t,
                            20201972 as core::ffi::c_int as fe_limb_t,
                            24051123 as core::ffi::c_int as fe_limb_t,
                            13742383 as core::ffi::c_int as fe_limb_t,
                            51471265 as core::ffi::c_int as fe_limb_t,
                            13295221 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            33338218 as core::ffi::c_int as fe_limb_t,
                            25048699 as core::ffi::c_int as fe_limb_t,
                            12532112 as core::ffi::c_int as fe_limb_t,
                            7977527 as core::ffi::c_int as fe_limb_t,
                            9106186 as core::ffi::c_int as fe_limb_t,
                            31839181 as core::ffi::c_int as fe_limb_t,
                            49388668 as core::ffi::c_int as fe_limb_t,
                            28941459 as core::ffi::c_int as fe_limb_t,
                            62657506 as core::ffi::c_int as fe_limb_t,
                            18884987 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            47063583 as core::ffi::c_int as fe_limb_t,
                            5454096 as core::ffi::c_int as fe_limb_t,
                            52762316 as core::ffi::c_int as fe_limb_t,
                            6447145 as core::ffi::c_int as fe_limb_t,
                            28862071 as core::ffi::c_int as fe_limb_t,
                            1883651 as core::ffi::c_int as fe_limb_t,
                            64639598 as core::ffi::c_int as fe_limb_t,
                            29412551 as core::ffi::c_int as fe_limb_t,
                            7770568 as core::ffi::c_int as fe_limb_t,
                            9620597 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            23208049 as core::ffi::c_int as fe_limb_t,
                            7979712 as core::ffi::c_int as fe_limb_t,
                            33071466 as core::ffi::c_int as fe_limb_t,
                            8149229 as core::ffi::c_int as fe_limb_t,
                            1758231 as core::ffi::c_int as fe_limb_t,
                            22719437 as core::ffi::c_int as fe_limb_t,
                            30945527 as core::ffi::c_int as fe_limb_t,
                            31860109 as core::ffi::c_int as fe_limb_t,
                            33606523 as core::ffi::c_int as fe_limb_t,
                            18786461 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            1439939 as core::ffi::c_int as fe_limb_t,
                            17283952 as core::ffi::c_int as fe_limb_t,
                            66028874 as core::ffi::c_int as fe_limb_t,
                            32760649 as core::ffi::c_int as fe_limb_t,
                            4625401 as core::ffi::c_int as fe_limb_t,
                            10647766 as core::ffi::c_int as fe_limb_t,
                            62065063 as core::ffi::c_int as fe_limb_t,
                            1220117 as core::ffi::c_int as fe_limb_t,
                            30494170 as core::ffi::c_int as fe_limb_t,
                            22113633 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            62071265 as core::ffi::c_int as fe_limb_t,
                            20526136 as core::ffi::c_int as fe_limb_t,
                            64138304 as core::ffi::c_int as fe_limb_t,
                            30492664 as core::ffi::c_int as fe_limb_t,
                            15640973 as core::ffi::c_int as fe_limb_t,
                            26852766 as core::ffi::c_int as fe_limb_t,
                            40369837 as core::ffi::c_int as fe_limb_t,
                            926049 as core::ffi::c_int as fe_limb_t,
                            65424525 as core::ffi::c_int as fe_limb_t,
                            20220784 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            13908495 as core::ffi::c_int as fe_limb_t,
                            30005160 as core::ffi::c_int as fe_limb_t,
                            30919927 as core::ffi::c_int as fe_limb_t,
                            27280607 as core::ffi::c_int as fe_limb_t,
                            45587000 as core::ffi::c_int as fe_limb_t,
                            7989038 as core::ffi::c_int as fe_limb_t,
                            9021034 as core::ffi::c_int as fe_limb_t,
                            9078865 as core::ffi::c_int as fe_limb_t,
                            3353509 as core::ffi::c_int as fe_limb_t,
                            4033511 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            37445433 as core::ffi::c_int as fe_limb_t,
                            18440821 as core::ffi::c_int as fe_limb_t,
                            32259990 as core::ffi::c_int as fe_limb_t,
                            33209950 as core::ffi::c_int as fe_limb_t,
                            24295848 as core::ffi::c_int as fe_limb_t,
                            20642309 as core::ffi::c_int as fe_limb_t,
                            23161162 as core::ffi::c_int as fe_limb_t,
                            8839127 as core::ffi::c_int as fe_limb_t,
                            27485041 as core::ffi::c_int as fe_limb_t,
                            7356032 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
    [
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            9661008 as core::ffi::c_int as fe_limb_t,
                            705443 as core::ffi::c_int as fe_limb_t,
                            11980065 as core::ffi::c_int as fe_limb_t,
                            28184278 as core::ffi::c_int as fe_limb_t,
                            65480320 as core::ffi::c_int as fe_limb_t,
                            14661172 as core::ffi::c_int as fe_limb_t,
                            60762722 as core::ffi::c_int as fe_limb_t,
                            2625014 as core::ffi::c_int as fe_limb_t,
                            28431036 as core::ffi::c_int as fe_limb_t,
                            16782598 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            43269631 as core::ffi::c_int as fe_limb_t,
                            25243016 as core::ffi::c_int as fe_limb_t,
                            41163352 as core::ffi::c_int as fe_limb_t,
                            7480957 as core::ffi::c_int as fe_limb_t,
                            49427195 as core::ffi::c_int as fe_limb_t,
                            25200248 as core::ffi::c_int as fe_limb_t,
                            44562891 as core::ffi::c_int as fe_limb_t,
                            14150564 as core::ffi::c_int as fe_limb_t,
                            15970762 as core::ffi::c_int as fe_limb_t,
                            4099461 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            29262576 as core::ffi::c_int as fe_limb_t,
                            16756590 as core::ffi::c_int as fe_limb_t,
                            26350592 as core::ffi::c_int as fe_limb_t,
                            24760869 as core::ffi::c_int as fe_limb_t,
                            8529670 as core::ffi::c_int as fe_limb_t,
                            22346382 as core::ffi::c_int as fe_limb_t,
                            13617292 as core::ffi::c_int as fe_limb_t,
                            23617289 as core::ffi::c_int as fe_limb_t,
                            11465738 as core::ffi::c_int as fe_limb_t,
                            8317062 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            41615764 as core::ffi::c_int as fe_limb_t,
                            26591503 as core::ffi::c_int as fe_limb_t,
                            32500199 as core::ffi::c_int as fe_limb_t,
                            24135381 as core::ffi::c_int as fe_limb_t,
                            44070139 as core::ffi::c_int as fe_limb_t,
                            31252209 as core::ffi::c_int as fe_limb_t,
                            14898636 as core::ffi::c_int as fe_limb_t,
                            3848455 as core::ffi::c_int as fe_limb_t,
                            20969334 as core::ffi::c_int as fe_limb_t,
                            28396916 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            46724414 as core::ffi::c_int as fe_limb_t,
                            19206718 as core::ffi::c_int as fe_limb_t,
                            48772458 as core::ffi::c_int as fe_limb_t,
                            13884721 as core::ffi::c_int as fe_limb_t,
                            34069410 as core::ffi::c_int as fe_limb_t,
                            2842113 as core::ffi::c_int as fe_limb_t,
                            45498038 as core::ffi::c_int as fe_limb_t,
                            29904543 as core::ffi::c_int as fe_limb_t,
                            11177094 as core::ffi::c_int as fe_limb_t,
                            14989547 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            42612143 as core::ffi::c_int as fe_limb_t,
                            21838415 as core::ffi::c_int as fe_limb_t,
                            16959895 as core::ffi::c_int as fe_limb_t,
                            2278463 as core::ffi::c_int as fe_limb_t,
                            12066309 as core::ffi::c_int as fe_limb_t,
                            10137771 as core::ffi::c_int as fe_limb_t,
                            13515641 as core::ffi::c_int as fe_limb_t,
                            2581286 as core::ffi::c_int as fe_limb_t,
                            38621356 as core::ffi::c_int as fe_limb_t,
                            9930239 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            49357223 as core::ffi::c_int as fe_limb_t,
                            31456605 as core::ffi::c_int as fe_limb_t,
                            16544299 as core::ffi::c_int as fe_limb_t,
                            20545132 as core::ffi::c_int as fe_limb_t,
                            51194056 as core::ffi::c_int as fe_limb_t,
                            18605350 as core::ffi::c_int as fe_limb_t,
                            18345766 as core::ffi::c_int as fe_limb_t,
                            20150679 as core::ffi::c_int as fe_limb_t,
                            16291480 as core::ffi::c_int as fe_limb_t,
                            28240394 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            33879670 as core::ffi::c_int as fe_limb_t,
                            2553287 as core::ffi::c_int as fe_limb_t,
                            32678213 as core::ffi::c_int as fe_limb_t,
                            9875984 as core::ffi::c_int as fe_limb_t,
                            8534129 as core::ffi::c_int as fe_limb_t,
                            6889387 as core::ffi::c_int as fe_limb_t,
                            57432090 as core::ffi::c_int as fe_limb_t,
                            6957616 as core::ffi::c_int as fe_limb_t,
                            4368891 as core::ffi::c_int as fe_limb_t,
                            9788741 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            16660737 as core::ffi::c_int as fe_limb_t,
                            7281060 as core::ffi::c_int as fe_limb_t,
                            56278106 as core::ffi::c_int as fe_limb_t,
                            12911819 as core::ffi::c_int as fe_limb_t,
                            20108584 as core::ffi::c_int as fe_limb_t,
                            25452756 as core::ffi::c_int as fe_limb_t,
                            45386327 as core::ffi::c_int as fe_limb_t,
                            24941283 as core::ffi::c_int as fe_limb_t,
                            16250551 as core::ffi::c_int as fe_limb_t,
                            22443329 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            47343357 as core::ffi::c_int as fe_limb_t,
                            2390525 as core::ffi::c_int as fe_limb_t,
                            50557833 as core::ffi::c_int as fe_limb_t,
                            14161979 as core::ffi::c_int as fe_limb_t,
                            1905286 as core::ffi::c_int as fe_limb_t,
                            6414907 as core::ffi::c_int as fe_limb_t,
                            4689584 as core::ffi::c_int as fe_limb_t,
                            10604807 as core::ffi::c_int as fe_limb_t,
                            36918461 as core::ffi::c_int as fe_limb_t,
                            4782746 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            65754325 as core::ffi::c_int as fe_limb_t,
                            14736940 as core::ffi::c_int as fe_limb_t,
                            59741422 as core::ffi::c_int as fe_limb_t,
                            20261545 as core::ffi::c_int as fe_limb_t,
                            7710541 as core::ffi::c_int as fe_limb_t,
                            19398842 as core::ffi::c_int as fe_limb_t,
                            57127292 as core::ffi::c_int as fe_limb_t,
                            4383044 as core::ffi::c_int as fe_limb_t,
                            22546403 as core::ffi::c_int as fe_limb_t,
                            437323 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            31665558 as core::ffi::c_int as fe_limb_t,
                            21373968 as core::ffi::c_int as fe_limb_t,
                            50922033 as core::ffi::c_int as fe_limb_t,
                            1491338 as core::ffi::c_int as fe_limb_t,
                            48740239 as core::ffi::c_int as fe_limb_t,
                            3294681 as core::ffi::c_int as fe_limb_t,
                            27343084 as core::ffi::c_int as fe_limb_t,
                            2786261 as core::ffi::c_int as fe_limb_t,
                            36475274 as core::ffi::c_int as fe_limb_t,
                            19457415 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            52641566 as core::ffi::c_int as fe_limb_t,
                            32870716 as core::ffi::c_int as fe_limb_t,
                            33734756 as core::ffi::c_int as fe_limb_t,
                            7448551 as core::ffi::c_int as fe_limb_t,
                            19294360 as core::ffi::c_int as fe_limb_t,
                            14334329 as core::ffi::c_int as fe_limb_t,
                            47418233 as core::ffi::c_int as fe_limb_t,
                            2355318 as core::ffi::c_int as fe_limb_t,
                            47824193 as core::ffi::c_int as fe_limb_t,
                            27440058 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            15121312 as core::ffi::c_int as fe_limb_t,
                            17758270 as core::ffi::c_int as fe_limb_t,
                            6377019 as core::ffi::c_int as fe_limb_t,
                            27523071 as core::ffi::c_int as fe_limb_t,
                            56310752 as core::ffi::c_int as fe_limb_t,
                            20596586 as core::ffi::c_int as fe_limb_t,
                            18952176 as core::ffi::c_int as fe_limb_t,
                            15496498 as core::ffi::c_int as fe_limb_t,
                            37728731 as core::ffi::c_int as fe_limb_t,
                            11754227 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            64471568 as core::ffi::c_int as fe_limb_t,
                            20071356 as core::ffi::c_int as fe_limb_t,
                            8488726 as core::ffi::c_int as fe_limb_t,
                            19250536 as core::ffi::c_int as fe_limb_t,
                            12728760 as core::ffi::c_int as fe_limb_t,
                            31931939 as core::ffi::c_int as fe_limb_t,
                            7141595 as core::ffi::c_int as fe_limb_t,
                            11724556 as core::ffi::c_int as fe_limb_t,
                            22761615 as core::ffi::c_int as fe_limb_t,
                            23420291 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            16918416 as core::ffi::c_int as fe_limb_t,
                            11729663 as core::ffi::c_int as fe_limb_t,
                            49025285 as core::ffi::c_int as fe_limb_t,
                            3022986 as core::ffi::c_int as fe_limb_t,
                            36093132 as core::ffi::c_int as fe_limb_t,
                            20214772 as core::ffi::c_int as fe_limb_t,
                            38367678 as core::ffi::c_int as fe_limb_t,
                            21327038 as core::ffi::c_int as fe_limb_t,
                            32851221 as core::ffi::c_int as fe_limb_t,
                            11717399 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            11166615 as core::ffi::c_int as fe_limb_t,
                            7338049 as core::ffi::c_int as fe_limb_t,
                            60386341 as core::ffi::c_int as fe_limb_t,
                            4531519 as core::ffi::c_int as fe_limb_t,
                            37640192 as core::ffi::c_int as fe_limb_t,
                            26252376 as core::ffi::c_int as fe_limb_t,
                            31474878 as core::ffi::c_int as fe_limb_t,
                            3483633 as core::ffi::c_int as fe_limb_t,
                            65915689 as core::ffi::c_int as fe_limb_t,
                            29523600 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            66923210 as core::ffi::c_int as fe_limb_t,
                            9921304 as core::ffi::c_int as fe_limb_t,
                            31456609 as core::ffi::c_int as fe_limb_t,
                            20017994 as core::ffi::c_int as fe_limb_t,
                            55095045 as core::ffi::c_int as fe_limb_t,
                            13348922 as core::ffi::c_int as fe_limb_t,
                            33142652 as core::ffi::c_int as fe_limb_t,
                            6546660 as core::ffi::c_int as fe_limb_t,
                            47123585 as core::ffi::c_int as fe_limb_t,
                            29606055 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            34648249 as core::ffi::c_int as fe_limb_t,
                            11266711 as core::ffi::c_int as fe_limb_t,
                            55911757 as core::ffi::c_int as fe_limb_t,
                            25655328 as core::ffi::c_int as fe_limb_t,
                            31703693 as core::ffi::c_int as fe_limb_t,
                            3855903 as core::ffi::c_int as fe_limb_t,
                            58571733 as core::ffi::c_int as fe_limb_t,
                            20721383 as core::ffi::c_int as fe_limb_t,
                            36336829 as core::ffi::c_int as fe_limb_t,
                            18068118 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            49102387 as core::ffi::c_int as fe_limb_t,
                            12709067 as core::ffi::c_int as fe_limb_t,
                            3991746 as core::ffi::c_int as fe_limb_t,
                            27075244 as core::ffi::c_int as fe_limb_t,
                            45617340 as core::ffi::c_int as fe_limb_t,
                            23004006 as core::ffi::c_int as fe_limb_t,
                            35973516 as core::ffi::c_int as fe_limb_t,
                            17504552 as core::ffi::c_int as fe_limb_t,
                            10928916 as core::ffi::c_int as fe_limb_t,
                            3011958 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            60151107 as core::ffi::c_int as fe_limb_t,
                            17960094 as core::ffi::c_int as fe_limb_t,
                            31696058 as core::ffi::c_int as fe_limb_t,
                            334240 as core::ffi::c_int as fe_limb_t,
                            29576716 as core::ffi::c_int as fe_limb_t,
                            14796075 as core::ffi::c_int as fe_limb_t,
                            36277808 as core::ffi::c_int as fe_limb_t,
                            20749251 as core::ffi::c_int as fe_limb_t,
                            18008030 as core::ffi::c_int as fe_limb_t,
                            10258577 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
        {
            let init = ge_precomp {
                yplusx: {
                    let init = fe_loose {
                        v: [
                            44660220 as core::ffi::c_int as fe_limb_t,
                            15655568 as core::ffi::c_int as fe_limb_t,
                            7018479 as core::ffi::c_int as fe_limb_t,
                            29144429 as core::ffi::c_int as fe_limb_t,
                            36794597 as core::ffi::c_int as fe_limb_t,
                            32352840 as core::ffi::c_int as fe_limb_t,
                            65255398 as core::ffi::c_int as fe_limb_t,
                            1367119 as core::ffi::c_int as fe_limb_t,
                            25127874 as core::ffi::c_int as fe_limb_t,
                            6671743 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                yminusx: {
                    let init = fe_loose {
                        v: [
                            29701166 as core::ffi::c_int as fe_limb_t,
                            19180498 as core::ffi::c_int as fe_limb_t,
                            56230743 as core::ffi::c_int as fe_limb_t,
                            9279287 as core::ffi::c_int as fe_limb_t,
                            67091296 as core::ffi::c_int as fe_limb_t,
                            13127209 as core::ffi::c_int as fe_limb_t,
                            21382910 as core::ffi::c_int as fe_limb_t,
                            11042292 as core::ffi::c_int as fe_limb_t,
                            25838796 as core::ffi::c_int as fe_limb_t,
                            4642684 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
                xy2d: {
                    let init = fe_loose {
                        v: [
                            46678630 as core::ffi::c_int as fe_limb_t,
                            14955536 as core::ffi::c_int as fe_limb_t,
                            42982517 as core::ffi::c_int as fe_limb_t,
                            8124618 as core::ffi::c_int as fe_limb_t,
                            61739576 as core::ffi::c_int as fe_limb_t,
                            27563961 as core::ffi::c_int as fe_limb_t,
                            30468146 as core::ffi::c_int as fe_limb_t,
                            19653792 as core::ffi::c_int as fe_limb_t,
                            18423288 as core::ffi::c_int as fe_limb_t,
                            4177476 as core::ffi::c_int as fe_limb_t,
                        ],
                    };
                    init
                },
            };
            init
        },
    ],
];
static mut Bi: [ge_precomp; 8] = [
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        25967493 as core::ffi::c_int as fe_limb_t,
                        19198397 as core::ffi::c_int as fe_limb_t,
                        29566455 as core::ffi::c_int as fe_limb_t,
                        3660896 as core::ffi::c_int as fe_limb_t,
                        54414519 as core::ffi::c_int as fe_limb_t,
                        4014786 as core::ffi::c_int as fe_limb_t,
                        27544626 as core::ffi::c_int as fe_limb_t,
                        21800161 as core::ffi::c_int as fe_limb_t,
                        61029707 as core::ffi::c_int as fe_limb_t,
                        2047604 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        54563134 as core::ffi::c_int as fe_limb_t,
                        934261 as core::ffi::c_int as fe_limb_t,
                        64385954 as core::ffi::c_int as fe_limb_t,
                        3049989 as core::ffi::c_int as fe_limb_t,
                        66381436 as core::ffi::c_int as fe_limb_t,
                        9406985 as core::ffi::c_int as fe_limb_t,
                        12720692 as core::ffi::c_int as fe_limb_t,
                        5043384 as core::ffi::c_int as fe_limb_t,
                        19500929 as core::ffi::c_int as fe_limb_t,
                        18085054 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        58370664 as core::ffi::c_int as fe_limb_t,
                        4489569 as core::ffi::c_int as fe_limb_t,
                        9688441 as core::ffi::c_int as fe_limb_t,
                        18769238 as core::ffi::c_int as fe_limb_t,
                        10184608 as core::ffi::c_int as fe_limb_t,
                        21191052 as core::ffi::c_int as fe_limb_t,
                        29287918 as core::ffi::c_int as fe_limb_t,
                        11864899 as core::ffi::c_int as fe_limb_t,
                        42594502 as core::ffi::c_int as fe_limb_t,
                        29115885 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        15636272 as core::ffi::c_int as fe_limb_t,
                        23865875 as core::ffi::c_int as fe_limb_t,
                        24204772 as core::ffi::c_int as fe_limb_t,
                        25642034 as core::ffi::c_int as fe_limb_t,
                        616976 as core::ffi::c_int as fe_limb_t,
                        16869170 as core::ffi::c_int as fe_limb_t,
                        27787599 as core::ffi::c_int as fe_limb_t,
                        18782243 as core::ffi::c_int as fe_limb_t,
                        28944399 as core::ffi::c_int as fe_limb_t,
                        32004408 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        16568933 as core::ffi::c_int as fe_limb_t,
                        4717097 as core::ffi::c_int as fe_limb_t,
                        55552716 as core::ffi::c_int as fe_limb_t,
                        32452109 as core::ffi::c_int as fe_limb_t,
                        15682895 as core::ffi::c_int as fe_limb_t,
                        21747389 as core::ffi::c_int as fe_limb_t,
                        16354576 as core::ffi::c_int as fe_limb_t,
                        21778470 as core::ffi::c_int as fe_limb_t,
                        7689661 as core::ffi::c_int as fe_limb_t,
                        11199574 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        30464137 as core::ffi::c_int as fe_limb_t,
                        27578307 as core::ffi::c_int as fe_limb_t,
                        55329429 as core::ffi::c_int as fe_limb_t,
                        17883566 as core::ffi::c_int as fe_limb_t,
                        23220364 as core::ffi::c_int as fe_limb_t,
                        15915852 as core::ffi::c_int as fe_limb_t,
                        7512774 as core::ffi::c_int as fe_limb_t,
                        10017326 as core::ffi::c_int as fe_limb_t,
                        49359771 as core::ffi::c_int as fe_limb_t,
                        23634074 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        10861363 as core::ffi::c_int as fe_limb_t,
                        11473154 as core::ffi::c_int as fe_limb_t,
                        27284546 as core::ffi::c_int as fe_limb_t,
                        1981175 as core::ffi::c_int as fe_limb_t,
                        37044515 as core::ffi::c_int as fe_limb_t,
                        12577860 as core::ffi::c_int as fe_limb_t,
                        32867885 as core::ffi::c_int as fe_limb_t,
                        14515107 as core::ffi::c_int as fe_limb_t,
                        51670560 as core::ffi::c_int as fe_limb_t,
                        10819379 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        4708026 as core::ffi::c_int as fe_limb_t,
                        6336745 as core::ffi::c_int as fe_limb_t,
                        20377586 as core::ffi::c_int as fe_limb_t,
                        9066809 as core::ffi::c_int as fe_limb_t,
                        55836755 as core::ffi::c_int as fe_limb_t,
                        6594695 as core::ffi::c_int as fe_limb_t,
                        41455196 as core::ffi::c_int as fe_limb_t,
                        12483687 as core::ffi::c_int as fe_limb_t,
                        54440373 as core::ffi::c_int as fe_limb_t,
                        5581305 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        19563141 as core::ffi::c_int as fe_limb_t,
                        16186464 as core::ffi::c_int as fe_limb_t,
                        37722007 as core::ffi::c_int as fe_limb_t,
                        4097518 as core::ffi::c_int as fe_limb_t,
                        10237984 as core::ffi::c_int as fe_limb_t,
                        29206317 as core::ffi::c_int as fe_limb_t,
                        28542349 as core::ffi::c_int as fe_limb_t,
                        13850243 as core::ffi::c_int as fe_limb_t,
                        43430843 as core::ffi::c_int as fe_limb_t,
                        17738489 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        5153727 as core::ffi::c_int as fe_limb_t,
                        9909285 as core::ffi::c_int as fe_limb_t,
                        1723747 as core::ffi::c_int as fe_limb_t,
                        30776558 as core::ffi::c_int as fe_limb_t,
                        30523604 as core::ffi::c_int as fe_limb_t,
                        5516873 as core::ffi::c_int as fe_limb_t,
                        19480852 as core::ffi::c_int as fe_limb_t,
                        5230134 as core::ffi::c_int as fe_limb_t,
                        43156425 as core::ffi::c_int as fe_limb_t,
                        18378665 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        36839857 as core::ffi::c_int as fe_limb_t,
                        30090922 as core::ffi::c_int as fe_limb_t,
                        7665485 as core::ffi::c_int as fe_limb_t,
                        10083793 as core::ffi::c_int as fe_limb_t,
                        28475525 as core::ffi::c_int as fe_limb_t,
                        1649722 as core::ffi::c_int as fe_limb_t,
                        20654025 as core::ffi::c_int as fe_limb_t,
                        16520125 as core::ffi::c_int as fe_limb_t,
                        30598449 as core::ffi::c_int as fe_limb_t,
                        7715701 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        28881826 as core::ffi::c_int as fe_limb_t,
                        14381568 as core::ffi::c_int as fe_limb_t,
                        9657904 as core::ffi::c_int as fe_limb_t,
                        3680757 as core::ffi::c_int as fe_limb_t,
                        46927229 as core::ffi::c_int as fe_limb_t,
                        7843315 as core::ffi::c_int as fe_limb_t,
                        35708204 as core::ffi::c_int as fe_limb_t,
                        1370707 as core::ffi::c_int as fe_limb_t,
                        29794553 as core::ffi::c_int as fe_limb_t,
                        32145132 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        44589871 as core::ffi::c_int as fe_limb_t,
                        26862249 as core::ffi::c_int as fe_limb_t,
                        14201701 as core::ffi::c_int as fe_limb_t,
                        24808930 as core::ffi::c_int as fe_limb_t,
                        43598457 as core::ffi::c_int as fe_limb_t,
                        8844725 as core::ffi::c_int as fe_limb_t,
                        18474211 as core::ffi::c_int as fe_limb_t,
                        32192982 as core::ffi::c_int as fe_limb_t,
                        54046167 as core::ffi::c_int as fe_limb_t,
                        13821876 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        60653668 as core::ffi::c_int as fe_limb_t,
                        25714560 as core::ffi::c_int as fe_limb_t,
                        3374701 as core::ffi::c_int as fe_limb_t,
                        28813570 as core::ffi::c_int as fe_limb_t,
                        40010246 as core::ffi::c_int as fe_limb_t,
                        22982724 as core::ffi::c_int as fe_limb_t,
                        31655027 as core::ffi::c_int as fe_limb_t,
                        26342105 as core::ffi::c_int as fe_limb_t,
                        18853321 as core::ffi::c_int as fe_limb_t,
                        19333481 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        4566811 as core::ffi::c_int as fe_limb_t,
                        20590564 as core::ffi::c_int as fe_limb_t,
                        38133974 as core::ffi::c_int as fe_limb_t,
                        21313742 as core::ffi::c_int as fe_limb_t,
                        59506191 as core::ffi::c_int as fe_limb_t,
                        30723862 as core::ffi::c_int as fe_limb_t,
                        58594505 as core::ffi::c_int as fe_limb_t,
                        23123294 as core::ffi::c_int as fe_limb_t,
                        2207752 as core::ffi::c_int as fe_limb_t,
                        30344648 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        41954014 as core::ffi::c_int as fe_limb_t,
                        29368610 as core::ffi::c_int as fe_limb_t,
                        29681143 as core::ffi::c_int as fe_limb_t,
                        7868801 as core::ffi::c_int as fe_limb_t,
                        60254203 as core::ffi::c_int as fe_limb_t,
                        24130566 as core::ffi::c_int as fe_limb_t,
                        54671499 as core::ffi::c_int as fe_limb_t,
                        32891431 as core::ffi::c_int as fe_limb_t,
                        35997400 as core::ffi::c_int as fe_limb_t,
                        17421995 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        25576264 as core::ffi::c_int as fe_limb_t,
                        30851218 as core::ffi::c_int as fe_limb_t,
                        7349803 as core::ffi::c_int as fe_limb_t,
                        21739588 as core::ffi::c_int as fe_limb_t,
                        16472781 as core::ffi::c_int as fe_limb_t,
                        9300885 as core::ffi::c_int as fe_limb_t,
                        3844789 as core::ffi::c_int as fe_limb_t,
                        15725684 as core::ffi::c_int as fe_limb_t,
                        171356 as core::ffi::c_int as fe_limb_t,
                        6466918 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        23103977 as core::ffi::c_int as fe_limb_t,
                        13316479 as core::ffi::c_int as fe_limb_t,
                        9739013 as core::ffi::c_int as fe_limb_t,
                        17404951 as core::ffi::c_int as fe_limb_t,
                        817874 as core::ffi::c_int as fe_limb_t,
                        18515490 as core::ffi::c_int as fe_limb_t,
                        8965338 as core::ffi::c_int as fe_limb_t,
                        19466374 as core::ffi::c_int as fe_limb_t,
                        36393951 as core::ffi::c_int as fe_limb_t,
                        16193876 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        33587053 as core::ffi::c_int as fe_limb_t,
                        3180712 as core::ffi::c_int as fe_limb_t,
                        64714734 as core::ffi::c_int as fe_limb_t,
                        14003686 as core::ffi::c_int as fe_limb_t,
                        50205390 as core::ffi::c_int as fe_limb_t,
                        17283591 as core::ffi::c_int as fe_limb_t,
                        17238397 as core::ffi::c_int as fe_limb_t,
                        4729455 as core::ffi::c_int as fe_limb_t,
                        49034351 as core::ffi::c_int as fe_limb_t,
                        9256799 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        41926547 as core::ffi::c_int as fe_limb_t,
                        29380300 as core::ffi::c_int as fe_limb_t,
                        32336397 as core::ffi::c_int as fe_limb_t,
                        5036987 as core::ffi::c_int as fe_limb_t,
                        45872047 as core::ffi::c_int as fe_limb_t,
                        11360616 as core::ffi::c_int as fe_limb_t,
                        22616405 as core::ffi::c_int as fe_limb_t,
                        9761698 as core::ffi::c_int as fe_limb_t,
                        47281666 as core::ffi::c_int as fe_limb_t,
                        630304 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        53388152 as core::ffi::c_int as fe_limb_t,
                        2639452 as core::ffi::c_int as fe_limb_t,
                        42871404 as core::ffi::c_int as fe_limb_t,
                        26147950 as core::ffi::c_int as fe_limb_t,
                        9494426 as core::ffi::c_int as fe_limb_t,
                        27780403 as core::ffi::c_int as fe_limb_t,
                        60554312 as core::ffi::c_int as fe_limb_t,
                        17593437 as core::ffi::c_int as fe_limb_t,
                        64659607 as core::ffi::c_int as fe_limb_t,
                        19263131 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
    {
        let init = ge_precomp {
            yplusx: {
                let init = fe_loose {
                    v: [
                        63957664 as core::ffi::c_int as fe_limb_t,
                        28508356 as core::ffi::c_int as fe_limb_t,
                        9282713 as core::ffi::c_int as fe_limb_t,
                        6866145 as core::ffi::c_int as fe_limb_t,
                        35201802 as core::ffi::c_int as fe_limb_t,
                        32691408 as core::ffi::c_int as fe_limb_t,
                        48168288 as core::ffi::c_int as fe_limb_t,
                        15033783 as core::ffi::c_int as fe_limb_t,
                        25105118 as core::ffi::c_int as fe_limb_t,
                        25659556 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            yminusx: {
                let init = fe_loose {
                    v: [
                        42782475 as core::ffi::c_int as fe_limb_t,
                        15950225 as core::ffi::c_int as fe_limb_t,
                        35307649 as core::ffi::c_int as fe_limb_t,
                        18961608 as core::ffi::c_int as fe_limb_t,
                        55446126 as core::ffi::c_int as fe_limb_t,
                        28463506 as core::ffi::c_int as fe_limb_t,
                        1573891 as core::ffi::c_int as fe_limb_t,
                        30928545 as core::ffi::c_int as fe_limb_t,
                        2198789 as core::ffi::c_int as fe_limb_t,
                        17749813 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
            xy2d: {
                let init = fe_loose {
                    v: [
                        64009494 as core::ffi::c_int as fe_limb_t,
                        10324966 as core::ffi::c_int as fe_limb_t,
                        64867251 as core::ffi::c_int as fe_limb_t,
                        7453182 as core::ffi::c_int as fe_limb_t,
                        61661885 as core::ffi::c_int as fe_limb_t,
                        30818928 as core::ffi::c_int as fe_limb_t,
                        53296841 as core::ffi::c_int as fe_limb_t,
                        17317989 as core::ffi::c_int as fe_limb_t,
                        34647629 as core::ffi::c_int as fe_limb_t,
                        21263748 as core::ffi::c_int as fe_limb_t,
                    ],
                };
                init
            },
        };
        init
    },
];
unsafe extern "C" fn fiat_25519_addcarryx_u26(
    out1: *mut uint32_t,
    out2: *mut fiat_25519_uint1,
    arg1: fiat_25519_uint1,
    arg2: uint32_t,
    arg3: uint32_t,
) {
    let x1: uint32_t = (arg1 as core::ffi::c_uint)
        .wrapping_add(arg2)
        .wrapping_add(arg3);
    let x2: uint32_t = x1 & 0x3ffffff as core::ffi::c_uint;
    let x3: fiat_25519_uint1 = (x1 >> 26 as core::ffi::c_int) as fiat_25519_uint1;
    *out1 = x2;
    *out2 = x3;
}
unsafe extern "C" fn fiat_25519_subborrowx_u26(
    out1: *mut uint32_t,
    out2: *mut fiat_25519_uint1,
    arg1: fiat_25519_uint1,
    arg2: uint32_t,
    arg3: uint32_t,
) {
    let x1: int32_t = arg2.wrapping_sub(arg1 as core::ffi::c_uint) as int32_t - arg3 as int32_t;
    let x2: fiat_25519_int1 = (x1 >> 26 as core::ffi::c_int) as fiat_25519_int1;
    let x3: uint32_t = x1 as core::ffi::c_uint & 0x3ffffff as core::ffi::c_uint;
    *out1 = x3;
    *out2 = (0 as core::ffi::c_int - x2 as core::ffi::c_int) as fiat_25519_uint1;
}
unsafe extern "C" fn fiat_25519_addcarryx_u25(
    out1: *mut uint32_t,
    out2: *mut fiat_25519_uint1,
    arg1: fiat_25519_uint1,
    arg2: uint32_t,
    arg3: uint32_t,
) {
    let x1: uint32_t = (arg1 as core::ffi::c_uint)
        .wrapping_add(arg2)
        .wrapping_add(arg3);
    let x2: uint32_t = x1 & 0x1ffffff as core::ffi::c_uint;
    let x3: fiat_25519_uint1 = (x1 >> 25 as core::ffi::c_int) as fiat_25519_uint1;
    *out1 = x2;
    *out2 = x3;
}
unsafe extern "C" fn fiat_25519_subborrowx_u25(
    out1: *mut uint32_t,
    out2: *mut fiat_25519_uint1,
    arg1: fiat_25519_uint1,
    arg2: uint32_t,
    arg3: uint32_t,
) {
    let x1: int32_t = arg2.wrapping_sub(arg1 as core::ffi::c_uint) as int32_t - arg3 as int32_t;
    let x2: fiat_25519_int1 = (x1 >> 25 as core::ffi::c_int) as fiat_25519_int1;
    let x3: uint32_t = x1 as core::ffi::c_uint & 0x1ffffff as core::ffi::c_uint;
    *out1 = x3;
    *out2 = (0 as core::ffi::c_int - x2 as core::ffi::c_int) as fiat_25519_uint1;
}
unsafe extern "C" fn fiat_25519_cmovznz_u32(
    out1: *mut uint32_t,
    arg1: fiat_25519_uint1,
    arg2: uint32_t,
    arg3: uint32_t,
) {
    let x1: fiat_25519_uint1 = (arg1 != 0) as core::ffi::c_int as fiat_25519_uint1;
    let x2: uint32_t = (0 as core::ffi::c_int - x1 as core::ffi::c_int) as fiat_25519_int1
        as core::ffi::c_uint
        & 0xffffffff as core::ffi::c_uint;
    let x3: uint32_t = value_barrier_u32(x2) & arg3 | value_barrier_u32(!x2) & arg2;
    *out1 = x3;
}
unsafe extern "C" fn fiat_25519_carry_mul(
    out1: *mut uint32_t,
    arg1: *const uint32_t,
    arg2: *const uint32_t,
) {
    let x1: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(9 as core::ffi::c_int as isize))
            .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x2: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(8 as core::ffi::c_int as isize))
            .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x3: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(7 as core::ffi::c_int as isize))
            .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x4: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(6 as core::ffi::c_int as isize))
            .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x5: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(5 as core::ffi::c_int as isize))
            .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x6: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(4 as core::ffi::c_int as isize))
            .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x7: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(3 as core::ffi::c_int as isize))
            .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x8: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(2 as core::ffi::c_int as isize))
            .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x9: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(
        (*arg2.offset(1 as core::ffi::c_int as isize))
            .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
    );
    let x10: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x11: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x12: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x13: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(6 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x14: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x15: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(4 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x16: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x17: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(2 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x18: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x19: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x20: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x21: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(6 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x22: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x23: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(4 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x24: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x25: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x26: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x27: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x28: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(6 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x29: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x30: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(4 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x31: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x32: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x33: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x34: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(6 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x35: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x36: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x37: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x38: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x39: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(6 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x40: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x41: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x42: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x43: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x44: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(8 as core::ffi::c_int as isize))
                .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x45: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(9 as core::ffi::c_int as isize))
                .wrapping_mul(0x26 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x46: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x47: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(1 as core::ffi::c_int as isize) as u64);
    let x48: uint64_t = (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x49: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x50: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(1 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x51: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x52: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(3 as core::ffi::c_int as isize) as u64);
    let x53: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x54: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(1 as core::ffi::c_int as isize) as u64);
    let x55: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x56: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x57: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x58: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x59: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(1 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x60: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x61: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(5 as core::ffi::c_int as isize) as u64);
    let x62: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x63: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(3 as core::ffi::c_int as isize) as u64);
    let x64: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x65: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(1 as core::ffi::c_int as isize) as u64);
    let x66: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x67: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(6 as core::ffi::c_int as isize) as u64);
    let x68: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x69: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x70: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x71: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x72: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(1 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x73: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x74: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(7 as core::ffi::c_int as isize) as u64);
    let x75: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(6 as core::ffi::c_int as isize) as u64);
    let x76: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(5 as core::ffi::c_int as isize) as u64);
    let x77: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x78: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(3 as core::ffi::c_int as isize) as u64);
    let x79: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x80: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(1 as core::ffi::c_int as isize) as u64);
    let x81: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x82: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(8 as core::ffi::c_int as isize) as u64);
    let x83: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(7 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x84: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(6 as core::ffi::c_int as isize) as u64);
    let x85: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(5 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x86: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x87: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x88: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x89: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg2.offset(1 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x90: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x91: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(9 as core::ffi::c_int as isize) as u64);
    let x92: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(8 as core::ffi::c_int as isize) as u64);
    let x93: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(7 as core::ffi::c_int as isize) as u64);
    let x94: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(6 as core::ffi::c_int as isize) as u64);
    let x95: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(5 as core::ffi::c_int as isize) as u64);
    let x96: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(4 as core::ffi::c_int as isize) as u64);
    let x97: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(3 as core::ffi::c_int as isize) as u64);
    let x98: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(2 as core::ffi::c_int as isize) as u64);
    let x99: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(1 as core::ffi::c_int as isize) as u64);
    let x100: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg2.offset(0 as core::ffi::c_int as isize) as u64);
    let x101: uint64_t = x100.wrapping_add(x45.wrapping_add(x44.wrapping_add(
        x42.wrapping_add(x39.wrapping_add(
            x35.wrapping_add(x30.wrapping_add(x24.wrapping_add(x17.wrapping_add(x9)))),
        )),
    )));
    let x102: uint64_t = x101 >> 26 as core::ffi::c_int;
    let x103: uint32_t = (x101 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x104: uint64_t = x91.wrapping_add(x82.wrapping_add(x74.wrapping_add(
        x67.wrapping_add(x61.wrapping_add(
            x56.wrapping_add(x52.wrapping_add(x49.wrapping_add(x47.wrapping_add(x46)))),
        )),
    )));
    let x105: uint64_t = x92.wrapping_add(x83.wrapping_add(x75.wrapping_add(
        x68.wrapping_add(x62.wrapping_add(
            x57.wrapping_add(x53.wrapping_add(x50.wrapping_add(x48.wrapping_add(x1)))),
        )),
    )));
    let x106: uint64_t = x93.wrapping_add(x84.wrapping_add(x76.wrapping_add(
        x69.wrapping_add(x63.wrapping_add(
            x58.wrapping_add(x54.wrapping_add(x51.wrapping_add(x10.wrapping_add(x2)))),
        )),
    )));
    let x107: uint64_t = x94.wrapping_add(x85.wrapping_add(x77.wrapping_add(
        x70.wrapping_add(x64.wrapping_add(
            x59.wrapping_add(x55.wrapping_add(x18.wrapping_add(x11.wrapping_add(x3)))),
        )),
    )));
    let x108: uint64_t = x95.wrapping_add(x86.wrapping_add(x78.wrapping_add(
        x71.wrapping_add(x65.wrapping_add(
            x60.wrapping_add(x25.wrapping_add(x19.wrapping_add(x12.wrapping_add(x4)))),
        )),
    )));
    let x109: uint64_t = x96.wrapping_add(x87.wrapping_add(x79.wrapping_add(
        x72.wrapping_add(x66.wrapping_add(
            x31.wrapping_add(x26.wrapping_add(x20.wrapping_add(x13.wrapping_add(x5)))),
        )),
    )));
    let x110: uint64_t = x97.wrapping_add(x88.wrapping_add(x80.wrapping_add(
        x73.wrapping_add(x36.wrapping_add(
            x32.wrapping_add(x27.wrapping_add(x21.wrapping_add(x14.wrapping_add(x6)))),
        )),
    )));
    let x111: uint64_t = x98.wrapping_add(x89.wrapping_add(x81.wrapping_add(
        x40.wrapping_add(x37.wrapping_add(
            x33.wrapping_add(x28.wrapping_add(x22.wrapping_add(x15.wrapping_add(x7)))),
        )),
    )));
    let x112: uint64_t = x99.wrapping_add(x90.wrapping_add(x43.wrapping_add(
        x41.wrapping_add(x38.wrapping_add(
            x34.wrapping_add(x29.wrapping_add(x23.wrapping_add(x16.wrapping_add(x8)))),
        )),
    )));
    let x113: uint64_t = x102.wrapping_add(x112);
    let x114: uint64_t = x113 >> 25 as core::ffi::c_int;
    let x115: uint32_t = (x113 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x116: uint64_t = x114.wrapping_add(x111);
    let x117: uint64_t = x116 >> 26 as core::ffi::c_int;
    let x118: uint32_t = (x116 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x119: uint64_t = x117.wrapping_add(x110);
    let x120: uint64_t = x119 >> 25 as core::ffi::c_int;
    let x121: uint32_t = (x119 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x122: uint64_t = x120.wrapping_add(x109);
    let x123: uint64_t = x122 >> 26 as core::ffi::c_int;
    let x124: uint32_t = (x122 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x125: uint64_t = x123.wrapping_add(x108);
    let x126: uint64_t = x125 >> 25 as core::ffi::c_int;
    let x127: uint32_t = (x125 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x128: uint64_t = x126.wrapping_add(x107);
    let x129: uint64_t = x128 >> 26 as core::ffi::c_int;
    let x130: uint32_t = (x128 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x131: uint64_t = x129.wrapping_add(x106);
    let x132: uint64_t = x131 >> 25 as core::ffi::c_int;
    let x133: uint32_t = (x131 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x134: uint64_t = x132.wrapping_add(x105);
    let x135: uint64_t = x134 >> 26 as core::ffi::c_int;
    let x136: uint32_t = (x134 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x137: uint64_t = x135.wrapping_add(x104);
    let x138: uint64_t = x137 >> 25 as core::ffi::c_int;
    let x139: uint32_t = (x137 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x140: uint64_t = x138.wrapping_mul(0x13 as core::ffi::c_int as u64);
    let x141: uint64_t = (x103 as u64).wrapping_add(x140);
    let x142: uint32_t = (x141 >> 26 as core::ffi::c_int) as uint32_t;
    let x143: uint32_t = (x141 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x144: uint32_t = x142.wrapping_add(x115);
    let x145: fiat_25519_uint1 = (x144 >> 25 as core::ffi::c_int) as fiat_25519_uint1;
    let x146: uint32_t = x144 & 0x1ffffff as core::ffi::c_uint;
    let x147: uint32_t = (x145 as core::ffi::c_uint).wrapping_add(x118);
    *out1.offset(0 as core::ffi::c_int as isize) = x143;
    *out1.offset(1 as core::ffi::c_int as isize) = x146;
    *out1.offset(2 as core::ffi::c_int as isize) = x147;
    *out1.offset(3 as core::ffi::c_int as isize) = x121;
    *out1.offset(4 as core::ffi::c_int as isize) = x124;
    *out1.offset(5 as core::ffi::c_int as isize) = x127;
    *out1.offset(6 as core::ffi::c_int as isize) = x130;
    *out1.offset(7 as core::ffi::c_int as isize) = x133;
    *out1.offset(8 as core::ffi::c_int as isize) = x136;
    *out1.offset(9 as core::ffi::c_int as isize) = x139;
}
unsafe extern "C" fn fiat_25519_carry_square(out1: *mut uint32_t, arg1: *const uint32_t) {
    let x1: uint32_t = (*arg1.offset(9 as core::ffi::c_int as isize))
        .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x2: uint32_t = x1.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x3: uint32_t = (*arg1.offset(9 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x4: uint32_t = (*arg1.offset(8 as core::ffi::c_int as isize))
        .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x5: uint64_t = (x4 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64);
    let x6: uint32_t = (*arg1.offset(8 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x7: uint32_t = (*arg1.offset(7 as core::ffi::c_int as isize))
        .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x8: uint32_t = x7.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x9: uint32_t = (*arg1.offset(7 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x10: uint32_t = (*arg1.offset(6 as core::ffi::c_int as isize))
        .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x11: uint64_t = (x10 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64);
    let x12: uint32_t = (*arg1.offset(6 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x13: uint32_t = (*arg1.offset(5 as core::ffi::c_int as isize))
        .wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x14: uint32_t = (*arg1.offset(5 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x15: uint32_t = (*arg1.offset(4 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x16: uint32_t = (*arg1.offset(3 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x17: uint32_t = (*arg1.offset(2 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x18: uint32_t = (*arg1.offset(1 as core::ffi::c_int as isize))
        .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint);
    let x19: uint64_t = (*arg1.offset(9 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x1.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x20: uint64_t =
        (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x2 as u64);
    let x21: uint64_t =
        (*arg1.offset(8 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x4 as u64);
    let x22: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x2 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x23: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x24: uint64_t = (*arg1.offset(7 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x7.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x25: uint64_t =
        (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x2 as u64);
    let x26: uint64_t = (*arg1.offset(6 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x27: uint64_t =
        (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x8 as u64);
    let x28: uint64_t =
        (*arg1.offset(6 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x10 as u64);
    let x29: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x2 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x30: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x31: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x8 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x32: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as u64).wrapping_mul(x11);
    let x33: uint64_t = (*arg1.offset(5 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x13.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x34: uint64_t =
        (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x2 as u64);
    let x35: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x36: uint64_t =
        (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x8 as u64);
    let x37: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as u64).wrapping_mul(x11);
    let x38: uint64_t =
        (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x14 as u64);
    let x39: uint64_t = (*arg1.offset(4 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg1.offset(4 as core::ffi::c_int as isize) as u64);
    let x40: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x2 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x41: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x42: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x8 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x43: uint64_t =
        (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x12 as u64);
    let x44: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x14.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x45: uint64_t =
        (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x15 as u64);
    let x46: uint64_t = (*arg1.offset(3 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg1.offset(3 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x47: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x2 as u64);
    let x48: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as u64).wrapping_mul(x5);
    let x49: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x9 as u64);
    let x50: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x12 as u64);
    let x51: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x14 as u64);
    let x52: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x15 as u64);
    let x53: uint64_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x16 as u64);
    let x54: uint64_t = (*arg1.offset(2 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg1.offset(2 as core::ffi::c_int as isize) as u64);
    let x55: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as u64)
        .wrapping_mul((x2 as uint64_t).wrapping_mul(0x2 as core::ffi::c_int as u64));
    let x56: uint64_t =
        (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x6 as u64);
    let x57: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x9.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x58: uint64_t =
        (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x12 as u64);
    let x59: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x14.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x60: uint64_t =
        (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x15 as u64);
    let x61: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(x16.wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64);
    let x62: uint64_t =
        (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x17 as u64);
    let x63: uint64_t = (*arg1.offset(1 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(
            (*arg1.offset(1 as core::ffi::c_int as isize))
                .wrapping_mul(0x2 as core::ffi::c_int as core::ffi::c_uint) as u64,
        );
    let x64: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x3 as u64);
    let x65: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x6 as u64);
    let x66: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x9 as u64);
    let x67: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x12 as u64);
    let x68: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x14 as u64);
    let x69: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x15 as u64);
    let x70: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x16 as u64);
    let x71: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x17 as u64);
    let x72: uint64_t =
        (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t).wrapping_mul(x18 as u64);
    let x73: uint64_t = (*arg1.offset(0 as core::ffi::c_int as isize) as uint64_t)
        .wrapping_mul(*arg1.offset(0 as core::ffi::c_int as isize) as u64);
    let x74: uint64_t = x73
        .wrapping_add(x55.wrapping_add(x48.wrapping_add(x42.wrapping_add(x37.wrapping_add(x33)))));
    let x75: uint64_t = x74 >> 26 as core::ffi::c_int;
    let x76: uint32_t = (x74 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x77: uint64_t =
        x64.wrapping_add(x56.wrapping_add(x49.wrapping_add(x43.wrapping_add(x38))));
    let x78: uint64_t = x65
        .wrapping_add(x57.wrapping_add(x50.wrapping_add(x44.wrapping_add(x39.wrapping_add(x19)))));
    let x79: uint64_t =
        x66.wrapping_add(x58.wrapping_add(x51.wrapping_add(x45.wrapping_add(x20))));
    let x80: uint64_t = x67
        .wrapping_add(x59.wrapping_add(x52.wrapping_add(x46.wrapping_add(x22.wrapping_add(x21)))));
    let x81: uint64_t =
        x68.wrapping_add(x60.wrapping_add(x53.wrapping_add(x25.wrapping_add(x23))));
    let x82: uint64_t = x69
        .wrapping_add(x61.wrapping_add(x54.wrapping_add(x29.wrapping_add(x26.wrapping_add(x24)))));
    let x83: uint64_t =
        x70.wrapping_add(x62.wrapping_add(x34.wrapping_add(x30.wrapping_add(x27))));
    let x84: uint64_t = x71
        .wrapping_add(x63.wrapping_add(x40.wrapping_add(x35.wrapping_add(x31.wrapping_add(x28)))));
    let x85: uint64_t =
        x72.wrapping_add(x47.wrapping_add(x41.wrapping_add(x36.wrapping_add(x32))));
    let x86: uint64_t = x75.wrapping_add(x85);
    let x87: uint64_t = x86 >> 25 as core::ffi::c_int;
    let x88: uint32_t = (x86 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x89: uint64_t = x87.wrapping_add(x84);
    let x90: uint64_t = x89 >> 26 as core::ffi::c_int;
    let x91: uint32_t = (x89 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x92: uint64_t = x90.wrapping_add(x83);
    let x93: uint64_t = x92 >> 25 as core::ffi::c_int;
    let x94: uint32_t = (x92 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x95: uint64_t = x93.wrapping_add(x82);
    let x96: uint64_t = x95 >> 26 as core::ffi::c_int;
    let x97: uint32_t = (x95 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x98: uint64_t = x96.wrapping_add(x81);
    let x99: uint64_t = x98 >> 25 as core::ffi::c_int;
    let x100: uint32_t = (x98 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x101: uint64_t = x99.wrapping_add(x80);
    let x102: uint64_t = x101 >> 26 as core::ffi::c_int;
    let x103: uint32_t = (x101 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x104: uint64_t = x102.wrapping_add(x79);
    let x105: uint64_t = x104 >> 25 as core::ffi::c_int;
    let x106: uint32_t = (x104 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x107: uint64_t = x105.wrapping_add(x78);
    let x108: uint64_t = x107 >> 26 as core::ffi::c_int;
    let x109: uint32_t = (x107 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x110: uint64_t = x108.wrapping_add(x77);
    let x111: uint64_t = x110 >> 25 as core::ffi::c_int;
    let x112: uint32_t = (x110 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x113: uint64_t = x111.wrapping_mul(0x13 as core::ffi::c_int as u64);
    let x114: uint64_t = (x76 as u64).wrapping_add(x113);
    let x115: uint32_t = (x114 >> 26 as core::ffi::c_int) as uint32_t;
    let x116: uint32_t = (x114 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x117: uint32_t = x115.wrapping_add(x88);
    let x118: fiat_25519_uint1 = (x117 >> 25 as core::ffi::c_int) as fiat_25519_uint1;
    let x119: uint32_t = x117 & 0x1ffffff as core::ffi::c_uint;
    let x120: uint32_t = (x118 as core::ffi::c_uint).wrapping_add(x91);
    *out1.offset(0 as core::ffi::c_int as isize) = x116;
    *out1.offset(1 as core::ffi::c_int as isize) = x119;
    *out1.offset(2 as core::ffi::c_int as isize) = x120;
    *out1.offset(3 as core::ffi::c_int as isize) = x94;
    *out1.offset(4 as core::ffi::c_int as isize) = x97;
    *out1.offset(5 as core::ffi::c_int as isize) = x100;
    *out1.offset(6 as core::ffi::c_int as isize) = x103;
    *out1.offset(7 as core::ffi::c_int as isize) = x106;
    *out1.offset(8 as core::ffi::c_int as isize) = x109;
    *out1.offset(9 as core::ffi::c_int as isize) = x112;
}
unsafe extern "C" fn fiat_25519_carry(out1: *mut uint32_t, arg1: *const uint32_t) {
    let x1: uint32_t = *arg1.offset(0 as core::ffi::c_int as isize);
    let x2: uint32_t =
        (x1 >> 26 as core::ffi::c_int).wrapping_add(*arg1.offset(1 as core::ffi::c_int as isize));
    let x3: uint32_t =
        (x2 >> 25 as core::ffi::c_int).wrapping_add(*arg1.offset(2 as core::ffi::c_int as isize));
    let x4: uint32_t =
        (x3 >> 26 as core::ffi::c_int).wrapping_add(*arg1.offset(3 as core::ffi::c_int as isize));
    let x5: uint32_t =
        (x4 >> 25 as core::ffi::c_int).wrapping_add(*arg1.offset(4 as core::ffi::c_int as isize));
    let x6: uint32_t =
        (x5 >> 26 as core::ffi::c_int).wrapping_add(*arg1.offset(5 as core::ffi::c_int as isize));
    let x7: uint32_t =
        (x6 >> 25 as core::ffi::c_int).wrapping_add(*arg1.offset(6 as core::ffi::c_int as isize));
    let x8: uint32_t =
        (x7 >> 26 as core::ffi::c_int).wrapping_add(*arg1.offset(7 as core::ffi::c_int as isize));
    let x9: uint32_t =
        (x8 >> 25 as core::ffi::c_int).wrapping_add(*arg1.offset(8 as core::ffi::c_int as isize));
    let x10: uint32_t =
        (x9 >> 26 as core::ffi::c_int).wrapping_add(*arg1.offset(9 as core::ffi::c_int as isize));
    let x11: uint32_t = (x1 & 0x3ffffff as core::ffi::c_uint).wrapping_add(
        (x10 >> 25 as core::ffi::c_int).wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint),
    );
    let x12: uint32_t = ((x11 >> 26 as core::ffi::c_int) as fiat_25519_uint1
        as core::ffi::c_uint)
        .wrapping_add(x2 & 0x1ffffff as core::ffi::c_uint);
    let x13: uint32_t = x11 & 0x3ffffff as core::ffi::c_uint;
    let x14: uint32_t = x12 & 0x1ffffff as core::ffi::c_uint;
    let x15: uint32_t = ((x12 >> 25 as core::ffi::c_int) as fiat_25519_uint1
        as core::ffi::c_uint)
        .wrapping_add(x3 & 0x3ffffff as core::ffi::c_uint);
    let x16: uint32_t = x4 & 0x1ffffff as core::ffi::c_uint;
    let x17: uint32_t = x5 & 0x3ffffff as core::ffi::c_uint;
    let x18: uint32_t = x6 & 0x1ffffff as core::ffi::c_uint;
    let x19: uint32_t = x7 & 0x3ffffff as core::ffi::c_uint;
    let x20: uint32_t = x8 & 0x1ffffff as core::ffi::c_uint;
    let x21: uint32_t = x9 & 0x3ffffff as core::ffi::c_uint;
    let x22: uint32_t = x10 & 0x1ffffff as core::ffi::c_uint;
    *out1.offset(0 as core::ffi::c_int as isize) = x13;
    *out1.offset(1 as core::ffi::c_int as isize) = x14;
    *out1.offset(2 as core::ffi::c_int as isize) = x15;
    *out1.offset(3 as core::ffi::c_int as isize) = x16;
    *out1.offset(4 as core::ffi::c_int as isize) = x17;
    *out1.offset(5 as core::ffi::c_int as isize) = x18;
    *out1.offset(6 as core::ffi::c_int as isize) = x19;
    *out1.offset(7 as core::ffi::c_int as isize) = x20;
    *out1.offset(8 as core::ffi::c_int as isize) = x21;
    *out1.offset(9 as core::ffi::c_int as isize) = x22;
}
unsafe extern "C" fn fiat_25519_add(
    out1: *mut uint32_t,
    arg1: *const uint32_t,
    arg2: *const uint32_t,
) {
    let x1: uint32_t = (*arg1.offset(0 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(0 as core::ffi::c_int as isize));
    let x2: uint32_t = (*arg1.offset(1 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(1 as core::ffi::c_int as isize));
    let x3: uint32_t = (*arg1.offset(2 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(2 as core::ffi::c_int as isize));
    let x4: uint32_t = (*arg1.offset(3 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(3 as core::ffi::c_int as isize));
    let x5: uint32_t = (*arg1.offset(4 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(4 as core::ffi::c_int as isize));
    let x6: uint32_t = (*arg1.offset(5 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(5 as core::ffi::c_int as isize));
    let x7: uint32_t = (*arg1.offset(6 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(6 as core::ffi::c_int as isize));
    let x8: uint32_t = (*arg1.offset(7 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(7 as core::ffi::c_int as isize));
    let x9: uint32_t = (*arg1.offset(8 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(8 as core::ffi::c_int as isize));
    let x10: uint32_t = (*arg1.offset(9 as core::ffi::c_int as isize))
        .wrapping_add(*arg2.offset(9 as core::ffi::c_int as isize));
    *out1.offset(0 as core::ffi::c_int as isize) = x1;
    *out1.offset(1 as core::ffi::c_int as isize) = x2;
    *out1.offset(2 as core::ffi::c_int as isize) = x3;
    *out1.offset(3 as core::ffi::c_int as isize) = x4;
    *out1.offset(4 as core::ffi::c_int as isize) = x5;
    *out1.offset(5 as core::ffi::c_int as isize) = x6;
    *out1.offset(6 as core::ffi::c_int as isize) = x7;
    *out1.offset(7 as core::ffi::c_int as isize) = x8;
    *out1.offset(8 as core::ffi::c_int as isize) = x9;
    *out1.offset(9 as core::ffi::c_int as isize) = x10;
}
unsafe extern "C" fn fiat_25519_sub(
    out1: *mut uint32_t,
    arg1: *const uint32_t,
    arg2: *const uint32_t,
) {
    let x1: uint32_t = (0x7ffffda as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(0 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(0 as core::ffi::c_int as isize));
    let x2: uint32_t = (0x3fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(1 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(1 as core::ffi::c_int as isize));
    let x3: uint32_t = (0x7fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(2 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(2 as core::ffi::c_int as isize));
    let x4: uint32_t = (0x3fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(3 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(3 as core::ffi::c_int as isize));
    let x5: uint32_t = (0x7fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(4 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(4 as core::ffi::c_int as isize));
    let x6: uint32_t = (0x3fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(5 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(5 as core::ffi::c_int as isize));
    let x7: uint32_t = (0x7fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(6 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(6 as core::ffi::c_int as isize));
    let x8: uint32_t = (0x3fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(7 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(7 as core::ffi::c_int as isize));
    let x9: uint32_t = (0x7fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(8 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(8 as core::ffi::c_int as isize));
    let x10: uint32_t = (0x3fffffe as core::ffi::c_uint)
        .wrapping_add(*arg1.offset(9 as core::ffi::c_int as isize))
        .wrapping_sub(*arg2.offset(9 as core::ffi::c_int as isize));
    *out1.offset(0 as core::ffi::c_int as isize) = x1;
    *out1.offset(1 as core::ffi::c_int as isize) = x2;
    *out1.offset(2 as core::ffi::c_int as isize) = x3;
    *out1.offset(3 as core::ffi::c_int as isize) = x4;
    *out1.offset(4 as core::ffi::c_int as isize) = x5;
    *out1.offset(5 as core::ffi::c_int as isize) = x6;
    *out1.offset(6 as core::ffi::c_int as isize) = x7;
    *out1.offset(7 as core::ffi::c_int as isize) = x8;
    *out1.offset(8 as core::ffi::c_int as isize) = x9;
    *out1.offset(9 as core::ffi::c_int as isize) = x10;
}
unsafe extern "C" fn fiat_25519_opp(out1: *mut uint32_t, arg1: *const uint32_t) {
    let x1: uint32_t =
        (0x7ffffda as core::ffi::c_uint).wrapping_sub(*arg1.offset(0 as core::ffi::c_int as isize));
    let x2: uint32_t =
        (0x3fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(1 as core::ffi::c_int as isize));
    let x3: uint32_t =
        (0x7fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(2 as core::ffi::c_int as isize));
    let x4: uint32_t =
        (0x3fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(3 as core::ffi::c_int as isize));
    let x5: uint32_t =
        (0x7fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(4 as core::ffi::c_int as isize));
    let x6: uint32_t =
        (0x3fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(5 as core::ffi::c_int as isize));
    let x7: uint32_t =
        (0x7fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(6 as core::ffi::c_int as isize));
    let x8: uint32_t =
        (0x3fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(7 as core::ffi::c_int as isize));
    let x9: uint32_t =
        (0x7fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(8 as core::ffi::c_int as isize));
    let x10: uint32_t =
        (0x3fffffe as core::ffi::c_uint).wrapping_sub(*arg1.offset(9 as core::ffi::c_int as isize));
    *out1.offset(0 as core::ffi::c_int as isize) = x1;
    *out1.offset(1 as core::ffi::c_int as isize) = x2;
    *out1.offset(2 as core::ffi::c_int as isize) = x3;
    *out1.offset(3 as core::ffi::c_int as isize) = x4;
    *out1.offset(4 as core::ffi::c_int as isize) = x5;
    *out1.offset(5 as core::ffi::c_int as isize) = x6;
    *out1.offset(6 as core::ffi::c_int as isize) = x7;
    *out1.offset(7 as core::ffi::c_int as isize) = x8;
    *out1.offset(8 as core::ffi::c_int as isize) = x9;
    *out1.offset(9 as core::ffi::c_int as isize) = x10;
}
unsafe extern "C" fn _25519_selectznz(
    out1: *mut uint32_t,
    arg1: fiat_25519_uint1,
    arg2: *const uint32_t,
    arg3: *const uint32_t,
) {
    let mut x1: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x1,
        arg1,
        *arg2.offset(0 as core::ffi::c_int as isize),
        *arg3.offset(0 as core::ffi::c_int as isize),
    );
    let mut x2: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x2,
        arg1,
        *arg2.offset(1 as core::ffi::c_int as isize),
        *arg3.offset(1 as core::ffi::c_int as isize),
    );
    let mut x3: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x3,
        arg1,
        *arg2.offset(2 as core::ffi::c_int as isize),
        *arg3.offset(2 as core::ffi::c_int as isize),
    );
    let mut x4: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x4,
        arg1,
        *arg2.offset(3 as core::ffi::c_int as isize),
        *arg3.offset(3 as core::ffi::c_int as isize),
    );
    let mut x5: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x5,
        arg1,
        *arg2.offset(4 as core::ffi::c_int as isize),
        *arg3.offset(4 as core::ffi::c_int as isize),
    );
    let mut x6: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x6,
        arg1,
        *arg2.offset(5 as core::ffi::c_int as isize),
        *arg3.offset(5 as core::ffi::c_int as isize),
    );
    let mut x7: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x7,
        arg1,
        *arg2.offset(6 as core::ffi::c_int as isize),
        *arg3.offset(6 as core::ffi::c_int as isize),
    );
    let mut x8: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x8,
        arg1,
        *arg2.offset(7 as core::ffi::c_int as isize),
        *arg3.offset(7 as core::ffi::c_int as isize),
    );
    let mut x9: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x9,
        arg1,
        *arg2.offset(8 as core::ffi::c_int as isize),
        *arg3.offset(8 as core::ffi::c_int as isize),
    );
    let mut x10: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x10,
        arg1,
        *arg2.offset(9 as core::ffi::c_int as isize),
        *arg3.offset(9 as core::ffi::c_int as isize),
    );
    *out1.offset(0 as core::ffi::c_int as isize) = x1;
    *out1.offset(1 as core::ffi::c_int as isize) = x2;
    *out1.offset(2 as core::ffi::c_int as isize) = x3;
    *out1.offset(3 as core::ffi::c_int as isize) = x4;
    *out1.offset(4 as core::ffi::c_int as isize) = x5;
    *out1.offset(5 as core::ffi::c_int as isize) = x6;
    *out1.offset(6 as core::ffi::c_int as isize) = x7;
    *out1.offset(7 as core::ffi::c_int as isize) = x8;
    *out1.offset(8 as core::ffi::c_int as isize) = x9;
    *out1.offset(9 as core::ffi::c_int as isize) = x10;
}
unsafe extern "C" fn fiat_25519_to_bytes(out1: *mut uint8_t, arg1: *const uint32_t) {
    let mut x1: uint32_t = 0;
    let mut x2: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u26(
        &mut x1,
        &mut x2,
        0 as core::ffi::c_int as fiat_25519_uint1,
        *arg1.offset(0 as core::ffi::c_int as isize),
        0x3ffffed as core::ffi::c_uint,
    );
    let mut x3: uint32_t = 0;
    let mut x4: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u25(
        &mut x3,
        &mut x4,
        x2,
        *arg1.offset(1 as core::ffi::c_int as isize),
        0x1ffffff as core::ffi::c_uint,
    );
    let mut x5: uint32_t = 0;
    let mut x6: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u26(
        &mut x5,
        &mut x6,
        x4,
        *arg1.offset(2 as core::ffi::c_int as isize),
        0x3ffffff as core::ffi::c_uint,
    );
    let mut x7: uint32_t = 0;
    let mut x8: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u25(
        &mut x7,
        &mut x8,
        x6,
        *arg1.offset(3 as core::ffi::c_int as isize),
        0x1ffffff as core::ffi::c_uint,
    );
    let mut x9: uint32_t = 0;
    let mut x10: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u26(
        &mut x9,
        &mut x10,
        x8,
        *arg1.offset(4 as core::ffi::c_int as isize),
        0x3ffffff as core::ffi::c_uint,
    );
    let mut x11: uint32_t = 0;
    let mut x12: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u25(
        &mut x11,
        &mut x12,
        x10,
        *arg1.offset(5 as core::ffi::c_int as isize),
        0x1ffffff as core::ffi::c_uint,
    );
    let mut x13: uint32_t = 0;
    let mut x14: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u26(
        &mut x13,
        &mut x14,
        x12,
        *arg1.offset(6 as core::ffi::c_int as isize),
        0x3ffffff as core::ffi::c_uint,
    );
    let mut x15: uint32_t = 0;
    let mut x16: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u25(
        &mut x15,
        &mut x16,
        x14,
        *arg1.offset(7 as core::ffi::c_int as isize),
        0x1ffffff as core::ffi::c_uint,
    );
    let mut x17: uint32_t = 0;
    let mut x18: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u26(
        &mut x17,
        &mut x18,
        x16,
        *arg1.offset(8 as core::ffi::c_int as isize),
        0x3ffffff as core::ffi::c_uint,
    );
    let mut x19: uint32_t = 0;
    let mut x20: fiat_25519_uint1 = 0;
    fiat_25519_subborrowx_u25(
        &mut x19,
        &mut x20,
        x18,
        *arg1.offset(9 as core::ffi::c_int as isize),
        0x1ffffff as core::ffi::c_uint,
    );
    let mut x21: uint32_t = 0;
    fiat_25519_cmovznz_u32(
        &mut x21,
        x20,
        0 as core::ffi::c_int as uint32_t,
        0xffffffff as core::ffi::c_uint,
    );
    let mut x22: uint32_t = 0;
    let mut x23: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u26(
        &mut x22,
        &mut x23,
        0 as core::ffi::c_int as fiat_25519_uint1,
        x1,
        x21 & 0x3ffffed as core::ffi::c_uint,
    );
    let mut x24: uint32_t = 0;
    let mut x25: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u25(
        &mut x24,
        &mut x25,
        x23,
        x3,
        x21 & 0x1ffffff as core::ffi::c_uint,
    );
    let mut x26: uint32_t = 0;
    let mut x27: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u26(
        &mut x26,
        &mut x27,
        x25,
        x5,
        x21 & 0x3ffffff as core::ffi::c_uint,
    );
    let mut x28: uint32_t = 0;
    let mut x29: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u25(
        &mut x28,
        &mut x29,
        x27,
        x7,
        x21 & 0x1ffffff as core::ffi::c_uint,
    );
    let mut x30: uint32_t = 0;
    let mut x31: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u26(
        &mut x30,
        &mut x31,
        x29,
        x9,
        x21 & 0x3ffffff as core::ffi::c_uint,
    );
    let mut x32: uint32_t = 0;
    let mut x33: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u25(
        &mut x32,
        &mut x33,
        x31,
        x11,
        x21 & 0x1ffffff as core::ffi::c_uint,
    );
    let mut x34: uint32_t = 0;
    let mut x35: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u26(
        &mut x34,
        &mut x35,
        x33,
        x13,
        x21 & 0x3ffffff as core::ffi::c_uint,
    );
    let mut x36: uint32_t = 0;
    let mut x37: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u25(
        &mut x36,
        &mut x37,
        x35,
        x15,
        x21 & 0x1ffffff as core::ffi::c_uint,
    );
    let mut x38: uint32_t = 0;
    let mut x39: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u26(
        &mut x38,
        &mut x39,
        x37,
        x17,
        x21 & 0x3ffffff as core::ffi::c_uint,
    );
    let mut x40: uint32_t = 0;
    let mut x41: fiat_25519_uint1 = 0;
    fiat_25519_addcarryx_u25(
        &mut x40,
        &mut x41,
        x39,
        x19,
        x21 & 0x1ffffff as core::ffi::c_uint,
    );
    let x42: uint32_t = x40 << 6 as core::ffi::c_int;
    let x43: uint32_t = x38 << 4 as core::ffi::c_int;
    let x44: uint32_t = x36 << 3 as core::ffi::c_int;
    let x45: uint32_t = x34.wrapping_mul(0x2 as core::ffi::c_int as uint32_t);
    let x46: uint32_t = x30 << 6 as core::ffi::c_int;
    let x47: uint32_t = x28 << 5 as core::ffi::c_int;
    let x48: uint32_t = x26 << 3 as core::ffi::c_int;
    let x49: uint32_t = x24 << 2 as core::ffi::c_int;
    let x50: uint32_t = x22 >> 8 as core::ffi::c_int;
    let x51: uint8_t = (x22 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x52: uint32_t = x50 >> 8 as core::ffi::c_int;
    let x53: uint8_t = (x50 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x54: uint8_t = (x52 >> 8 as core::ffi::c_int) as uint8_t;
    let x55: uint8_t = (x52 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x56: uint32_t = (x54 as core::ffi::c_uint).wrapping_add(x49);
    let x57: uint32_t = x56 >> 8 as core::ffi::c_int;
    let x58: uint8_t = (x56 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x59: uint32_t = x57 >> 8 as core::ffi::c_int;
    let x60: uint8_t = (x57 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x61: uint8_t = (x59 >> 8 as core::ffi::c_int) as uint8_t;
    let x62: uint8_t = (x59 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x63: uint32_t = (x61 as core::ffi::c_uint).wrapping_add(x48);
    let x64: uint32_t = x63 >> 8 as core::ffi::c_int;
    let x65: uint8_t = (x63 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x66: uint32_t = x64 >> 8 as core::ffi::c_int;
    let x67: uint8_t = (x64 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x68: uint8_t = (x66 >> 8 as core::ffi::c_int) as uint8_t;
    let x69: uint8_t = (x66 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x70: uint32_t = (x68 as core::ffi::c_uint).wrapping_add(x47);
    let x71: uint32_t = x70 >> 8 as core::ffi::c_int;
    let x72: uint8_t = (x70 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x73: uint32_t = x71 >> 8 as core::ffi::c_int;
    let x74: uint8_t = (x71 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x75: uint8_t = (x73 >> 8 as core::ffi::c_int) as uint8_t;
    let x76: uint8_t = (x73 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x77: uint32_t = (x75 as core::ffi::c_uint).wrapping_add(x46);
    let x78: uint32_t = x77 >> 8 as core::ffi::c_int;
    let x79: uint8_t = (x77 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x80: uint32_t = x78 >> 8 as core::ffi::c_int;
    let x81: uint8_t = (x78 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x82: uint8_t = (x80 >> 8 as core::ffi::c_int) as uint8_t;
    let x83: uint8_t = (x80 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x84: uint8_t = (x82 as core::ffi::c_int & 0xff as core::ffi::c_int) as uint8_t;
    let x85: uint32_t = x32 >> 8 as core::ffi::c_int;
    let x86: uint8_t = (x32 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x87: uint32_t = x85 >> 8 as core::ffi::c_int;
    let x88: uint8_t = (x85 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x89: fiat_25519_uint1 = (x87 >> 8 as core::ffi::c_int) as fiat_25519_uint1;
    let x90: uint8_t = (x87 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x91: uint32_t = (x89 as core::ffi::c_uint).wrapping_add(x45);
    let x92: uint32_t = x91 >> 8 as core::ffi::c_int;
    let x93: uint8_t = (x91 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x94: uint32_t = x92 >> 8 as core::ffi::c_int;
    let x95: uint8_t = (x92 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x96: uint8_t = (x94 >> 8 as core::ffi::c_int) as uint8_t;
    let x97: uint8_t = (x94 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x98: uint32_t = (x96 as core::ffi::c_uint).wrapping_add(x44);
    let x99: uint32_t = x98 >> 8 as core::ffi::c_int;
    let x100: uint8_t = (x98 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x101: uint32_t = x99 >> 8 as core::ffi::c_int;
    let x102: uint8_t = (x99 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x103: uint8_t = (x101 >> 8 as core::ffi::c_int) as uint8_t;
    let x104: uint8_t = (x101 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x105: uint32_t = (x103 as core::ffi::c_uint).wrapping_add(x43);
    let x106: uint32_t = x105 >> 8 as core::ffi::c_int;
    let x107: uint8_t = (x105 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x108: uint32_t = x106 >> 8 as core::ffi::c_int;
    let x109: uint8_t = (x106 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x110: uint8_t = (x108 >> 8 as core::ffi::c_int) as uint8_t;
    let x111: uint8_t = (x108 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x112: uint32_t = (x110 as core::ffi::c_uint).wrapping_add(x42);
    let x113: uint32_t = x112 >> 8 as core::ffi::c_int;
    let x114: uint8_t = (x112 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x115: uint32_t = x113 >> 8 as core::ffi::c_int;
    let x116: uint8_t = (x113 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    let x117: uint8_t = (x115 >> 8 as core::ffi::c_int) as uint8_t;
    let x118: uint8_t = (x115 & 0xff as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    *out1.offset(0 as core::ffi::c_int as isize) = x51;
    *out1.offset(1 as core::ffi::c_int as isize) = x53;
    *out1.offset(2 as core::ffi::c_int as isize) = x55;
    *out1.offset(3 as core::ffi::c_int as isize) = x58;
    *out1.offset(4 as core::ffi::c_int as isize) = x60;
    *out1.offset(5 as core::ffi::c_int as isize) = x62;
    *out1.offset(6 as core::ffi::c_int as isize) = x65;
    *out1.offset(7 as core::ffi::c_int as isize) = x67;
    *out1.offset(8 as core::ffi::c_int as isize) = x69;
    *out1.offset(9 as core::ffi::c_int as isize) = x72;
    *out1.offset(10 as core::ffi::c_int as isize) = x74;
    *out1.offset(11 as core::ffi::c_int as isize) = x76;
    *out1.offset(12 as core::ffi::c_int as isize) = x79;
    *out1.offset(13 as core::ffi::c_int as isize) = x81;
    *out1.offset(14 as core::ffi::c_int as isize) = x83;
    *out1.offset(15 as core::ffi::c_int as isize) = x84;
    *out1.offset(16 as core::ffi::c_int as isize) = x86;
    *out1.offset(17 as core::ffi::c_int as isize) = x88;
    *out1.offset(18 as core::ffi::c_int as isize) = x90;
    *out1.offset(19 as core::ffi::c_int as isize) = x93;
    *out1.offset(20 as core::ffi::c_int as isize) = x95;
    *out1.offset(21 as core::ffi::c_int as isize) = x97;
    *out1.offset(22 as core::ffi::c_int as isize) = x100;
    *out1.offset(23 as core::ffi::c_int as isize) = x102;
    *out1.offset(24 as core::ffi::c_int as isize) = x104;
    *out1.offset(25 as core::ffi::c_int as isize) = x107;
    *out1.offset(26 as core::ffi::c_int as isize) = x109;
    *out1.offset(27 as core::ffi::c_int as isize) = x111;
    *out1.offset(28 as core::ffi::c_int as isize) = x114;
    *out1.offset(29 as core::ffi::c_int as isize) = x116;
    *out1.offset(30 as core::ffi::c_int as isize) = x118;
    *out1.offset(31 as core::ffi::c_int as isize) = x117;
}
unsafe extern "C" fn fiat_25519_from_bytes(out1: *mut uint32_t, arg1: *const uint8_t) {
    let x1: uint32_t =
        (*arg1.offset(31 as core::ffi::c_int as isize) as uint32_t) << 18 as core::ffi::c_int;
    let x2: uint32_t =
        (*arg1.offset(30 as core::ffi::c_int as isize) as uint32_t) << 10 as core::ffi::c_int;
    let x3: uint32_t =
        (*arg1.offset(29 as core::ffi::c_int as isize) as uint32_t) << 2 as core::ffi::c_int;
    let x4: uint32_t =
        (*arg1.offset(28 as core::ffi::c_int as isize) as uint32_t) << 20 as core::ffi::c_int;
    let x5: uint32_t =
        (*arg1.offset(27 as core::ffi::c_int as isize) as uint32_t) << 12 as core::ffi::c_int;
    let x6: uint32_t =
        (*arg1.offset(26 as core::ffi::c_int as isize) as uint32_t) << 4 as core::ffi::c_int;
    let x7: uint32_t =
        (*arg1.offset(25 as core::ffi::c_int as isize) as uint32_t) << 21 as core::ffi::c_int;
    let x8: uint32_t =
        (*arg1.offset(24 as core::ffi::c_int as isize) as uint32_t) << 13 as core::ffi::c_int;
    let x9: uint32_t =
        (*arg1.offset(23 as core::ffi::c_int as isize) as uint32_t) << 5 as core::ffi::c_int;
    let x10: uint32_t =
        (*arg1.offset(22 as core::ffi::c_int as isize) as uint32_t) << 23 as core::ffi::c_int;
    let x11: uint32_t =
        (*arg1.offset(21 as core::ffi::c_int as isize) as uint32_t) << 15 as core::ffi::c_int;
    let x12: uint32_t =
        (*arg1.offset(20 as core::ffi::c_int as isize) as uint32_t) << 7 as core::ffi::c_int;
    let x13: uint32_t =
        (*arg1.offset(19 as core::ffi::c_int as isize) as uint32_t) << 24 as core::ffi::c_int;
    let x14: uint32_t =
        (*arg1.offset(18 as core::ffi::c_int as isize) as uint32_t) << 16 as core::ffi::c_int;
    let x15: uint32_t =
        (*arg1.offset(17 as core::ffi::c_int as isize) as uint32_t) << 8 as core::ffi::c_int;
    let x16: uint8_t = *arg1.offset(16 as core::ffi::c_int as isize);
    let x17: uint32_t =
        (*arg1.offset(15 as core::ffi::c_int as isize) as uint32_t) << 18 as core::ffi::c_int;
    let x18: uint32_t =
        (*arg1.offset(14 as core::ffi::c_int as isize) as uint32_t) << 10 as core::ffi::c_int;
    let x19: uint32_t =
        (*arg1.offset(13 as core::ffi::c_int as isize) as uint32_t) << 2 as core::ffi::c_int;
    let x20: uint32_t =
        (*arg1.offset(12 as core::ffi::c_int as isize) as uint32_t) << 19 as core::ffi::c_int;
    let x21: uint32_t =
        (*arg1.offset(11 as core::ffi::c_int as isize) as uint32_t) << 11 as core::ffi::c_int;
    let x22: uint32_t =
        (*arg1.offset(10 as core::ffi::c_int as isize) as uint32_t) << 3 as core::ffi::c_int;
    let x23: uint32_t =
        (*arg1.offset(9 as core::ffi::c_int as isize) as uint32_t) << 21 as core::ffi::c_int;
    let x24: uint32_t =
        (*arg1.offset(8 as core::ffi::c_int as isize) as uint32_t) << 13 as core::ffi::c_int;
    let x25: uint32_t =
        (*arg1.offset(7 as core::ffi::c_int as isize) as uint32_t) << 5 as core::ffi::c_int;
    let x26: uint32_t =
        (*arg1.offset(6 as core::ffi::c_int as isize) as uint32_t) << 22 as core::ffi::c_int;
    let x27: uint32_t =
        (*arg1.offset(5 as core::ffi::c_int as isize) as uint32_t) << 14 as core::ffi::c_int;
    let x28: uint32_t =
        (*arg1.offset(4 as core::ffi::c_int as isize) as uint32_t) << 6 as core::ffi::c_int;
    let x29: uint32_t =
        (*arg1.offset(3 as core::ffi::c_int as isize) as uint32_t) << 24 as core::ffi::c_int;
    let x30: uint32_t =
        (*arg1.offset(2 as core::ffi::c_int as isize) as uint32_t) << 16 as core::ffi::c_int;
    let x31: uint32_t =
        (*arg1.offset(1 as core::ffi::c_int as isize) as uint32_t) << 8 as core::ffi::c_int;
    let x32: uint8_t = *arg1.offset(0 as core::ffi::c_int as isize);
    let x33: uint32_t =
        (x32 as core::ffi::c_uint).wrapping_add(x31.wrapping_add(x30.wrapping_add(x29)));
    let x34: uint8_t = (x33 >> 26 as core::ffi::c_int) as uint8_t;
    let x35: uint32_t = x33 & 0x3ffffff as core::ffi::c_uint;
    let x36: uint32_t = x3.wrapping_add(x2.wrapping_add(x1));
    let x37: uint32_t = x6.wrapping_add(x5.wrapping_add(x4));
    let x38: uint32_t = x9.wrapping_add(x8.wrapping_add(x7));
    let x39: uint32_t = x12.wrapping_add(x11.wrapping_add(x10));
    let x40: uint32_t =
        (x16 as core::ffi::c_uint).wrapping_add(x15.wrapping_add(x14.wrapping_add(x13)));
    let x41: uint32_t = x19.wrapping_add(x18.wrapping_add(x17));
    let x42: uint32_t = x22.wrapping_add(x21.wrapping_add(x20));
    let x43: uint32_t = x25.wrapping_add(x24.wrapping_add(x23));
    let x44: uint32_t = x28.wrapping_add(x27.wrapping_add(x26));
    let x45: uint32_t = (x34 as core::ffi::c_uint).wrapping_add(x44);
    let x46: uint8_t = (x45 >> 25 as core::ffi::c_int) as uint8_t;
    let x47: uint32_t = x45 & 0x1ffffff as core::ffi::c_uint;
    let x48: uint32_t = (x46 as core::ffi::c_uint).wrapping_add(x43);
    let x49: uint8_t = (x48 >> 26 as core::ffi::c_int) as uint8_t;
    let x50: uint32_t = x48 & 0x3ffffff as core::ffi::c_uint;
    let x51: uint32_t = (x49 as core::ffi::c_uint).wrapping_add(x42);
    let x52: uint8_t = (x51 >> 25 as core::ffi::c_int) as uint8_t;
    let x53: uint32_t = x51 & 0x1ffffff as core::ffi::c_uint;
    let x54: uint32_t = (x52 as core::ffi::c_uint).wrapping_add(x41);
    let x55: uint32_t = x54 & 0x3ffffff as core::ffi::c_uint;
    let x56: uint8_t = (x40 >> 25 as core::ffi::c_int) as uint8_t;
    let x57: uint32_t = x40 & 0x1ffffff as core::ffi::c_uint;
    let x58: uint32_t = (x56 as core::ffi::c_uint).wrapping_add(x39);
    let x59: uint8_t = (x58 >> 26 as core::ffi::c_int) as uint8_t;
    let x60: uint32_t = x58 & 0x3ffffff as core::ffi::c_uint;
    let x61: uint32_t = (x59 as core::ffi::c_uint).wrapping_add(x38);
    let x62: uint8_t = (x61 >> 25 as core::ffi::c_int) as uint8_t;
    let x63: uint32_t = x61 & 0x1ffffff as core::ffi::c_uint;
    let x64: uint32_t = (x62 as core::ffi::c_uint).wrapping_add(x37);
    let x65: uint8_t = (x64 >> 26 as core::ffi::c_int) as uint8_t;
    let x66: uint32_t = x64 & 0x3ffffff as core::ffi::c_uint;
    let x67: uint32_t = (x65 as core::ffi::c_uint).wrapping_add(x36);
    *out1.offset(0 as core::ffi::c_int as isize) = x35;
    *out1.offset(1 as core::ffi::c_int as isize) = x47;
    *out1.offset(2 as core::ffi::c_int as isize) = x50;
    *out1.offset(3 as core::ffi::c_int as isize) = x53;
    *out1.offset(4 as core::ffi::c_int as isize) = x55;
    *out1.offset(5 as core::ffi::c_int as isize) = x57;
    *out1.offset(6 as core::ffi::c_int as isize) = x60;
    *out1.offset(7 as core::ffi::c_int as isize) = x63;
    *out1.offset(8 as core::ffi::c_int as isize) = x66;
    *out1.offset(9 as core::ffi::c_int as isize) = x67;
}
unsafe extern "C" fn fiat_25519_carry_scmul_121666(
    out1: *mut uint32_t,
    arg1: *const uint32_t,
) {
    let x1: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(9 as core::ffi::c_int as isize) as u64);
    let x2: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(8 as core::ffi::c_int as isize) as u64);
    let x3: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(7 as core::ffi::c_int as isize) as u64);
    let x4: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(6 as core::ffi::c_int as isize) as u64);
    let x5: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(5 as core::ffi::c_int as isize) as u64);
    let x6: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(4 as core::ffi::c_int as isize) as u64);
    let x7: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(3 as core::ffi::c_int as isize) as u64);
    let x8: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(2 as core::ffi::c_int as isize) as u64);
    let x9: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(1 as core::ffi::c_int as isize) as u64);
    let x10: uint64_t = (0x1db42 as core::ffi::c_uint as uint64_t)
        .wrapping_mul(*arg1.offset(0 as core::ffi::c_int as isize) as u64);
    let x11: uint32_t = (x10 >> 26 as core::ffi::c_int) as uint32_t;
    let x12: uint32_t = (x10 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x13: uint64_t = (x11 as u64).wrapping_add(x9);
    let x14: uint32_t = (x13 >> 25 as core::ffi::c_int) as uint32_t;
    let x15: uint32_t = (x13 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x16: uint64_t = (x14 as u64).wrapping_add(x8);
    let x17: uint32_t = (x16 >> 26 as core::ffi::c_int) as uint32_t;
    let x18: uint32_t = (x16 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x19: uint64_t = (x17 as u64).wrapping_add(x7);
    let x20: uint32_t = (x19 >> 25 as core::ffi::c_int) as uint32_t;
    let x21: uint32_t = (x19 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x22: uint64_t = (x20 as u64).wrapping_add(x6);
    let x23: uint32_t = (x22 >> 26 as core::ffi::c_int) as uint32_t;
    let x24: uint32_t = (x22 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x25: uint64_t = (x23 as u64).wrapping_add(x5);
    let x26: uint32_t = (x25 >> 25 as core::ffi::c_int) as uint32_t;
    let x27: uint32_t = (x25 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x28: uint64_t = (x26 as u64).wrapping_add(x4);
    let x29: uint32_t = (x28 >> 26 as core::ffi::c_int) as uint32_t;
    let x30: uint32_t = (x28 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x31: uint64_t = (x29 as u64).wrapping_add(x3);
    let x32: uint32_t = (x31 >> 25 as core::ffi::c_int) as uint32_t;
    let x33: uint32_t = (x31 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x34: uint64_t = (x32 as u64).wrapping_add(x2);
    let x35: uint32_t = (x34 >> 26 as core::ffi::c_int) as uint32_t;
    let x36: uint32_t = (x34 & 0x3ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x37: uint64_t = (x35 as u64).wrapping_add(x1);
    let x38: uint32_t = (x37 >> 25 as core::ffi::c_int) as uint32_t;
    let x39: uint32_t = (x37 & 0x1ffffff as core::ffi::c_uint as u64) as uint32_t;
    let x40: uint32_t = x38.wrapping_mul(0x13 as core::ffi::c_int as core::ffi::c_uint);
    let x41: uint32_t = x12.wrapping_add(x40);
    let x42: fiat_25519_uint1 = (x41 >> 26 as core::ffi::c_int) as fiat_25519_uint1;
    let x43: uint32_t = x41 & 0x3ffffff as core::ffi::c_uint;
    let x44: uint32_t = (x42 as core::ffi::c_uint).wrapping_add(x15);
    let x45: fiat_25519_uint1 = (x44 >> 25 as core::ffi::c_int) as fiat_25519_uint1;
    let x46: uint32_t = x44 & 0x1ffffff as core::ffi::c_uint;
    let x47: uint32_t = (x45 as core::ffi::c_uint).wrapping_add(x18);
    *out1.offset(0 as core::ffi::c_int as isize) = x43;
    *out1.offset(1 as core::ffi::c_int as isize) = x46;
    *out1.offset(2 as core::ffi::c_int as isize) = x47;
    *out1.offset(3 as core::ffi::c_int as isize) = x21;
    *out1.offset(4 as core::ffi::c_int as isize) = x24;
    *out1.offset(5 as core::ffi::c_int as isize) = x27;
    *out1.offset(6 as core::ffi::c_int as isize) = x30;
    *out1.offset(7 as core::ffi::c_int as isize) = x33;
    *out1.offset(8 as core::ffi::c_int as isize) = x36;
    *out1.offset(9 as core::ffi::c_int as isize) = x39;
}
unsafe extern "C" fn load_3(in_0: *const uint8_t) -> uint64_t {
    let mut result: uint64_t;
    result = *in_0.offset(0 as core::ffi::c_int as isize) as uint64_t;
    result |= (*in_0.offset(1 as core::ffi::c_int as isize) as uint64_t) << 8 as core::ffi::c_int;
    result |= (*in_0.offset(2 as core::ffi::c_int as isize) as uint64_t) << 16 as core::ffi::c_int;
    return result;
}
unsafe extern "C" fn load_4(in_0: *const uint8_t) -> uint64_t {
    let mut result: uint64_t;
    result = *in_0.offset(0 as core::ffi::c_int as isize) as uint64_t;
    result |= (*in_0.offset(1 as core::ffi::c_int as isize) as uint64_t) << 8 as core::ffi::c_int;
    result |= (*in_0.offset(2 as core::ffi::c_int as isize) as uint64_t) << 16 as core::ffi::c_int;
    result |= (*in_0.offset(3 as core::ffi::c_int as isize) as uint64_t) << 24 as core::ffi::c_int;
    return result;
}
unsafe extern "C" fn fe_frombytes_strict(h: *mut fe, s: *const uint8_t) {
    fiat_25519_from_bytes(((*h).v).as_mut_ptr(), s);
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_frombytes(h: *mut fe, s: *const uint8_t) {
    let mut s_copy: [uint8_t; 32] = [0; 32];
    let _ = GFp_memcpy(
        s_copy.as_mut_ptr() as *mut core::ffi::c_void,
        s as *const core::ffi::c_void,
        32 as core::ffi::c_int as size_t,
    );
    s_copy[31 as core::ffi::c_int as usize] = (s_copy[31 as core::ffi::c_int as usize]
        as core::ffi::c_int
        & 0x7f as core::ffi::c_int) as uint8_t;
    fe_frombytes_strict(h, s_copy.as_mut_ptr() as *const uint8_t);
}
unsafe extern "C" fn fe_tobytes(s: *mut uint8_t, f: *const fe) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_to_bytes(s, ((*f).v).as_ptr());
}
unsafe extern "C" fn fe_0(h: *mut fe) {
    let _ = GFp_memset(
        h as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<fe>() as u32,
    );
}
unsafe extern "C" fn fe_loose_0(h: *mut fe_loose) {
    let _ = GFp_memset(
        h as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<fe_loose>() as u32,
    );
}
unsafe extern "C" fn fe_1(mut h: *mut fe) {
    let _ = GFp_memset(
        h as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<fe>() as u32,
    );
    (*h).v[0 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as fe_limb_t;
}
unsafe extern "C" fn fe_loose_1(mut h: *mut fe_loose) {
    let _ = GFp_memset(
        h as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<fe_loose>() as u32,
    );
    (*h).v[0 as core::ffi::c_int as usize] = 1 as core::ffi::c_int as fe_limb_t;
}
unsafe extern "C" fn fe_add(h: *mut fe_loose, f: *const fe, g: *const fe) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
    fiat_25519_add(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
    let mut _assert_fe_i_1: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_1 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_1 = _assert_fe_i_1.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_sub(h: *mut fe_loose, f: *const fe, g: *const fe) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
    fiat_25519_sub(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
    let mut _assert_fe_i_1: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_1 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_1 = _assert_fe_i_1.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_carry(h: *mut fe, f: *const fe_loose) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_carry(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_mul_impl(
    out: *mut fe_limb_t,
    in1: *const fe_limb_t,
    in2: *const fe_limb_t,
) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
    fiat_25519_carry_mul(out, in1, in2);
    let mut _assert_fe_i_1: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_1 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_1 = _assert_fe_i_1.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_mul_ltt(h: *mut fe_loose, f: *const fe, g: *const fe) {
    fe_mul_impl(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
}
unsafe extern "C" fn fe_mul_ttt(h: *mut fe, f: *const fe, g: *const fe) {
    fe_mul_impl(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
}
unsafe extern "C" fn fe_mul_tlt(h: *mut fe, f: *const fe_loose, g: *const fe) {
    fe_mul_impl(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
}
unsafe extern "C" fn fe_mul_ttl(h: *mut fe, f: *const fe, g: *const fe_loose) {
    fe_mul_impl(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
}
unsafe extern "C" fn fe_mul_tll(h: *mut fe, f: *const fe_loose, g: *const fe_loose) {
    fe_mul_impl(((*h).v).as_mut_ptr(), ((*f).v).as_ptr(), ((*g).v).as_ptr());
}
unsafe extern "C" fn fe_sq_tl(h: *mut fe, f: *const fe_loose) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_carry_square(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_sq_tt(h: *mut fe, f: *const fe) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_carry_square(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_cswap(mut f: *mut fe, mut g: *mut fe, mut b: fe_limb_t) {
    b = (0 as core::ffi::c_int as core::ffi::c_uint).wrapping_sub(b);
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while i < 10 as core::ffi::c_int as core::ffi::c_uint {
        let mut x: fe_limb_t = (*f).v[i as usize] ^ (*g).v[i as usize];
        x &= b;
        (*f).v[i as usize] ^= x;
        (*g).v[i as usize] ^= x;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_mul121666(h: *mut fe, f: *const fe_loose) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_carry_scmul_121666(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_neg(h: *mut fe_loose, f: *const fe) {
    let mut _assert_fe_i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i = _assert_fe_i.wrapping_add(1);
    }
    fiat_25519_opp(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
    let mut _assert_fe_i_0: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while _assert_fe_i_0 < 10 as core::ffi::c_int as core::ffi::c_uint {
        _assert_fe_i_0 = _assert_fe_i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_cmov(mut f: *mut fe_loose, g: *const fe_loose, mut b: fe_limb_t) {
    b = (0 as core::ffi::c_int as core::ffi::c_uint).wrapping_sub(b);
    let mut i: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    while i < 10 as core::ffi::c_int as core::ffi::c_uint {
        let mut x: fe_limb_t = (*f).v[i as usize] ^ (*g).v[i as usize];
        x &= b;
        (*f).v[i as usize] ^= x;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn fe_copy(h: *mut fe, f: *const fe) {
    fe_limbs_copy(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
}
unsafe extern "C" fn fe_copy_lt(h: *mut fe_loose, f: *const fe) {
    fe_limbs_copy(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
}
unsafe extern "C" fn fe_copy_ll(h: *mut fe_loose, f: *const fe_loose) {
    fe_limbs_copy(((*h).v).as_mut_ptr(), ((*f).v).as_ptr());
}
unsafe extern "C" fn fe_loose_invert(out: *mut fe, z: *const fe_loose) {
    let mut t0: fe = fe { v: [0; 10] };
    let mut t1: fe = fe { v: [0; 10] };
    let mut t2: fe = fe { v: [0; 10] };
    let mut t3: fe = fe { v: [0; 10] };
    let mut i: core::ffi::c_int;
    fe_sq_tl(&mut t0, z);
    fe_sq_tt(&mut t1, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_tlt(&mut t1, z, &mut t1);
    fe_mul_ttt(&mut t0, &mut t0, &mut t1);
    fe_sq_tt(&mut t2, &mut t0);
    fe_mul_ttt(&mut t1, &mut t1, &mut t2);
    fe_sq_tt(&mut t2, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 5 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t2, &mut t1);
    fe_sq_tt(&mut t2, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 10 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t2, &mut t2, &mut t1);
    fe_sq_tt(&mut t3, &mut t2);
    i = 1 as core::ffi::c_int;
    while i < 20 as core::ffi::c_int {
        fe_sq_tt(&mut t3, &mut t3);
        i += 1;
    }
    fe_mul_ttt(&mut t2, &mut t3, &mut t2);
    fe_sq_tt(&mut t2, &mut t2);
    i = 1 as core::ffi::c_int;
    while i < 10 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t2, &mut t1);
    fe_sq_tt(&mut t2, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 50 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t2, &mut t2, &mut t1);
    fe_sq_tt(&mut t3, &mut t2);
    i = 1 as core::ffi::c_int;
    while i < 100 as core::ffi::c_int {
        fe_sq_tt(&mut t3, &mut t3);
        i += 1;
    }
    fe_mul_ttt(&mut t2, &mut t3, &mut t2);
    fe_sq_tt(&mut t2, &mut t2);
    i = 1 as core::ffi::c_int;
    while i < 50 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t2, &mut t1);
    fe_sq_tt(&mut t1, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 5 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(out, &mut t1, &mut t0);
}
unsafe extern "C" fn fe_invert(out: *mut fe, z: *const fe) {
    let mut l: fe_loose = fe_loose { v: [0; 10] };
    fe_copy_lt(&mut l, z);
    fe_loose_invert(out, &mut l);
}
unsafe extern "C" fn fe_isnonzero(f: *const fe_loose) -> core::ffi::c_int {
    let mut tight: fe = fe { v: [0; 10] };
    fe_carry(&mut tight, f);
    let mut s: [uint8_t; 32] = [0; 32];
    fe_tobytes(s.as_mut_ptr(), &mut tight);
    static mut zero: [uint8_t; 32] = [
        0 as core::ffi::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    return (GFp_memcmp(
        s.as_mut_ptr(),
        zero.as_ptr(),
        ::core::mem::size_of::<[uint8_t; 32]>() as u32,
    ) != 0 as core::ffi::c_int) as core::ffi::c_int;
}
unsafe extern "C" fn fe_isnegative(f: *const fe) -> core::ffi::c_int {
    let mut s: [uint8_t; 32] = [0; 32];
    fe_tobytes(s.as_mut_ptr(), f);
    return s[0 as core::ffi::c_int as usize] as core::ffi::c_int & 1 as core::ffi::c_int;
}
unsafe extern "C" fn fe_sq2_tt(h: *mut fe, f: *const fe) {
    fe_sq_tt(h, f);
    let mut tmp: fe_loose = fe_loose { v: [0; 10] };
    fe_add(&mut tmp, h, h);
    fe_carry(h, &mut tmp);
}
unsafe extern "C" fn fe_pow22523(out: *mut fe, z: *const fe) {
    let mut t0: fe = fe { v: [0; 10] };
    let mut t1: fe = fe { v: [0; 10] };
    let mut t2: fe = fe { v: [0; 10] };
    let mut i: core::ffi::c_int;
    fe_sq_tt(&mut t0, z);
    fe_sq_tt(&mut t1, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t1, z, &mut t1);
    fe_mul_ttt(&mut t0, &mut t0, &mut t1);
    fe_sq_tt(&mut t0, &mut t0);
    fe_mul_ttt(&mut t0, &mut t1, &mut t0);
    fe_sq_tt(&mut t1, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 5 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t0, &mut t1, &mut t0);
    fe_sq_tt(&mut t1, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 10 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t1, &mut t0);
    fe_sq_tt(&mut t2, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 20 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t2, &mut t1);
    fe_sq_tt(&mut t1, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 10 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t0, &mut t1, &mut t0);
    fe_sq_tt(&mut t1, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 50 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t1, &mut t0);
    fe_sq_tt(&mut t2, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 100 as core::ffi::c_int {
        fe_sq_tt(&mut t2, &mut t2);
        i += 1;
    }
    fe_mul_ttt(&mut t1, &mut t2, &mut t1);
    fe_sq_tt(&mut t1, &mut t1);
    i = 1 as core::ffi::c_int;
    while i < 50 as core::ffi::c_int {
        fe_sq_tt(&mut t1, &mut t1);
        i += 1;
    }
    fe_mul_ttt(&mut t0, &mut t1, &mut t0);
    fe_sq_tt(&mut t0, &mut t0);
    i = 1 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        fe_sq_tt(&mut t0, &mut t0);
        i += 1;
    }
    fe_mul_ttt(out, &mut t0, z);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_ge_frombytes_vartime(
    h: *mut ge_p3,
    s: *const uint8_t,
) -> core::ffi::c_int {
    let mut u: fe = fe { v: [0; 10] };
    let mut v: fe_loose = fe_loose { v: [0; 10] };
    let mut v3: fe = fe { v: [0; 10] };
    let mut vxx: fe = fe { v: [0; 10] };
    let mut check: fe_loose = fe_loose { v: [0; 10] };
    fe_frombytes(&mut (*h).Y, s);
    fe_1(&mut (*h).Z);
    fe_sq_tt(&mut v3, &mut (*h).Y);
    fe_mul_ttt(&mut vxx, &mut v3, &d);
    fe_sub(&mut v, &mut v3, &mut (*h).Z);
    fe_carry(&mut u, &mut v);
    fe_add(&mut v, &mut vxx, &mut (*h).Z);
    fe_sq_tl(&mut v3, &mut v);
    fe_mul_ttl(&mut v3, &mut v3, &mut v);
    fe_sq_tt(&mut (*h).X, &mut v3);
    fe_mul_ttl(&mut (*h).X, &mut (*h).X, &mut v);
    fe_mul_ttt(&mut (*h).X, &mut (*h).X, &mut u);
    fe_pow22523(&mut (*h).X, &mut (*h).X);
    fe_mul_ttt(&mut (*h).X, &mut (*h).X, &mut v3);
    fe_mul_ttt(&mut (*h).X, &mut (*h).X, &mut u);
    fe_sq_tt(&mut vxx, &mut (*h).X);
    fe_mul_ttl(&mut vxx, &mut vxx, &mut v);
    fe_sub(&mut check, &mut vxx, &mut u);
    if fe_isnonzero(&mut check) != 0 {
        fe_add(&mut check, &mut vxx, &mut u);
        if fe_isnonzero(&mut check) != 0 {
            return 0 as core::ffi::c_int;
        }
        fe_mul_ttt(&mut (*h).X, &mut (*h).X, &sqrtm1);
    }
    if fe_isnegative(&mut (*h).X)
        != *s.offset(31 as core::ffi::c_int as isize) as core::ffi::c_int >> 7 as core::ffi::c_int
    {
        let mut t: fe_loose = fe_loose { v: [0; 10] };
        fe_neg(&mut t, &mut (*h).X);
        fe_carry(&mut (*h).X, &mut t);
    }
    fe_mul_ttt(&mut (*h).T, &mut (*h).X, &mut (*h).Y);
    return 1 as core::ffi::c_int;
}
unsafe extern "C" fn ge_p2_0(h: *mut ge_p2) {
    fe_0(&mut (*h).X);
    fe_1(&mut (*h).Y);
    fe_1(&mut (*h).Z);
}
unsafe extern "C" fn ge_p3_0(h: *mut ge_p3) {
    fe_0(&mut (*h).X);
    fe_1(&mut (*h).Y);
    fe_1(&mut (*h).Z);
    fe_0(&mut (*h).T);
}
unsafe extern "C" fn ge_precomp_0(h: *mut ge_precomp) {
    fe_loose_1(&mut (*h).yplusx);
    fe_loose_1(&mut (*h).yminusx);
    fe_loose_0(&mut (*h).xy2d);
}
unsafe extern "C" fn ge_p3_to_p2(r: *mut ge_p2, p: *const ge_p3) {
    fe_copy(&mut (*r).X, &(*p).X);
    fe_copy(&mut (*r).Y, &(*p).Y);
    fe_copy(&mut (*r).Z, &(*p).Z);
}
unsafe extern "C" fn x25519_ge_p3_to_cached(r: *mut ge_cached, p: *const ge_p3) {
    fe_add(&mut (*r).YplusX, &(*p).Y, &(*p).X);
    fe_sub(&mut (*r).YminusX, &(*p).Y, &(*p).X);
    fe_copy_lt(&mut (*r).Z, &(*p).Z);
    fe_mul_ltt(&mut (*r).T2d, &(*p).T, &d2);
}
unsafe extern "C" fn x25519_ge_p1p1_to_p2(r: *mut ge_p2, p: *const ge_p1p1) {
    fe_mul_tll(&mut (*r).X, &(*p).X, &(*p).T);
    fe_mul_tll(&mut (*r).Y, &(*p).Y, &(*p).Z);
    fe_mul_tll(&mut (*r).Z, &(*p).Z, &(*p).T);
}
unsafe extern "C" fn x25519_ge_p1p1_to_p3(r: *mut ge_p3, p: *const ge_p1p1) {
    fe_mul_tll(&mut (*r).X, &(*p).X, &(*p).T);
    fe_mul_tll(&mut (*r).Y, &(*p).Y, &(*p).Z);
    fe_mul_tll(&mut (*r).Z, &(*p).Z, &(*p).T);
    fe_mul_tll(&mut (*r).T, &(*p).X, &(*p).Y);
}
unsafe extern "C" fn ge_p2_dbl(r: *mut ge_p1p1, p: *const ge_p2) {
    let mut trX: fe = fe { v: [0; 10] };
    let mut trZ: fe = fe { v: [0; 10] };
    let mut trT: fe = fe { v: [0; 10] };
    let mut t0: fe = fe { v: [0; 10] };
    fe_sq_tt(&mut trX, &(*p).X);
    fe_sq_tt(&mut trZ, &(*p).Y);
    fe_sq2_tt(&mut trT, &(*p).Z);
    fe_add(&mut (*r).Y, &(*p).X, &(*p).Y);
    fe_sq_tl(&mut t0, &mut (*r).Y);
    fe_add(&mut (*r).Y, &mut trZ, &mut trX);
    fe_sub(&mut (*r).Z, &mut trZ, &mut trX);
    fe_carry(&mut trZ, &mut (*r).Y);
    fe_sub(&mut (*r).X, &mut t0, &mut trZ);
    fe_carry(&mut trZ, &mut (*r).Z);
    fe_sub(&mut (*r).T, &mut trT, &mut trZ);
}
unsafe extern "C" fn ge_p3_dbl(r: *mut ge_p1p1, p: *const ge_p3) {
    let mut q: ge_p2 = ge_p2 {
        X: fe { v: [0; 10] },
        Y: fe { v: [0; 10] },
        Z: fe { v: [0; 10] },
    };
    ge_p3_to_p2(&mut q, p);
    ge_p2_dbl(r, &mut q);
}
unsafe extern "C" fn ge_madd(r: *mut ge_p1p1, p: *const ge_p3, q: *const ge_precomp) {
    let mut trY: fe = fe { v: [0; 10] };
    let mut trZ: fe = fe { v: [0; 10] };
    let mut trT: fe = fe { v: [0; 10] };
    fe_add(&mut (*r).X, &(*p).Y, &(*p).X);
    fe_sub(&mut (*r).Y, &(*p).Y, &(*p).X);
    fe_mul_tll(&mut trZ, &mut (*r).X, &(*q).yplusx);
    fe_mul_tll(&mut trY, &mut (*r).Y, &(*q).yminusx);
    fe_mul_tlt(&mut trT, &(*q).xy2d, &(*p).T);
    fe_add(&mut (*r).T, &(*p).Z, &(*p).Z);
    fe_sub(&mut (*r).X, &mut trZ, &mut trY);
    fe_add(&mut (*r).Y, &mut trZ, &mut trY);
    fe_carry(&mut trZ, &mut (*r).T);
    fe_add(&mut (*r).Z, &mut trZ, &mut trT);
    fe_sub(&mut (*r).T, &mut trZ, &mut trT);
}
unsafe extern "C" fn ge_msub(r: *mut ge_p1p1, p: *const ge_p3, q: *const ge_precomp) {
    let mut trY: fe = fe { v: [0; 10] };
    let mut trZ: fe = fe { v: [0; 10] };
    let mut trT: fe = fe { v: [0; 10] };
    fe_add(&mut (*r).X, &(*p).Y, &(*p).X);
    fe_sub(&mut (*r).Y, &(*p).Y, &(*p).X);
    fe_mul_tll(&mut trZ, &mut (*r).X, &(*q).yminusx);
    fe_mul_tll(&mut trY, &mut (*r).Y, &(*q).yplusx);
    fe_mul_tlt(&mut trT, &(*q).xy2d, &(*p).T);
    fe_add(&mut (*r).T, &(*p).Z, &(*p).Z);
    fe_sub(&mut (*r).X, &mut trZ, &mut trY);
    fe_add(&mut (*r).Y, &mut trZ, &mut trY);
    fe_carry(&mut trZ, &mut (*r).T);
    fe_sub(&mut (*r).Z, &mut trZ, &mut trT);
    fe_add(&mut (*r).T, &mut trZ, &mut trT);
}
unsafe extern "C" fn x25519_ge_add(
    r: *mut ge_p1p1,
    p: *const ge_p3,
    q: *const ge_cached,
) {
    let mut trX: fe = fe { v: [0; 10] };
    let mut trY: fe = fe { v: [0; 10] };
    let mut trZ: fe = fe { v: [0; 10] };
    let mut trT: fe = fe { v: [0; 10] };
    fe_add(&mut (*r).X, &(*p).Y, &(*p).X);
    fe_sub(&mut (*r).Y, &(*p).Y, &(*p).X);
    fe_mul_tll(&mut trZ, &mut (*r).X, &(*q).YplusX);
    fe_mul_tll(&mut trY, &mut (*r).Y, &(*q).YminusX);
    fe_mul_tlt(&mut trT, &(*q).T2d, &(*p).T);
    fe_mul_ttl(&mut trX, &(*p).Z, &(*q).Z);
    fe_add(&mut (*r).T, &mut trX, &mut trX);
    fe_sub(&mut (*r).X, &mut trZ, &mut trY);
    fe_add(&mut (*r).Y, &mut trZ, &mut trY);
    fe_carry(&mut trZ, &mut (*r).T);
    fe_add(&mut (*r).Z, &mut trZ, &mut trT);
    fe_sub(&mut (*r).T, &mut trZ, &mut trT);
}
unsafe extern "C" fn x25519_ge_sub(
    r: *mut ge_p1p1,
    p: *const ge_p3,
    q: *const ge_cached,
) {
    let mut trX: fe = fe { v: [0; 10] };
    let mut trY: fe = fe { v: [0; 10] };
    let mut trZ: fe = fe { v: [0; 10] };
    let mut trT: fe = fe { v: [0; 10] };
    fe_add(&mut (*r).X, &(*p).Y, &(*p).X);
    fe_sub(&mut (*r).Y, &(*p).Y, &(*p).X);
    fe_mul_tll(&mut trZ, &mut (*r).X, &(*q).YminusX);
    fe_mul_tll(&mut trY, &mut (*r).Y, &(*q).YplusX);
    fe_mul_tlt(&mut trT, &(*q).T2d, &(*p).T);
    fe_mul_ttl(&mut trX, &(*p).Z, &(*q).Z);
    fe_add(&mut (*r).T, &mut trX, &mut trX);
    fe_sub(&mut (*r).X, &mut trZ, &mut trY);
    fe_add(&mut (*r).Y, &mut trZ, &mut trY);
    fe_carry(&mut trZ, &mut (*r).T);
    fe_sub(&mut (*r).Z, &mut trZ, &mut trT);
    fe_add(&mut (*r).T, &mut trZ, &mut trT);
}
unsafe extern "C" fn equal(b: core::ffi::c_schar, c: core::ffi::c_schar) -> uint8_t {
    let ub: uint8_t = b as uint8_t;
    let uc: uint8_t = c as uint8_t;
    let x: uint8_t = (ub as core::ffi::c_int ^ uc as core::ffi::c_int) as uint8_t;
    let mut y: uint32_t = x as uint32_t;
    y = (y as core::ffi::c_uint).wrapping_sub(1 as core::ffi::c_int as core::ffi::c_uint)
        as uint32_t as uint32_t;
    y >>= 31 as core::ffi::c_int;
    return y as uint8_t;
}
unsafe extern "C" fn cmov(t: *mut ge_precomp, u: *const ge_precomp, b: uint8_t) {
    fe_cmov(&mut (*t).yplusx, &(*u).yplusx, b as fe_limb_t);
    fe_cmov(&mut (*t).yminusx, &(*u).yminusx, b as fe_limb_t);
    fe_cmov(&mut (*t).xy2d, &(*u).xy2d, b as fe_limb_t);
}
unsafe extern "C" fn negative(b: core::ffi::c_schar) -> uint8_t {
    let mut x: uint32_t = b as uint32_t;
    x >>= 31 as core::ffi::c_int;
    return x as uint8_t;
}
unsafe extern "C" fn table_select(
    t: *mut ge_precomp,
    pos: core::ffi::c_int,
    b: core::ffi::c_schar,
) {
    let mut minust: ge_precomp = ge_precomp {
        yplusx: fe_loose { v: [0; 10] },
        yminusx: fe_loose { v: [0; 10] },
        xy2d: fe_loose { v: [0; 10] },
    };
    let bnegative: uint8_t = negative(b);
    let babs: uint8_t = (b as core::ffi::c_int
        - (((-(bnegative as core::ffi::c_int) & b as core::ffi::c_int) as uint8_t
            as core::ffi::c_int)
            << 1 as core::ffi::c_int)) as uint8_t;
    ge_precomp_0(t);
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(0 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            1 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(1 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            2 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(2 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            3 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(3 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            4 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(4 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            5 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(5 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            6 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(6 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            7 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    cmov(
        t,
        &*(*k25519Precomp.as_ptr().offset(pos as isize))
            .as_ptr()
            .offset(7 as core::ffi::c_int as isize),
        equal(
            babs as core::ffi::c_schar,
            8 as core::ffi::c_int as core::ffi::c_schar,
        ),
    );
    fe_copy_ll(&mut minust.yplusx, &mut (*t).yminusx);
    fe_copy_ll(&mut minust.yminusx, &mut (*t).yplusx);
    let mut tmp: fe = fe { v: [0; 10] };
    fe_carry(&mut tmp, &mut (*t).xy2d);
    fe_neg(&mut minust.xy2d, &mut tmp);
    cmov(t, &mut minust, bnegative);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_ge_scalarmult_base(h: *mut ge_p3, a: *const uint8_t) {
    let mut e: [core::ffi::c_schar; 64] = [0; 64];
    let mut carry: core::ffi::c_schar;
    let mut r: ge_p1p1 = ge_p1p1 {
        X: fe_loose { v: [0; 10] },
        Y: fe_loose { v: [0; 10] },
        Z: fe_loose { v: [0; 10] },
        T: fe_loose { v: [0; 10] },
    };
    let mut s: ge_p2 = ge_p2 {
        X: fe { v: [0; 10] },
        Y: fe { v: [0; 10] },
        Z: fe { v: [0; 10] },
    };
    let mut t: ge_precomp = ge_precomp {
        yplusx: fe_loose { v: [0; 10] },
        yminusx: fe_loose { v: [0; 10] },
        xy2d: fe_loose { v: [0; 10] },
    };
    let mut i: core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 32 as core::ffi::c_int {
        e[(2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as usize] =
            (*a.offset(i as isize) as core::ffi::c_int >> 0 as core::ffi::c_int
                & 15 as core::ffi::c_int) as core::ffi::c_schar;
        e[(2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as usize] =
            (*a.offset(i as isize) as core::ffi::c_int >> 4 as core::ffi::c_int
                & 15 as core::ffi::c_int) as core::ffi::c_schar;
        i += 1;
    }
    carry = 0 as core::ffi::c_int as core::ffi::c_schar;
    i = 0 as core::ffi::c_int;
    while i < 63 as core::ffi::c_int {
        e[i as usize] =
            (e[i as usize] as core::ffi::c_int + carry as core::ffi::c_int) as core::ffi::c_schar;
        carry = (e[i as usize] as core::ffi::c_int + 8 as core::ffi::c_int) as core::ffi::c_schar;
        carry = (carry as core::ffi::c_int >> 4 as core::ffi::c_int) as core::ffi::c_schar;
        e[i as usize] = (e[i as usize] as core::ffi::c_int
            - ((carry as core::ffi::c_int) << 4 as core::ffi::c_int))
            as core::ffi::c_schar;
        i += 1;
    }
    e[63 as core::ffi::c_int as usize] = (e[63 as core::ffi::c_int as usize] as core::ffi::c_int
        + carry as core::ffi::c_int) as core::ffi::c_schar;
    ge_p3_0(h);
    i = 1 as core::ffi::c_int;
    while i < 64 as core::ffi::c_int {
        table_select(&mut t, i / 2 as core::ffi::c_int, e[i as usize]);
        ge_madd(&mut r, h, &mut t);
        x25519_ge_p1p1_to_p3(h, &mut r);
        i += 2 as core::ffi::c_int;
    }
    ge_p3_dbl(&mut r, h);
    x25519_ge_p1p1_to_p2(&mut s, &mut r);
    ge_p2_dbl(&mut r, &mut s);
    x25519_ge_p1p1_to_p2(&mut s, &mut r);
    ge_p2_dbl(&mut r, &mut s);
    x25519_ge_p1p1_to_p2(&mut s, &mut r);
    ge_p2_dbl(&mut r, &mut s);
    x25519_ge_p1p1_to_p3(h, &mut r);
    i = 0 as core::ffi::c_int;
    while i < 64 as core::ffi::c_int {
        table_select(&mut t, i / 2 as core::ffi::c_int, e[i as usize]);
        ge_madd(&mut r, h, &mut t);
        x25519_ge_p1p1_to_p3(h, &mut r);
        i += 2 as core::ffi::c_int;
    }
}
unsafe extern "C" fn slide(r: *mut core::ffi::c_schar, a: *const uint8_t) {
    let mut i: core::ffi::c_int;
    let mut b: core::ffi::c_int;
    let mut k: core::ffi::c_int;
    i = 0 as core::ffi::c_int;
    while i < 256 as core::ffi::c_int {
        *r.offset(i as isize) = (1 as core::ffi::c_int
            & *a.offset((i >> 3 as core::ffi::c_int) as isize) as core::ffi::c_int
                >> (i & 7 as core::ffi::c_int))
            as core::ffi::c_schar;
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < 256 as core::ffi::c_int {
        if *r.offset(i as isize) != 0 {
            b = 1 as core::ffi::c_int;
            while b <= 6 as core::ffi::c_int && i + b < 256 as core::ffi::c_int {
                if *r.offset((i + b) as isize) != 0 {
                    if *r.offset(i as isize) as core::ffi::c_int
                        + ((*r.offset((i + b) as isize) as core::ffi::c_int) << b)
                        <= 15 as core::ffi::c_int
                    {
                        let ref mut fresh0 = *r.offset(i as isize);
                        *fresh0 = (*fresh0 as core::ffi::c_int
                            + ((*r.offset((i + b) as isize) as core::ffi::c_int) << b))
                            as core::ffi::c_schar;
                        *r.offset((i + b) as isize) = 0 as core::ffi::c_int as core::ffi::c_schar;
                    } else {
                        if !(*r.offset(i as isize) as core::ffi::c_int
                            - ((*r.offset((i + b) as isize) as core::ffi::c_int) << b)
                            >= -(15 as core::ffi::c_int))
                        {
                            break;
                        }
                        let ref mut fresh1 = *r.offset(i as isize);
                        *fresh1 = (*fresh1 as core::ffi::c_int
                            - ((*r.offset((i + b) as isize) as core::ffi::c_int) << b))
                            as core::ffi::c_schar;
                        k = i + b;
                        while k < 256 as core::ffi::c_int {
                            if *r.offset(k as isize) == 0 {
                                *r.offset(k as isize) = 1 as core::ffi::c_int as core::ffi::c_schar;
                                break;
                            } else {
                                *r.offset(k as isize) = 0 as core::ffi::c_int as core::ffi::c_schar;
                                k += 1;
                            }
                        }
                    }
                }
                b += 1;
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn ge_double_scalarmult_vartime(
    r: *mut ge_p2,
    a: *const uint8_t,
    A: *const ge_p3,
    b: *const uint8_t,
) {
    let mut aslide: [core::ffi::c_schar; 256] = [0; 256];
    let mut bslide: [core::ffi::c_schar; 256] = [0; 256];
    let mut Ai: [ge_cached; 8] = [ge_cached {
        YplusX: fe_loose { v: [0; 10] },
        YminusX: fe_loose { v: [0; 10] },
        Z: fe_loose { v: [0; 10] },
        T2d: fe_loose { v: [0; 10] },
    }; 8];
    let mut t: ge_p1p1 = ge_p1p1 {
        X: fe_loose { v: [0; 10] },
        Y: fe_loose { v: [0; 10] },
        Z: fe_loose { v: [0; 10] },
        T: fe_loose { v: [0; 10] },
    };
    let mut u: ge_p3 = ge_p3 {
        X: fe { v: [0; 10] },
        Y: fe { v: [0; 10] },
        Z: fe { v: [0; 10] },
        T: fe { v: [0; 10] },
    };
    let mut A2: ge_p3 = ge_p3 {
        X: fe { v: [0; 10] },
        Y: fe { v: [0; 10] },
        Z: fe { v: [0; 10] },
        T: fe { v: [0; 10] },
    };
    let mut i: core::ffi::c_int;
    slide(aslide.as_mut_ptr(), a);
    slide(bslide.as_mut_ptr(), b);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
        A,
    );
    ge_p3_dbl(&mut t, A);
    x25519_ge_p1p1_to_p3(&mut A2, &mut t);
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(0 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(1 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(2 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(2 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(3 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(3 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(4 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(4 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(5 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(5 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(6 as core::ffi::c_int as isize),
        &mut u,
    );
    x25519_ge_add(
        &mut t,
        &mut A2,
        &mut *Ai.as_mut_ptr().offset(6 as core::ffi::c_int as isize),
    );
    x25519_ge_p1p1_to_p3(&mut u, &mut t);
    x25519_ge_p3_to_cached(
        &mut *Ai.as_mut_ptr().offset(7 as core::ffi::c_int as isize),
        &mut u,
    );
    ge_p2_0(r);
    i = 255 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        if aslide[i as usize] as core::ffi::c_int != 0
            || bslide[i as usize] as core::ffi::c_int != 0
        {
            break;
        }
        i -= 1;
    }
    while i >= 0 as core::ffi::c_int {
        ge_p2_dbl(&mut t, r);
        if aslide[i as usize] as core::ffi::c_int > 0 as core::ffi::c_int {
            x25519_ge_p1p1_to_p3(&mut u, &mut t);
            x25519_ge_add(
                &mut t,
                &mut u,
                &mut *Ai.as_mut_ptr().offset(
                    (*aslide.as_mut_ptr().offset(i as isize) as core::ffi::c_int
                        / 2 as core::ffi::c_int) as isize,
                ),
            );
        } else if (aslide[i as usize] as core::ffi::c_int) < 0 as core::ffi::c_int {
            x25519_ge_p1p1_to_p3(&mut u, &mut t);
            x25519_ge_sub(
                &mut t,
                &mut u,
                &mut *Ai.as_mut_ptr().offset(
                    (-(*aslide.as_mut_ptr().offset(i as isize) as core::ffi::c_int)
                        / 2 as core::ffi::c_int) as isize,
                ),
            );
        }
        if bslide[i as usize] as core::ffi::c_int > 0 as core::ffi::c_int {
            x25519_ge_p1p1_to_p3(&mut u, &mut t);
            ge_madd(
                &mut t,
                &mut u,
                &*Bi.as_ptr().offset(
                    (*bslide.as_mut_ptr().offset(i as isize) as core::ffi::c_int
                        / 2 as core::ffi::c_int) as isize,
                ),
            );
        } else if (bslide[i as usize] as core::ffi::c_int) < 0 as core::ffi::c_int {
            x25519_ge_p1p1_to_p3(&mut u, &mut t);
            ge_msub(
                &mut t,
                &mut u,
                &*Bi.as_ptr().offset(
                    (-(*bslide.as_mut_ptr().offset(i as isize) as core::ffi::c_int)
                        / 2 as core::ffi::c_int) as isize,
                ),
            );
        }
        x25519_ge_p1p1_to_p2(r, &mut t);
        i -= 1;
    }
}
#[inline]
unsafe extern "C" fn int64_lshift21(a: int64_t) -> int64_t {
    return ((a as uint64_t) << 21 as core::ffi::c_int) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_sc_reduce(s: *mut uint8_t) {
    let mut s0: int64_t =
        (2097151 as core::ffi::c_int as u64 & load_3(s as *const uint8_t)) as int64_t;
    let mut s1: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(2 as core::ffi::c_int as isize) as *const uint8_t)
            >> 5 as core::ffi::c_int) as int64_t;
    let mut s2: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(5 as core::ffi::c_int as isize) as *const uint8_t)
            >> 2 as core::ffi::c_int) as int64_t;
    let mut s3: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(7 as core::ffi::c_int as isize) as *const uint8_t)
            >> 7 as core::ffi::c_int) as int64_t;
    let mut s4: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(10 as core::ffi::c_int as isize) as *const uint8_t)
            >> 4 as core::ffi::c_int) as int64_t;
    let mut s5: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(13 as core::ffi::c_int as isize) as *const uint8_t)
            >> 1 as core::ffi::c_int) as int64_t;
    let mut s6: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(15 as core::ffi::c_int as isize) as *const uint8_t)
            >> 6 as core::ffi::c_int) as int64_t;
    let mut s7: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(18 as core::ffi::c_int as isize) as *const uint8_t)
            >> 3 as core::ffi::c_int) as int64_t;
    let mut s8: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(21 as core::ffi::c_int as isize) as *const uint8_t))
        as int64_t;
    let mut s9: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(23 as core::ffi::c_int as isize) as *const uint8_t)
            >> 5 as core::ffi::c_int) as int64_t;
    let mut s10: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(26 as core::ffi::c_int as isize) as *const uint8_t)
            >> 2 as core::ffi::c_int) as int64_t;
    let mut s11: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(28 as core::ffi::c_int as isize) as *const uint8_t)
            >> 7 as core::ffi::c_int) as int64_t;
    let mut s12: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(31 as core::ffi::c_int as isize) as *const uint8_t)
            >> 4 as core::ffi::c_int) as int64_t;
    let mut s13: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(34 as core::ffi::c_int as isize) as *const uint8_t)
            >> 1 as core::ffi::c_int) as int64_t;
    let mut s14: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(36 as core::ffi::c_int as isize) as *const uint8_t)
            >> 6 as core::ffi::c_int) as int64_t;
    let mut s15: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(39 as core::ffi::c_int as isize) as *const uint8_t)
            >> 3 as core::ffi::c_int) as int64_t;
    let mut s16: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(42 as core::ffi::c_int as isize) as *const uint8_t))
        as int64_t;
    let mut s17: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(44 as core::ffi::c_int as isize) as *const uint8_t)
            >> 5 as core::ffi::c_int) as int64_t;
    let s18: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(47 as core::ffi::c_int as isize) as *const uint8_t)
            >> 2 as core::ffi::c_int) as int64_t;
    let s19: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(49 as core::ffi::c_int as isize) as *const uint8_t)
            >> 7 as core::ffi::c_int) as int64_t;
    let s20: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(52 as core::ffi::c_int as isize) as *const uint8_t)
            >> 4 as core::ffi::c_int) as int64_t;
    let s21: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(s.offset(55 as core::ffi::c_int as isize) as *const uint8_t)
            >> 1 as core::ffi::c_int) as int64_t;
    let s22: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(s.offset(57 as core::ffi::c_int as isize) as *const uint8_t)
            >> 6 as core::ffi::c_int) as int64_t;
    let s23: int64_t = (load_4(s.offset(60 as core::ffi::c_int as isize) as *const uint8_t)
        >> 3 as core::ffi::c_int) as int64_t;
    let mut carry0: int64_t;
    let mut carry1: int64_t;
    let mut carry2: int64_t;
    let mut carry3: int64_t;
    let mut carry4: int64_t;
    let mut carry5: int64_t;
    let mut carry6: int64_t;
    let mut carry7: int64_t;
    let mut carry8: int64_t;
    let mut carry9: int64_t;
    let mut carry10: int64_t;
    let mut carry11: int64_t;
    let carry12: int64_t;
    let carry13: int64_t;
    let carry14: int64_t;
    let carry15: int64_t;
    let carry16: int64_t;
    s11 += s23 * 666643 as core::ffi::c_int as i64;
    s12 += s23 * 470296 as core::ffi::c_int as i64;
    s13 += s23 * 654183 as core::ffi::c_int as i64;
    s14 -= s23 * 997805 as core::ffi::c_int as i64;
    s15 += s23 * 136657 as core::ffi::c_int as i64;
    s16 -= s23 * 683901 as core::ffi::c_int as i64;
    s23 as core::ffi::c_int as int64_t;
    s10 += s22 * 666643 as core::ffi::c_int as i64;
    s11 += s22 * 470296 as core::ffi::c_int as i64;
    s12 += s22 * 654183 as core::ffi::c_int as i64;
    s13 -= s22 * 997805 as core::ffi::c_int as i64;
    s14 += s22 * 136657 as core::ffi::c_int as i64;
    s15 -= s22 * 683901 as core::ffi::c_int as i64;
    s22 as core::ffi::c_int as int64_t;
    s9 += s21 * 666643 as core::ffi::c_int as i64;
    s10 += s21 * 470296 as core::ffi::c_int as i64;
    s11 += s21 * 654183 as core::ffi::c_int as i64;
    s12 -= s21 * 997805 as core::ffi::c_int as i64;
    s13 += s21 * 136657 as core::ffi::c_int as i64;
    s14 -= s21 * 683901 as core::ffi::c_int as i64;
    s21 as core::ffi::c_int as int64_t;
    s8 += s20 * 666643 as core::ffi::c_int as i64;
    s9 += s20 * 470296 as core::ffi::c_int as i64;
    s10 += s20 * 654183 as core::ffi::c_int as i64;
    s11 -= s20 * 997805 as core::ffi::c_int as i64;
    s12 += s20 * 136657 as core::ffi::c_int as i64;
    s13 -= s20 * 683901 as core::ffi::c_int as i64;
    s20 as core::ffi::c_int as int64_t;
    s7 += s19 * 666643 as core::ffi::c_int as i64;
    s8 += s19 * 470296 as core::ffi::c_int as i64;
    s9 += s19 * 654183 as core::ffi::c_int as i64;
    s10 -= s19 * 997805 as core::ffi::c_int as i64;
    s11 += s19 * 136657 as core::ffi::c_int as i64;
    s12 -= s19 * 683901 as core::ffi::c_int as i64;
    s19 as core::ffi::c_int as int64_t;
    s6 += s18 * 666643 as core::ffi::c_int as i64;
    s7 += s18 * 470296 as core::ffi::c_int as i64;
    s8 += s18 * 654183 as core::ffi::c_int as i64;
    s9 -= s18 * 997805 as core::ffi::c_int as i64;
    s10 += s18 * 136657 as core::ffi::c_int as i64;
    s11 -= s18 * 683901 as core::ffi::c_int as i64;
    s18 as core::ffi::c_int as int64_t;
    carry6 =
        s6 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry8 =
        s8 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry10 =
        s10 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry12 =
        s12 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s13 += carry12;
    s12 -= int64_lshift21(carry12);
    carry14 =
        s14 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s15 += carry14;
    s14 -= int64_lshift21(carry14);
    carry16 =
        s16 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s17 += carry16;
    s16 -= int64_lshift21(carry16);
    carry7 =
        s7 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry9 =
        s9 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry11 =
        s11 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    carry13 =
        s13 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s14 += carry13;
    s13 -= int64_lshift21(carry13);
    carry15 =
        s15 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s16 += carry15;
    s15 -= int64_lshift21(carry15);
    s5 += s17 * 666643 as core::ffi::c_int as i64;
    s6 += s17 * 470296 as core::ffi::c_int as i64;
    s7 += s17 * 654183 as core::ffi::c_int as i64;
    s8 -= s17 * 997805 as core::ffi::c_int as i64;
    s9 += s17 * 136657 as core::ffi::c_int as i64;
    s10 -= s17 * 683901 as core::ffi::c_int as i64;
    s17 as core::ffi::c_int as int64_t;
    s4 += s16 * 666643 as core::ffi::c_int as i64;
    s5 += s16 * 470296 as core::ffi::c_int as i64;
    s6 += s16 * 654183 as core::ffi::c_int as i64;
    s7 -= s16 * 997805 as core::ffi::c_int as i64;
    s8 += s16 * 136657 as core::ffi::c_int as i64;
    s9 -= s16 * 683901 as core::ffi::c_int as i64;
    s16 as core::ffi::c_int as int64_t;
    s3 += s15 * 666643 as core::ffi::c_int as i64;
    s4 += s15 * 470296 as core::ffi::c_int as i64;
    s5 += s15 * 654183 as core::ffi::c_int as i64;
    s6 -= s15 * 997805 as core::ffi::c_int as i64;
    s7 += s15 * 136657 as core::ffi::c_int as i64;
    s8 -= s15 * 683901 as core::ffi::c_int as i64;
    s15 as core::ffi::c_int as int64_t;
    s2 += s14 * 666643 as core::ffi::c_int as i64;
    s3 += s14 * 470296 as core::ffi::c_int as i64;
    s4 += s14 * 654183 as core::ffi::c_int as i64;
    s5 -= s14 * 997805 as core::ffi::c_int as i64;
    s6 += s14 * 136657 as core::ffi::c_int as i64;
    s7 -= s14 * 683901 as core::ffi::c_int as i64;
    s14 as core::ffi::c_int as int64_t;
    s1 += s13 * 666643 as core::ffi::c_int as i64;
    s2 += s13 * 470296 as core::ffi::c_int as i64;
    s3 += s13 * 654183 as core::ffi::c_int as i64;
    s4 -= s13 * 997805 as core::ffi::c_int as i64;
    s5 += s13 * 136657 as core::ffi::c_int as i64;
    s6 -= s13 * 683901 as core::ffi::c_int as i64;
    s13 as core::ffi::c_int as int64_t;
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 = 0 as core::ffi::c_int as int64_t;
    carry0 =
        s0 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry2 =
        s2 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry4 =
        s4 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry6 =
        s6 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry8 =
        s8 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry10 =
        s10 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry1 =
        s1 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry3 =
        s3 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry5 =
        s5 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry7 =
        s7 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry9 =
        s9 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry11 =
        s11 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 = 0 as core::ffi::c_int as int64_t;
    carry0 = s0 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry1 = s1 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry2 = s2 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry3 = s3 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry4 = s4 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry5 = s5 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry6 = s6 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry7 = s7 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry8 = s8 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry9 = s9 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry10 = s10 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry11 = s11 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 as core::ffi::c_int as int64_t;
    carry0 = s0 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry1 = s1 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry2 = s2 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry3 = s3 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry4 = s4 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry5 = s5 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry6 = s6 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry7 = s7 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry8 = s8 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry9 = s9 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry10 = s10 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    *s.offset(0 as core::ffi::c_int as isize) = (s0 >> 0 as core::ffi::c_int) as uint8_t;
    *s.offset(1 as core::ffi::c_int as isize) = (s0 >> 8 as core::ffi::c_int) as uint8_t;
    *s.offset(2 as core::ffi::c_int as isize) =
        (s0 >> 16 as core::ffi::c_int | s1 << 5 as core::ffi::c_int) as uint8_t;
    *s.offset(3 as core::ffi::c_int as isize) = (s1 >> 3 as core::ffi::c_int) as uint8_t;
    *s.offset(4 as core::ffi::c_int as isize) = (s1 >> 11 as core::ffi::c_int) as uint8_t;
    *s.offset(5 as core::ffi::c_int as isize) =
        (s1 >> 19 as core::ffi::c_int | s2 << 2 as core::ffi::c_int) as uint8_t;
    *s.offset(6 as core::ffi::c_int as isize) = (s2 >> 6 as core::ffi::c_int) as uint8_t;
    *s.offset(7 as core::ffi::c_int as isize) =
        (s2 >> 14 as core::ffi::c_int | s3 << 7 as core::ffi::c_int) as uint8_t;
    *s.offset(8 as core::ffi::c_int as isize) = (s3 >> 1 as core::ffi::c_int) as uint8_t;
    *s.offset(9 as core::ffi::c_int as isize) = (s3 >> 9 as core::ffi::c_int) as uint8_t;
    *s.offset(10 as core::ffi::c_int as isize) =
        (s3 >> 17 as core::ffi::c_int | s4 << 4 as core::ffi::c_int) as uint8_t;
    *s.offset(11 as core::ffi::c_int as isize) = (s4 >> 4 as core::ffi::c_int) as uint8_t;
    *s.offset(12 as core::ffi::c_int as isize) = (s4 >> 12 as core::ffi::c_int) as uint8_t;
    *s.offset(13 as core::ffi::c_int as isize) =
        (s4 >> 20 as core::ffi::c_int | s5 << 1 as core::ffi::c_int) as uint8_t;
    *s.offset(14 as core::ffi::c_int as isize) = (s5 >> 7 as core::ffi::c_int) as uint8_t;
    *s.offset(15 as core::ffi::c_int as isize) =
        (s5 >> 15 as core::ffi::c_int | s6 << 6 as core::ffi::c_int) as uint8_t;
    *s.offset(16 as core::ffi::c_int as isize) = (s6 >> 2 as core::ffi::c_int) as uint8_t;
    *s.offset(17 as core::ffi::c_int as isize) = (s6 >> 10 as core::ffi::c_int) as uint8_t;
    *s.offset(18 as core::ffi::c_int as isize) =
        (s6 >> 18 as core::ffi::c_int | s7 << 3 as core::ffi::c_int) as uint8_t;
    *s.offset(19 as core::ffi::c_int as isize) = (s7 >> 5 as core::ffi::c_int) as uint8_t;
    *s.offset(20 as core::ffi::c_int as isize) = (s7 >> 13 as core::ffi::c_int) as uint8_t;
    *s.offset(21 as core::ffi::c_int as isize) = (s8 >> 0 as core::ffi::c_int) as uint8_t;
    *s.offset(22 as core::ffi::c_int as isize) = (s8 >> 8 as core::ffi::c_int) as uint8_t;
    *s.offset(23 as core::ffi::c_int as isize) =
        (s8 >> 16 as core::ffi::c_int | s9 << 5 as core::ffi::c_int) as uint8_t;
    *s.offset(24 as core::ffi::c_int as isize) = (s9 >> 3 as core::ffi::c_int) as uint8_t;
    *s.offset(25 as core::ffi::c_int as isize) = (s9 >> 11 as core::ffi::c_int) as uint8_t;
    *s.offset(26 as core::ffi::c_int as isize) =
        (s9 >> 19 as core::ffi::c_int | s10 << 2 as core::ffi::c_int) as uint8_t;
    *s.offset(27 as core::ffi::c_int as isize) = (s10 >> 6 as core::ffi::c_int) as uint8_t;
    *s.offset(28 as core::ffi::c_int as isize) =
        (s10 >> 14 as core::ffi::c_int | s11 << 7 as core::ffi::c_int) as uint8_t;
    *s.offset(29 as core::ffi::c_int as isize) = (s11 >> 1 as core::ffi::c_int) as uint8_t;
    *s.offset(30 as core::ffi::c_int as isize) = (s11 >> 9 as core::ffi::c_int) as uint8_t;
    *s.offset(31 as core::ffi::c_int as isize) = (s11 >> 17 as core::ffi::c_int) as uint8_t;
}
unsafe extern "C" fn sc_muladd(
    s: *mut uint8_t,
    a: *const uint8_t,
    b: *const uint8_t,
    c: *const uint8_t,
) {
    let a0: int64_t = (2097151 as core::ffi::c_int as u64 & load_3(a)) as int64_t;
    let a1: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(a.offset(2 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let a2: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(a.offset(5 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let a3: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(a.offset(7 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int)
        as int64_t;
    let a4: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(a.offset(10 as core::ffi::c_int as isize)) >> 4 as core::ffi::c_int)
        as int64_t;
    let a5: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(a.offset(13 as core::ffi::c_int as isize)) >> 1 as core::ffi::c_int)
        as int64_t;
    let a6: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(a.offset(15 as core::ffi::c_int as isize)) >> 6 as core::ffi::c_int)
        as int64_t;
    let a7: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(a.offset(18 as core::ffi::c_int as isize)) >> 3 as core::ffi::c_int)
        as int64_t;
    let a8: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(a.offset(21 as core::ffi::c_int as isize))) as int64_t;
    let a9: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(a.offset(23 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let a10: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(a.offset(26 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let a11: int64_t =
        (load_4(a.offset(28 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int) as int64_t;
    let b0: int64_t = (2097151 as core::ffi::c_int as u64 & load_3(b)) as int64_t;
    let b1: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(b.offset(2 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let b2: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(b.offset(5 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let b3: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(b.offset(7 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int)
        as int64_t;
    let b4: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(b.offset(10 as core::ffi::c_int as isize)) >> 4 as core::ffi::c_int)
        as int64_t;
    let b5: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(b.offset(13 as core::ffi::c_int as isize)) >> 1 as core::ffi::c_int)
        as int64_t;
    let b6: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(b.offset(15 as core::ffi::c_int as isize)) >> 6 as core::ffi::c_int)
        as int64_t;
    let b7: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(b.offset(18 as core::ffi::c_int as isize)) >> 3 as core::ffi::c_int)
        as int64_t;
    let b8: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(b.offset(21 as core::ffi::c_int as isize))) as int64_t;
    let b9: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(b.offset(23 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let b10: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(b.offset(26 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let b11: int64_t =
        (load_4(b.offset(28 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int) as int64_t;
    let c0: int64_t = (2097151 as core::ffi::c_int as u64 & load_3(c)) as int64_t;
    let c1: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(c.offset(2 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let c2: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(c.offset(5 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let c3: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(c.offset(7 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int)
        as int64_t;
    let c4: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(c.offset(10 as core::ffi::c_int as isize)) >> 4 as core::ffi::c_int)
        as int64_t;
    let c5: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(c.offset(13 as core::ffi::c_int as isize)) >> 1 as core::ffi::c_int)
        as int64_t;
    let c6: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(c.offset(15 as core::ffi::c_int as isize)) >> 6 as core::ffi::c_int)
        as int64_t;
    let c7: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(c.offset(18 as core::ffi::c_int as isize)) >> 3 as core::ffi::c_int)
        as int64_t;
    let c8: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(c.offset(21 as core::ffi::c_int as isize))) as int64_t;
    let c9: int64_t = (2097151 as core::ffi::c_int as u64
        & load_4(c.offset(23 as core::ffi::c_int as isize)) >> 5 as core::ffi::c_int)
        as int64_t;
    let c10: int64_t = (2097151 as core::ffi::c_int as u64
        & load_3(c.offset(26 as core::ffi::c_int as isize)) >> 2 as core::ffi::c_int)
        as int64_t;
    let c11: int64_t =
        (load_4(c.offset(28 as core::ffi::c_int as isize)) >> 7 as core::ffi::c_int) as int64_t;
    let mut s0: int64_t;
    let mut s1: int64_t;
    let mut s2: int64_t;
    let mut s3: int64_t;
    let mut s4: int64_t;
    let mut s5: int64_t;
    let mut s6: int64_t;
    let mut s7: int64_t;
    let mut s8: int64_t;
    let mut s9: int64_t;
    let mut s10: int64_t;
    let mut s11: int64_t;
    let mut s12: int64_t;
    let mut s13: int64_t;
    let mut s14: int64_t;
    let mut s15: int64_t;
    let mut s16: int64_t;
    let mut s17: int64_t;
    let mut s18: int64_t;
    let mut s19: int64_t;
    let mut s20: int64_t;
    let mut s21: int64_t;
    let mut s22: int64_t;
    let mut s23: int64_t;
    let mut carry0: int64_t;
    let mut carry1: int64_t;
    let mut carry2: int64_t;
    let mut carry3: int64_t;
    let mut carry4: int64_t;
    let mut carry5: int64_t;
    let mut carry6: int64_t;
    let mut carry7: int64_t;
    let mut carry8: int64_t;
    let mut carry9: int64_t;
    let mut carry10: int64_t;
    let mut carry11: int64_t;
    let mut carry12: int64_t;
    let mut carry13: int64_t;
    let mut carry14: int64_t;
    let mut carry15: int64_t;
    let mut carry16: int64_t;
    let carry17: int64_t;
    let carry18: int64_t;
    let carry19: int64_t;
    let carry20: int64_t;
    let carry21: int64_t;
    let carry22: int64_t;
    s0 = c0 + a0 * b0;
    s1 = c1 + a0 * b1 + a1 * b0;
    s2 = c2 + a0 * b2 + a1 * b1 + a2 * b0;
    s3 = c3 + a0 * b3 + a1 * b2 + a2 * b1 + a3 * b0;
    s4 = c4 + a0 * b4 + a1 * b3 + a2 * b2 + a3 * b1 + a4 * b0;
    s5 = c5 + a0 * b5 + a1 * b4 + a2 * b3 + a3 * b2 + a4 * b1 + a5 * b0;
    s6 = c6 + a0 * b6 + a1 * b5 + a2 * b4 + a3 * b3 + a4 * b2 + a5 * b1 + a6 * b0;
    s7 = c7 + a0 * b7 + a1 * b6 + a2 * b5 + a3 * b4 + a4 * b3 + a5 * b2 + a6 * b1 + a7 * b0;
    s8 = c8
        + a0 * b8
        + a1 * b7
        + a2 * b6
        + a3 * b5
        + a4 * b4
        + a5 * b3
        + a6 * b2
        + a7 * b1
        + a8 * b0;
    s9 = c9
        + a0 * b9
        + a1 * b8
        + a2 * b7
        + a3 * b6
        + a4 * b5
        + a5 * b4
        + a6 * b3
        + a7 * b2
        + a8 * b1
        + a9 * b0;
    s10 = c10
        + a0 * b10
        + a1 * b9
        + a2 * b8
        + a3 * b7
        + a4 * b6
        + a5 * b5
        + a6 * b4
        + a7 * b3
        + a8 * b2
        + a9 * b1
        + a10 * b0;
    s11 = c11
        + a0 * b11
        + a1 * b10
        + a2 * b9
        + a3 * b8
        + a4 * b7
        + a5 * b6
        + a6 * b5
        + a7 * b4
        + a8 * b3
        + a9 * b2
        + a10 * b1
        + a11 * b0;
    s12 = a1 * b11
        + a2 * b10
        + a3 * b9
        + a4 * b8
        + a5 * b7
        + a6 * b6
        + a7 * b5
        + a8 * b4
        + a9 * b3
        + a10 * b2
        + a11 * b1;
    s13 = a2 * b11
        + a3 * b10
        + a4 * b9
        + a5 * b8
        + a6 * b7
        + a7 * b6
        + a8 * b5
        + a9 * b4
        + a10 * b3
        + a11 * b2;
    s14 =
        a3 * b11 + a4 * b10 + a5 * b9 + a6 * b8 + a7 * b7 + a8 * b6 + a9 * b5 + a10 * b4 + a11 * b3;
    s15 = a4 * b11 + a5 * b10 + a6 * b9 + a7 * b8 + a8 * b7 + a9 * b6 + a10 * b5 + a11 * b4;
    s16 = a5 * b11 + a6 * b10 + a7 * b9 + a8 * b8 + a9 * b7 + a10 * b6 + a11 * b5;
    s17 = a6 * b11 + a7 * b10 + a8 * b9 + a9 * b8 + a10 * b7 + a11 * b6;
    s18 = a7 * b11 + a8 * b10 + a9 * b9 + a10 * b8 + a11 * b7;
    s19 = a8 * b11 + a9 * b10 + a10 * b9 + a11 * b8;
    s20 = a9 * b11 + a10 * b10 + a11 * b9;
    s21 = a10 * b11 + a11 * b10;
    s22 = a11 * b11;
    s23 = 0 as core::ffi::c_int as int64_t;
    carry0 =
        s0 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry2 =
        s2 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry4 =
        s4 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry6 =
        s6 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry8 =
        s8 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry10 =
        s10 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry12 =
        s12 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s13 += carry12;
    s12 -= int64_lshift21(carry12);
    carry14 =
        s14 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s15 += carry14;
    s14 -= int64_lshift21(carry14);
    carry16 =
        s16 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s17 += carry16;
    s16 -= int64_lshift21(carry16);
    carry18 =
        s18 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s19 += carry18;
    s18 -= int64_lshift21(carry18);
    carry20 =
        s20 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s21 += carry20;
    s20 -= int64_lshift21(carry20);
    carry22 =
        s22 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s23 += carry22;
    s22 -= int64_lshift21(carry22);
    carry1 =
        s1 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry3 =
        s3 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry5 =
        s5 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry7 =
        s7 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry9 =
        s9 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry11 =
        s11 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    carry13 =
        s13 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s14 += carry13;
    s13 -= int64_lshift21(carry13);
    carry15 =
        s15 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s16 += carry15;
    s15 -= int64_lshift21(carry15);
    carry17 =
        s17 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s18 += carry17;
    s17 -= int64_lshift21(carry17);
    carry19 =
        s19 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s20 += carry19;
    s19 -= int64_lshift21(carry19);
    carry21 =
        s21 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s22 += carry21;
    s21 -= int64_lshift21(carry21);
    s11 += s23 * 666643 as core::ffi::c_int as i64;
    s12 += s23 * 470296 as core::ffi::c_int as i64;
    s13 += s23 * 654183 as core::ffi::c_int as i64;
    s14 -= s23 * 997805 as core::ffi::c_int as i64;
    s15 += s23 * 136657 as core::ffi::c_int as i64;
    s16 -= s23 * 683901 as core::ffi::c_int as i64;
    s23 as core::ffi::c_int as int64_t;
    s10 += s22 * 666643 as core::ffi::c_int as i64;
    s11 += s22 * 470296 as core::ffi::c_int as i64;
    s12 += s22 * 654183 as core::ffi::c_int as i64;
    s13 -= s22 * 997805 as core::ffi::c_int as i64;
    s14 += s22 * 136657 as core::ffi::c_int as i64;
    s15 -= s22 * 683901 as core::ffi::c_int as i64;
    s22 as core::ffi::c_int as int64_t;
    s9 += s21 * 666643 as core::ffi::c_int as i64;
    s10 += s21 * 470296 as core::ffi::c_int as i64;
    s11 += s21 * 654183 as core::ffi::c_int as i64;
    s12 -= s21 * 997805 as core::ffi::c_int as i64;
    s13 += s21 * 136657 as core::ffi::c_int as i64;
    s14 -= s21 * 683901 as core::ffi::c_int as i64;
    s21 as core::ffi::c_int as int64_t;
    s8 += s20 * 666643 as core::ffi::c_int as i64;
    s9 += s20 * 470296 as core::ffi::c_int as i64;
    s10 += s20 * 654183 as core::ffi::c_int as i64;
    s11 -= s20 * 997805 as core::ffi::c_int as i64;
    s12 += s20 * 136657 as core::ffi::c_int as i64;
    s13 -= s20 * 683901 as core::ffi::c_int as i64;
    s20 as core::ffi::c_int as int64_t;
    s7 += s19 * 666643 as core::ffi::c_int as i64;
    s8 += s19 * 470296 as core::ffi::c_int as i64;
    s9 += s19 * 654183 as core::ffi::c_int as i64;
    s10 -= s19 * 997805 as core::ffi::c_int as i64;
    s11 += s19 * 136657 as core::ffi::c_int as i64;
    s12 -= s19 * 683901 as core::ffi::c_int as i64;
    s19 as core::ffi::c_int as int64_t;
    s6 += s18 * 666643 as core::ffi::c_int as i64;
    s7 += s18 * 470296 as core::ffi::c_int as i64;
    s8 += s18 * 654183 as core::ffi::c_int as i64;
    s9 -= s18 * 997805 as core::ffi::c_int as i64;
    s10 += s18 * 136657 as core::ffi::c_int as i64;
    s11 -= s18 * 683901 as core::ffi::c_int as i64;
    s18 as core::ffi::c_int as int64_t;
    carry6 =
        s6 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry8 =
        s8 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry10 =
        s10 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry12 =
        s12 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s13 += carry12;
    s12 -= int64_lshift21(carry12);
    carry14 =
        s14 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s15 += carry14;
    s14 -= int64_lshift21(carry14);
    carry16 =
        s16 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s17 += carry16;
    s16 -= int64_lshift21(carry16);
    carry7 =
        s7 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry9 =
        s9 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry11 =
        s11 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    carry13 =
        s13 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s14 += carry13;
    s13 -= int64_lshift21(carry13);
    carry15 =
        s15 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s16 += carry15;
    s15 -= int64_lshift21(carry15);
    s5 += s17 * 666643 as core::ffi::c_int as i64;
    s6 += s17 * 470296 as core::ffi::c_int as i64;
    s7 += s17 * 654183 as core::ffi::c_int as i64;
    s8 -= s17 * 997805 as core::ffi::c_int as i64;
    s9 += s17 * 136657 as core::ffi::c_int as i64;
    s10 -= s17 * 683901 as core::ffi::c_int as i64;
    s17 as core::ffi::c_int as int64_t;
    s4 += s16 * 666643 as core::ffi::c_int as i64;
    s5 += s16 * 470296 as core::ffi::c_int as i64;
    s6 += s16 * 654183 as core::ffi::c_int as i64;
    s7 -= s16 * 997805 as core::ffi::c_int as i64;
    s8 += s16 * 136657 as core::ffi::c_int as i64;
    s9 -= s16 * 683901 as core::ffi::c_int as i64;
    s16 as core::ffi::c_int as int64_t;
    s3 += s15 * 666643 as core::ffi::c_int as i64;
    s4 += s15 * 470296 as core::ffi::c_int as i64;
    s5 += s15 * 654183 as core::ffi::c_int as i64;
    s6 -= s15 * 997805 as core::ffi::c_int as i64;
    s7 += s15 * 136657 as core::ffi::c_int as i64;
    s8 -= s15 * 683901 as core::ffi::c_int as i64;
    s15 as core::ffi::c_int as int64_t;
    s2 += s14 * 666643 as core::ffi::c_int as i64;
    s3 += s14 * 470296 as core::ffi::c_int as i64;
    s4 += s14 * 654183 as core::ffi::c_int as i64;
    s5 -= s14 * 997805 as core::ffi::c_int as i64;
    s6 += s14 * 136657 as core::ffi::c_int as i64;
    s7 -= s14 * 683901 as core::ffi::c_int as i64;
    s14 as core::ffi::c_int as int64_t;
    s1 += s13 * 666643 as core::ffi::c_int as i64;
    s2 += s13 * 470296 as core::ffi::c_int as i64;
    s3 += s13 * 654183 as core::ffi::c_int as i64;
    s4 -= s13 * 997805 as core::ffi::c_int as i64;
    s5 += s13 * 136657 as core::ffi::c_int as i64;
    s6 -= s13 * 683901 as core::ffi::c_int as i64;
    s13 as core::ffi::c_int as int64_t;
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 = 0 as core::ffi::c_int as int64_t;
    carry0 =
        s0 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry2 =
        s2 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry4 =
        s4 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry6 =
        s6 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry8 =
        s8 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry10 =
        s10 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry1 =
        s1 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry3 =
        s3 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry5 =
        s5 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry7 =
        s7 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry9 =
        s9 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry11 =
        s11 + ((1 as core::ffi::c_int) << 20 as core::ffi::c_int) as i64 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 = 0 as core::ffi::c_int as int64_t;
    carry0 = s0 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry1 = s1 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry2 = s2 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry3 = s3 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry4 = s4 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry5 = s5 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry6 = s6 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry7 = s7 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry8 = s8 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry9 = s9 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry10 = s10 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    carry11 = s11 >> 21 as core::ffi::c_int;
    s12 += carry11;
    s11 -= int64_lshift21(carry11);
    s0 += s12 * 666643 as core::ffi::c_int as i64;
    s1 += s12 * 470296 as core::ffi::c_int as i64;
    s2 += s12 * 654183 as core::ffi::c_int as i64;
    s3 -= s12 * 997805 as core::ffi::c_int as i64;
    s4 += s12 * 136657 as core::ffi::c_int as i64;
    s5 -= s12 * 683901 as core::ffi::c_int as i64;
    s12 as core::ffi::c_int as int64_t;
    carry0 = s0 >> 21 as core::ffi::c_int;
    s1 += carry0;
    s0 -= int64_lshift21(carry0);
    carry1 = s1 >> 21 as core::ffi::c_int;
    s2 += carry1;
    s1 -= int64_lshift21(carry1);
    carry2 = s2 >> 21 as core::ffi::c_int;
    s3 += carry2;
    s2 -= int64_lshift21(carry2);
    carry3 = s3 >> 21 as core::ffi::c_int;
    s4 += carry3;
    s3 -= int64_lshift21(carry3);
    carry4 = s4 >> 21 as core::ffi::c_int;
    s5 += carry4;
    s4 -= int64_lshift21(carry4);
    carry5 = s5 >> 21 as core::ffi::c_int;
    s6 += carry5;
    s5 -= int64_lshift21(carry5);
    carry6 = s6 >> 21 as core::ffi::c_int;
    s7 += carry6;
    s6 -= int64_lshift21(carry6);
    carry7 = s7 >> 21 as core::ffi::c_int;
    s8 += carry7;
    s7 -= int64_lshift21(carry7);
    carry8 = s8 >> 21 as core::ffi::c_int;
    s9 += carry8;
    s8 -= int64_lshift21(carry8);
    carry9 = s9 >> 21 as core::ffi::c_int;
    s10 += carry9;
    s9 -= int64_lshift21(carry9);
    carry10 = s10 >> 21 as core::ffi::c_int;
    s11 += carry10;
    s10 -= int64_lshift21(carry10);
    *s.offset(0 as core::ffi::c_int as isize) = (s0 >> 0 as core::ffi::c_int) as uint8_t;
    *s.offset(1 as core::ffi::c_int as isize) = (s0 >> 8 as core::ffi::c_int) as uint8_t;
    *s.offset(2 as core::ffi::c_int as isize) =
        (s0 >> 16 as core::ffi::c_int | s1 << 5 as core::ffi::c_int) as uint8_t;
    *s.offset(3 as core::ffi::c_int as isize) = (s1 >> 3 as core::ffi::c_int) as uint8_t;
    *s.offset(4 as core::ffi::c_int as isize) = (s1 >> 11 as core::ffi::c_int) as uint8_t;
    *s.offset(5 as core::ffi::c_int as isize) =
        (s1 >> 19 as core::ffi::c_int | s2 << 2 as core::ffi::c_int) as uint8_t;
    *s.offset(6 as core::ffi::c_int as isize) = (s2 >> 6 as core::ffi::c_int) as uint8_t;
    *s.offset(7 as core::ffi::c_int as isize) =
        (s2 >> 14 as core::ffi::c_int | s3 << 7 as core::ffi::c_int) as uint8_t;
    *s.offset(8 as core::ffi::c_int as isize) = (s3 >> 1 as core::ffi::c_int) as uint8_t;
    *s.offset(9 as core::ffi::c_int as isize) = (s3 >> 9 as core::ffi::c_int) as uint8_t;
    *s.offset(10 as core::ffi::c_int as isize) =
        (s3 >> 17 as core::ffi::c_int | s4 << 4 as core::ffi::c_int) as uint8_t;
    *s.offset(11 as core::ffi::c_int as isize) = (s4 >> 4 as core::ffi::c_int) as uint8_t;
    *s.offset(12 as core::ffi::c_int as isize) = (s4 >> 12 as core::ffi::c_int) as uint8_t;
    *s.offset(13 as core::ffi::c_int as isize) =
        (s4 >> 20 as core::ffi::c_int | s5 << 1 as core::ffi::c_int) as uint8_t;
    *s.offset(14 as core::ffi::c_int as isize) = (s5 >> 7 as core::ffi::c_int) as uint8_t;
    *s.offset(15 as core::ffi::c_int as isize) =
        (s5 >> 15 as core::ffi::c_int | s6 << 6 as core::ffi::c_int) as uint8_t;
    *s.offset(16 as core::ffi::c_int as isize) = (s6 >> 2 as core::ffi::c_int) as uint8_t;
    *s.offset(17 as core::ffi::c_int as isize) = (s6 >> 10 as core::ffi::c_int) as uint8_t;
    *s.offset(18 as core::ffi::c_int as isize) =
        (s6 >> 18 as core::ffi::c_int | s7 << 3 as core::ffi::c_int) as uint8_t;
    *s.offset(19 as core::ffi::c_int as isize) = (s7 >> 5 as core::ffi::c_int) as uint8_t;
    *s.offset(20 as core::ffi::c_int as isize) = (s7 >> 13 as core::ffi::c_int) as uint8_t;
    *s.offset(21 as core::ffi::c_int as isize) = (s8 >> 0 as core::ffi::c_int) as uint8_t;
    *s.offset(22 as core::ffi::c_int as isize) = (s8 >> 8 as core::ffi::c_int) as uint8_t;
    *s.offset(23 as core::ffi::c_int as isize) =
        (s8 >> 16 as core::ffi::c_int | s9 << 5 as core::ffi::c_int) as uint8_t;
    *s.offset(24 as core::ffi::c_int as isize) = (s9 >> 3 as core::ffi::c_int) as uint8_t;
    *s.offset(25 as core::ffi::c_int as isize) = (s9 >> 11 as core::ffi::c_int) as uint8_t;
    *s.offset(26 as core::ffi::c_int as isize) =
        (s9 >> 19 as core::ffi::c_int | s10 << 2 as core::ffi::c_int) as uint8_t;
    *s.offset(27 as core::ffi::c_int as isize) = (s10 >> 6 as core::ffi::c_int) as uint8_t;
    *s.offset(28 as core::ffi::c_int as isize) =
        (s10 >> 14 as core::ffi::c_int | s11 << 7 as core::ffi::c_int) as uint8_t;
    *s.offset(29 as core::ffi::c_int as isize) = (s11 >> 1 as core::ffi::c_int) as uint8_t;
    *s.offset(30 as core::ffi::c_int as isize) = (s11 >> 9 as core::ffi::c_int) as uint8_t;
    *s.offset(31 as core::ffi::c_int as isize) = (s11 >> 17 as core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_scalar_mult_generic_masked(
    out: *mut uint8_t,
    scalar_masked: *const uint8_t,
    point: *const uint8_t,
) {
    let mut x1: fe = fe { v: [0; 10] };
    let mut x2: fe = fe { v: [0; 10] };
    let mut z2: fe = fe { v: [0; 10] };
    let mut x3: fe = fe { v: [0; 10] };
    let mut z3: fe = fe { v: [0; 10] };
    let mut tmp0: fe = fe { v: [0; 10] };
    let mut tmp1: fe = fe { v: [0; 10] };
    let mut x2l: fe_loose = fe_loose { v: [0; 10] };
    let mut z2l: fe_loose = fe_loose { v: [0; 10] };
    let mut x3l: fe_loose = fe_loose { v: [0; 10] };
    let mut tmp0l: fe_loose = fe_loose { v: [0; 10] };
    let mut tmp1l: fe_loose = fe_loose { v: [0; 10] };
    let mut e: [uint8_t; 32] = [0; 32];
    let _ = GFp_memcpy(
        e.as_mut_ptr() as *mut core::ffi::c_void,
        scalar_masked as *const core::ffi::c_void,
        32 as core::ffi::c_int as size_t,
    );
    fe_frombytes(&mut x1, point);
    fe_1(&mut x2);
    fe_0(&mut z2);
    fe_copy(&mut x3, &mut x1);
    fe_1(&mut z3);
    let mut swap: core::ffi::c_uint = 0 as core::ffi::c_int as core::ffi::c_uint;
    let mut pos: core::ffi::c_int;
    pos = 254 as core::ffi::c_int;
    while pos >= 0 as core::ffi::c_int {
        let b: core::ffi::c_uint = (1 as core::ffi::c_int
            & e[(pos / 8 as core::ffi::c_int) as usize] as core::ffi::c_int
                >> (pos & 7 as core::ffi::c_int))
            as core::ffi::c_uint;
        swap ^= b;
        fe_cswap(&mut x2, &mut x3, swap);
        fe_cswap(&mut z2, &mut z3, swap);
        swap = b;
        fe_sub(&mut tmp0l, &mut x3, &mut z3);
        fe_sub(&mut tmp1l, &mut x2, &mut z2);
        fe_add(&mut x2l, &mut x2, &mut z2);
        fe_add(&mut z2l, &mut x3, &mut z3);
        fe_mul_tll(&mut z3, &mut tmp0l, &mut x2l);
        fe_mul_tll(&mut z2, &mut z2l, &mut tmp1l);
        fe_sq_tl(&mut tmp0, &mut tmp1l);
        fe_sq_tl(&mut tmp1, &mut x2l);
        fe_add(&mut x3l, &mut z3, &mut z2);
        fe_sub(&mut z2l, &mut z3, &mut z2);
        fe_mul_ttt(&mut x2, &mut tmp1, &mut tmp0);
        fe_sub(&mut tmp1l, &mut tmp1, &mut tmp0);
        fe_sq_tl(&mut z2, &mut z2l);
        fe_mul121666(&mut z3, &mut tmp1l);
        fe_sq_tl(&mut x3, &mut x3l);
        fe_add(&mut tmp0l, &mut tmp0, &mut z3);
        fe_mul_ttt(&mut z3, &mut x1, &mut z2);
        fe_mul_tll(&mut z2, &mut tmp1l, &mut tmp0l);
        pos -= 1;
    }
    fe_cswap(&mut x2, &mut x3, swap);
    fe_cswap(&mut z2, &mut z3, swap);
    fe_invert(&mut z2, &mut z2);
    fe_mul_ttt(&mut x2, &mut x2, &mut z2);
    fe_tobytes(out, &mut x2);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_public_from_private_generic_masked(
    out_public_value: *mut uint8_t,
    private_key_masked: *const uint8_t,
) {
    let mut e: [uint8_t; 32] = [0; 32];
    let _ = GFp_memcpy(
        e.as_mut_ptr() as *mut core::ffi::c_void,
        private_key_masked as *const core::ffi::c_void,
        32 as core::ffi::c_int as size_t,
    );
    let mut A: ge_p3 = ge_p3 {
        X: fe { v: [0; 10] },
        Y: fe { v: [0; 10] },
        Z: fe { v: [0; 10] },
        T: fe { v: [0; 10] },
    };
    GFp_x25519_ge_scalarmult_base(&mut A, e.as_mut_ptr());
    let mut zplusy: fe_loose = fe_loose { v: [0; 10] };
    let mut zminusy: fe_loose = fe_loose { v: [0; 10] };
    let mut zminusy_inv: fe = fe { v: [0; 10] };
    fe_add(&mut zplusy, &mut A.Z, &mut A.Y);
    fe_sub(&mut zminusy, &mut A.Z, &mut A.Y);
    fe_loose_invert(&mut zminusy_inv, &mut zminusy);
    fe_mul_tlt(&mut zminusy_inv, &mut zplusy, &mut zminusy_inv);
    fe_tobytes(out_public_value, &mut zminusy_inv);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_fe_invert(out: *mut fe, z: *const fe) {
    fe_invert(out, z);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_fe_isnegative(f: *const fe) -> uint8_t {
    return fe_isnegative(f) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_fe_mul_ttt(h: *mut fe, f: *const fe, g: *const fe) {
    fe_mul_ttt(h, f, g);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_fe_neg(f: *mut fe) {
    let mut t: fe_loose = fe_loose { v: [0; 10] };
    fe_neg(&mut t, f);
    fe_carry(f, &mut t);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_fe_tobytes(s: *mut uint8_t, h: *const fe) {
    fe_tobytes(s, h);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_ge_double_scalarmult_vartime(
    r: *mut ge_p2,
    a: *const uint8_t,
    A: *const ge_p3,
    b: *const uint8_t,
) {
    ge_double_scalarmult_vartime(r, a, A, b);
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_sc_mask(a: *mut uint8_t) {
    let ref mut fresh2 = *a.offset(0 as core::ffi::c_int as isize);
    *fresh2 = (*fresh2 as core::ffi::c_int & 248 as core::ffi::c_int) as uint8_t;
    let ref mut fresh3 = *a.offset(31 as core::ffi::c_int as isize);
    *fresh3 = (*fresh3 as core::ffi::c_int & 127 as core::ffi::c_int) as uint8_t;
    let ref mut fresh4 = *a.offset(31 as core::ffi::c_int as isize);
    *fresh4 = (*fresh4 as core::ffi::c_int | 64 as core::ffi::c_int) as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_x25519_sc_muladd(
    s: *mut uint8_t,
    a: *const uint8_t,
    b: *const uint8_t,
    c: *const uint8_t,
) {
    sc_muladd(s, a, b, c);
}
