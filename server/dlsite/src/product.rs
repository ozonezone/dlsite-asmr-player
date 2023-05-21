use crate::{circle::Circle, genre::Genre, DlsiteClient, Result};
use chrono::NaiveDate;
use scraper::Html;
use url::Url;

use self::{ajax::parse_product_json, common::parse_product};

mod ajax;
mod common;

#[derive(Debug, PartialEq, Eq)]
pub enum AgeRating {
    AllAges,
    R,
    Adult,
}

pub type Id = String;

#[derive(Debug)]
pub struct Product {
    pub id: String,
    pub title: String,
    pub work_type: WorkType,
    pub released_at: NaiveDate,
    pub age_rating: AgeRating,
    pub genre: Vec<Genre>,
    pub circle: Circle,
    pub price: u64,
    pub sale_count: u64,
    pub review_count: Option<u64>,
    pub rating: Option<f64>,
    pub rate_count: Option<u64>,
    pub images: Vec<Url>,
    pub people: ProductPeople,
}

#[derive(Debug)]
pub struct ProductPeople {
    pub author: Option<Vec<String>>,
    pub scenario: Option<Vec<String>>,
    pub illustrator: Option<Vec<String>>,
    pub voice_actor: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WorkType {
    Voice,
    Unknown,
}

impl DlsiteClient {
    pub async fn get_product(&self, product_id: &str) -> Result<Product> {
        let text = self
            .get(&format!("/work/=/product_id/{}", product_id))
            .await?;
        let html = Html::parse_document(&text);

        let json_str = self
            .get(&format!("/product/info/ajax?product_id={}", product_id))
            .await?;
        let product_ajax = parse_product_json(&json_str, product_id)?;

        parse_product(product_id, product_ajax, &html)
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::{
        circle::Circle,
        genre::Genre,
        product::{AgeRating, WorkType},
        DlsiteClient,
    };

    #[tokio::test]
    async fn get_product_1() {
        let client = DlsiteClient::default();
        let res = client.get_product("RJ403038").await.unwrap();
        assert_eq!(res.id, "RJ403038".to_string());
        assert_eq!(
            res.title,
            "【ブルーアーカイブ】ユウカASMR～頑張るあなたのすぐそばに～".to_string()
        );
        assert_eq!(
            res.circle,
            Circle {
                name: "Yostar".to_string(),
                id: "RG62982".to_string()
            }
        );
        assert_eq!(res.work_type, WorkType::Voice);
        assert_eq!(
            res.released_at,
            NaiveDate::from_ymd_opt(2022, 7, 17).unwrap()
        );
        assert_eq!(res.age_rating, AgeRating::AllAges);
        assert_eq!(res.people.voice_actor, Some(vec!["春花らん".to_string()]));
        assert!(res.sale_count > 50000);
        assert!(res.genre.contains(&Genre {
            name: "ASMR".to_string(),
            id: "497".to_string()
        }));

        dbg!(&res);
    }

    #[tokio::test]
    async fn get_product_2() {
        let client = DlsiteClient::default();
        let res = client.get_product("RJ01017217").await.unwrap();
        assert_eq!(res.id, "RJ01017217".to_string());
        assert_eq!(
            res.title,
            "【イヤーキャンドル】道草屋-なつな3-たぬさんこんにちは【ずぶ濡れシャンプー】"
                .to_string()
        );
        assert_eq!(
            res.circle,
            Circle {
                name: "桃色CODE".to_string(),
                id: "RG24350".to_string()
            }
        );
        assert_eq!(res.work_type, WorkType::Voice);
        assert_eq!(
            res.released_at,
            NaiveDate::from_ymd_opt(2023, 1, 21).unwrap()
        );
        assert_eq!(res.age_rating, AgeRating::Adult);
        assert_eq!(
            res.people.voice_actor,
            Some(vec!["丹羽うさぎ".to_string(), "藤堂れんげ".to_string()])
        );
        assert_eq!(res.people.author, Some(vec!["桃鳥".to_string()]));
        assert!(res.sale_count > 10000);
        assert!(res.genre.contains(&Genre {
            name: "ASMR".to_string(),
            id: "497".to_string()
        }));

        dbg!(&res);
    }
}
