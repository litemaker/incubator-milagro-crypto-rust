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
use bls24::big::NLEN;
use types::{CurvePairingType, CurveType, ModType, SexticTwist, SignOfX};

// Base Bits= 29
// bls24 Modulus

pub const MODULUS: [Chunk; NLEN] = [
    0xA06152B, 0x2260B3A, 0xB4C36BE, 0x5FFC5D0, 0xBDB6A64, 0x5B78E2E, 0x1C1A28CA, 0x10E6441B,
    0x1F244061, 0xB4704F0, 0x141E5CCD, 0x9837504, 0x3F2E77E, 0xD763740, 0x1316EA0E, 0xF0079,
    0x555C,
];
pub const R2MODP: [Chunk; NLEN] = [
    0x8533EA9, 0x6A02789, 0x183B24DE, 0x1E45ECF8, 0xC8F8F37, 0x10CAD209, 0x4C0C4B8, 0x9B1FABD,
    0xDEBE4C0, 0xDC353F9, 0x18A18E26, 0x10F489BB, 0x31206A5, 0x19673BBF, 0x6BE69F9, 0xB091169,
    0x9CD,
];
pub const MCONST: Chunk = 0x95FE7D;
pub const FRA: [Chunk; NLEN] = [
    0x1BF96F1D, 0xAE53A55, 0x31BFEEB, 0x183FF17A, 0x6237469, 0x12A4F4F1, 0x12101FE3, 0x16E79D94,
    0xFF59267, 0x5EB4EB4, 0x78CC49F, 0x274BA33, 0x149184F3, 0x16C6DCBA, 0x1C90B694, 0x10F729CE,
    0x4BBC,
];
pub const FRB: [Chunk; NLEN] = [
    0xE0CA60E, 0x1740D0E4, 0x83037D2, 0xDBFD456, 0x5B7F5FA, 0x1312993D, 0xA0A08E6, 0x19FEA687,
    0xF2EADF9, 0x55BB63C, 0xC91982E, 0x70EBAD1, 0xF61628B, 0x16AF5A85, 0x16863379, 0xF17D6AA,
    0x99F,
];

