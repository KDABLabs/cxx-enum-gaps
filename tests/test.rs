// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Nicolas Qiu Guichard <nicolas.guichard@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_enum_gaps::cxx_enum_gaps;

#[cxx_enum_gaps]
#[cxx::bridge]
mod ffi {
    enum CxxEnum {
        A,
        B,
        C = 10,
        D = 5,
    }
}

use ffi::CxxEnum;

fn main() {
    let a = CxxEnum::A;
    match a {
        CxxEnum::A => {}
        CxxEnum::B => {
            panic!()
        }
        CxxEnum::C => {
            panic!()
        }
        CxxEnum::D => {
            panic!()
        }
        _CxxEnum!() => {
            panic!()
        }
    }

    let b = CxxEnum::B;
    match b {
        CxxEnum::A => {
            panic!()
        }
        CxxEnum::B => {}
        CxxEnum::C => {
            panic!()
        }
        CxxEnum::D => {
            panic!()
        }
        _CxxEnum!() => {
            panic!()
        }
    }

    let c = CxxEnum::C;
    match c {
        CxxEnum::A => {
            panic!()
        }
        CxxEnum::B => {
            panic!()
        }
        CxxEnum::C => {}
        CxxEnum::D => {
            panic!()
        }
        _CxxEnum!() => {
            panic!()
        }
    }

    let d = CxxEnum::D;
    match d {
        CxxEnum::A => {
            panic!()
        }
        CxxEnum::B => {
            panic!()
        }
        CxxEnum::C => {
            panic!()
        }
        CxxEnum::D => {}
        _CxxEnum!() => {
            panic!()
        }
    }

    let e = CxxEnum { repr: 6 };
    match e {
        CxxEnum::A => {
            panic!()
        }
        CxxEnum::B => {
            panic!()
        }
        CxxEnum::C => {
            panic!()
        }
        CxxEnum::D => {
            panic!()
        }
        _CxxEnum!() => {}
    }

    let f = CxxEnum { repr: 7 };
    match f {
        CxxEnum::A => {
            panic!()
        }
        CxxEnum::B => {
            panic!()
        }
        CxxEnum::C => {
            panic!()
        }
        CxxEnum::D => {
            panic!()
        }
        _CxxEnum!(repr) => {
            assert_eq!(repr, 7)
        }
    }
}
