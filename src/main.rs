// comment
// Base 64 encoding docs
// https://datatracker.ietf.org/doc/html/rfc4648#section-4
//
//         0 A            17 R            34 i            51 z
//         1 B            18 S            35 j            52 0
//         2 C            19 T            36 k            53 1
//         3 D            20 U            37 l            54 2
//         4 E            21 V            38 m            55 3
//         5 F            22 W            39 n            56 4
//         6 G            23 X            40 o            57 5
//         7 H            24 Y            41 p            58 6
//         8 I            25 Z            42 q            59 7
//         9 J            26 a            43 r            60 8
//        10 K            27 b            44 s            61 9
//        11 L            28 c            45 t            62 +
//        12 M            29 d            46 u            63 /
//        13 N            30 e            47 v
//        14 O            31 f            48 w         (pad) =
//        15 P            32 g            49 x
//        16 Q            33 h            50 y
//

fn mapping(key: u8) -> Option<char> {
    let uppercase_base = 0x41; // 'A'
    let lowercase_base = 0x61; // 'a'
    let digits_base = 0x30; // '0'
    match key {
        0..=25 => char::from_u32((uppercase_base + key).into()),
        26..=51 => char::from_u32((lowercase_base + key - 26).into()),
        52..=61 => char::from_u32((digits_base + key - 52).into()),
        62 => Some('+'),
        63 => Some('/'),
        _ => None,
    }
}

fn process_triplet(bytes: &[u8]) -> String {
    assert!(bytes.len() > 0);
    assert!(bytes.len() <= 3);
    let mut padded_bytes = [0, 0, 0, 0];
    for i in 0..bytes.len() {
        padded_bytes[i] = bytes[i];
    }

    let h0 = (padded_bytes[0] >> 2) & 0x3f;
    let h1 = ((padded_bytes[0] & 0x3) << 4) | ((padded_bytes[1] >> 4) & 0xf);
    let h2 = ((padded_bytes[1] & 0xf) << 2) | ((padded_bytes[2] >> 6) & 0x3);
    let h3 = padded_bytes[2] & 0x3f;

    let h0: char = mapping(h0).unwrap();
    let h1: char = mapping(h1).unwrap();
    let h2 = match bytes.len() {
        1 => '=',
        2..=3 => mapping(h2).unwrap(),
        _ => panic!("how can the bytes len be greater than 3"),
    };
    let h3 = match bytes.len() {
        1..=2 => '=',
        3 => mapping(h3).unwrap(),
        _ => panic!("how can the bytes len be greater than 3"),
    };
    let s: String = [h0, h1, h2, h3].iter().collect();
    s
}

fn hex_to_b64(bytes: &[u8]) -> String {
    let mut b64 = String::new();
    // take triplets of bytes and chunk into 4 pieces of 6bits.
    // then index into mapping and append char to string
    // handle the edgecases for last three
    for chunk in bytes.chunks(3) {
        b64 += &process_triplet(chunk);
    }
    b64
}

fn hex_xor(bytes1: &[u8], bytes2: &[u8]) -> Vec<u8> {
    let mut v = vec![];
    assert_eq!(bytes1.len(), bytes2.len());
    for i in 0..bytes1.len() {
        v.push(bytes1[i] ^ bytes2[i]);
    }
    v
}

fn main() {
    println!("Hello, world!");

    let c = char::from_u32(0x41).unwrap();

    println!("{}", c);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_success_mapping() {
        assert_eq!(mapping(0), Some('A'));
        assert_eq!(mapping(26), Some('a'));
        assert_eq!(mapping(52), Some('0'));
        assert_eq!(mapping(62), Some('+'));
        assert_eq!(mapping(63), Some('/'));
        assert_eq!(mapping(64), None);
    }

    #[test]
    fn test_success_set_1_challenge_1() {
        let input: &[u8] = &[
            0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f,
            0x75, 0x72, 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20,
            0x61, 0x20, 0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75,
            0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d,
        ];
        let output: String =
            String::from("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

        assert_eq!(hex_to_b64(input), output);
    }

    #[test]
    fn test_success_hex_to_b64() {
        let input: &[u8] = &[0x41];
        let output: String = String::from("QQ==");
        assert_eq!(hex_to_b64(input), output);

        let input: &[u8] = &[0x41, 0x41];
        let output: String = String::from("QUE=");
        assert_eq!(hex_to_b64(input), output);

        let input: &[u8] = &[0x41, 0x41, 0x41];
        let output: String = String::from("QUFB");
        assert_eq!(hex_to_b64(input), output);
    }

    #[test]
    fn test_hex_xor() {
        let i1: &[u8] = &[0x0];
        let i2: &[u8] = &[0x1];
        assert_eq!( hex_xor(i1,i2), &[0x1]);

        let i1: &[u8] = &[0x1];
        let i2: &[u8] = &[0x1];
        assert_eq!( hex_xor(i1,i2), &[0x0]);

        let i1: &[u8] = &[0x3];
        let i2: &[u8] = &[0xf];
        assert_eq!( hex_xor(i1,i2), &[0xc]);
     }

    #[test]
    fn test_success_set_1_challenge_2() {
        let i1: &[u8] = &[0x1c,0x01,0x11,0x00,0x1f,0x01,0x01,0x00,0x06,0x1a,0x02,0x4b,0x53,0x53,0x50,0x09,0x18,0x1c];
        let i2: &[u8] = &[0x68,0x69,0x74,0x20,0x74,0x68,0x65,0x20,0x62,0x75,0x6c,0x6c,0x27,0x73,0x20,0x65,0x79,0x65];
        let o: &[u8] = &[0x74,0x68,0x65,0x20,0x6b,0x69,0x64,0x20,0x64,0x6f,0x6e,0x27,0x74,0x20,0x70,0x6c,0x61,0x79];

        assert_eq!(hex_xor(i1,i2), o);
    }
}
