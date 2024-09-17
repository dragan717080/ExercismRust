fn main() {
    let input = &[1048576];
    let output = to_bytes(input);
    let expected = vec![0xc0, 0x80, 0x0];
    assert_eq!(output, expected);
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut res = vec![];

    for value in values {
        res.append(&mut to_bytes_single(*value));
    }
    res
}

fn to_bytes_single(mut value: u32) -> Vec<u8> {
    let mut res = Vec::with_capacity(4);

    if value == 0 {
        return vec![0];
    }

    while value > 0 {
        let mut tmp = (value & 0x7f) as u8;
        value >>= 7;

        if !res.is_empty() {
            tmp |= 0x80;
        }

        res.push(tmp);
    }

    res.reverse();
    res
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut res = vec![];
    let mut tmp = 0;
    for (i, b) in bytes.iter().enumerate() {
        if (tmp & 0xfe_00_00_00) > 0 {
            return Err(Error::Overflow);
        }

        tmp = (tmp << 7) | u32::from(b & 0x7f);

        if 0x80 & b == 0 {
            res.push(tmp);
            tmp = 0;
        } else {
            if i + 1 == bytes.len() {
                return Err(Error::IncompleteNumber);
            }
        }
    }

    Ok(res)
}
