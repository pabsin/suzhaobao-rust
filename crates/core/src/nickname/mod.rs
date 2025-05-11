mod names;

use rand::{Rng, rng, rngs::ThreadRng};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    sync::LazyLock,
};

const THREAD_RNG: LazyLock<RefCell<ThreadRng>> = LazyLock::new(|| RefCell::new(rng()));

/// 性别枚举
#[derive(Copy, Clone, Debug)]
pub enum Gender {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 间性
    Intersex,
}

/// 生成时可能发生的错误信息
#[derive(Debug)]
pub struct GenError(String);

impl Display for GenError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 生成一个随机名字。
///
/// 根据指定的性别和名字长度生成一个名字。如果名字长度为0，则不限制长度。
///
/// # 参数
///
/// * `gender` - 指定的性别，可以是`Gender::Male`、`Gender::Female`或`Gender::Intersex`。
/// * `LEN` - 名字的长度，可以是任意正整数或0。
///
/// # 返回值
///
/// 返回一个`Result`类型，成功时包含一个静态字符串切片，表示生成的名字；失败时包含一个`GenError`，表示生成名字时可能出现的错误。
///
/// # 示例
///
/// ```
/// use random_nickname2::{ random, Gender };
///
/// match random::<4>(Gender::Female) {
///     Ok(name) => println!("生成的名字是: {}", name),
///     Err(e) => println!("生成名字时出错: {}", e),
/// }
/// ```
///
/// # 错误处理
///
/// 如果没有符合条件的名字，将返回一个`GenError`，其中包含错误信息。
///
pub fn random<const LEN: usize>(gender: Gender) -> Result<&'static str, GenError> {
    use names::NAMES;
    let data = NAMES
        .iter()
        .filter_map(move |(name, gender2)| match (gender, gender2) {
            (Gender::Male, 0) | (Gender::Female, 1) | (Gender::Intersex, _)
                if LEN > 0 && LEN == name.chars().collect::<Vec<_>>().len() || LEN <= 0 =>
            {
                Some(name)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if data.is_empty() {
        return Err(GenError(format!("No result for {:?}: len {}", gender, LEN)));
    }
    let index = THREAD_RNG.borrow_mut().random_range(0..data.len());
    Ok(data[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(random::<1>(Gender::Female).is_ok());
        assert!(random::<2>(Gender::Female).is_ok());
        assert!(random::<3>(Gender::Female).is_ok());
        assert!(random::<4>(Gender::Female).is_ok());
        assert!(random::<1>(Gender::Male).is_ok());
        assert!(random::<2>(Gender::Male).is_ok());
        assert!(random::<3>(Gender::Male).is_ok());
        assert!(random::<4>(Gender::Male).is_ok());
    }
}
