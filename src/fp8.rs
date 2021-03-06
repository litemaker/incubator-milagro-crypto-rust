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

use super::big::Big;
use super::fp::FP;
use super::fp2::FP2;
use super::fp4::FP4;
//use std::str::SplitWhitespace;

#[derive(Copy, Clone)]
pub struct FP8 {
    a: FP4,
    b: FP4,
}

impl FP8 {
    pub fn new() -> FP8 {
        FP8 {
            a: FP4::new(),
            b: FP4::new(),
        }
    }

    pub fn new_int(a: isize) -> FP8 {
        let mut f = FP8::new();
        f.a.copy(&FP4::new_int(a));
        f.b.zero();
        return f;
    }

    pub fn new_copy(x: &FP8) -> FP8 {
        let mut f = FP8::new();
        f.a.copy(&x.a);
        f.b.copy(&x.b);
        return f;
    }

    pub fn new_fp4s(c: &FP4, d: &FP4) -> FP8 {
        let mut f = FP8::new();
        f.a.copy(c);
        f.b.copy(d);
        return f;
    }

    pub fn new_fp4(c: &FP4) -> FP8 {
        let mut f = FP8::new();
        f.a.copy(c);
        f.b.zero();
        return f;
    }

    pub fn set_fp4s(&mut self, c: &FP4, d: &FP4) {
        self.a.copy(&c);
        self.b.copy(&d);
    }

    pub fn set_fp4(&mut self, c: &FP4) {
        self.a.copy(&c);
        self.b.zero();
    }

    pub fn set_fp4h(&mut self, c: &FP4) {
        self.b.copy(&c);
        self.a.zero();
    }

    /* reduce components mod Modulus */
    pub fn reduce(&mut self) {
        self.a.reduce();
        self.b.reduce();
    }

    /* normalise components of w */
    pub fn norm(&mut self) {
        self.a.norm();
        self.b.norm();
    }

    pub fn cmove(&mut self, g: &FP8, d: isize) {
        self.a.cmove(&g.a, d);
        self.b.cmove(&g.b, d);
    }

    /* test self=0 ? */
    pub fn iszilch(&self) -> bool {
        return self.a.iszilch() && self.b.iszilch();
    }

    /* test self=1 ? */
    pub fn isunity(&self) -> bool {
        let one = FP4::new_int(1);
        return self.a.equals(&one) && self.b.iszilch();
    }

    /* test is w real? That is in a+ib test b is zero */
    pub fn isreal(&mut self) -> bool {
        return self.b.iszilch();
    }
    /* extract real part a */
    pub fn real(&self) -> FP4 {
        let f = FP4::new_copy(&self.a);
        return f;
    }

    pub fn geta(&self) -> FP4 {
        return self.a;
        //        let f = FP4::new_copy(&self.a);
        //        return f;
    }
    /* extract imaginary part b */
    pub fn getb(&self) -> FP4 {
        return self.b;
        //        let f = FP4::new_copy(&self.b);
        //        return f;
    }

    /* test self=x */
    pub fn equals(&self, x: &FP8) -> bool {
        return self.a.equals(&x.a) && self.b.equals(&x.b);
    }
    /* copy self=x */
    pub fn copy(&mut self, x: &FP8) {
        self.a.copy(&x.a);
        self.b.copy(&x.b);
    }

    /* set self=0 */
    pub fn zero(&mut self) {
        self.a.zero();
        self.b.zero();
    }

    /* set self=1 */
    pub fn one(&mut self) {
        self.a.one();
        self.b.zero();
    }

    /* negate self mod Modulus */
    pub fn neg(&mut self) {
        self.norm();
        let mut m = FP4::new_copy(&self.a);
        let mut t = FP4::new();

        m.add(&self.b);
        m.neg();

        t.copy(&m);
        t.add(&self.b);
        self.b.copy(&m);
        self.b.add(&self.a);
        self.a.copy(&t);
        self.norm();
    }

    /* set to a-ib */
    pub fn conj(&mut self) {
        self.b.neg();
        self.norm();
    }

    /* self=-conjugate(self) */
    pub fn nconj(&mut self) {
        self.a.neg();
        self.norm();
    }

    /* self+=a */
    pub fn add(&mut self, x: &FP8) {
        self.a.add(&x.a);
        self.b.add(&x.b);
    }

    pub fn padd(&mut self, x: &FP4) {
        self.a.add(x);
    }

