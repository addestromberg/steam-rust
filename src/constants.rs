
// Specific gas constant for water/vapor (IF97)
pub const R: f64 = 0.461526e-3; // in MJ/(kg·K)

pub const REGION_4_TABLE: [f64; 11] = [
    0.0,                       // Fill for readability
    0.11670521452767e4,        // n1
   -0.72421316703206e6,        // n2
   -0.17073846940092e2,        // n3
    0.12020824702470e5,        // n4
   -0.32325550322333e7,        // n5
    0.14915108613530e2,        // n6
   -0.48232657361591e4,        // n7
    0.40511340542057e6,        // n8
   -0.23855557567849e0,        // n9
    0.65017534844798e3,        // n10
];

pub struct R1Coefficient {
    pub i: i32,
    pub j: i32,
    pub n: f64,
}

pub const REGION_1_TABLE: [R1Coefficient; 35] = [
    // Dummy so index matches IF97 (1–34)
    R1Coefficient { i: 0, j: 0, n: 0.0 },

    R1Coefficient { i: 0,  j: -2,  n:  0.14632971213167e+0 },   // n1
    R1Coefficient { i: 0,  j: -1,  n: -0.84548187169114e+0 },   // n2
    R1Coefficient { i: 0,  j:  0,  n: -0.37563603672040e+1 },   // n3
    R1Coefficient { i: 0,  j:  1,  n:  0.33855169168385e+1 },   // n4   
    R1Coefficient { i: 0,  j:  2,  n: -0.95791963387872e+0 },   // n5
    R1Coefficient { i: 0,  j:  3,  n:  0.15772038513228e+0 },   // n6
    R1Coefficient { i: 0,  j:  4,  n: -0.16616417199501e-1 },   // n7
    R1Coefficient { i: 0,  j:  5,  n:  0.81214629983568e-3 },   // n8

    R1Coefficient { i: 1,  j: -9,  n:  0.28319080123804e-3 },   // n9
    R1Coefficient { i: 1,  j: -7,  n: -0.60706301565874e-3 },   // n10
    R1Coefficient { i: 1,  j: -1,  n: -0.18990068218419e-1 },   // n11
    R1Coefficient { i: 1,  j:  0,  n: -0.32529748770505e-1 },   // n12
    R1Coefficient { i: 1,  j:  1,  n: -0.21841717175414e-1 },   // n13
    R1Coefficient { i: 1,  j:  3,  n: -0.52838357969930e-4 },   // n14

    R1Coefficient { i: 2,  j: -3,  n: -0.47184321073267e-3 },   // n15
    R1Coefficient { i: 2,  j:  0,  n: -0.30001780793026e-3 },   // n16
    R1Coefficient { i: 2,  j:  1,  n:  0.47661393906987e-4 },   // n17
    R1Coefficient { i: 2,  j:  3,  n: -0.44141845330846e-5 },   // n18

    R1Coefficient { i: 2,  j: 17,  n: -0.72694996297594e-15 },  // n19

    R1Coefficient { i: 3,  j: -4,  n: -0.31679644845054e-4 },   // n20
    R1Coefficient { i: 3,  j:  0,  n: -0.28270797985312e-5 },   // n21
    R1Coefficient { i: 3,  j:  6,  n: -0.85205128120103e-9 },   // n22

    R1Coefficient { i: 4,  j: -5,  n: -0.22425281908000e-5 },   // n23
    R1Coefficient { i: 4,  j: -2,  n: -0.65171222895601e-6 },   // n24
    R1Coefficient { i: 4,  j: 10,  n: -0.14341729937924e-12 },  // n25

    R1Coefficient { i: 5,  j: -8,  n: -0.40516996860117e-6 },   // n26

    R1Coefficient { i: 8,  j: -11, n: -0.12734301741641e-8 },   // n27
    R1Coefficient { i: 8,  j: -6,  n: -0.17424871230634e-9 },   // n28

    R1Coefficient { i: 21, j: -29, n: -0.68762131295531e-18 },  // n29
    R1Coefficient { i: 23, j: -31, n:  0.14478307828521e-19 },  // n30
    R1Coefficient { i: 29, j: -38, n:  0.26335781662795e-22 },  // n31
    R1Coefficient { i: 30, j: -39, n: -0.11947622640071e-22 },  // n32
    R1Coefficient { i: 31, j: -40, n:  0.18228094581404e-23 },  // n33
    R1Coefficient { i: 32, j: -41, n: -0.93537087292458e-25 },  // n34
];
