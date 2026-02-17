//! # version 版本号工具库
//! 此工具库提供了处理版本号并提供版本号比较的工具函数
//!
//! ## 快速开始
//! ```
//! let version_old = Version::build_string("1.0.0").unwrap();
//! let version_new = Version::build_string("1.1.0").unwrap();
//!
//! if version_old.is_newer(version_new) {
//!     println!("{} 是新版本新", version_new.to_string());
//! }
//! ```

use std::num::ParseIntError;

///
/// 表示一个版本号的结构体
///
/// 包含了 major(主版本号) minor(次版本号) patch(补丁版本号) 和 可选的suffix(版本后缀)
///
/// ```
/// // 传统的构建方式
/// let version = Version {
///     major: 1,
///     minor: 0,
///     patch: 0,
///     suffix: String::from("")
/// }
///
/// // 基于现有字符串
/// let version_s = Version::build_string("1.0.0").unwrap();
/// ```
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    suffix: String,
}

impl Version {

    /// 通过字符串构建 Version 结构体对象
    /// # 参数
    /// `version` -
    /// 字符串必须遵循此结构
    /// ```"XX.XX.XX-YY"```
    /// 或者
    /// ```"XX.XX-YY"```
    /// 其中 YY 部分可缺省，此时的形式为
    /// ```"XX.XX.XX"```
    ///
    /// # 返回值
    /// Ok(Version) - 版本号对象
    /// Err(ParseIntError) - 解析错误
    pub fn build_string(version: &str) -> Result<Version, ParseIntError> {
        // 分割版本号和后缀
        let version_suffix: Vec<&str> = version.split("-").collect();
        // 分割版本号
        let major_minor_patch: Vec<&str> = version_suffix[0].split(".").collect();
        let suffix : String;

        // 检测版本号是否存在后缀
        if version_suffix.len() == 1 {
            suffix = "".to_string();
        } else {
            suffix = version_suffix[1].to_string(); // 后缀类型
        }

        // 解析版本号为整数
        // 错误将传递上层
        let major = major_minor_patch[0].parse::<u8>()?;
        let minor = major_minor_patch[1].parse::<u8>()?;
        // 对缺失补丁版本号特殊处理
        let patch : u8;
        if major_minor_patch.len() > 2 {
            patch = major_minor_patch[2].parse::<u8>()?;
        } else {
            patch = 0;
        }

        // 返回Version对象
        Ok(Version {
            major,
            minor,
            patch,
            suffix,
        })
    }

    /// 比较版本
    ///
    pub fn is_newer(&self, other: Version) -> bool {
        self.major < other.major // 判断大版本
            || (self.major == other.major && self.minor < other.minor // 判断小版本
            || (self.major == other.major && self.minor == other.minor && self.patch < other.patch // 判断补丁版本
            || (self.major == other.major && self.minor == other.minor && self.patch == other.patch &&
            (self.suffix.trim().is_empty() && !other.suffix.trim().is_empty()) // 判断后缀
        )))
    }

    pub fn to_string(&self) -> String {
        if self.suffix.trim().is_empty() {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}.{}-{}", self.major, self.minor, self.patch, self.suffix)
        }
    }
}

mod tests {
    use crate::{Version};

    /// 测试版本比较
    #[test]
    fn test_newer() {
        let v_old = Version::build_string("1.0.0").unwrap();
        let v_new = Version::build_string("1.1.0").unwrap();

        // 断言比较
        assert_eq!(v_old.is_newer(v_new), true)
    }

    /// 测试版本对象创建
    #[test]
    fn test_build() {
        let v_not_suffix = Version::build_string("1.0.0").unwrap();
        let v_has_suffix = Version::build_string("1.0.0-beta").unwrap();
        let v_less_patch = Version::build_string("1.0-beta").unwrap();

        println!("v_not_suffix: {}\nv_has_suffix: {}\nv_less_patch: {}\n",
                 v_not_suffix.to_string(), v_has_suffix.to_string(), v_less_patch.to_string()
        )
    }

    /// 测试版本无后缀优先于后缀
    #[test]
    fn test_suffix() {
        let v_has_suffix = Version::build_string("1.0.0").unwrap();
        let v_has_not_suffix = Version::build_string("1.0.0-beta").unwrap();

        // 断言比较
        assert_eq!(v_has_suffix.is_newer(v_has_not_suffix), true)
    }

    /// 测试错误的版本号数字
    #[test]
    #[should_panic]
    fn  test_error_number() {
        let _ = Version::build_string("homo.114514.1919810").unwrap();
    }
}