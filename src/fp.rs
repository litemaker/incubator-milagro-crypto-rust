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

use super::super::arch;
use super::super::arch::Chunk;
use super::big;
use super::big::Big;
use super::dbig::DBig;
use super::rom;
use std::str::FromStr;
use types::ModType;

#[derive(Copy, Clone)]
pub struct FP {
    pub x: Big,
    pub xes: i32,
}

impl PartialEq for FP {
    fn eq(&self, other: &FP) -> bool {
        self.equals(other)
    }
}

impl fmt::Display for FP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP: [ {} ]", self.x)
    }
}

impl fmt::Debug for FP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FP: [ {} ]", self.x)
    }
}

pub use super::rom::{MOD8, MODBITS, MODTYPE, SH};
use std::fmt;
use std::str::SplitWhitespace;

pub const FEXCESS: i32 = (((1 as i32) << SH) - 1);
pub const OMASK: Chunk = (-1) << (MODBITS % big::BASEBITS);
pub const TBITS: usize = MODBITS % big::BASEBITS; // Number of active bits in top word
pub const TMASK: Chunk = (1 << TBITS) - 1;

impl FP {
    // Constructors
    pub fn new() -> FP {
        FP {
            x: Big::new(),
            xes: 1,
        }
    }

    pub fn new_int(a: isize) -> FP {
        let mut f = FP::new();
        f.x.inc(a);
        f.nres();
        return f;
    }

    pub fn new_copy(y: &FP) -> FP {
        let mut f = FP::new();
        f.x.copy(&(y.x));
        f.xes = y.xes;
        return f;
    }

    pub fn new_big(y: &Big) -> FP {
        let mut f = FP::new();
        f.x.copy(y);
        f.nres();
        return f;
    }

    pub fn nres(&mut self) {
        if MODTYPE != ModType::PseudoMersenne && MODTYPE != ModType::GeneralisedMersenne {
            let r = Big::new_ints(&rom::R2MODP);
            let mut d = Big::mul(&(self.x), &r);
            self.x.copy(&FP::modulo(&mut d));
            self.xes = 2;
        } else {
            self.xes = 1;
        }
    }

    pub fn from_hex_iter(iter: &mut SplitWhitespace) -> FP {
        let xes = i32::from_str(iter.next().unwrap()).unwrap();
        let x = iter.next().unwrap();
        FP {
            x: Big::fromstring(x.to_string()),
            xes,
        }
    }

    pub fn from_hex(val: String) -> FP {
        let mut s = val.split_whitespace();
        FP::from_hex_iter(&mut s)
    }

    pub fn to_hex(&self) -> String {
        let x = self.x;
        let big = x.tostring();
        format!("{} {}", self.xes, big)
    }

    /// convert back to regular form
    pub fn redc(&mut self) -> Big {
        if MODTYPE != ModType::PseudoMersenne && MODTYPE != ModType::GeneralisedMersenne {
            let mut d = DBig::new_scopy(&(self.x));
            return FP::modulo(&mut d);
        }
        Big::new_copy(&(self.x))
    }

