use crate::{circle::Circle, genre::Genre, DlsiteClient, Result};
use chrono::NaiveDate;
use url::Url;

pub mod ajax;
pub mod html;
pub mod review;
#[cfg(test)]
mod tests;

/// The age rating of a product on DLsite.
#[derive(Debug, PartialEq, Eq)]
pub enum AgeRating {
    AllAges,
    R,
    Adult,
}

/// A product on DLsite.
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
    pub series: Option<String>,
    pub sale_count: u64,
    pub review_count: Option<u64>,
    pub rating: Option<f64>,
    pub rate_count: Option<u64>,
    pub images: Vec<Url>,
    pub people: ProductPeople,
    pub reviewer_genre: Vec<(Genre, u32)>,
}

/// People who contributed to a product on DLsite.
#[derive(Debug)]
pub struct ProductPeople {
    pub author: Option<Vec<String>>,
    pub scenario: Option<Vec<String>>,
    pub illustrator: Option<Vec<String>>,
    pub voice_actor: Option<Vec<String>>,
}

/// The type of a product on DLsite. Currently, only voice products are supported.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum WorkType {
    Voice,
    Unknown,
}

impl DlsiteClient {
    /// Get information about a product (also called "work").
    /// This function will make 3 requests to DLsite: one to get the HTML page, one to get the AJAX data and one to get the review data.
    /// Especially, review data can be used as independent information.
    ///
    /// # Arguments
    /// * `product_id` - The product ID to get information about. Example: `RJ123456`.
    /// NOTE: This must be capitalized.
    ///
    /// # Example
    /// ```
    /// use dlsite::{DlsiteClient, Product};
    /// use tokio;
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = DlsiteClient::new().unwrap();
    ///     let product = client.get_product("RJ123456").await.unwrap();
    ///     println!("{:#?}", product);
    /// }
    /// ```
    pub async fn get_product(&self, product_id: &str) -> Result<Product> {
        let (html_data, ajax_data, review_data) = tokio::try_join!(
            self.get_product_html(product_id),
            self.get_product_ajax(product_id),
            self.get_product_review(product_id, 6, 1, true, review::ReviewSortOrder::New)
        )?;

        Ok(Product {
            id: product_id.to_string(),
            title: ajax_data.work_name,
            work_type: ajax_data.work_type,
            released_at: html_data.released_at,
            age_rating: html_data.age_rating,
            genre: html_data.genre,
            series: html_data.series,
            circle: html_data.circle,
            price: ajax_data.price,
            rating: ajax_data.rate_average_2dp,
            rate_count: ajax_data.rate_count,
            sale_count: ajax_data.dl_count,
            review_count: ajax_data.review_count,
            images: html_data.images,
            people: html_data.people,
            reviewer_genre: review_data.reviewer_genre_list.unwrap_or_default(),
        })
    }
}
