use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::Rng;

#[allow(dead_code)] // avoids warnings when not used in both server and client
pub fn solve(x: &BigUint, k: &BigUint, c: &BigUint, q: &BigUint) -> BigUint {
    // s = (k - c * x) mod q
    if k >= &(c * x) {
        (k - c * x).modpow(&BigUint::one(), q)
    } else {
        q - (c * x - k).modpow(&BigUint::one(), q)
    }
}

pub struct VerifyParams<'a> {
    pub g: &'a BigUint,
    pub h: &'a BigUint,
    pub p: &'a BigUint,
    pub y1: &'a BigUint,
    pub y2: &'a BigUint,
    pub r1: &'a BigUint,
    pub r2: &'a BigUint,
    pub c: &'a BigUint,
    pub s: &'a BigUint,
}

#[allow(dead_code)] // avoids warnings when not used in both server and client
pub fn verify(params: VerifyParams) -> bool {
    // R1 = g^s * Y1^c
    let eq1 = *params.r1
        == (params.g.modpow(params.s, params.p) * params.y1.modpow(params.c, params.p))
            .modpow(&BigUint::one(), params.p);
    // R2 = h^s * Y2^c
    let eq2 = *params.r2
        == (params.h.modpow(params.s, params.p) * params.y2.modpow(params.c, params.p))
            .modpow(&BigUint::one(), params.p);

    eq1 && eq2
}

pub fn random_number() -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint(256)
}

#[allow(dead_code)] // avoids warnings when not used in both server and client
pub fn random_string(number: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(number)
        .map(char::from)
        .collect()
}

pub fn serialize(number: &BigUint) -> Vec<u8> {
    number.to_bytes_be()
}

pub fn deserialize(bytes: &[u8]) -> BigUint {
    BigUint::from_bytes_be(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_success_01() {
        assert_eq!(
            solve(
                &BigUint::from(7u32),
                &BigUint::from(24u32),
                &BigUint::from(2u32),
                &BigUint::from(9u32)
            ),
            BigUint::from(1u32)
        );
    }

    #[test]
    fn test_solve_success_02() {
        assert_eq!(
            solve(
                &BigUint::from(11u32),
                &BigUint::from(5u32),
                &BigUint::from(4u32),
                &BigUint::from(10u32)
            ),
            BigUint::from(1u32)
        );
    }

    #[test]
    fn test_solve_failure() {
        assert_ne!(
            solve(
                &BigUint::from(11u32),
                &BigUint::from(5u32),
                &BigUint::from(4u32),
                &BigUint::from(10u32)
            ),
            BigUint::from(10u32)
        );
    }

    #[test]
    fn test_auth_steps_success() {
        let g = &BigUint::from(4u32);
        let h = &BigUint::from(9u32);
        let q = &BigUint::from(11u32);
        let p = &BigUint::from(23u32);
        let x = &BigUint::from(6u32);

        // registration step
        let y1 = g.modpow(x, p);
        let y2 = h.modpow(x, p);

        // Generate a random number for the challenge
        let k = random_number();

        let r1 = g.modpow(&k, p);
        let r2 = h.modpow(&k, p);

        // Solve for the response using the challenge and random number
        let c = random_number();

        let s = solve(x, &k, &c, q);

        let params = VerifyParams {
            g,
            h,
            p,
            y1: &y1,
            y2: &y2,
            r1: &r1,
            r2: &r2,
            c: &c,
            s: &s,
        };
        assert!(verify(params));

        // Verify with an incorrect response to ensure validation fails
        let incorrect_params = VerifyParams {
            g,
            h,
            p,
            y1: &y1,
            y2: &y2,
            r1: &r1,
            r2: &r2,
            c: &c,
            s: &(s + BigUint::one()).modpow(&BigUint::one(), q),
        };

        assert!(!verify(incorrect_params));
    }

    #[test]
    fn test_auth_steps_failure() {
        let g = &BigUint::from(4u32);
        let h = &BigUint::from(9u32);
        let q = &BigUint::from(11u32);
        let p = &BigUint::from(23u32);
        let x = &BigUint::from(6u32);

        // registration step
        let y1 = g.modpow(x, p);
        let y2 = h.modpow(x, p);

        // Generate a random number for the challenge
        let k = random_number();

        let r1 = g.modpow(&k, p);
        let r2 = h.modpow(&k, p);

        // Solve for the response using the challenge and random number
        let c = random_number();

        let s = solve(x, &k, &c, q);

        // Verify with an incorrect response to ensure validation fails
        let incorrect_params = VerifyParams {
            g,
            h,
            p,
            y1: &y1,
            y2: &y2,
            r1: &r1,
            r2: &r2,
            c: &c,
            s: &(s + BigUint::one()).modpow(&BigUint::one(), q),
        };

        assert!(!verify(incorrect_params));
    }

    #[test]
    fn test_verify_success() {
        let g = &BigUint::from(4u32);
        let h = &BigUint::from(9u32);
        let p = &BigUint::from(23u32);
        let y1 = &BigUint::from(2u32);
        let y2 = &BigUint::from(3u32);
        let r1 = &BigUint::from(8u32);
        let r2 = &BigUint::from(4u32);
        let c = &BigUint::from(4u32);
        let s = &BigUint::from(5u32);

        let params = VerifyParams {
            g,
            h,
            p,
            y1,
            y2,
            r1,
            r2,
            c,
            s,
        };

        assert!(verify(params));
    }
    #[test]
    fn test_verify_fails() {
        let g = &BigUint::from(4u32);
        let h = &BigUint::from(9u32);
        let q = &BigUint::from(11u32);
        let p = &BigUint::from(23u32);
        let y1 = &BigUint::from(2u32);
        let y2 = &BigUint::from(3u32);
        let r1 = &BigUint::from(8u32);
        let r2 = &BigUint::from(4u32);
        let c = &BigUint::from(4u32);
        let s = &BigUint::from(5u32);

        // Verify with an incorrect response to ensure validation fails
        let incorrect_params = VerifyParams {
            g,
            h,
            p,
            y1,
            y2,
            r1,
            r2,
            c,
            s: &(s + BigUint::one()).modpow(&BigUint::one(), q),
        };

        assert!(!verify(incorrect_params));
    }
}