    /// reduce a DBig to a Big using the appropriate form of the modulus
    pub fn modulo(d: &mut DBig) -> Big {
        if MODTYPE == ModType::PseudoMersenne {
            let mut b = Big::new();
            let mut t = d.split(MODBITS);
            b.dcopy(&d);
            let v = t.pmul(rom::MCONST as isize);

            t.add(&b);
            t.norm();

            let tw = t.w[big::NLEN - 1];
            t.w[big::NLEN - 1] &= TMASK;
            t.w[0] += rom::MCONST * ((tw >> TBITS) + (v << (big::BASEBITS - TBITS)));
            t.norm();
            return t;
        }

        if MODTYPE == ModType::MontgomeryFriendly {
            let mut b = Big::new();
            for i in 0..big::NLEN {
                let x = d.w[i];

                let tuple = Big::muladd(x, rom::MCONST - 1, x, d.w[big::NLEN + i - 1]);
                d.w[big::NLEN + i] += tuple.0;
                d.w[big::NLEN + i - 1] = tuple.1;
            }

            b.zero();

            for i in 0..big::NLEN {
                b.w[i] = d.w[big::NLEN + i];
            }
            b.norm();
            return b;
        }

        if MODTYPE == ModType::GeneralisedMersenne {
            // GoldiLocks Only
            let mut b = Big::new();
            let t = d.split(MODBITS);
            let rm2 = (MODBITS / 2) as usize;
            b.dcopy(&d);
            b.add(&t);
            let mut dd = DBig::new_scopy(&t);
            dd.shl(rm2);

            let mut tt = dd.split(MODBITS);
            let lo = Big::new_dcopy(&dd);
            b.add(&tt);
            b.add(&lo);
            b.norm();
            tt.shl(rm2);
            b.add(&tt);

            let carry = b.w[big::NLEN - 1] >> TBITS;
            b.w[big::NLEN - 1] &= TMASK;
            b.w[0] += carry;

            b.w[(224 / big::BASEBITS) as usize] += carry << (224 % big::BASEBITS);
            b.norm();
            return b;
        }
        if MODTYPE == ModType::NotSpecial {
            let m = Big::new_ints(&rom::MODULUS);
            return Big::monty(&m, rom::MCONST, d);
        }
        Big::new()
    }

    /// convert to string
    pub fn tostring(&mut self) -> String {
        self.redc().tostring()
    }

    /// reduce this mod Modulus
    pub fn reduce(&mut self) {
        let mut m = Big::new_ints(&rom::MODULUS);
        let mut r = Big::new_copy(&m);
        let mut sb: usize;
        self.x.norm();
        if self.xes > 16 {
            let q = FP::quo(&self.x, &m);
            let carry = r.pmul(q);
            r.w[big::NLEN - 1] += carry << big::BASEBITS; // correction - put any carry out back in again
            self.x.sub(&r);
            self.x.norm();
            sb = 2;
        } else {
            sb = FP::logb2((self.xes - 1) as u32);
        }
        m.fshl(sb);

        while sb > 0 {
            let sr = Big::ssn(&mut r, &self.x, &mut m);
            self.x.cmove(&r, 1 - sr);
            sb -= 1;
        }

        self.xes = 1;
    }

    /// test this=0?
    pub fn iszilch(&self) -> bool {
        let mut a = FP::new_copy(self);
        a.reduce();
        a.x.iszilch()
    }

    /// copy from FP b
    pub fn copy(&mut self, b: &FP) {
        self.x.copy(&(b.x));
        self.xes = b.xes;
    }

    /// copy from Big b
    pub fn bcopy(&mut self, b: &Big) {
        self.x.copy(&b);
        self.nres();
    }

    /// set this=0
    pub fn zero(&mut self) {
        self.x.zero();
        self.xes = 1;
    }

    /// set this=1
    pub fn one(&mut self) {
        self.x.one();
        self.nres()
    }

    /// normalise this
    pub fn norm(&mut self) {
        self.x.norm();
    }

    /// swap FPs depending on d
    pub fn cswap(&mut self, b: &mut FP, d: isize) {
        self.x.cswap(&mut (b.x), d);
        let mut c = d as i32;
        c = !(c - 1);
        let t = c & (self.xes ^ b.xes);
        self.xes ^= t;
        b.xes ^= t;
    }

    /// copy FPs depending on d
    pub fn cmove(&mut self, b: &FP, d: isize) {
        self.x.cmove(&(b.x), d);
        let c = d as i32;
        self.xes ^= (self.xes ^ b.xes) & (-c);
    }

