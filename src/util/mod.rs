pub mod res;
pub mod cache;
pub mod error;

use rand::{distr::Alphanumeric, Rng};

/// 生成指定长度的随机字符串
pub fn random_str(len: usize) -> String{
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
