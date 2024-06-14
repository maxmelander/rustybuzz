/* == Start of generated table == */
/*
 * The following table is generated by running:
 *
 *   ./gen-use-table.py IndicSyllabicCategory.txt IndicPositionalCategory.txt ArabicShaping.txt DerivedCoreProperties.txt UnicodeData.txt Blocks.txt Scripts.txt IndicSyllabicCategory-Additional.txt IndicPositionalCategory-Additional.txt
 *
 * on files with these headers:
 *
 * # IndicSyllabicCategory-14.0.0.txt
 * # Date: 2021-05-22, 01:01:00 GMT [KW, RP]
 * # IndicPositionalCategory-14.0.0.txt
 * # Date: 2021-05-22, 01:01:00 GMT [KW, RP]
 * # ArabicShaping-14.0.0.txt
 * # Date: 2021-05-21, 01:54:00 GMT [KW, RP]
 * # DerivedCoreProperties-14.0.0.txt
 * # Date: 2021-08-12, 23:12:53 GMT
 * # Blocks-14.0.0.txt
 * # Date: 2021-01-22, 23:29:00 GMT [KW]
 * # Scripts-14.0.0.txt
 * # Date: 2021-07-10, 00:35:31 GMT
 * # Override values For Indic_Syllabic_Category
 * # Not derivable
 * # Initial version based on Unicode 7.0 by Andrew Glass 2014-03-17
 * # Updated for Unicode 10.0 by Andrew Glass 2017-07-25
 * # Updated for Unicode 12.1 by Andrew Glass 2019-05-24
 * # Updated for Unicode 13.0 by Andrew Glass 2020-07-28
 * # Updated for Unicode 14.0 by Andrew Glass 2021-09-25
 * # Override values For Indic_Positional_Category
 * # Not derivable
 * # Initial version based on Unicode 7.0 by Andrew Glass 2014-03-17
 * # Updated for Unicode 10.0 by Andrew Glass 2017-07-25
 * # Ammended for Unicode 10.0 by Andrew Glass 2018-09-21
 * # Updated for L2/19-083    by Andrew Glass 2019-05-06
 * # Updated for Unicode 12.1 by Andrew Glass 2019-05-30
 * # Updated for Unicode 13.0 by Andrew Glass 2020-07-28
 * # Updated for Unicode 14.0 by Andrew Glass 2021-09-28
 * UnicodeData.txt does not have a header.
 */

use super::ot_shaper_use::category::*;

