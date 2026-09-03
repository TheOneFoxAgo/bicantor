use num_bigint::BigUint;

pub fn decode(code: BigUint) -> (BigUint, BigUint) {
    let d = ((BigUint::ONE + BigUint::new_const(8) * &code).sqrt() - BigUint::ONE) >> 1;
    let y = code - ((&d * (&d + BigUint::ONE)) >> 1);
    let x = d - &y;
    (x, y)
}
pub fn encode(x: BigUint, y: BigUint) -> BigUint {
    let d = x + &y;
    let d_p1 = &d + BigUint::ONE;
    ((d * d_p1) >> 1) + y
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn decode_encode() {
        for i in [0, 1, 4, 32, 12] {
            let number = BigUint::new_const(i);
            let (x, y) = decode(number.clone());
            let new_number = encode(x, y);
            assert_eq!(number, new_number);
        }
    }

    #[test]
    fn encode_decode() {
        for (x, y) in [(0, 0), (3, 2), (10, 0), (0, 10), (5, 12)] {
            let (x, y) = (BigUint::new_const(x), BigUint::new_const(y));
            let number = encode(x.clone(), y.clone());
            let (new_x, new_y) = decode(number.clone());
            assert_eq!(x, new_x);
            assert_eq!(y, new_y);
        }
    }
}
