// SPDX-License-Identifier: Apache-2.0

use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct U16 {
    data: u16,
}

#[derive(Debug)]
struct U8 {
    data: u8,
}

impl Into<U16> for U8 {
    fn into(self) -> U16 {
        U16 {
            data: self.data.into(),
        }
    }
}

println!("{}", abc);
write!(fd, "{}", ... );

impl U8 {
    pub fn to_u16(&self) -> U16 {
        U16 {
            data: self.data.into(),
        }
    }
}


fn main() {
    let a = U8 { data: 8 };
    let b: U16 = a.into();
    let c: U16 = a.to_u16();
    println!("HAHA {:?}", b);
}
