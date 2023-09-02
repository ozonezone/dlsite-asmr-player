pub mod modify;
pub mod read;
pub mod read_browse;

crate::prisma::product::include!(product_detailed {
    circle
    genres: include {
        genre
    }
    user_genres: include {
        genre
    }
    creators
});
