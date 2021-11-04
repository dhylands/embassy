#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

#[path = "../example_common.rs"]
mod example_common;
use defmt::*;
use embassy::executor::Spawner;
use embassy::time::{Duration, Timer};
use embassy_nrf::pwm::{Prescaler, SimplePwm};
use embassy_nrf::Peripherals;

// for i in range(1024): print(int((math.sin(i/512*math.pi)*0.4+0.5)**2*32767), ', ', end='')
static DUTY: [u16; 1024] = [
    8191, 8272, 8353, 8434, 8516, 8598, 8681, 8764, 8847, 8931, 9015, 9099, 9184, 9269, 9354, 9440,
    9526, 9613, 9700, 9787, 9874, 9962, 10050, 10139, 10227, 10316, 10406, 10495, 10585, 10675,
    10766, 10857, 10948, 11039, 11131, 11223, 11315, 11407, 11500, 11592, 11685, 11779, 11872,
    11966, 12060, 12154, 12248, 12343, 12438, 12533, 12628, 12723, 12818, 12914, 13010, 13106,
    13202, 13298, 13394, 13491, 13587, 13684, 13781, 13878, 13975, 14072, 14169, 14266, 14364,
    14461, 14558, 14656, 14754, 14851, 14949, 15046, 15144, 15242, 15339, 15437, 15535, 15632,
    15730, 15828, 15925, 16023, 16120, 16218, 16315, 16412, 16510, 16607, 16704, 16801, 16898,
    16995, 17091, 17188, 17284, 17380, 17477, 17572, 17668, 17764, 17859, 17955, 18050, 18145,
    18239, 18334, 18428, 18522, 18616, 18710, 18803, 18896, 18989, 19082, 19174, 19266, 19358,
    19449, 19540, 19631, 19722, 19812, 19902, 19991, 20081, 20169, 20258, 20346, 20434, 20521,
    20608, 20695, 20781, 20867, 20952, 21037, 21122, 21206, 21290, 21373, 21456, 21538, 21620,
    21701, 21782, 21863, 21943, 22022, 22101, 22179, 22257, 22335, 22412, 22488, 22564, 22639,
    22714, 22788, 22861, 22934, 23007, 23079, 23150, 23220, 23290, 23360, 23429, 23497, 23564,
    23631, 23698, 23763, 23828, 23892, 23956, 24019, 24081, 24143, 24204, 24264, 24324, 24383,
    24441, 24499, 24555, 24611, 24667, 24721, 24775, 24828, 24881, 24933, 24983, 25034, 25083,
    25132, 25180, 25227, 25273, 25319, 25363, 25407, 25451, 25493, 25535, 25575, 25615, 25655,
    25693, 25731, 25767, 25803, 25838, 25873, 25906, 25939, 25971, 26002, 26032, 26061, 26089,
    26117, 26144, 26170, 26195, 26219, 26242, 26264, 26286, 26307, 26327, 26346, 26364, 26381,
    26397, 26413, 26427, 26441, 26454, 26466, 26477, 26487, 26496, 26505, 26512, 26519, 26525,
    26530, 26534, 26537, 26539, 26540, 26541, 26540, 26539, 26537, 26534, 26530, 26525, 26519,
    26512, 26505, 26496, 26487, 26477, 26466, 26454, 26441, 26427, 26413, 26397, 26381, 26364,
    26346, 26327, 26307, 26286, 26264, 26242, 26219, 26195, 26170, 26144, 26117, 26089, 26061,
    26032, 26002, 25971, 25939, 25906, 25873, 25838, 25803, 25767, 25731, 25693, 25655, 25615,
    25575, 25535, 25493, 25451, 25407, 25363, 25319, 25273, 25227, 25180, 25132, 25083, 25034,
    24983, 24933, 24881, 24828, 24775, 24721, 24667, 24611, 24555, 24499, 24441, 24383, 24324,
    24264, 24204, 24143, 24081, 24019, 23956, 23892, 23828, 23763, 23698, 23631, 23564, 23497,
    23429, 23360, 23290, 23220, 23150, 23079, 23007, 22934, 22861, 22788, 22714, 22639, 22564,
    22488, 22412, 22335, 22257, 22179, 22101, 22022, 21943, 21863, 21782, 21701, 21620, 21538,
    21456, 21373, 21290, 21206, 21122, 21037, 20952, 20867, 20781, 20695, 20608, 20521, 20434,
    20346, 20258, 20169, 20081, 19991, 19902, 19812, 19722, 19631, 19540, 19449, 19358, 19266,
    19174, 19082, 18989, 18896, 18803, 18710, 18616, 18522, 18428, 18334, 18239, 18145, 18050,
    17955, 17859, 17764, 17668, 17572, 17477, 17380, 17284, 17188, 17091, 16995, 16898, 16801,
    16704, 16607, 16510, 16412, 16315, 16218, 16120, 16023, 15925, 15828, 15730, 15632, 15535,
    15437, 15339, 15242, 15144, 15046, 14949, 14851, 14754, 14656, 14558, 14461, 14364, 14266,
    14169, 14072, 13975, 13878, 13781, 13684, 13587, 13491, 13394, 13298, 13202, 13106, 13010,
    12914, 12818, 12723, 12628, 12533, 12438, 12343, 12248, 12154, 12060, 11966, 11872, 11779,
    11685, 11592, 11500, 11407, 11315, 11223, 11131, 11039, 10948, 10857, 10766, 10675, 10585,
    10495, 10406, 10316, 10227, 10139, 10050, 9962, 9874, 9787, 9700, 9613, 9526, 9440, 9354, 9269,
    9184, 9099, 9015, 8931, 8847, 8764, 8681, 8598, 8516, 8434, 8353, 8272, 8191, 8111, 8031, 7952,
    7873, 7794, 7716, 7638, 7561, 7484, 7407, 7331, 7255, 7180, 7105, 7031, 6957, 6883, 6810, 6738,
    6665, 6594, 6522, 6451, 6381, 6311, 6241, 6172, 6104, 6036, 5968, 5901, 5834, 5767, 5702, 5636,
    5571, 5507, 5443, 5379, 5316, 5253, 5191, 5130, 5068, 5008, 4947, 4888, 4828, 4769, 4711, 4653,
    4596, 4539, 4482, 4426, 4371, 4316, 4261, 4207, 4153, 4100, 4047, 3995, 3943, 3892, 3841, 3791,
    3741, 3691, 3642, 3594, 3546, 3498, 3451, 3404, 3358, 3312, 3267, 3222, 3178, 3134, 3090, 3047,
    3005, 2962, 2921, 2879, 2839, 2798, 2758, 2719, 2680, 2641, 2603, 2565, 2528, 2491, 2454, 2418,
    2382, 2347, 2312, 2278, 2244, 2210, 2177, 2144, 2112, 2080, 2048, 2017, 1986, 1956, 1926, 1896,
    1867, 1838, 1810, 1781, 1754, 1726, 1699, 1673, 1646, 1620, 1595, 1570, 1545, 1520, 1496, 1472,
    1449, 1426, 1403, 1380, 1358, 1336, 1315, 1294, 1273, 1252, 1232, 1212, 1192, 1173, 1154, 1135,
    1117, 1099, 1081, 1063, 1046, 1029, 1012, 996, 980, 964, 948, 933, 918, 903, 888, 874, 860,
    846, 833, 819, 806, 793, 781, 768, 756, 744, 733, 721, 710, 699, 688, 677, 667, 657, 647, 637,
    627, 618, 609, 599, 591, 582, 574, 565, 557, 549, 541, 534, 526, 519, 512, 505, 498, 492, 485,
    479, 473, 467, 461, 455, 450, 444, 439, 434, 429, 424, 419, 415, 410, 406, 402, 398, 394, 390,
    386, 383, 379, 376, 373, 370, 367, 364, 361, 359, 356, 354, 351, 349, 347, 345, 343, 342, 340,
    338, 337, 336, 334, 333, 332, 331, 330, 330, 329, 328, 328, 328, 327, 327, 327, 327, 327, 328,
    328, 328, 329, 330, 330, 331, 332, 333, 334, 336, 337, 338, 340, 342, 343, 345, 347, 349, 351,
    354, 356, 359, 361, 364, 367, 370, 373, 376, 379, 383, 386, 390, 394, 398, 402, 406, 410, 415,
    419, 424, 429, 434, 439, 444, 450, 455, 461, 467, 473, 479, 485, 492, 498, 505, 512, 519, 526,
    534, 541, 549, 557, 565, 574, 582, 591, 599, 609, 618, 627, 637, 647, 657, 667, 677, 688, 699,
    710, 721, 733, 744, 756, 768, 781, 793, 806, 819, 833, 846, 860, 874, 888, 903, 918, 933, 948,
    964, 980, 996, 1012, 1029, 1046, 1063, 1081, 1099, 1117, 1135, 1154, 1173, 1192, 1212, 1232,
    1252, 1273, 1294, 1315, 1336, 1358, 1380, 1403, 1426, 1449, 1472, 1496, 1520, 1545, 1570, 1595,
    1620, 1646, 1673, 1699, 1726, 1754, 1781, 1810, 1838, 1867, 1896, 1926, 1956, 1986, 2017, 2048,
    2080, 2112, 2144, 2177, 2210, 2244, 2278, 2312, 2347, 2382, 2418, 2454, 2491, 2528, 2565, 2603,
    2641, 2680, 2719, 2758, 2798, 2839, 2879, 2921, 2962, 3005, 3047, 3090, 3134, 3178, 3222, 3267,
    3312, 3358, 3404, 3451, 3498, 3546, 3594, 3642, 3691, 3741, 3791, 3841, 3892, 3943, 3995, 4047,
    4100, 4153, 4207, 4261, 4316, 4371, 4426, 4482, 4539, 4596, 4653, 4711, 4769, 4828, 4888, 4947,
    5008, 5068, 5130, 5191, 5253, 5316, 5379, 5443, 5507, 5571, 5636, 5702, 5767, 5834, 5901, 5968,
    6036, 6104, 6172, 6241, 6311, 6381, 6451, 6522, 6594, 6665, 6738, 6810, 6883, 6957, 7031, 7105,
    7180, 7255, 7331, 7407, 7484, 7561, 7638, 7716, 7794, 7873, 7952, 8031, 8111,
];

#[embassy::main]
async fn main(_spawner: Spawner, p: Peripherals) {
    let pwm = SimplePwm::new(p.PWM0, p.P0_13, p.P0_14, p.P0_16, p.P0_15);
    pwm.set_prescaler(Prescaler::Div1);
    pwm.set_max_duty(32767);
    info!("pwm initialized!");

    let mut i = 0;
    loop {
        i += 1;
        pwm.set_duty(0, DUTY[i % 1024]);
        pwm.set_duty(1, DUTY[(i + 256) % 1024]);
        pwm.set_duty(2, DUTY[(i + 512) % 1024]);
        pwm.set_duty(3, DUTY[(i + 768) % 1024]);
        Timer::after(Duration::from_millis(3)).await;
    }
}
