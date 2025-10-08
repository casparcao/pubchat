use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Brand {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub rating: Decimal,
    pub review_count: i32,
    #[sqlx(json)]
    pub platform_scores: PlatformScores,
    #[sqlx(json)]
    pub social_scores: SocialScores,
    pub is_top_brand: bool,
    #[sqlx(json)]
    pub advantages: Vec<String>,
    pub market_share: Option<Decimal>,
    #[sqlx(json)]
    pub price_range: PriceRange,
}

#[derive(Debug, Deserialize)]
pub struct CreateBrand {
    pub name: String,
    pub category: String,
    pub rating: Decimal,
    pub review_count: i32,
    pub platform_scores: i16,
    pub social_scores: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformScores {
  pub jd: i16,     // 京东评分 (0-5)
  pub tmall: i16,   // 天猫评分 (0-5)
  pub taobao: i16,  // 淘宝评分 (0-5)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocialScores {
  pub zhihu: i16,       // 知乎评分 (0-5)
  pub xiaohongshu: i16, // 小红书评分 (0-5)
  pub weibo: i16,       // 微博评分 (0-5)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceRange {
  pub min: Decimal,  // 最低价格
  pub max: Decimal,  // 最高价格
}