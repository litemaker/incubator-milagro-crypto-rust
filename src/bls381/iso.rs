/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

use super::super::big::Big;
use super::super::ecp2::ECP2;
use super::super::fp::FP;
use super::super::fp2::FP2;
use super::sqrt_division_chain::sqrt_division_chain;

/**************************************************
* 3-Isogeny Constants
**************************************************/
pub const ISO3_B2_I: isize = 1012;
lazy_static! {
    // Curve parameters of ISO-3 y^2 = x^3 + ax + b
    pub static ref ISO3_A2: FP2 = FP2::new_ints(0, 240);
    pub static ref ISO3_B2: FP2 = FP2::new_ints(1012, 1012);
    pub static ref ISO3_E2: FP2 = FP2::new_ints(-2, -1);

    // Roots of unity and eta
    // TODO: Convert the following hex::decode() into [i64; NLEN] hence Big::from_ints() can be used
    pub static ref SQRT_I: FP = FP::new_big(&Big::frombytes(&hex::decode("06af0e0437ff400b6831e36d6bd17ffe48395dabc2d3435e77f76e17009241c5ee67992f72ec05f4c81084fbede3cc09").unwrap()));
    pub static ref EV1: FP = FP::new_big(&Big::frombytes(&hex::decode("0699be3b8c6870965e5bf892ad5d2cc7b0e85a117402dfd83b7f4a947e02d978498255a2aaec0ac627b5afbdf1bf1c90").unwrap()));
    pub static ref EV2: FP = FP::new_big(&Big::frombytes(&hex::decode("08157cd83046453f5dd0972b6e3949e4288020b5b8a9cc99ca07e27089a2ce2436d965026adad3ef7baba37f2183e9b5").unwrap()));
    pub static ref EV3: FP = FP::new_big(&Big::frombytes(&hex::decode("0ab1c2ffdd6c253ca155231eb3e71ba044fd562f6f72bc5bad5ec46a0b7a3b0247cf08ce6c6317f40edbc653a72dee17").unwrap()));
    pub static ref EV4: FP = FP::new_big(&Big::frombytes(&hex::decode("0aa404866706722864480885d68ad0ccac1967c7544b447873cc37e0181271e006df72162a3d3e0287bf597fbf7f8fc1").unwrap()));

    // ISO-3 Mapping values
    pub static ref XNUM: [FP2; 4] = [
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("05c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97d6").unwrap()),
            &Big::frombytes(&hex::decode("05c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97d6").unwrap())
        ),
        FP2::new_bigs(
            &Big::new(),
            &Big::frombytes(&hex::decode("11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71a").unwrap())
        ),
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71e").unwrap()),
            &Big::frombytes(&hex::decode("08ab05f8bdd54cde190937e76bc3e447cc27c3d6fbd7063fcd104635a790520c0a395554e5c6aaaa9354ffffffffe38d").unwrap())
        ),
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("171d6541fa38ccfaed6dea691f5fb614cb14b4e7f4e810aa22d6108f142b85757098e38d0f671c7188e2aaaaaaaa5ed1").unwrap()),
            &Big::new()
        )
    ];
    pub static ref XDEN: [FP2; 4] = [
        FP2::new_bigs(
            &Big::new(),
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa63").unwrap())
        ),
        FP2::new_bigs(
            &Big::new_int(12),
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa9f").unwrap())
        ),
        FP2::new_int(1),
        FP2::new(),
    ];
    pub static ref YNUM: [FP2; 4] = [
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("1530477c7ab4113b59a4c18b076d11930f7da5d4a07f649bf54439d87d27e500fc8c25ebf8c92f6812cfc71c71c6d706").unwrap()),
            &Big::frombytes(&hex::decode("1530477c7ab4113b59a4c18b076d11930f7da5d4a07f649bf54439d87d27e500fc8c25ebf8c92f6812cfc71c71c6d706").unwrap())
        ),
        FP2::new_bigs(
            &Big::new(),
            &Big::frombytes(&hex::decode("05c759507e8e333ebb5b7a9a47d7ed8532c52d39fd3a042a88b58423c50ae15d5c2638e343d9c71c6238aaaaaaaa97be").unwrap())
        ),
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("11560bf17baa99bc32126fced787c88f984f87adf7ae0c7f9a208c6b4f20a4181472aaa9cb8d555526a9ffffffffc71c").unwrap()),
            &Big::frombytes(&hex::decode("08ab05f8bdd54cde190937e76bc3e447cc27c3d6fbd7063fcd104635a790520c0a395554e5c6aaaa9354ffffffffe38f").unwrap())
        ),
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("124c9ad43b6cf79bfbf7043de3811ad0761b0f37a1e26286b0e977c69aa274524e79097a56dc4bd9e1b371c71c718b10").unwrap()),
            &Big::new()
        )
    ];
    pub static ref YDEN: [FP2; 4] = [
        FP2::new_bigs(
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa8fb").unwrap()),
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa8fb").unwrap())
        ),
        FP2::new_bigs(
            &Big::new(),
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffa9d3").unwrap())
        ),
        FP2::new_bigs(
            &Big::new_int(18),
            &Big::frombytes(&hex::decode("1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaa99").unwrap())
        ),
        FP2::new_ints(1, 0)
    ];
}

