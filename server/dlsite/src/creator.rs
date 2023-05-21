use crate::search::SearchProductEntry;

pub struct Creator {
    pub name: String,
}

impl Creator {
    pub fn search_works(&self) -> Vec<SearchProductEntry> {
        unimplemented!()
    }
}
