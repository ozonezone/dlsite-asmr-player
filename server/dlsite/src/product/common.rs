use std::collections::HashMap;

use chrono::NaiveDate;
use scraper::{ElementRef, Html, Selector};
use url::Url;

use crate::{circle::Circle, genre::Genre, utils::ToParseError, DlsiteError, Result};

use super::{ajax::ProductAjax, AgeRating, Product, ProductPeople, WorkType};

pub(super) fn get_work_outline_table(html: &Html) -> HashMap<String, ElementRef> {
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

pub(super) fn parse_product(id: &str, json: ProductAjax, html: &Html) -> Result<Product> {
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

    let released_at = work_outline_table
        .get("販売日")
        .to_parse_error("No released_at found")?
        .text()
        .next()
        .to_parse_error("No released_at found")?;

    let released_at = NaiveDate::parse_from_str(released_at, "%Y年%m月%d日")
        .map_err(|_| DlsiteError::ParseError("Failed to parse released_at".to_string()))?;

    Ok(Product {
        id: id.to_string(),
        title: json.work_name,
        work_type: match &*json.work_type {
            "SOU" => WorkType::Voice,
            _ => WorkType::Unknown,
        },
        released_at,
        age_rating,
        genre,
        circle,
        price: json.price,
        rating: json.rate_average_2dp,
        rate_count: json.rate_count,
        sale_count: json.dl_count,
        review_count: json.review_count,
        images,
        people: parse_product_people(html)?,
    })
}

pub(super) fn parse_product_people(html: &Html) -> Result<ProductPeople> {
    let work_outline_table = get_work_outline_table(html);

    let author: Option<Vec<String>> = work_outline_table.get("作者").map(|element| {
        element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|element| element.text().next().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    });

    let voice_actor: Option<Vec<String>> = work_outline_table.get("声優").map(|element| {
        element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|element| element.text().next().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    });

    let scenario: Option<Vec<String>> = work_outline_table.get("シナリオ").map(|element| {
        element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|element| element.text().next().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    });

    let illustrator: Option<Vec<String>> = work_outline_table.get("イラスト").map(|element| {
        element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|element| element.text().next().map(|s| s.to_string()))
            .collect::<Vec<_>>()
    });

    Ok(ProductPeople {
        author,
        scenario,
        illustrator,
        voice_actor,
    })
}
