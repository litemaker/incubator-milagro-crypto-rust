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

use super::super::arch::Chunk;
use bls381::big::NLEN;
use types::{CurvePairingType, CurveType, ModType, SexticTwist, SignOfX};

// Base Bits= 29
// bls381 Modulus

pub const MODULUS: [Chunk; NLEN] = [
    0x1FFFAAAB, 0xFF7FFFF, 0x14FFFFEE, 0x17FFFD62, 0xF6241EA, 0x9507B58, 0xAFD9CC3, 0x109E70A2,
    0x1764774B, 0x121A5D66, 0x12C6E9ED, 0x12FFCD34, 0x111EA3, 0xD,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x15BEF7AE, 0x1031CD0E, 0x2DD93E8, 0x9226323, 0xE6E2CD2, 0x11684DAA, 0x1170E5DB, 0x88E25B1,
    0x1B366399, 0x1C536F47, 0xD1F9CBC, 0x278B67F, 0x1EA66A2B, 0xC,
];
pub const MCONST: Chunk = 0x1FFCFFFD;
pub const FRA: [Chunk; NLEN] = [
    0x12235FB8, 0x83BAF6C, 0x19E04F63, 0x1D4A7AC7, 0xB9C4F67, 0x1EBC25D, 0x1D3DEC91, 0x1FA797AB,
    0x1F0FD603, 0x1016068, 0x108C6FAD, 0x5760CCF, 0x104D3BF0, 0xC,
];
pub const FRB: [Chunk; NLEN] = [
    0xDDC4AF3, 0x7BC5093, 0x1B1FB08B, 0x1AB5829A, 0x3C5F282, 0x764B8FB, 0xDBFB032, 0x10F6D8F6,
    0x1854A147, 0x1118FCFD, 0x23A7A40, 0xD89C065, 0xFC3E2B3, 0x0,
];

pub const CURVE_COF_I: isize = 0;
pub const CURVE_A: isize = 0;
pub const CURVE_B_I: isize = 4;
pub const CURVE_B: [Chunk; NLEN] = [
    0x4, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x1, 0x1FFFFFF8, 0x1F96FFBF, 0x1B4805FF, 0x1D80553B, 0xC0404D0, 0x1520CCE7, 0xA6533AF,
    0x73EDA7, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0x1B22C6BB, 0x19D78056, 0x1E86BBFE, 0xBD07FF2, 0x1AC586C5, 0x1D1F8B8D, 0x4168538, 0x9F2EE97,
    0xFC3688C, 0x27D4D60, 0x9A558E3, 0x32FAF28, 0x1F1D3A73, 0xB,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x6C5E7E1, 0x551194A, 0x222B903, 0x198E8945, 0xB3EDD03, 0xC659602, 0xBD8036C, 0x12BABA01,
    0x4FCF5E0, 0xBA0EC57, 0x8278C3B, 0x75541E3, 0xB3F481E, 0x4,
];

pub const CURVE_BNX: [Chunk; NLEN] = [
    0x10000, 0x10080000, 0x34, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_COF: [Chunk; NLEN] = [
    0xAAAB, 0x55558, 0x157855A3, 0x191800AA, 0x396, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_CRU: [Chunk; NLEN] = [
    0x1FFEFFFE, 0x100FFFFF, 0x280008B, 0xFB026C4, 0x9688DE1, 0x149DF37C, 0x1FAB76CE, 0xED41EE,
    0x11BA69C6, 0x1EFBB672, 0x17C659CB, 0x0, 0x0, 0x0,
];

pub const CURVE_PXA: [Chunk; NLEN] = [
    0x121BDB8, 0x402B646, 0x16EFBF5, 0x18064D50, 0x1D1770BA, 0x5B23D71, 0xC0AD144, 0x1A9F4807,
    0x11C6E47A, 0x196E2882, 0x9820149, 0x11E1522, 0x4AA2B2F, 0x1,
];
pub const CURVE_PXB: [Chunk; NLEN] = [
    0x1D042B7E, 0xD63E82A, 0x51755F9, 0x19E22427, 0x15049334, 0x10DDEE3F, 0x186AD769, 0x1A132416,
    0x5596BD0, 0x4413A7B, 0x1F6B34E8, 0x4E33EC0, 0x1E02B605, 0x9,
];
pub const CURVE_PYA: [Chunk; NLEN] = [
    0x8B82801, 0xC9AA430, 0xB28A278, 0x15939877, 0xD12C923, 0xD34A8B0, 0xE9DB50A, 0x155197BA,
    0x1AADFD9B, 0x16D171A8, 0x3327371, 0x4FADC23, 0xE5D5277, 0x6,
];
pub const CURVE_PYB: [Chunk; NLEN] = [
    0x105F79BE, 0x15483AFF, 0x1B07686A, 0xE1A4EB9, 0x99AB3F3, 0x955AB97, 0xEBC99D2, 0xFD0B4EC,
    0x19CB3E28, 0x15E145C, 0xCAB34AC, 0x1D4E6998, 0x6C4A02, 0x3,
];
pub const CURVE_W: [[Chunk; NLEN]; 2] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_SB: [[[Chunk; NLEN]; 2]; 2] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];
pub const CURVE_WB: [[Chunk; NLEN]; 4] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_BB: [[[Chunk; NLEN]; 4]; 4] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = false;

pub const MODBYTES: usize = 48;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 381;
pub const MOD8: usize = 3;
pub const MODTYPE: ModType = ModType::NotSpecial;
pub const SH: usize = 14;

pub const CURVETYPE: CurveType = CurveType::Weierstrass;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::Bls;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::MType;
pub const ATE_BITS: usize = 65;
pub const SIGN_OF_X: SignOfX = SignOfX::NegativeX;
pub const HASH_TYPE: usize = 32;
pub const AESKEY: usize = 16;