    /// this*=b mod Modulus
    pub fn mul(&mut self, b: &FP) {
        if i64::from(self.xes) * i64::from(b.xes) > i64::from(FEXCESS) {
            self.reduce()
        }

        let mut d = Big::mul(&(self.x), &(b.x));
        self.x.copy(&FP::modulo(&mut d));
        self.xes = 2;
    }

    fn logb2(w: u32) -> usize {
        let mut v = w;
        v |= v >> 1;
        v |= v >> 2;
        v |= v >> 4;
        v |= v >> 8;
        v |= v >> 16;

        v = v - ((v >> 1) & 0x55555555);
        v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
        ((((v + (v >> 4)) & 0xF0F0F0F).wrapping_mul(0x1010101)) >> 24) as usize
    }

    /// Find approximation to quotient of a/m
    /// Out by at most 2.
    /// Note that MAXXES is bounded to be 2-bits less than half a word
    fn quo(n: &Big, m: &Big) -> isize {
        let hb = arch::CHUNK / 2;

        if TBITS < hb {
            let sh = hb - TBITS;
            let num = (n.w[big::NLEN - 1] << sh) | (n.w[big::NLEN - 2] >> (big::BASEBITS - sh));
            let den = (m.w[big::NLEN - 1] << sh) | (m.w[big::NLEN - 2] >> (big::BASEBITS - sh));
            return (num / (den + 1)) as isize;
        } else {
            let num = n.w[big::NLEN - 1];
            let den = m.w[big::NLEN - 1];
            return (num / (den + 1)) as isize;
        }
    }

    /// this = -this mod Modulus
    pub fn neg(&mut self) {
        let mut p = Big::new_ints(&rom::MODULUS);
        let sb = FP::logb2((self.xes - 1) as u32);

        p.fshl(sb);
        self.x.rsub(&p);
        self.xes = 1 << (sb as i32) + 1;
        if self.xes > FEXCESS {
            self.reduce()
        }
    }

    /// this*=c mod Modulus, where c is a small int
    pub fn imul(&mut self, c: isize) {
        let mut cc = c;
        let mut s = false;
        if cc < 0 {
            cc = -cc;
            s = true;
        }

        if MODTYPE == ModType::PseudoMersenne || MODTYPE == ModType::GeneralisedMersenne {
            let mut d = self.x.pxmul(cc);
            self.x.copy(&FP::modulo(&mut d));
            self.xes = 2
        } else {
            if self.xes * (cc as i32) <= FEXCESS {
                self.x.pmul(cc);
                self.xes *= cc as i32;
            } else {
                let n = FP::new_int(cc);
                self.mul(&n);
            }
        }

        if s {
            self.neg();
            self.norm();
        }
    }

    /// self*=self mod Modulus
    pub fn sqr(&mut self) {
        if i64::from(self.xes) * i64::from(self.xes) > i64::from(FEXCESS) {
            self.reduce()
        }

        let mut d = Big::sqr(&(self.x));
        self.x.copy(&FP::modulo(&mut d));
        self.xes = 2
    }

    /// self+=b
    pub fn add(&mut self, b: &FP) {
        self.x.add(&(b.x));
        self.xes += b.xes;
        if self.xes > FEXCESS {
            self.reduce()
        }
    }

    /// self+=self
    pub fn dbl(&mut self) {
        self.x.dbl();
        self.xes += self.xes;
        if self.xes > FEXCESS {
            self.reduce()
        }
    }

    /// self-=b
    pub fn sub(&mut self, b: &FP) {
        let mut n = FP::new_copy(b);
        n.neg();
        self.add(&n);
    }

    /// self=b-self
    pub fn rsub(&mut self, b: &FP) {
        self.neg();
        self.add(&b);
    }

    /// self/=2 mod Modulus
    pub fn div2(&mut self) {
        if self.x.parity() == 0 {
            self.x.fshr(1);
        } else {
            let p = Big::new_ints(&rom::MODULUS);
            self.x.add(&p);
            self.x.norm();
            self.x.fshr(1);
        }
    }

