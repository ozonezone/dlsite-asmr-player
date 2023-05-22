use std::collections::HashMap;

use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::{circle::Circle, genre::Genre, utils::ToParseError, DlsiteClient, DlsiteError, Result};

use super::{AgeRating, ProductPeople};

#[derive(Debug)]
pub struct ProductHtml {
    pub released_at: NaiveDate,
    pub age_rating: AgeRating,
    pub circle: Circle,
    pub images: Vec<Url>,
    pub people: ProductPeople,
    pub genre: Vec<Genre>,
}

impl DlsiteClient {
    #[async_backtrace::framed]
    pub(super) async fn get_product_html(&self, product_id: &str) -> Result<ProductHtml> {
        let path = format!("/work/=/product_id/{}", product_id);
        let html = self.get(&path).await?;
        let html = Html::parse_document(&html);

        parse_product_html(&html)
    }
}

fn get_work_outline_table(html: &Html) -> HashMap<String, ElementRef> {
    let mut map = HashMap::new();
    for element in html.select(&Selector::parse("#work_outline tr").unwrap()) {
        let th = element.select(&Selector::parse("th").unwrap()).next();
        let td = element.select(&Selector::parse("td").unwrap()).next();
        if let (Some(th), Some(td)) = (th, td) {
            let th = th.text().next();
            if let Some(th) = th {
                let th = th.trim().to_string();
                map.insert(th, td);
            }
        }
    }
    map
}

fn parse_product_html(html: &Html) -> Result<ProductHtml> {
    let circle = html
        .select(&Selector::parse("#work_maker .maker_name a").unwrap())
        .next()
        .to_parse_error("No circle found")?;
    let circle = Circle {
        name: circle
            .text()
            .next()
            .to_parse_error("No circle name found")?
            .to_string(),
        id: circle
            .value()
            .attr("href")
            .to_parse_error("No circle id found")?
            .split('/')
            .last()
            .to_parse_error("Failed to parse circle id")?
            .split('.')
            .next()
            .to_parse_error("Failed to parse circle id")?
            .to_string(),
    };

    let images: Vec<Url> = html
        .select(&Selector::parse(".work_slider img").unwrap())
        .map(|element| {
            let srcset = element
                .value()
                .attr("srcset")
                .to_parse_error("Img tag appears but no src found")?;
            format!("https:{}", srcset).parse().map_err(|e| {
                DlsiteError::ParseError(format!("Failed to parse url: {} ({})", e, srcset))
            })
        })
        .collect::<Result<_>>()?;

    // work_outline_table
    let work_outline_table = get_work_outline_table(html);

    let age_rating = work_outline_table
        .get("年齢指定")
        .to_parse_error("No age rating found")?
        .select(&Selector::parse("span").unwrap())
        .next()
        .to_parse_error("No age rating found")?
        .inner_html();
    let age_rating = match &*age_rating {
        "全年齢" => AgeRating::AllAges,
        "18禁" => AgeRating::Adult,
        _ => AgeRating::R,
    };

    let released_at = work_outline_table
        .get("販売日")
        .to_parse_error("No released_at found")?
        .text()
        .next()
        .to_parse_error("No released_at found")?;
    let released_at = NaiveDate::parse_from_str(released_at, "%Y年%m月%d日")
        .map_err(|_| DlsiteError::ParseError("Failed to parse released_at".to_string()))?;

    let genre = work_outline_table
        .get("ジャンル")
        .to_parse_error("No genre found")?
        .select(&Selector::parse("a").unwrap())
        .filter_map(|element| {
            let name = element.text().next()?.to_string();
            let mut id = None;
            let mut next = false;
            element.value().attr("href")?.split('/').for_each(|s| {
                if next {
                    id = Some(s.to_string());
                    next = false;
                }
                if s == "genre" {
                    next = true;
                }
            });
            id.map(|id| Genre { name, id })
        })
        .collect::<Vec<_>>();

    Ok(ProductHtml {
        released_at,
        age_rating,
        circle,
        images,
        people: parse_product_people(html)?,
        genre,
    })
}

pub(super) fn parse_product_people(html: &Html) -> Result<ProductPeople> {
    let work_outline_table = get_work_outline_table(html);

    macro_rules! get_people {
        ($key:literal) => {
            work_outline_table
                .get($key)
                .map(|element| {
                    element
                        .select(&Selector::parse("a").unwrap())
                        .filter_map(|element| element.text().next().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .filter(|v| !v.is_empty())
        };
    }

    Ok(ProductPeople {
        author: get_people!("作者"),
        scenario: get_people!("シナリオ"),
        illustrator: get_people!("イラスト"),
        voice_actor: get_people!("声優"),
    })
}
