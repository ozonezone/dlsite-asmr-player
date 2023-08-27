//! Common used interfaces.

use serde_with::DeserializeFromStr;
use strum::{Display, EnumString};

/// Work category
#[derive(Display, EnumString, Debug, PartialEq, Clone, DeserializeFromStr)]
pub enum WorkType {
    /// アクション
    ACN,
    /// クイズ
    QIZ,
    ADV,
    RPG,
    TBL,
    DNV,
    SLN,
    TYP,
    STG,
    PZL,
    ETC,

    /// マンガ
    MNG,
    /// 劇画
    SCM,
    /// webtoon
    WBT,

    /// CG・イラスト
    ICG,

    // Novel
    /// ノベル
    NRE,
    /// 官能小説
    KSV,

    /// 動画
    MOV,

    /// ボイス・ASMR
    SOU,

    /// 音楽
    MUS,

    // Tool
    /// ツール
    TOL,
    /// 画像素材
    IMT,
    /// 音素材
    AMT,

    /// その他
    ET3,
    /// ボイスコミック
    VCM,

    #[strum(default)]
    Unknown(String),
}

/// Age category
#[derive(Display, Debug, Clone, PartialEq)]
#[strum(serialize_all = "snake_case")]
pub enum AgeCategory {
    R15,
    Adult,
    General,
}