    /// Return this^(p-3)/4 or this^(p-5)/8
    ///
    /// https://eprint.iacr.org/2018/1038
    pub fn fpow(&mut self) -> FP {
        let ac: [isize; 11] = [1, 2, 3, 6, 12, 15, 30, 60, 120, 240, 255];
        let mut xp: [FP; 11] = [
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
        ];
        // phase 1
        let mut t = FP::new();
        xp[0].copy(&self); // 1
        xp[1].copy(&self);
        xp[1].sqr(); // 2
        t.copy(&xp[1]);
        xp[2].copy(&t);
        xp[2].mul(&self); // 3
        t.copy(&xp[2]);
        xp[3].copy(&t);
        xp[3].sqr(); // 6
        t.copy(&xp[3]);
        xp[4].copy(&t);
        xp[4].sqr(); // 12
        t.copy(&xp[4]);
        t.mul(&xp[2]);
        xp[5].copy(&t); // 15
        t.copy(&xp[5]);
        xp[6].copy(&t);
        xp[6].sqr(); // 30
        t.copy(&xp[6]);
        xp[7].copy(&t);
        xp[7].sqr(); // 60
        t.copy(&xp[7]);
        xp[8].copy(&t);
        xp[8].sqr(); // 120
        t.copy(&xp[8]);
        xp[9].copy(&t);
        xp[9].sqr(); // 240
        t.copy(&xp[9]);
        t.mul(&xp[5]);
        xp[10].copy(&t); // 255

        let mut n = MODBITS as isize;
        let c: isize;

        if MODTYPE == ModType::GeneralisedMersenne {
            // Goldilocks ONLY
            n /= 2;
        }

        if MOD8 == 5 {
            n -= 3;
            c = ((rom::MCONST as isize) + 5) / 8;
        } else {
            n -= 2;
            c = ((rom::MCONST as isize) + 3) / 4;
        }
        let mut bw = 0;
        let mut w = 1;
        while w < c {
            w *= 2;
            bw += 1;
        }
        let mut k = w - c;

        let mut i = 10;
        let mut key = FP::new();
        if k != 0 {
            while ac[i] > k {
                i -= 1;
            }
            key.copy(&xp[i]);
            k -= ac[i];
        }
        while k != 0 {
            i -= 1;
            if ac[i] > k {
                continue;
            }
            key.mul(&xp[i]);
            k -= ac[i];
        }
        // phase 2
        t.copy(&xp[2]);
        xp[1].copy(&t);
        t.copy(&xp[5]);
        xp[2].copy(&t);
        t.copy(&xp[10]);
        xp[3].copy(&t);

        let mut j = 3;
        let mut m = 8;
        let nw = n - bw;
        let mut r = FP::new();

        while 2 * m < nw {
            t.copy(&xp[j]);
            j += 1;
            for _ in 0..m {
                t.sqr();
            }
            r.copy(&xp[j - 1]);
            r.mul(&t);
            xp[j].copy(&r);
            m *= 2;
        }
        let mut lo = nw - m;
        r.copy(&xp[j]);

        while lo != 0 {
            m /= 2;
            j -= 1;
            if lo < m {
                continue;
            }
            lo -= m;
            t.copy(&r);
            for _ in 0..m {
                t.sqr();
            }
            r.copy(&t);
            r.mul(&xp[j]);
        }
        // phase 3
        if bw != 0 {
            for _ in 0..bw {
                r.sqr();
            }
            r.mul(&key);
        }
        if MODTYPE == ModType::GeneralisedMersenne {
            // Goldilocks ONLY
            key.copy(&r);
            r.sqr();
            r.mul(&self);
            for _ in 0..=n {
                r.sqr();
            }
            r.mul(&key);
        }
        r
    }