    pub fn dbl(&mut self) {
        self.a.dbl();
        self.b.dbl();
    }

    /* self-=a */
    pub fn sub(&mut self, x: &FP8) {
        let mut m = FP8::new_copy(x);
        m.neg();
        self.add(&m);
    }

    /* this-=x */
    pub fn rsub(&mut self, x: &FP8) {
        self.neg();
        self.add(x);
    }

    /* self*=s, where s is an FP4 */
    pub fn pmul(&mut self, s: &FP4) {
        self.a.mul(s);
        self.b.mul(s);
    }

    /* self*=s, where s is an FP2 */
    pub fn qmul(&mut self, s: &FP2) {
        self.a.pmul(s);
        self.b.pmul(s);
    }

    /* self*=s, where s is an FP */
    pub fn tmul(&mut self, s: &FP) {
        self.a.qmul(s);
        self.b.qmul(s);
    }

    /* self*=i, where i is an int */
    pub fn imul(&mut self, c: isize) {
        self.a.imul(c);
        self.b.imul(c);
    }

    /* self*=self */

    pub fn sqr(&mut self) {
        let mut t1 = FP4::new_copy(&self.a);
        let mut t2 = FP4::new_copy(&self.b);
        let mut t3 = FP4::new_copy(&self.a);

        t3.mul(&self.b);
        t1.add(&self.b);
        t2.times_i();

        t2.add(&self.a);

        t1.norm();
        t2.norm();

        self.a.copy(&t1);

        self.a.mul(&t2);

        t2.copy(&t3);
        t2.times_i();
        t2.add(&t3);
        t2.norm();
        t2.neg();
        self.a.add(&t2);

        t3.dbl();
        self.b.copy(&t3);

        self.norm();
    }

    /* self*=y */
    pub fn mul(&mut self, y: &FP8) {
        //self.norm();

        let mut t1 = FP4::new_copy(&self.a);
        let mut t2 = FP4::new_copy(&self.b);
        let mut t3 = FP4::new();
        let mut t4 = FP4::new_copy(&self.b);

        t1.mul(&y.a);
        t2.mul(&y.b);
        t3.copy(&y.b);
        t3.add(&y.a);
        t4.add(&self.a);

        t3.norm();
        t4.norm();

        t4.mul(&t3);

        t3.copy(&t1);
        t3.neg();
        t4.add(&t3);
        t4.norm();

        t3.copy(&t2);
        t3.neg();
        self.b.copy(&t4);
        self.b.add(&t3);

        t2.times_i();
        self.a.copy(&t2);
        self.a.add(&t1);

        self.norm();
    }

    /* output to hex string */
    pub fn tostring(&mut self) -> String {
        return format!("[{},{}]", self.a.tostring(), self.b.tostring());
    }

    /* self=1/self */
    pub fn inverse(&mut self) {
        //self.norm();

        let mut t1 = FP4::new_copy(&self.a);
        let mut t2 = FP4::new_copy(&self.b);

        t1.sqr();
        t2.sqr();
        t2.times_i();
        t2.norm();
        t1.sub(&t2);
        t1.norm();
        t1.inverse();
        self.a.mul(&t1);
        t1.neg();
        t1.norm();
        self.b.mul(&t1);
    }

    /* self*=i where i = sqrt(-1+sqrt(-1)) */
    pub fn times_i(&mut self) {
        let mut s = FP4::new_copy(&self.b);
        let t = FP4::new_copy(&self.a);
        s.times_i();
        self.a.copy(&s);
        self.b.copy(&t);

        self.norm();
    }

    pub fn times_i2(&mut self) {
        self.a.times_i();
        self.b.times_i();
    }

    /* self=self^p using Frobenius */
    pub fn frob(&mut self, f: &FP2) {
        let mut ff = FP2::new_copy(f);
        ff.sqr();
        ff.mul_ip();
        ff.norm();
        self.a.frob(&ff);
        self.b.frob(&ff);
        self.b.pmul(f);
        self.b.times_i();
    }

    /* self=self^e */
    pub fn pow(&self, e: &Big) -> FP8 {
        let mut w = FP8::new_copy(self);
        w.norm();
        let mut z = Big::new_copy(&e);
        let mut r = FP8::new_int(1);
        z.norm();
        loop {
            let bt = z.parity();
            z.fshr(1);
            if bt == 1 {
                r.mul(&w)
            };
            if z.iszilch() {
                break;
            }
            w.sqr();
        }
        r.reduce();
        return r;
    }

