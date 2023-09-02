use dlsite::interface::AgeCategory;

use crate::interface::BrowseQuery;

fn process_value(s: &str) -> String {
    s.replace('_', " ")
}

pub fn parse_query(query: String) -> BrowseQuery {
    let mut words = vec![];
    let mut genres = vec![];
    let mut circles = vec![];
    let mut creators = vec![];
    let mut age_category = None;

    query.split_whitespace().for_each(|keyword| {
        if let Some((k, v)) = keyword.split_once(':') {
            match k {
                "genre" => genres.push(process_value(v)),
                "circle" => circles.push(process_value(v)),
                "creator" => creators.push(process_value(v)),
                "age" => match &*v.to_lowercase() {
                    "r15" => age_category = Some(AgeCategory::R15),
                    "r-15" => age_category = Some(AgeCategory::R15),
                    "general" => age_category = Some(AgeCategory::General),
                    "adult" => age_category = Some(AgeCategory::Adult),
                    "r18" => age_category = Some(AgeCategory::Adult),
                    "r-18" => age_category = Some(AgeCategory::Adult),
                    _ => words.push(keyword.to_string()),
                },
                _ => words.push(keyword.to_string()),
            }
        } else {
            words.push(keyword.to_string())
        };
    });

    BrowseQuery {
        words,
        genres,
        circles,
        creators,
        age_category,
    }
}
