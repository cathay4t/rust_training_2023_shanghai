// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone)]
struct AbcError {
    kind: ErrorKind,
    msg: String,
}

impl std::fmt::Display for AbcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}:{}", self.kind, self.msg)
    }
}

impl std::error::Error for AbcError {}

#[derive(Debug, Clone, Copy)]
enum ErrorKind {
    NotFound,
}

// Raise error when no odd found
fn count_odd(data: &[u16]) -> Result<usize, AbcError> {
    let count = data.iter().filter(|i| *i % 2 == 1).count();
    if count == 0 {
        Err(AbcError {
            kind: ErrorKind::NotFound,
            msg: "No odd number found".into(),
        })
    } else {
        Ok(count)
    }
}

fn main() {
    let a = vec![1u16, 2, 3, 4];

    let count = count_odd(&a)?;

    println!("Got {:?}", count);
}