    /* XTR xtr_a function */
    pub fn xtr_a(&mut self, w: &FP8, y: &FP8, z: &FP8) {
        let mut r = FP8::new_copy(w);
        let mut t = FP8::new_copy(w);

        r.sub(y);
        r.norm();
        r.pmul(&self.a);
        t.add(y);
        t.norm();
        t.pmul(&self.b);
        t.times_i();

        self.copy(&r);
        self.add(&t);
        self.add(z);

        self.norm();
    }

    /* XTR xtr_d function */
    pub fn xtr_d(&mut self) {
        let mut w = FP8::new_copy(self);
        self.sqr();
        w.conj();
        w.dbl();
        w.norm();
        self.sub(&w);
        self.reduce();
    }

    /* r=x^n using XTR method on traces of FP24s */
    pub fn xtr_pow(&self, n: &Big) -> FP8 {
        let mut sf = FP8::new_copy(self);
        sf.norm();
        let mut a = FP8::new_int(3);
        let mut b = FP8::new_copy(&sf);
        let mut c = FP8::new_copy(&b);
        c.xtr_d();
        let mut t = FP8::new();
        let mut r = FP8::new();

        let par = n.parity();
        let mut v = Big::new_copy(n);
        v.norm();
        v.fshr(1);
        if par == 0 {
            v.dec(1);
            v.norm();
        }

        let nb = v.nbits();
        for i in (0..nb).rev() {
            if v.bit(i) != 1 {
                t.copy(&b);
                sf.conj();
                c.conj();
                b.xtr_a(&a, &sf, &c);
                sf.conj();
                c.copy(&t);
                c.xtr_d();
                a.xtr_d();
            } else {
                t.copy(&a);
                t.conj();
                a.copy(&b);
                a.xtr_d();
                b.xtr_a(&c, &sf, &t);
                c.xtr_d();
            }
        }
        if par == 0 {
            r.copy(&c)
        } else {
            r.copy(&b)
        }
        r.reduce();
        return r;
    }

    /* r=ck^a.cl^n using XTR double exponentiation method on traces of FP12s. See Stam thesis. */
    pub fn xtr_pow2(&mut self, ck: &FP8, ckml: &FP8, ckm2l: &FP8, a: &Big, b: &Big) -> FP8 {
        let mut e = Big::new_copy(a);
        let mut d = Big::new_copy(b);
        let mut w = Big::new();
        e.norm();
        d.norm();

        let mut cu = FP8::new_copy(ck); // can probably be passed in w/o copying
        let mut cv = FP8::new_copy(self);
        let mut cumv = FP8::new_copy(ckml);
        let mut cum2v = FP8::new_copy(ckm2l);
        let mut r = FP8::new();
        let mut t = FP8::new();

        let mut f2: usize = 0;
        while d.parity() == 0 && e.parity() == 0 {
            d.fshr(1);
            e.fshr(1);
            f2 += 1;
        }

        while Big::comp(&d, &e) != 0 {
            if Big::comp(&d, &e) > 0 {
                w.copy(&e);
                w.imul(4);
                w.norm();
                if Big::comp(&d, &w) <= 0 {
                    w.copy(&d);
                    d.copy(&e);
                    e.rsub(&w);
                    e.norm();

                    t.copy(&cv);
                    t.xtr_a(&cu, &cumv, &cum2v);
                    cum2v.copy(&cumv);
                    cum2v.conj();
                    cumv.copy(&cv);
                    cv.copy(&cu);
                    cu.copy(&t);
                } else {
                    if d.parity() == 0 {
                        d.fshr(1);
                        r.copy(&cum2v);
                        r.conj();
                        t.copy(&cumv);
                        t.xtr_a(&cu, &cv, &r);
                        cum2v.copy(&cumv);
                        cum2v.xtr_d();
                        cumv.copy(&t);
                        cu.xtr_d();
                    } else {
                        if e.parity() == 1 {
                            d.sub(&e);
                            d.norm();
                            d.fshr(1);
                            t.copy(&cv);
                            t.xtr_a(&cu, &cumv, &cum2v);
                            cu.xtr_d();
                            cum2v.copy(&cv);
                            cum2v.xtr_d();
                            cum2v.conj();
                            cv.copy(&t);
                        } else {
                            w.copy(&d);
                            d.copy(&e);
                            d.fshr(1);
                            e.copy(&w);
                            t.copy(&cumv);
                            t.xtr_d();
                            cumv.copy(&cum2v);
                            cumv.conj();
                            cum2v.copy(&t);
                            cum2v.conj();
                            t.copy(&cv);
                            t.xtr_d();
                            cv.copy(&cu);
                            cu.copy(&t);
                        }
                    }
                }
            }
            if Big::comp(&d, &e) < 0 {
                w.copy(&d);
                w.imul(4);
                w.norm();
                if Big::comp(&e, &w) <= 0 {
                    e.sub(&d);
                    e.norm();
                    t.copy(&cv);
                    t.xtr_a(&cu, &cumv, &cum2v);
                    cum2v.copy(&cumv);
                    cumv.copy(&cu);
                    cu.copy(&t);
                } else {
                    if e.parity() == 0 {
                        w.copy(&d);
                        d.copy(&e);
                        d.fshr(1);
                        e.copy(&w);
                        t.copy(&cumv);
                        t.xtr_d();
                        cumv.copy(&cum2v);
                        cumv.conj();
                        cum2v.copy(&t);
                        cum2v.conj();
                        t.copy(&cv);
                        t.xtr_d();
                        cv.copy(&cu);
                        cu.copy(&t);
                    } else {
                        if d.parity() == 1 {
                            w.copy(&e);
                            e.copy(&d);
                            w.sub(&d);
                            w.norm();
                            d.copy(&w);
                            d.fshr(1);
                            t.copy(&cv);
                            t.xtr_a(&cu, &cumv, &cum2v);
                            cumv.conj();
                            cum2v.copy(&cu);
                            cum2v.xtr_d();
                            cum2v.conj();
                            cu.copy(&cv);
                            cu.xtr_d();
                            cv.copy(&t);
                        } else {
                            d.fshr(1);
                            r.copy(&cum2v);
                            r.conj();
                            t.copy(&cumv);
                            t.xtr_a(&cu, &cv, &r);
                            cum2v.copy(&cumv);
                            cum2v.xtr_d();
                            cumv.copy(&t);
                            cu.xtr_d();
                        }
                    }
                }
            }
        }
        r.copy(&cv);
        r.xtr_a(&cu, &cumv, &cum2v);
        for _ in 0..f2 {
            r.xtr_d()
        }
        r = r.xtr_pow(&mut d);
        return r;
    }

