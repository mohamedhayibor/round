# round ![](https://travis-ci.org/mohamedhayibor/round.svg?branch=master)

This crate provides utilities to round your floats with precision from 1 to 10.

## Usage

In your project's Cargo.toml include the package name and version, like so:
```rust
[dependencies]
round = "0.1.0" // or any latest version
```

#### example
```rust
extern crate round;
use round::round;

fn main() {
    let test_n = round(8.9534, 2);  // 8.95
    let test_x = round(8.9536, 3);  // 8.954
    let test_y = round(8.9536, -1); // 8.95
}
```

> defaults to 2 decimal rounding if your rounding (i32) is negative or greater than 10.

## Raison d'etre

Turns out I am in need of this type of operation almost every day. Hopefully it will save you some time as well.

## License

This library is distributed with the GPLv2 software license.

```
    round (rust library - crate)
    Copyright (C) 2016 - Mohamed Hayibor

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
```
