<!--
SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Nicolas Qiu Guichard <nicolas.guichard@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# cxx-enum-gaps

[![GitHub](https://img.shields.io/badge/github-KDABLabs%2Fcxx--enum--gaps-informational?logo=github)](https://github.com/KDABLabs/cxx-enum-gaps)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/kdablabs/cxx-enum-gaps/nix-flake-check.yml)
![License (MIT/Apache2.0)](https://img.shields.io/crates/l/cxx-qt)
[![REUSE status](https://api.reuse.software/badge/github.com/KDABLabs/cxx-enum-gaps)](https://api.reuse.software/info/github.com/KDABLabs/cxx-enum-gaps)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

The upstream cxx docs state about shared enums:
> Pattern matching with match still works but will require you to write wildcard arms to handle the situation of an enum value that is not one of the listed variants.

This is unfortunate as there is no way to check that all listed enum variants are actually handled. IOW this code does not even warn:

```rust
#[cxx::bridge]
mod ffi {
    enum CxxEnum {
        A,
        B,
        C = 5,
    }
}

use ffi::CxxEnum;

match t {
    CxxEnum::A => {},
    CxxEnum::B => {},
    _ => {},
}
```

This generates an extra _CxxEnum! macro that matches all the ranges that are not listed by the enum. When using that macro instead of the `_` catch-all, we get compile-time errors if we forget to match variants:

```rust
#[cxx_enum_gaps]
#[cxx::bridge]
mod ffi {
    enum CxxEnum {
        A,
        B,
        C = 5,
    }
}

use ffi::CxxEnum;

match t {
    CxxEnum::A => {},
    CxxEnum::B => {},
    _CxxEnum!() => {},
}
// Error Enum{ repr: 5 } case not covered
```

The generated macro looks like:

```rust
macro_rules! _CxxEnum {
    () => { CxxEnum{ repr: 4..5 | 6.. } };
    ($i:ident) => { CxxEnum { repr: $i @ (4..5 | 6..) } };
}
```

It can also be used to ergonomically retrieve the inner value in the unlisted case:

```rust
match t {
    CxxEnum::A => {},
    CxxEnum::B => {},
    CxxEnum::C => {},
    _CxxEnum!(value) => panic!("Unexpected CxxEnum value {value}"),
}
```

This was originally proposed upstream at https://github.com/dtolnay/cxx/pull/1376.

## Licensing

cxx-enum-gaps is Copyright (C) Klarälvdalens Datakonsult AB, and is available under
the terms of the [MIT](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/MIT.txt)
or the [Apache-2.0](https://github.com/KDAB/cxx-qt/blob/main/LICENSES/Apache-2.0.txt)
licenses.

The `src/syntax` subdirectory is copied from cxx and Copyright (C) David Tolnay.

Contact KDAB at <info@kdab.com> to inquire about additional features or
services related to this project.

## About KDAB

The KDAB Group is the global No.1 software consultancy for Qt, C++ and
OpenGL applications across desktop, embedded and mobile platforms.

The KDAB Group provides consulting and mentoring for developing Qt applications
from scratch and in porting from all popular and legacy frameworks to Qt.
We continue to help develop parts of Qt and are one of the major contributors
to the Qt Project. We can give advanced or standard trainings anywhere
around the globe on Qt as well as C++, OpenGL, 3D and more.

Please visit <https://www.kdab.com> to meet the people who write code like this.
