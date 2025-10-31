pub fn string_to_hex(s: String) -> Vec<u8> {
    assert!(s.len() & 1 == 0);

    (0..(&s).len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

pub fn hex_to_string(bytes: &[u8]) -> String {
    let s: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    assert!(s.len() & 1 == 0);
    s
}

pub fn hex_xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    let mut v = vec![];
    assert_eq!(bytes1.len(), bytes2.len());
    for i in 0..bytes1.len() {
        v.push(bytes1[i] ^ bytes2[i]);
    }
    v
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_hex_xor() {
        let i1: &[u8] = &[0x0];
        let i2: &[u8] = &[0x1];
        assert_eq!(hex_xor(i1, i2), &[0x1]);

        let i1: &[u8] = &[0x1];
        let i2: &[u8] = &[0x1];
        assert_eq!(hex_xor(i1, i2), &[0x0]);

        let i1: &[u8] = &[0x3];
        let i2: &[u8] = &[0xf];
        assert_eq!(hex_xor(i1, i2), &[0xc]);
    }

    #[test]
    #[should_panic]
    fn test_failure_string_to_hex() {
        let i = String::from("41424");
        string_to_hex(i);
    }

    #[test]
    fn test_success_string_to_hex() {
        let i = String::from("41");
        let e = &[0x41];
        assert_eq!(string_to_hex(i), e);

        let i = String::from("4142");
        let e = &[0x41, 0x42];
        assert_eq!(string_to_hex(i), e);
    }

    #[test]
    fn test_success_hex_to_string() {
        let i = &[0x41];
        let e = String::from("41");
        assert_eq!(hex_to_string(i), e);

        let i = &[0x41, 0x42];
        let e = String::from("4142");
        assert_eq!(hex_to_string(i), e);
    }
}
