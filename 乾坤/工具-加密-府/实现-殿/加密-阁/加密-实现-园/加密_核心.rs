//! 加密_核心.rs - SHA256 标准实现（不依赖 sha2 crate，手写 SHA-256）
//!
//! Round 11.5 演示：端到端真实可用（自给自足：无 sha2 依赖，标准库手写）
//! falsifiable：对空字符串 sha256 输出 = e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

use super::super::super::super::接口_殿::哈希_阁::哈希_契约_园::{哈希, SHA256};

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn 右旋(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

pub fn sha256(输入: &[u8]) -> [u8; 32] {
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let mut msg = 输入.to_vec();
    let len_bits = (msg.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&len_bits.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = 右旋(w[i - 15], 7) ^ 右旋(w[i - 15], 18) ^ (w[i - 15] >> 3);
            let s1 = 右旋(w[i - 2], 17) ^ 右旋(w[i - 2], 19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_ = h[7];
        for i in 0..64 {
            let s1 = 右旋(e, 6) ^ 右旋(e, 11) ^ 右旋(e, 25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = h_
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = 右旋(a, 2) ^ 右旋(a, 13) ^ 右旋(a, 22);
            let mj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(mj);
            h_ = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_);
    }
    let mut 结果 = [0u8; 32];
    for i in 0..8 {
        结果[i * 4..i * 4 + 4].copy_from_slice(&h[i].to_be_bytes());
    }
    结果
}

impl 哈希 for SHA256 {
    type 输出 = [u8; 32];
    fn 计算(&self, 输入: &[u8]) -> Self::输出 {
        sha256(输入)
    }
}

#[cfg(test)]
use super::super::super::super::接口_殿::校验_接口_阁::校验_契约_园::校验;

#[cfg(test)]
mod 测试 {
    use super::*;
    #[test]
    fn 测试_空字符串_sha256() {
        let h = sha256(b"");
        let 期望 = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f,
            0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b,
            0x78, 0x52, 0xb8, 0x55,
        ];
        assert_eq!(h, 期望);
    }
    #[test]
    fn 测试_abc_sha256() {
        let h = sha256(b"abc");
        let 期望 = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad,
        ];
        assert_eq!(h, 期望);
    }
    #[test]
    fn 测试_等值校验() {
        let v = crate::等值校验;
        let h = sha256(b"abc");
        assert!(v.验证(b"abc", &h));
        assert!(!v.验证(b"abd", &h));
    }
}