    /// self=1/self mod Modulus
    pub fn inverse(&mut self) {
        if MODTYPE == ModType::PseudoMersenne || MODTYPE == ModType::GeneralisedMersenne {
            let mut y = self.fpow();
            if MOD8 == 5 {
                let mut t = FP::new_copy(self);
                t.sqr();
                self.mul(&t);
                y.sqr();
            }
            y.sqr();
            y.sqr();
            self.mul(&y);
        } else {
            // Constant time inversion using Fermat's little theorem.
            // Fermat's little theorem says for a prime p and for any a < p, a^p = a % p => a^(p-1) = 1 % p => a^(p-2) = a^-1 % p
            let mut m2 = Big::new_ints(&rom::MODULUS);
            m2.dec(2);
            m2.norm();
            let inv = self.pow(&mut m2);
            self.copy(&inv);
        }
    }

    /// return TRUE if self==a
    pub fn equals(&self, a: &FP) -> bool {
        let mut f = FP::new_copy(self);
        let mut s = FP::new_copy(a);
        f.reduce();
        s.reduce();
        if Big::comp(&(f.x), &(s.x)) == 0 {
            return true;
        }
        return false;
    }

    /// return self^e mod Modulus
    pub fn pow(&mut self, e: &mut Big) -> FP {
        let mut tb: [FP; 16] = [
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
            FP::new(),
        ];
        const CT: usize = 1 + (big::NLEN * (big::BASEBITS as usize) + 3) / 4;
        let mut w: [i8; CT] = [0; CT];

        self.norm();
        let mut t = Big::new_copy(e);
        t.norm();
        let nb = 1 + (t.nbits() + 3) / 4;

        for i in 0..nb {
            let lsbs = t.lastbits(4);
            t.dec(lsbs);
            t.norm();
            w[i] = lsbs as i8;
            t.fshr(4);
        }
        tb[0].one();
        tb[1].copy(&self);

        let mut c = FP::new();
        for i in 2..16 {
            c.copy(&tb[i - 1]);
            tb[i].copy(&c);
            tb[i].mul(&self);
        }
        let mut r = FP::new_copy(&tb[w[nb - 1] as usize]);
        for i in (0..nb - 1).rev() {
            r.sqr();
            r.sqr();
            r.sqr();
            r.sqr();
            r.mul(&tb[w[i] as usize])
        }
        r.reduce();
        return r;
    }

    /// return sqrt(this) mod Modulus
    pub fn sqrt(&mut self) -> FP {
        self.reduce();

        if MOD8 == 5 {
            let v: FP;
            let mut i = FP::new_copy(self);
            i.x.shl(1);
            if MODTYPE == ModType::PseudoMersenne || MODTYPE == ModType::GeneralisedMersenne {
                v = i.fpow();
            } else {
                let mut p = Big::new_ints(&rom::MODULUS);
                p.dec(5);
                p.norm();
                p.shr(3);
                v = i.pow(&mut p);
            }
            i.mul(&v);
            i.mul(&v);
            i.x.dec(1);
            let mut r = FP::new_copy(self);
            r.mul(&v);
            r.mul(&i);
            r.reduce();
            return r;
        } else {
            let mut r: FP;
            if MODTYPE == ModType::PseudoMersenne || MODTYPE == ModType::GeneralisedMersenne {
                r = self.fpow();
                r.mul(self);
            } else {
                let mut p = Big::new_ints(&rom::MODULUS);
                p.inc(1);
                p.norm();
                p.shr(2);
                r = self.pow(&mut p);
            }
            return r;
        }
    }

    /// return jacobi symbol (this/Modulus)
    pub fn jacobi(&mut self) -> isize {
        let p = Big::new_ints(&rom::MODULUS);
        let mut w = self.redc();
        return w.jacobi(&p);
    }

    /// Checks if the field value is negative
    ///
    /// Negative if a > -a
    pub fn is_neg(&mut self) -> bool {
        let mut neg_a = self.clone();
        neg_a.neg();
        Big::comp(&self.redc(), &neg_a.redc()) > 0
    }
}