pub const CURVE_COF_I: isize = 0;
pub const CURVE_A: isize = 0;
pub const CURVE_B_I: isize = 19;
pub const CURVE_B: [Chunk; NLEN] = [
    0x13, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_ORDER: [Chunk; NLEN] = [
    0x10000001, 0xD047FF, 0x1FD54464, 0x1E3CE067, 0xE322DDA, 0x1D356F3F, 0x7433B44, 0x49091F9,
    0x1729CC2, 0x250286C, 0x16E62ED, 0xB403E1E, 0x1001000, 0x80, 0x0, 0x0, 0x0,
];
pub const CURVE_GX: [Chunk; NLEN] = [
    0xBE3CCD4, 0x33B07AF, 0x1B67D159, 0x3DFC5B5, 0xEBA1FCC, 0x1A3C1F84, 0x56BE204, 0xEF8DF1B,
    0x11AE2D84, 0x5FEE546, 0x161B3BF9, 0x183B20EE, 0x1EA5D99B, 0x14F0C5BF, 0xBE521B7, 0x17C682F9,
    0x1AB2,
];
pub const CURVE_GY: [Chunk; NLEN] = [
    0x121E5245, 0x65D2E56, 0x11577DB1, 0x16DACC11, 0x14F39746, 0x459F694, 0x12483FCF, 0xC828B04,
    0xFD63E5A, 0x7B1D52, 0xAFDE738, 0xF349254, 0x1A4529FF, 0x10E53353, 0xF91DEE1, 0x16E18D8A,
    0x47FC,
];

pub const CURVE_BNX: [Chunk; NLEN] = [
    0x11FF80, 0x80010, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
];
pub const CURVE_COF: [Chunk; NLEN] = [
    0x19F415AB, 0x1E0FFDFF, 0x15AAADFF, 0xAA, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    0x0, 0x0, 0x0,
];
pub const CURVE_CRU: [Chunk; NLEN] = [
    0xDD794A9, 0x1DE138A3, 0x2BCCE90, 0xC746127, 0x15223DDC, 0x1DD8890B, 0xED08DB7, 0xE24B9F,
    0xE379CE6, 0x37011AC, 0x11BAC820, 0x1EEFAD01, 0x200860F, 0x147218A6, 0xF16A209, 0xF0079,
    0x555C,
];
pub const CURVE_PXAA: [Chunk; NLEN] = [
    0x14E24678, 0x1F149A9B, 0x9609022, 0x1C186868, 0xCDEFC69, 0x1C87BB2E, 0x14A2235F, 0x7586755,
    0x5896747, 0x159BFE92, 0x3B5572E, 0x1710A521, 0x71EB14A, 0xC643C33, 0x12581DE5, 0x1BCA747D,
    0x959,
];
pub const CURVE_PXAB: [Chunk; NLEN] = [
    0x1FB099B8, 0x3FCF5D7, 0x4A91C0E, 0xC6EEB40, 0x11FC2385, 0x11B5AE8D, 0x1A9CC3E7, 0x194FE144,
    0x185DB2A5, 0x930E1C7, 0x14F85F9A, 0x1F2ED4E, 0x1D1BE5AD, 0xF26169C, 0xCF7F194, 0x1DA1062E,
    0x3B0D,
];
pub const CURVE_PXBA: [Chunk; NLEN] = [
    0x11AD15D3, 0xD0E6F38, 0x17DB85BB, 0x30A62F1, 0x1EA3E09A, 0x17B25FA1, 0x1B7959AC, 0x1165B19A,
    0x6C74FDB, 0x18F790E1, 0x12278FDA, 0x1E008F79, 0x103F329, 0x14619FF1, 0x1EBCAA8, 0xFF5A9CA,
    0x3EC2,
];
pub const CURVE_PXBB: [Chunk; NLEN] = [
    0x1EE0F480, 0x3D5943A, 0xF5B12E3, 0x128AADC8, 0x180E1CB9, 0x1EFD916F, 0x48BC7F, 0x1D5EE1FA,
    0x5698EF5, 0x11D6AED9, 0x1386BC6E, 0x196E900B, 0x1CE2E465, 0xC2A8ED3, 0x1E67DF99, 0x71B7940,
    0xA5B,
];
pub const CURVE_PYAA: [Chunk; NLEN] = [
    0x14781AA0, 0xC324C98, 0xEDC2AC, 0x16C13B46, 0x145FC44B, 0x12529530, 0x1310A8C4, 0x1768C5C0,
    0xE19AE68, 0x56E1C1D, 0x13DAF93F, 0x17E94366, 0xF901AD0, 0x76800CC, 0x10250D8B, 0x1E6BAE6D,
    0x5057,
];
pub const CURVE_PYAB: [Chunk; NLEN] = [
    0xEAE08FA, 0xDDF62BF, 0xA97E5AB, 0xF0EE97, 0x99A42CA, 0x1C326578, 0xF33DC11, 0x8B913F7,
    0xFEF8552, 0x19F35B90, 0x58DDBDE, 0xFC32FF2, 0x1587B5DF, 0xB5EB07A, 0x1A258DE0, 0x1692CC3D,
    0x2CE2,
];
pub const CURVE_PYBA: [Chunk; NLEN] = [
    0x5F0CC41, 0xB9813B5, 0x14C2A87D, 0xFF1264A, 0x19AF8A14, 0x6CE6C3, 0x2A7F8A2, 0x121DCA7D,
    0x7D37153, 0x19D21078, 0x15466DC7, 0x1362982B, 0x1DD3CB5B, 0x1CFC0D1C, 0x18C69AF8, 0x8CC7DC,
    0x1807,
];
pub const CURVE_PYBB: [Chunk; NLEN] = [
    0x115C1CAE, 0x78D9732, 0x16C26237, 0x5A81A6A, 0x1C38A777, 0x56121FE, 0x4DAD9D7, 0x1BEBA670,
    0xA1D72FC, 0xD60B274, 0x19734258, 0x1D621775, 0x4691771, 0x14206B68, 0x17B22DE4, 0x29D5B37,
    0x499D,
];
pub const CURVE_W: [[Chunk; NLEN]; 2] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_SB: [[[Chunk; NLEN]; 2]; 2] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];
pub const CURVE_WB: [[Chunk; NLEN]; 4] = [
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
    [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
    ],
];
pub const CURVE_BB: [[[Chunk; NLEN]; 4]; 4] = [
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
    [
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        [
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
    ],
];

pub const USE_GLV: bool = true;
pub const USE_GS_G2: bool = true;
pub const USE_GS_GT: bool = true;
pub const GT_STRONG: bool = true;

pub const MODBYTES: usize = 60;
pub const BASEBITS: usize = 29;

pub const MODBITS: usize = 479;
pub const MOD8: usize = 3;
pub const MODTYPE: ModType = ModType::NotSpecial;
pub const SH: usize = 14;

pub const CURVETYPE: CurveType = CurveType::Weierstrass;
pub const CURVE_PAIRING_TYPE: CurvePairingType = CurvePairingType::Bls;
pub const SEXTIC_TWIST: SexticTwist = SexticTwist::MType;
pub const ATE_BITS: usize = 49;
pub const SIGN_OF_X: SignOfX = SignOfX::PositiveX;
pub const HASH_TYPE: usize = 48;
pub const AESKEY: usize = 24;