const hb_use_u8: [u8; 1842] =
[
0,    1,    2,    3,    3,    3,    3,    3,    3,    3,    4,    3,    3,    3,    3,    5,
6,    7,    3,    8,    3,    3,    9,    3,   10,    3,    3,   11,    3,   12,   13,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,    3,
14,    0,    0,    1,    1,    2,    1,    1,    3,    4,    5,    6,    7,    8,    9,   10,
1,   11,   12,    1,    1,    1,    1,    1,    1,   13,   14,   15,   16,   17,   18,   19,
1,    1,   20,    1,    1,    1,    1,   21,    1,    1,    1,    1,    1,    1,    1,   22,
1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
1,    1,    1,    1,    1,    1,    1,    1,    1,    1,   23,   24,   25,   26,    1,    1,
1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
27,   28,    1,    1,    1,    1,    1,   29,    1,    1,    1,    1,   30,   31,    1,   32,
33,   34,   35,   36,   37,   38,   39,   40,   41,   42,   43,   44,   45,    1,   46,   47,
48,    1,   49,   49,   49,   49,   50,    1,    1,    1,    1,    1,    1,    1,    1,    1,
1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,   51,   52,    1,    1,
1,   53,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,   49,   54,    1,
1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,   55,    1,
1,    1,    1,   56,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,    1,
1,    1,    1,   57,   58,    1,    1,    1,    1,    1,    1,   59,    1,    1,    1,    1,
1,    1,   60,   61,   60,   60,   60,   60,   60,   60,   60,   60,   60,   60,   60,   60,
60,   60,    O,    O,    O,    O,    O,   GB,    O,    O,    B,    B,    B,    B,    B,    B,
O,    O,   GB,    O,    O,    O,    O,   WJ,    O,    O,    O,    O,FMPst,FMPst,    O,    O,
O,   GB,    O,    O,    O,  CGJ,    B,    O,    O,    O,    O,    O,    B,    B,    B,    B,
B,VMAbv,VMAbv,VMAbv,VMAbv,VMAbv,    O,    O,    B,    O,    O,VMAbv,    O,    O,    B,CMBlw,
CMBlw,CMBlw,VMAbv,VMAbv,VMAbv,VMPst,    B,    B, VAbv, VPst,CMBlw,    B, VPst, VPre, VPst, VBlw,
VBlw, VBlw, VBlw, VAbv, VAbv, VAbv, VAbv, VPst, VPst, VPst, VPst,    H, VPre, VPst,    O,VMAbv,
VMBlw,    O,    O, VAbv, VBlw, VBlw,    B,    B, VBlw, VBlw,   GB,VMAbv,VMPst,VMPst,    O,    B,
B,    B,    B,    O,    O,    B,    B,    O,    B,    B,    B,    O,    B,    O, VBlw,    O,
O, VPre, VPre,    O,    O, VPre, VPre,    H,    O,    O,    O,    O,    O, VPst,    B,    B,
O,    B,    B,    O,FMAbv,    O,    O,VMAbv,VMAbv,VMPst,    B,    B,    B,    O,    O,    O,
O,    B,    O,    B,    B,    O,CMBlw,    O, VPst, VPre, VPst, VBlw, VBlw,    O,    O,    O,
O, VAbv, VAbv,    O,    O, VAbv, VAbv,    H,    O,    O,    O,VMBlw,    O,    O,VMAbv,CMAbv,
GB,   GB,    O, MBlw,    O,    O, VBlw, VAbv,    O, VAbv, VAbv, VAbv,    O, VPst, VPst,    H,
O,    O,    O,    B,VMAbv,VMAbv,VMAbv,CMAbv,CMAbv,CMAbv,    O,VMAbv,VMPst,VMPst,CMBlw,    B,
VPst, VAbv,    O, VAbv, VAbv, VAbv,    O,    B,    O,    O,    O,    O,VMAbv,    O,    O,    O,
VPst, VPst, VAbv, VPst, VPst,    O,    O,    O, VPre, VPre, VPre,    O, VPre, VPre,VMAbv,VMPst,
VMPst,VMPst,VMAbv,    B,    B,    B,CMBlw,    B, VAbv, VAbv, VPst,    O, VAbv, VAbv, VAbv,    O,
VAbv, VAbv,    O, VAbv, VBlw,    O,    B,VMAbv,VMPst,VMPst,    O, VPst, VPst,    O,    O,   CS,
CS,    O,VMAbv,VMAbv,VMPst,VMPst,    B,    B,    B, VAbv, VAbv,    B, VPst, VPst, VPst, VPst,
VPst, VBlw, VBlw,    O, VPre, VPre, VPre,    H,    R,    O,    O,    O,  HVM,    O, VPst, VPst,
VAbv, VAbv, VBlw,    O, VBlw,    O, VPst, VPre, VPre, VPre, VPre, VPre, VPre, VPst, VBlw, VBlw,
O,    O,    O, FBlw,    O, FBlw,    O,CMAbv,    O,    O,    O,    O, VPst, VPre,    O,CMBlw,
VBlw, VAbv, VAbv, VBlw, VAbv, VAbv, VAbv, VAbv, VBlw, VBlw, VBlw, VBlw,VMAbv,    O, VBlw, VAbv,
VMAbv,VMAbv, VBlw,    O,VMAbv,VMAbv,    B,  SUB,  SUB,  SUB,  SUB,  SUB,  SUB,  SUB,    O,  SUB,
SUB,  SUB,  SUB,    O,    O,    O,    O,    O, FBlw,    O,    B,    B,    B, VPst, VPst, VAbv,
VAbv, VBlw, VBlw, VPre, VAbv, VAbv, VAbv, VAbv,VMAbv,VMBlw,VMPst,   IS, VAbv, MPst, MPre, MBlw,
MBlw,    B,    B,    B,    O,   GB,    O,    O,   GB,    O,    B,    B, VPst, VPst, VBlw, VBlw,
B,    B,    B,    B, MBlw, MBlw, MBlw,    B, VPst,VMPst,VMPst,    B,    B, VPst, VPst,VMPst,
VMPst,VMPst,VMPst,VMPst,    B,    B,    B, VAbv, VAbv, VAbv, VAbv,    B,    B,    B,    B,    B,
MBlw, VPst, VPre, VAbv, VAbv,VMPst,VMPst,VMPst,VMPst,VMPst,VMPst,VMBlw,    B,VMPst,    B,    B,
VMPst,VMPst, VPst, VAbv,    O,    O,    B,    B, VAbv, VBlw, VBlw, VPst,    O,    O, VPst,    O,
O,    O,    B,    O, VAbv, VBlw,  CGJ,  CGJ, VPst, VAbv, VAbv, VAbv, VAbv, VBlw, VBlw, VBlw,
VPre, VPre, VPre, VPre, VPre, VPre, VPre, VPre,VMAbv,VMPst, VPst,VMAbv,VMAbv,FMAbv, FAbv,CMAbv,
FMAbv,VMAbv,FMAbv, VAbv,   IS,FMAbv,    B,FMAbv,    O,    O,    O,    O,    B,  CGJ,  CGJ,  CGJ,
WJ,  CGJ,   GB,   GB,   GB,   GB,   GB,CMAbv,CMAbv,    B,    B,CMBlw,    B,    O,   GB,    B,
B,    B, VAbv, VAbv, VBlw, VPst, VPst, VAbv, VAbv, VAbv, VAbv,  SUB,  SUB,  SUB, FPst, FPst,
VMBlw, FPst, FPst, FPst, FPst, FPst, FPst, FBlw,VMAbv,FMBlw,VMPst,VMPst,    O,    O, VAbv, VPre,
VPst, VAbv,    B, MPre, MBlw,  SUB, FAbv, FAbv, MAbv,  SUB,  SUB,  SUB,  SUB,    O,   Sk, VPst,
VAbv, VPst, VAbv, VBlw, VBlw, VAbv, VBlw, VPst, VPre, VPre, VPre, VPre, VPre, VAbv,VMAbv,VMAbv,
VAbv,VMAbv,VMAbv,    O,    O,VMBlw,VMAbv,VMAbv,VMAbv, FAbv,VMPst,    B,    B,    B,CMAbv, VPst,
VAbv, VAbv, VBlw, VBlw, VBlw, VBlw, VAbv, VAbv, VPre, VPre, VPre, VPre, VAbv, VAbv,    H,    B,
B,    B,    O,    O,    O,SMAbv,SMBlw,SMAbv,SMAbv,SMAbv,SMAbv,SMAbv,SMAbv,SMAbv,VMAbv, FAbv,
VMPst,    B, VAbv, VBlw, VPre, VPst, VAbv, VAbv, VPst,   IS,  SUB,  SUB,    B,    B,    B,    B,
CMAbv, VPst, VAbv, VAbv, VPst, VPst, VPst, VAbv, VPst, VAbv, FAbv, FAbv,CMBlw,CMBlw,  SUB,  SUB,
VPst, VPre, VPre, VPre, VPst, VPst, VBlw, FAbv, FAbv, FAbv, FAbv, FAbv, FAbv, FAbv,VMPre,VMPre,
FMAbv,CMBlw,VMAbv,VMAbv,VMAbv,    O,VMBlw,VMBlw,VMBlw,VMBlw,VMBlw,VMBlw,VMAbv,VMAbv,VMAbv,VMPst,
VMBlw,VMBlw,VMBlw,    O,    O,    O,VMAbv,   CS,   CS,VMPst,VMAbv,VMAbv,   GB,    O,    O,    O,
O,FMAbv,    O,    O,    O,   WJ, ZWNJ,  CGJ,   WJ,   WJ,    O,    O,   WJ,   WJ,   WJ,   WJ,
WJ,    O,   WJ,   WJ,   WJ,   WJ,FMPst,    O,    O,    O,VMAbv,    O,    O,    O,    O,    O,
O,    H,    B,    B, VAbv,    B,    B,    B,    H,    B, VPst, VBlw, VAbv, VPst, VBlw,    O,
O,    O, MPst, VPst, VPst, VPst, VPst, VPst, VPst, VPst,    H,VMAbv,    O,    O,VMAbv,VMAbv,
B,    B,    O,    O,    B, VAbv,    B,    B, VAbv, VAbv, VAbv, VAbv, VAbv,VMBlw,VMBlw,VMBlw,
O,    O,    B,    B,    B, VBlw, VBlw, VBlw, VAbv, VBlw, VBlw, VBlw, VBlw, FAbv, FAbv, FAbv,
FPst, VPst,VMAbv,VMAbv, FAbv,VMPst,    B,    B,    B,CMAbv, VAbv, MBlw, MPst, MBlw,    H,    O,
O,    O,    B, VAbv,    O,    B,    B,VMAbv, VAbv, VAbv, VAbv, VBlw, VAbv, VPre, VPre, VAbv,
VBlw, MPst, MPre, MAbv, MBlw,    O,    B,    B,    B, FAbv, FAbv, FPst,    O,    O,   GB,   GB,
GB,    O,    O,    O,    B,VMPst,VMAbv,VMPst,    B,    B, VAbv,    B, VAbv, VAbv, VBlw,    B,
B, VAbv,    B,    B, VAbv,VMAbv,    B,VMAbv,    B,    O,    B,    B,    B, VPre, VBlw, VAbv,
VPre, VPst,    O,VMPst,   IS,    O, VPst, VAbv, VPst, VPst, VBlw, VPst, VPst,    O,VMPst, VBlw,
O,    O,  CGJ,  CGJ,  CGJ,  CGJ,   WJ,    O,    O,    O,    B, VBlw, VBlw, VBlw, VPst,VMBlw,
VMBlw,VMAbv,CMBlw,CMBlw,CMBlw,    O,    O,    O,    O,   IS,    B,CMBlw,CMBlw,    O,VMAbv,VMAbv,
VMAbv,CMAbv,    B,    B,    O, VAbv, VAbv,    O,    O,    O,    B,    B,VMBlw,VMBlw,VMBlw,    B,
B,    B,    B,    B,CMBlw,CMBlw,CMBlw,CMBlw,    O,    O,VMPst,VMAbv,VMPst,   CS,   CS,    B,
B,    B, VAbv, VAbv, VAbv, VAbv, VBlw, VBlw, VAbv, VAbv, VAbv, VAbv,    H,    O,    O,    O,
N,    N,    N,    N,    N,    N,    N,    N,    B,    B, VAbv,    B,    B, VAbv, VAbv,    B,
O,    O,    O,    O,    O,   HN,VMAbv,VMAbv,VMPst,    B, VPst, VPre, VPst, VBlw, VBlw, VAbv,
VAbv, VPst, VPst,    H,CMBlw,    O,    O,    O, VBlw,    O,VMAbv,VMAbv,VMAbv,    B, VPre, VBlw,
VAbv, VAbv, VBlw, VAbv, VAbv,   IS,CMAbv,    O,    B,    B,    B, VPst, VPst,    B,    B,    B,
B,CMBlw, VPre, VPst, VBlw, VBlw,    H,    B,    R,    R,    O,FMBlw,CMBlw, VAbv, VBlw,    O,
VPre,VMAbv,VMAbv,    H,CMAbv,CMAbv, VAbv,CMBlw, VBlw,    O,    B,    B,    O,CMBlw,CMBlw,    B,
VPst, VPst, VPst,    O,    O, VPre,    O,    O,VMAbv,VMAbv,    B, VPst, VPre, VPst, VPst, VPst,
H,VMAbv,VMAbv,VMPst,CMBlw,    B,    O,    O,FMAbv,    B,   CS,   CS,    O,    O, VBlw, VPre,
VAbv, VPre, VPre, VPst, VPre,VMAbv,VMAbv,VMAbv,    H,CMBlw,VMAbv,VMAbv,VMPst,    H,CMBlw,    O,
O,    O, VPst,VMAbv,VMPst,    H,VMPst, VAbv, VPre, VPst, VAbv, VAbv,    H,CMBlw,    O, MBlw,
MPre, MAbv, VBlw, VBlw, VPre, VAbv, VBlw, VBlw, VBlw, VAbv, VAbv, VAbv, VAbv,VMAbv,VMPst,    H,
CMBlw,    O, VPst, VPre,    O, VPre, VPre,    O,    O,VMAbv,VMAbv, VPst,   IS,    R, MPst,    R,
MPst,CMBlw,    O,    O, VAbv, VAbv, VPst, VPst,VMPst,VMPst,    H,    B,    O,    O, VPre,    O,
O,    O,    B, VAbv, VBlw, VBlw, VAbv, VAbv, VBlw,    B,    B,    B,    B,FMBlw, VBlw,VMAbv,
VMAbv,VMAbv,VMAbv,VMPst,    R, MBlw, MBlw, MBlw, MBlw,   GB,    O,   GB,    O,   IS, VAbv, VAbv,
VAbv, VPst,    R,    R,    R,    R,    R,    R, FBlw, FBlw, FBlw, FBlw, FBlw, FBlw, FBlw, FBlw,
VMAbv,VMPst,CMAbv,   IS,    O,    O, VBlw, VBlw, VBlw,    O,    O,    O,  SUB,  SUB, VBlw, VPre,
VBlw, VAbv, VPst,VMAbv,VMAbv,    O, VAbv, VAbv, VBlw,    O,    O,    O, VAbv,    O, VAbv, VAbv,
O, VAbv,VMAbv,VMAbv,CMBlw, VAbv, VBlw,   IS,    R, MBlw, VPst, VPst, VPst,    O, VPst,VMAbv,
VMPst,   IS,    B,    B,   GB, VAbv, VBlw, VPre, VPst,    O,    H,    H,    H,    H,    H,    H,
H,    B,    O,    O,    O,CMBlw,    O, VBlw, VBlw, VBlw,    O,    O,    O,VMBlw,VMBlw,VMBlw,
VMBlw,    O,    O,CMBlw,CMBlw,    O,    B,    B,VMAbv,    O,CMAbv,CMAbv,CMAbv,CMAbv,CMAbv,CMAbv,
CMAbv,    B,
];

