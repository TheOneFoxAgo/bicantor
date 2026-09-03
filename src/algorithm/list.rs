use super::pair;
use num_bigint::BigUint;

pub fn decode(mut code: BigUint) -> impl Iterator<Item = BigUint> {
    let mut numbers = vec![];
    if code != BigUint::ZERO {
        code -= BigUint::ONE;
        let len;
        (len, code) = pair::decode(code);
        for _ in 0..len.try_into().unwrap_or(usize::MAX) {
            let n;
            (n, code) = pair::decode(code);
            numbers.push(n);
        }
        numbers.push(code);
        numbers.reverse();
    }
    numbers.into_iter()
}
pub fn encode(mut iter: impl Iterator<Item = BigUint>) -> BigUint {
    let Some(acc) = iter.next() else {
        return BigUint::ZERO;
    };
    let (len, code) = iter.fold((0, acc), |(i, acc), n| (i + 1, pair::encode(n, acc)));
    pair::encode(BigUint::new_const(len), code) + BigUint::ONE
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_empty() {
        assert_eq!(decode(BigUint::ZERO).next(), None)
    }

    #[test]
    fn encode_empty() {
        assert_eq!(encode(std::iter::empty()), BigUint::ZERO)
    }

    #[test]
    fn decode_encode() {
        for number in [1, 0, 2341234, 3257893, 1234, 8] {
            let number = BigUint::new_const(number);
            let new_number = encode(decode(number.clone()));
            assert_eq!(number, new_number);
        }
    }

    #[test]
    fn decode_encode_sanity() {
        let x = encode(decode(BigUint::ZERO));
        let y = encode(decode(BigUint::ONE));
        assert_ne!(x, y);
    }

    #[test]
    fn encode_decode_long() {
        let seq: Vec<BigUint> = [
            12, 234, 523, 1, 3, 0, 0, 1598, 889231, 213, 2134, 9324, 123, 656, 0, 0,
        ]
        .into_iter()
        .map(BigUint::new_const)
        .collect();
        let code = encode(seq.iter().cloned());
        let decoded: Vec<BigUint> = decode(code).collect();
        assert_eq!(seq, decoded);
    }
}