/// 3-Isogeny Curve for Mapping to BLS12-381 extension ECP2
pub struct ISO3_FP2 {
    pub x: FP2,
    pub y: FP2,
    pub z: FP2,
}

impl ISO3_FP2 {
    /// Optimised Shallue-van de Woestijne-Ulas Method
    ///
    /// Adjusted https://eprint.iacr.org/2019/403
    /// such that projectives are (XZ, YZ, Z)
    pub fn swu_optimised(t: FP2) -> ISO3_FP2 {
        let mut t2 = t.clone(); // t
        let is_neg_t = t2.is_neg(); // store for later
        t2.sqr(); // t^2 (store for later)
        let mut et2 = t2.clone(); // et2 = t^2
        et2.mul(&ISO3_E2); // et2 = e * t^2
        let mut common = et2.clone(); // e * t^2
        common.sqr(); // e^2 * t^4
        common.add(&et2); // common = e^2 * t^4 + e * t^2

        // Numerator (x0)
        let mut x_numerator = common.clone();
        x_numerator.add(&FP2::new_ints(1, 0));
        x_numerator.mul(&ISO3_B2); // b * (e^2 * t^4 + e * t^2 + 1)

        // Denominator (x0)
        let mut x_denominator: FP2;
        // Deal with case where e^2 * t^4 + e * t^2 == 0
        if common.iszilch() {
            x_denominator = ISO3_E2.clone();
            x_denominator.mul(&ISO3_A2); // denominator = e * a
        } else {
            x_denominator = common.clone();
            x_denominator.mul(&ISO3_A2);
            x_denominator.neg();
        }

        // u = num^3 + a * num * den^2 + b * den^3
        // v = den^3
        let mut u = x_numerator.clone();
        u.sqr();
        u.mul(&x_numerator); // u = num^3

        let mut tmp1 = x_denominator.clone();
        tmp1.sqr(); // den^2
        let mut tmp2 = x_numerator.clone();
        tmp2.mul(&tmp1); // num * den^2
        tmp2.mul(&ISO3_A2); // a * num * den^2
        u.add(&tmp2); // u = num^3 + a * num * den^2

        tmp1.mul(&x_denominator); // den^3
        let mut v = tmp1.clone(); // den^3
        tmp1.mul(&ISO3_B2); // b * den^3
        u.add(&tmp1); // u = num^3 + a * num * den^2 + b * den^3

        // sqrt_candidate(x0) = sqrt(u / v)
        let (success, mut sqrt_candidate) = sqrt_division_fp2(&u, &v);
        let mut y = sqrt_candidate.clone();

        // Handle case where x0 is not square -> try x1
        sqrt_candidate.mul(&t2);
        sqrt_candidate.mul(&t); // sqrt_candidate(x1) = sqrt_candidate(x0) * t^3
        u.mul(&et2);
        u.mul(&et2);
        u.mul(&et2); // u(x1) = e^3 * t^6 * u(x0)

        let mut etas = etas();
        let mut success_2 = false;
        for (i, eta) in etas.iter_mut().enumerate() {
            tmp1 = sqrt_candidate.clone();
            tmp1.mul(&eta); // eta * sqrt_candidate(x1)

            tmp1.sqr(); // (eta * sqrt_candidate(x1)) ^ 2
            tmp1.mul(&v); // v * (eta * sqrt_candidate(x1)) ^ 2
            tmp1.sub(&u); // v * (eta * sqrt_candidate(x1)) ^ 2 - u`

            if tmp1.iszilch() && !success && !success_2 {
                // Valid solution found
                y = sqrt_candidate;
                y.mul(eta);
                success_2 = true;
            } else if i == 3 && !success && !success_2 {
                // No valid square root found
                panic!("Hash to curve - optimised SWU error");
            }
        }

        if !success {
            // Candidate 2 taken, update numerator
            x_numerator.mul(&et2);
        }

        // negate y if y and t oppose in signs
        if is_neg_t != y.is_neg() {
            y.neg();
        }

        // Projective mapping
        // X = x-num; Y = y * x-den; Z = x-den
        y.mul(&x_denominator);

        ISO3_FP2 {
            x: x_numerator,
            y: y,
            z: x_denominator,
        }
    }

