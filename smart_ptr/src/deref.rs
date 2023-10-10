// SPDX-License-Identifier: Apache-2.0

use std::fmt::Write;

#[derive(Debug)]
struct MySmartPointer {
    inner: String,
}

impl MySmartPointer {
    fn new(inner: String) -> Self {
        Self { inner }
    }
}

impl std::ops::Deref for MySmartPointer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for MySmartPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn main() {
    let mut foo = MySmartPointer::new("ABC".into());
    println!("{}", foo.as_str());
    write!(foo, "D");
    println!("{}", foo.as_str());
}