    /* this/=2 */
    pub fn div2(&mut self) {
        self.a.div2();
        self.b.div2();
    }

    pub fn div_i(&mut self) {
        let mut u = FP4::new_copy(&self.a);
        let v = FP4::new_copy(&self.b);
        u.div_i();
        self.a.copy(&v);
        self.b.copy(&u);
    }

    pub fn div_i2(&mut self) {
        self.a.div_i();
        self.b.div_i();
    }

    pub fn div_2i(&mut self) {
        let mut u = FP4::new_copy(&self.a);
        let mut v = FP4::new_copy(&self.b);
        u.div_2i();
        v.dbl();
        v.norm();
        self.a.copy(&v);
        self.b.copy(&u);
    }

    /* sqrt(a+ib) = sqrt(a+sqrt(a*a-n*b*b)/2)+ib/(2*sqrt(a+sqrt(a*a-n*b*b)/2)) */
    /* returns true if this is QR */
    pub fn sqrt(&mut self) -> bool {
        if self.iszilch() {
            return true;
        }

        let mut a = FP4::new_copy(&self.a);
        let mut s = FP4::new_copy(&self.b);
        let mut t = FP4::new_copy(&self.a);

        if s.iszilch() {
            if t.sqrt() {
                self.a.copy(&t);
                self.b.zero();
            } else {
                t.div_i();
                t.sqrt();
                self.b.copy(&t);
                self.a.zero();
            }
            return true;
        }
        s.sqr();
        a.sqr();
        s.times_i();
        s.norm();
        a.sub(&s);

        s.copy(&a);
        if !s.sqrt() {
            return false;
        }

        a.copy(&t);
        a.add(&s);
        a.norm();
        a.div2();

        if !a.sqrt() {
            a.copy(&t);
            a.sub(&s);
            a.norm();
            a.div2();
            if !a.sqrt() {
                return false;
            }
        }
        t.copy(&self.b);
        s.copy(&a);
        s.add(&a);
        s.inverse();

        t.mul(&s);
        self.a.copy(&a);
        self.b.copy(&t);

        return true;
    }
}