    /// Mapping from 3-Isogeny Curve to BLS12-381 ECP2
    ///
    /// Adjusted from https://eprint.iacr.org/2019/403
    /// to convert projectives to (XZ, YZ, Z)
    pub fn iso3_to_ecp2(&mut self) -> ECP2 {
        let polynomials_coefficients: [&[FP2; 4]; 4] = [&*XNUM, &*XDEN, &*YNUM, &*YDEN];
        let z_vals = z_powers(&self.z);

        // x-num, x-den, y-num, y-den
        let mut mapped_vals: [FP2; 4] = [FP2::new(), FP2::new(), FP2::new(), FP2::new()];

        // Horner caculation for evaluating polynomials
        for (i, polynomial) in polynomials_coefficients[..].iter().enumerate() {
            mapped_vals[i] = polynomial[polynomial.len() - 1].clone();
            for (z_index, value) in polynomial.iter().rev().skip(1).enumerate() {
                // Each value is a specific k for a polynomial
                let mut zk = value.clone();
                zk.mul(&z_vals[z_index]); // k(z_index) * z^(3 - z_index)

                mapped_vals[i].mul(&self.x);
                mapped_vals[i].add(&zk);
            }
        }

        // y-num multiplied by y
        mapped_vals[2].mul(&self.y);
        // y-den multiplied by z
        mapped_vals[3].mul(&self.z);

        let mut z_g2 = mapped_vals[1].clone(); // x-den
        z_g2.mul(&mapped_vals[3]); // x-den * y-den

        let mut x_g2 = mapped_vals[0].clone(); // x-num
        x_g2.mul(&mapped_vals[3]); // x-num * y-den

        let mut y_g2 = mapped_vals[2].clone(); // y-num
        y_g2.mul(&mapped_vals[1]); // y-num * x-den

        ECP2::new_projective(x_g2, y_g2, z_g2)
    }
}

// Returns z, z^2, z^3
fn z_powers(z: &FP2) -> [FP2; 3] {
    let mut two = z.clone();
    two.sqr();

    let mut three = two.clone();
    three.mul(&z);

    [z.clone(), two, three]
}

// Calculate sqrt(u/v) return value and and boolean if square root exists
fn sqrt_division_fp2(u: &FP2, v: &FP2) -> (bool, FP2) {
    // Calculate uv^15
    let mut tmp1 = v.clone(); // v
    let mut tmp2 = v.clone(); // v
    tmp1.sqr(); // v^2
    tmp2.mul(&tmp1); // v^3
    tmp1.sqr(); // v^4
    tmp2.mul(&tmp1); // v^7
    tmp1.sqr(); // v^8
    tmp1.mul(&tmp2); // v^15
    tmp1.mul(&u); // uv^15
    tmp2.mul(&u); // uv^7

    let mut sqrt_candidate = sqrt_division_chain(&tmp1); // (uv^15)^((p - 9) / 16)
    sqrt_candidate.mul(&tmp2); // uv^7 * (uv^15)^((p - 9) / 16)

    // Check against each of the roots of unity
    let mut return_success = false;
    let mut return_value = sqrt_candidate;
    let mut roots = roots_of_unity();
    for root in roots.iter_mut() {
        root.mul(&sqrt_candidate);

        // Check (root * sqrt_candidate)^2 * v - u == 0
        tmp1 = root.clone();
        tmp1.sqr();
        tmp1.mul(&v);
        tmp1.sub(&u);
        if tmp1.iszilch() && !return_success {
            return_success = true;
            return_value = *root;
        }
    }

    (return_success, return_value)
}

// Setup the 4 positive 8th roots of unity
fn roots_of_unity() -> [FP2; 4] {
    let a = FP2::new_ints(1, 0);
    let b = FP2::new_ints(0, 1);
    let c = FP2::new_fps(&SQRT_I, &SQRT_I);
    let mut neg_sqrt_1 = SQRT_I.clone();
    neg_sqrt_1.neg();
    let d = FP2::new_fps(&SQRT_I, &neg_sqrt_1);

    [a, b, c, d]
}

// Setup the four different roots of eta
// For details see https://github.com/algorand/bls_sigs_ref/blob/master/sage-impl/opt_sswu_g2.sage
fn etas() -> [FP2; 4] {
    let a = FP2::new_fps(&EV1, &EV2);
    let mut negative_ev2 = EV2.clone();
    negative_ev2.neg();
    let b = FP2::new_fps(&negative_ev2, &EV1);
    let c = FP2::new_fps(&EV3, &EV4);
    let mut negative_ev4 = EV4.clone();
    negative_ev4.neg();
    let d = FP2::new_fps(&negative_ev4, &EV3);

    [a, b, c, d]
}
