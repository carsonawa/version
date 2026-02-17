//! # version 版本号工具库
//! 此工具库提供了处理版本号并提供版本号比较的工具函数
//!
//! ## 快速开始
//! ```
//! use version::Version;
//! let version_old = Version::build_string("1.0.0").unwrap();
//! let version_new = Version::build_string("2.1.0").unwrap();
//!
//! if version_old.is_newer(&version_new) {
//!     println!("{} 是新版本", version_new.to_string());
//! } else {
//!     panic!("版本号判断错误")
//! }
//! ```

use std::num::ParseIntError;
use thiserror::Error;

///
/// 表示一个版本号的结构体
///
/// 包含了 major(主版本号) minor(次版本号) patch(补丁版本号) 和 可选的suffix(版本后缀)
///
/// ```
/// use version::Version;
///
/// // 基于现有字符串
/// let version_s = Version::build_string("1.0.0").unwrap();
/// println!("{}", version_s.to_string())
/// ```
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
    suffix: String,
}

///
/// 表示在解析操作期间可能发生的错误。
///
/// 这个枚举包含两种变体:
/// - `IntError`: 在解析整数时发生错误。它包装了标准的`ParseIntError`，以提供更多上下文特定的错误信息。
/// - `LengthError`: 当拆分操作的长度出现问题时返回的错误，表示输入或输出不符合预期的长度要求。
///
#[derive(Error, Debug)]
pub enum ParseError {

    #[error("解析数字失败: {0}")]
    IntError(#[from] ParseIntError),

    #[error("分割长度错误")]
    LengthError
}

impl Version {


    /// 通过字符串构建 Version 结构体对象
    ///
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
    /// Err(ParseError) - 解析错误
    ///
    /// # 示例
    /// ```
    /// use version::Version;
    ///
    /// let v = Version::build_string("1.0.0").unwrap();                // 主.副.补丁
    /// let v_suffix = Version::build_string("2.0.0-beta").unwrap();    // 主.副.补丁-后缀
    /// let v_major_minor = Version::build_string("1.2").unwrap();      // 主.副
    /// ```
    pub fn build_string(version: &str) -> Result<Version, ParseError> {
        // 分割版本号和后缀
        let version_suffix: Vec<&str> = version.split("-").collect();
        // 分割版本号
        let major_minor_patch: Vec<&str> = version_suffix[0].split(".").collect();
        let suffix : String;

        // 检查分割长度是否满足要求
        if major_minor_patch.len() < 2 || major_minor_patch.len() > 3 {
            // 如果不满足则报错
            return Err(ParseError::LengthError)
        }

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

    /// 比较传入的版本号是否为最新版本
    ///
    /// # 参数
    /// - `other` - 传入要比较的版本号的地址
    ///
    /// # 返回值
    /// `true` - 当传入版本号为最新时返回
    /// `false` - 其他情况返回
    ///
    /// # 注意
    /// 判断是否为新版本逻辑如下
    /// 1. 判断主版本号、副版本号、补丁版本号
    /// 2. 判断两者之一是否有后缀，有后缀的版本号默认被认为是新版本
    ///
    /// # 示例
    /// ```
    /// use version::Version;
    ///
    /// let v_old = Version::build_string("1.0.0").unwrap();
    /// let v_new = Version::build_string("2.0.0").unwrap();
    ///
    /// assert_eq!(v_old.is_newer(&v_new), true)
    /// ```
    pub fn is_newer(&self, other: &Version) -> bool {
        self.major < other.major // 判断大版本
            || (self.major == other.major && self.minor < other.minor // 判断小版本
            || (self.major == other.major && self.minor == other.minor && self.patch < other.patch // 判断补丁版本
            || (self.major == other.major && self.minor == other.minor && self.patch == other.patch &&
            (self.suffix.trim().is_empty() && !other.suffix.trim().is_empty()) // 判断后缀
        )))
    }

    /// 将版本号转化为字符串。
    ///
    /// # 返回值
    /// 以`[major].[minor].[patch]-[suffix]`或`[major].[minor].[patch]`形式输出
    ///
    ///
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
        assert_eq!(v_old.is_newer(&v_new), true);

        let v_new = Version::build_string("2.0.0").unwrap();

        // 断言比较
        assert_eq!(v_old.is_newer(&v_new), true);
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
        assert_eq!(v_has_suffix.is_newer(&v_has_not_suffix), true)
    }

    /// 测试错误的版本号数字
    #[test]
    #[should_panic]
    fn  test_error_number() {
        let _ = Version::build_string("homo.114514.1919810").unwrap();
    }

    /// 测试错误长度
    #[test]
    #[should_panic]
    fn test_error_length() {
        let _ = Version::build_string("1-beta").unwrap();
    }
}