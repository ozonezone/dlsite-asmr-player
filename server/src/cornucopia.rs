// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod types {
    pub mod public {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[allow(non_camel_case_types)]
        pub enum Age {
            all_ages,
            r,
            adult,
        }
        impl<'a> postgres_types::ToSql for Age {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                let s = match *self {
                    Age::all_ages => "all_ages",
                    Age::r => "r",
                    Age::adult => "adult",
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "age" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "all_ages" => true,
                            "r" => true,
                            "adult" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
            {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for Age {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<Age, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    "all_ages" => Ok(Age::all_ages),
                    "r" => Ok(Age::r),
                    "adult" => Ok(Age::adult),
                    s => Result::Err(Into::into(format!("invalid variant `{}`", s))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "age" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != 3 {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            "all_ages" => true,
                            "r" => true,
                            "adult" => true,
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}
#[allow(clippy::all, clippy::pedantic)]
#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(dead_code)]
pub mod queries {
    pub mod circle {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct UpsertCircleParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub id: T1,
            pub name: T2,
        }
        pub fn upsert_circle() -> UpsertCircleStmt {
            UpsertCircleStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO circle(id, name)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE SET name = $2",
            ))
        }
        pub struct UpsertCircleStmt(cornucopia_async::private::Stmt);
        impl UpsertCircleStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
                name: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[id, name]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpsertCircleParams<T1, T2>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpsertCircleStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpsertCircleParams<T1, T2>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.id, &params.name))
            }
        }
    }
    pub mod genre {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct UpsertGenreParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub id: T1,
            pub name: T2,
        }
        #[derive(Debug)]
        pub struct InsertProductGenreParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub product_id: T1,
            pub genre_id: T2,
        }
        #[derive(Debug)]
        pub struct UpsertProductUsergenreParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
        > {
            pub product_id: T1,
            pub genre_id: T2,
            pub count: i32,
        }
        pub fn upsert_genre() -> UpsertGenreStmt {
            UpsertGenreStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO genre(id, name)
VALUES ($1, $2)
ON CONFLICT (id) DO UPDATE SET name = $2",
            ))
        }
        pub struct UpsertGenreStmt(cornucopia_async::private::Stmt);
        impl UpsertGenreStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
                name: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[id, name]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpsertGenreParams<T1, T2>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpsertGenreStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpsertGenreParams<T1, T2>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.id, &params.name))
            }
        }
        pub fn insert_product_genre() -> InsertProductGenreStmt {
            InsertProductGenreStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO product_genre(product_id, genre_id)
VALUES ($1, $2)
ON CONFLICT (product_id, genre_id) DO NOTHING",
            ))
        }
        pub struct InsertProductGenreStmt(cornucopia_async::private::Stmt);
        impl InsertProductGenreStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                product_id: &'a T1,
                genre_id: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[product_id, genre_id]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertProductGenreParams<T1, T2>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertProductGenreStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertProductGenreParams<T1, T2>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.product_id, &params.genre_id))
            }
        }
        pub fn upsert_product_usergenre() -> UpsertProductUsergenreStmt {
            UpsertProductUsergenreStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO product_usergenre(product_id, genre_id, count)
VALUES ($1, $2, $3)
ON CONFLICT (product_id, genre_id) DO UPDATE SET count = $3",
            ))
        }
        pub struct UpsertProductUsergenreStmt(cornucopia_async::private::Stmt);
        impl UpsertProductUsergenreStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                product_id: &'a T1,
                genre_id: &'a T2,
                count: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[product_id, genre_id, count]).await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpsertProductUsergenreParams<T1, T2>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpsertProductUsergenreStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpsertProductUsergenreParams<T1, T2>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.product_id, &params.genre_id, &params.count))
            }
        }
    }
    pub mod product {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct UpsertProductParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
            T5: cornucopia_async::ArraySql<Item = T4>,
            T6: cornucopia_async::StringSql,
            T7: cornucopia_async::StringSql,
            T8: cornucopia_async::StringSql,
            T9: cornucopia_async::ArraySql<Item = T8>,
            T10: cornucopia_async::StringSql,
            T11: cornucopia_async::ArraySql<Item = T10>,
            T12: cornucopia_async::StringSql,
            T13: cornucopia_async::ArraySql<Item = T12>,
            T14: cornucopia_async::StringSql,
        > {
            pub id: T1,
            pub name: T2,
            pub description: Option<T3>,
            pub remote_image: T5,
            pub series: Option<T6>,
            pub circle_id: T7,
            pub actor: T9,
            pub author: T11,
            pub illustrator: T13,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: T14,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct GerProductAscReleasedAtParams {
            pub limit: i64,
            pub offset: i64,
        }
        pub struct StringQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> &str,
            mapper: fn(&str) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> StringQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(&str) -> R) -> StringQuery<'a, C, R, N> {
                StringQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GerProductAscReleasedAt {
            pub id: String,
            pub name: String,
            pub description: String,
            pub series: String,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: f64,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
        }
        pub struct GerProductAscReleasedAtBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: &'a str,
            pub series: &'a str,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: f64,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
        }
        impl<'a> From<GerProductAscReleasedAtBorrowed<'a>> for GerProductAscReleasedAt {
            fn from(
                GerProductAscReleasedAtBorrowed {
                    id,
                    name,
                    description,
                    series,
                    circle_id,
                    actor,
                    author,
                    illustrator,
                    price,
                    sale_count,
                    age,
                    released_at,
                    rating,
                    rating_count,
                    comment_count,
                    path,
                    remote_image,
                }: GerProductAscReleasedAtBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.into(),
                    series: series.into(),
                    circle_id: circle_id.into(),
                    actor: actor.map(|v| v.into()).collect(),
                    author: author.map(|v| v.into()).collect(),
                    illustrator: illustrator.map(|v| v.into()).collect(),
                    price,
                    sale_count,
                    age,
                    released_at,
                    rating,
                    rating_count,
                    comment_count,
                    path: path.into(),
                    remote_image: remote_image.map(|v| v.into()).collect(),
                }
            }
        }
        pub struct GerProductAscReleasedAtQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GerProductAscReleasedAtBorrowed,
            mapper: fn(GerProductAscReleasedAtBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GerProductAscReleasedAtQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GerProductAscReleasedAtBorrowed) -> R,
            ) -> GerProductAscReleasedAtQuery<'a, C, R, N> {
                GerProductAscReleasedAtQuery {
                    client: self.client,
                    params: self.params,
                    stmt: self.stmt,
                    extractor: self.extractor,
                    mapper,
                }
            }
            pub async fn one(self) -> Result<T, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                let row = self.client.query_one(stmt, &self.params).await?;
                Ok((self.mapper)((self.extractor)(&row)))
            }
            pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
                self.iter().await?.try_collect().await
            }
            pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
                let stmt = self.stmt.prepare(self.client).await?;
                Ok(self
                    .client
                    .query_opt(stmt, &self.params)
                    .await?
                    .map(|row| (self.mapper)((self.extractor)(&row))))
            }
            pub async fn iter(
                self,
            ) -> Result<
                impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'a,
                tokio_postgres::Error,
            > {
                let stmt = self.stmt.prepare(self.client).await?;
                let it = self
                    .client
                    .query_raw(stmt, cornucopia_async::private::slice_iter(&self.params))
                    .await?
                    .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                    .into_stream();
                Ok(it)
            }
        }
        pub fn product_ids() -> ProductIdsStmt {
            ProductIdsStmt(cornucopia_async::private::Stmt::new(
                "SELECT id FROM product",
            ))
        }
        pub struct ProductIdsStmt(cornucopia_async::private::Stmt);
        impl ProductIdsStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> StringQuery<'a, C, String, 0> {
                StringQuery {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn upsert_product() -> UpsertProductStmt {
            UpsertProductStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO product(id, name, description, remote_image, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path) 
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
ON CONFLICT (id) DO UPDATE SET
  name = EXCLUDED.name,
  description = EXCLUDED.description,
  series = EXCLUDED.series,
  remote_image = EXCLUDED.remote_image,
  circle_id = EXCLUDED.circle_id,
  actor = EXCLUDED.actor,
  author = EXCLUDED.author,
  illustrator = EXCLUDED.illustrator,
  price = EXCLUDED.price,
  sale_count = EXCLUDED.sale_count,
  age = EXCLUDED.age,
  released_at = EXCLUDED.released_at,
  rating = EXCLUDED.rating,
  rating_count = EXCLUDED.rating_count,
  comment_count = EXCLUDED.comment_count,
  path = EXCLUDED.path"))
        }
        pub struct UpsertProductStmt(cornucopia_async::private::Stmt);
        impl UpsertProductStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::ArraySql<Item = T4>,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
                T8: cornucopia_async::StringSql,
                T9: cornucopia_async::ArraySql<Item = T8>,
                T10: cornucopia_async::StringSql,
                T11: cornucopia_async::ArraySql<Item = T10>,
                T12: cornucopia_async::StringSql,
                T13: cornucopia_async::ArraySql<Item = T12>,
                T14: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
                name: &'a T2,
                description: &'a Option<T3>,
                remote_image: &'a T5,
                series: &'a Option<T6>,
                circle_id: &'a T7,
                actor: &'a T9,
                author: &'a T11,
                illustrator: &'a T13,
                price: &'a i32,
                sale_count: &'a i32,
                age: &'a super::super::types::public::Age,
                released_at: &'a time::Date,
                rating: &'a Option<f64>,
                rating_count: &'a i32,
                comment_count: &'a i32,
                path: &'a T14,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
                        &[
                            id,
                            name,
                            description,
                            remote_image,
                            series,
                            circle_id,
                            actor,
                            author,
                            illustrator,
                            price,
                            sale_count,
                            age,
                            released_at,
                            rating,
                            rating_count,
                            comment_count,
                            path,
                        ],
                    )
                    .await
            }
        }
        impl<
                'a,
                C: GenericClient + Send + Sync,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::ArraySql<Item = T4>,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::StringSql,
                T8: cornucopia_async::StringSql,
                T9: cornucopia_async::ArraySql<Item = T8>,
                T10: cornucopia_async::StringSql,
                T11: cornucopia_async::ArraySql<Item = T10>,
                T12: cornucopia_async::StringSql,
                T13: cornucopia_async::ArraySql<Item = T12>,
                T14: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                UpsertProductParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for UpsertProductStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a UpsertProductParams<
                    T1,
                    T2,
                    T3,
                    T4,
                    T5,
                    T6,
                    T7,
                    T8,
                    T9,
                    T10,
                    T11,
                    T12,
                    T13,
                    T14,
                >,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(
                    client,
                    &params.id,
                    &params.name,
                    &params.description,
                    &params.remote_image,
                    &params.series,
                    &params.circle_id,
                    &params.actor,
                    &params.author,
                    &params.illustrator,
                    &params.price,
                    &params.sale_count,
                    &params.age,
                    &params.released_at,
                    &params.rating,
                    &params.rating_count,
                    &params.comment_count,
                    &params.path,
                ))
            }
        }
        pub fn delete_product() -> DeleteProductStmt {
            DeleteProductStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM product WHERE id = ANY($1)",
            ))
        }
        pub struct DeleteProductStmt(cornucopia_async::private::Stmt);
        impl DeleteProductStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                ids: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[ids]).await
            }
        }
        pub fn delete_product_genre() -> DeleteProductGenreStmt {
            DeleteProductGenreStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM product_genre WHERE product_id = ANY($1)",
            ))
        }
        pub struct DeleteProductGenreStmt(cornucopia_async::private::Stmt);
        impl DeleteProductGenreStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                ids: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[ids]).await
            }
        }
        pub fn delete_product_usergenre() -> DeleteProductUsergenreStmt {
            DeleteProductUsergenreStmt(cornucopia_async::private::Stmt::new(
                "DELETE FROM product_usergenre WHERE product_id = ANY($1)",
            ))
        }
        pub struct DeleteProductUsergenreStmt(cornucopia_async::private::Stmt);
        impl DeleteProductUsergenreStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                ids: &'a T2,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[ids]).await
            }
        }
        pub fn ger_product_asc_released_at() -> GerProductAscReleasedAtStmt {
            GerProductAscReleasedAtStmt(cornucopia_async::private::Stmt::new(
                "SELECT * FROM product
ORDER BY released_at ASC
LIMIT $1
OFFSET $2",
            ))
        }
        pub struct GerProductAscReleasedAtStmt(cornucopia_async::private::Stmt);
        impl GerProductAscReleasedAtStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GerProductAscReleasedAtQuery<'a, C, GerProductAscReleasedAt, 2> {
                GerProductAscReleasedAtQuery {
                    client,
                    params: [limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GerProductAscReleasedAtBorrowed {
                        id: row.get(0),
                        name: row.get(1),
                        description: row.get(2),
                        series: row.get(3),
                        circle_id: row.get(4),
                        actor: row.get(5),
                        author: row.get(6),
                        illustrator: row.get(7),
                        price: row.get(8),
                        sale_count: row.get(9),
                        age: row.get(10),
                        released_at: row.get(11),
                        rating: row.get(12),
                        rating_count: row.get(13),
                        comment_count: row.get(14),
                        path: row.get(15),
                        remote_image: row.get(16),
                    },
                    mapper: |it| <GerProductAscReleasedAt>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                GerProductAscReleasedAtParams,
                GerProductAscReleasedAtQuery<'a, C, GerProductAscReleasedAt, 2>,
                C,
            > for GerProductAscReleasedAtStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GerProductAscReleasedAtParams,
            ) -> GerProductAscReleasedAtQuery<'a, C, GerProductAscReleasedAt, 2> {
                self.bind(client, &params.limit, &params.offset)
            }
        }
    }
}