const hb_use_u16: [u16; 2056] =
[
0,  0,  1,  2,  0,  0,  0,  0,  0,  0,  3,  4,  0,  5,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  6,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  7,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  8,  9, 10, 11,
0,  0,  0,  0,  9, 12,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
13,  9,  9, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 17, 25,
26, 20, 21, 27, 28, 29, 30, 31, 32, 33, 21, 34, 35,  0, 17, 36,
37, 20, 21, 38, 23, 39, 17, 40, 41, 42, 43, 44, 45, 46, 30,  0,
47, 48, 21, 49, 50, 51, 17,  0, 52, 48, 21, 53, 50, 54, 17, 55,
56, 48,  9, 57, 58, 59, 17,  0, 60, 61,  9, 62, 63, 64, 30, 65,
66, 67,  9, 68, 69,  9, 70, 71, 72, 73, 74, 75, 76,  0,  0,  0,
9,  9, 77, 78, 79, 80, 81, 82, 83, 84,  0,  0,  0,  0,  0,  0,
9, 85,  9, 86,  9, 87, 88, 89,  9,  9,  9, 90, 91, 92,  2,  0,
93,  0,  9,  9,  9,  9,  9, 94, 95,  9, 96,  0,  0,  0,  0,  0,
97, 98, 99,100, 30,  9,101,102,  9,  9,103,  9,104,105,  0,  0,
9,106,  9,  9,  9,107,108,109,  2,  2,  0,  0,  0,  0,  0,  0,
110,  9,  9,111,112,  2,113,114,115,  9,116,  9,  9,  9,117,118,
9,  9,119,120,121,  0,  0,  0,  0,  0,  0,  0,  0,122,123,124,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,125,
126,127,128,  0,  0,  0,129,130,131,  0,  0,  0,  0,  0,  0,132,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,133,  0,  0,  0,
0,  0,  0,  9,  9,  9,134,135,  0,  0,  0,  0,  0,  0,  0,  0,
136,  9,137,  0,  9,  9,  9,138,139,  9,  9,140,141,  2,142,143,
9,  9,144,  9,145,146,  0,  0,147,  9,  9,148,149,  2,150, 98,
9,  9,151,152,153,  2,  9,154,  9,  9,  9,155,156,  0,157,158,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  9,  9,159,  2,
160,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,161,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,162,
0,  0,  0,  0,  0,  0,  0,163,163,164, 33,165,  0,  0,  0,  0,
166,167,  9,168, 94,  0,  0,  0,  0,  0,  0,  0, 69,  9,169,  0,
0,  0,  0,  0,  0,  0,  0,  0,  9,170,171,  0,  0,  0,  0,  0,
9,  9,172,  2,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  9,  9,173,170,  0,  0,  0,  0,
0,  0,  0,  9,174,175,  0,  9,176,  0,  0,177,178,  0,  0,  0,
179,  9,  9,180,181,182,183,184,185,  9,  9,186,187,  0,  0,  0,
188,  9,189,190,191,  9,  9,192,185,  9,  9,193,194,105,195,102,
9, 33,196,197,  0,  0,  0,  0,198,199, 94,  9,  9,200,201,  2,
202, 20, 21,203,204,205,206,207,  0,  0,  0,  0,  0,  0,  0,  0,
9,  9,  9,208,209,210,211,  0,195,  9,  9,212,213,  2,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  9,  9,214,215,216,217,  0,  0,
9,  9,  9,218,219,  2,  0,  0,  9,  9,220,221,  2,  0,  0,  0,
9,222,223,103,224,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
9,  9,225,226,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
227,228,  9,229,230,  2,  0,  0,  0,  0,231,  9,  9,232,233,  0,
234,  9,  9,235,236,237,  9,  9,238,239,  0,  0,  0,  0,  0,  0,
21,  9,214,240,  7,  9, 70, 18,  9,241, 73,242,  0,  0,  0,  0,
243,  9,  9,244,245,  2,246,  9,247,248,  2,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  9,249,
9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,
9,  9, 98,250,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  2,  0,  0,  0,
9,  9,  9,251,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
9,  9,  9,  9,252,253,254,254,255,256,  0,  0,  0,  0,257,  0,
9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,  9,258,  0,  0,
9,  9,  9,  9,  9,  9,105, 70, 94,259,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,260,  0,  0,  0,  0,  0,  0,  0,  0,
9,  9, 70,261,262,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
0,  0,  0,  0,  0,  0,  0,  0,  0,  9,263,  0,  9,  9,264,  2,
9,  9,  9,  9,265,  2,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
129,129,129,129,129,129,129,129,129,129,129,129,129,129,129,129,
160,160,160,160,160,160,160,160,160,160,160,160,160,160,160,129,
0,  0,  0,  0,  0,  0,  0,  1,  2,  2,  3,  0,  4,  0,  0,  5,
6,  0,  0,  0,  0,  7,  0,  0,  0,  0,  0,  8,  9,  0,  0,  0,
0,  0, 10,  2,  2,  2,  2,  2,  2,  2, 11, 12, 12,  0, 13, 14,
2,  2, 15,  0, 16,  2,  2,  2,  2,  2, 17, 18, 19, 20, 21, 22,
23, 24,  2,  2, 25, 10,  2,  2, 10,  2,  2,  2, 26, 27,  2, 28,
28,  2,  2,  2,  2,  2, 29,  2, 30, 10,  3, 18, 19, 31, 32, 33,
0, 34,  0, 35,  3,  0,  0, 36, 37, 27, 38, 39, 29, 40,  3, 41,
42, 43, 44, 45, 46,  0, 27, 30,  0, 10,  2,  2, 47, 48,  0,  0,
37, 27,  2, 35, 35,  2,  2,  2, 29, 27,  3, 18, 19, 49, 50, 51,
0,  0, 52, 53, 54, 27,  2, 28, 29, 27,  3, 55,  0, 56,  0, 35,
57,  0,  0,  0, 58, 27, 38, 10, 29,  3, 40, 29, 39,  9, 38, 10,
2,  2,  3, 59, 60, 61, 62, 33,  0, 34,  0,  0, 63, 64,  2, 29,
29,  2,  2,  2,  2,  2,  3, 65, 21, 66, 67, 45,  0, 68, 38,  0,
69, 27,  2, 29,  2, 27,  3, 55,  0, 70,  0, 13, 71,  0,  0,  0,
72,  2,  2, 29,  2,  2, 73, 74, 75, 76, 62, 77,  0, 34,  0, 39,
54, 27,  2,  2,  2, 38, 10,  2, 35,  2,  2, 57,  2, 38, 78, 34,
79, 80, 81, 82, 59,  0,  0,  0,  3, 38,  0,  0,  0,  0, 83,  0,
2, 84, 85, 86,  2,  2, 27,  2,  2,  2,  2,  9, 87, 88, 89, 90,
91, 92,  2, 93, 94, 94, 95, 94, 94, 94, 94, 94, 94, 94, 94, 96,
0, 97,  0,  0,  2,  2, 98, 99,100,101,102,103,  2,  2,104,105,
2,106,107,108,109,110,111,112,113,114,  2,  2,115,116,117,118,
2,  2,119,120,121,122,  0, 39,121,123,  0,  0,121,  0,  0,  0,
2,  2,  2, 29,124,  0,  0,  0,  2,125,126,127,128,129,130,131,
132,  0,  0,133,  9, 39,134,135,  2,  2,  9,  0,136,137,  2,  2,
2,  2,138,  0,139,  2,  2,  2,  2,  2,  2, 38,140,141,142,  0,
143,144,145,  0,  2,  2,  2,  3,  2,  9,  0,  0,  2,  2,  2,  0,
2,  2,146,  0,  2,  2, 38,  0,  2, 73,147,  0,  2,148,149,150,
151,141,152,153,154, 12,155,156,157,158,  2,  2,  2,159,160,161,
162,163,  2,  9,  0,  0,164,165,166,  0,  0,  0,167,  2,  2,  2,
93,168,169,170,  2,171,172,173,174,  0,  0,  0,  2,175,176,177,
178,179,  0,  0,  2,  2,  3, 27,180,181,182,181,183,181,184, 46,
0,185,186,  0,  0,  0,187,  0,  0,  0,188,189,136,  4,  0,  0,
0,  0,190,191,192,192,192,192,  0,193,  0,  0,  6,193,  0,  0,
194,  0,  0,  0,  0,  0,  0,  9,  2,  2,  0, 39,  0,  0,  0,195,
196,197, 11,  2, 98,198,  0,199,  2,  0,  0,  0,112,  2,  2,  2,
2,200,201,201,201,202,  0,  0, 12, 12, 12, 12,203,  0,  0,204,
2,205,206,207,  2,208,209,210,211,  0,  0,  0,212,  2,  2,  2,
213, 79,127,214,215,  0,  0,  0,  2,216,  2,  2,  2,  2,217,218,
219,220,  0,  0,221,  2,  2,222, 27,223,224,225,226,227,114,228,
229,  0,  0,  0,  2,  2,230,231,  0,232,  0,  0, 98,233,234,235,
236,236,236,236,  0,  0,  0,188,192,192,237,  0,  2,  2, 38,  2,
38, 35,  2,  2, 35,  2, 35,  9,238, 68,  0,239,  2, 27, 27,  2,
2,  3,240,241,  2,242, 39,  2,  3,  0,  0,  0,  0,  0, 27, 38,
2,243,  0,  0,  2,  2,244,245,  2,246,181,181,247,  9,  0,  0,
248,249,  0,  0, 29, 38,  2,  2, 27,  9, 27,  0,250,251,  2,  2,
2,  2,252,160,253,254,  0,  0,255,256,256,256,256,257,  2,  2,
258,259,  0,260,261,  2,  2,  2,262,263,264,  0,265,  0,  0,  0,
266,  2,  2,  2,  2,208,253,267,268,269,  2,  2,  0,270,  0,  0,
271,  0,  0,  0, 98,272,160,252,273,  0,274,275, 27,  2,  2,  2,
2,  2,  2, 75,252,276,  0, 58,  2, 38, 29, 35,  2,  2,  2, 35,
2,  2,  2, 11,262, 20,277,  0, 12, 27,  2, 28, 29, 27,278,279,
21,280, 32, 33,  0, 34,  0, 10,106,281, 12,194, 12,194,  0,  0,
2,282,160,253,283,284,  0,  0,  2,  2,  3,285,286,  0,  0,  0,
262,160,287,288,289,  9,  0,  0,  2,  2,  2, 98,272, 83,128,290,
291,  0,  0,  0,  0,  0,  2, 83, 75,160,263,292,245,  0,  0,  0,
2,  2, 11,293,253,294,  9,  0,  2,  2, 38,295, 79,296, 20,  0,
2, 38,  0,  0,  2,  2,  2,262,297,298,299,  0,  2, 38, 57,  2,
2, 40,  2,  2,201,300,301,302,303,  0,  0,  0,  2,  2, 10,  2,
282,160,304,305,306,307,  0,  0,308,252,309,  2,310,311,312,313,
0,314,  0,  0,308,315, 19,  2,  2,316,317,318,318,319,320, 57,
89,321,252,290,322, 94, 94, 94,323,324,  0,  0,  2, 38, 35,  2,
113,325,326,327,328,329,  0,  0,  2, 35, 29,  2,  2,  2,106,330,
50,331,  0,  0,332,333,  0,  0,334,335,  9,  0, 12,180,  0,  0,
2,  2, 38,336,337,160,160,160,160,160,160,160,160,160,  0,338,
339,  0,  0,  0,  0,  9,  0,  0,  2,  3,  0,  0,  2,  2,  3,340,
188,192,191,  0, 12,266,  2,  3,  2,  2,  3, 10,  2,  2,  2,341,
2,  2,  2, 12,  2,342,343,  0,
];

pub(crate) fn hb_use_get_category(u: u32) -> u8 {
    if u<921600 {
        hb_use_u8[466+(((hb_use_u16[992+(((hb_use_u16[((hb_use_u8[226+(((hb_use_u8[u as usize>>2>>2>>4>>4] as usize)<<4)+((u as usize>>2>>2>>4)&15usize))] as usize)<<4)+((u as usize>>2>>2)&15usize)])<<2) as usize +((u as usize>>2)&3usize))] as usize)<<2)+((u as usize)&3usize))]
    } else {
        O
    }
}
