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
        #[derive(Debug)]
        pub struct GetCircleProductReleasedAtAscParams<T1: cornucopia_async::StringSql> {
            pub circle_id: T1,
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Debug)]
        pub struct GetCircleProductReleasedAtDescParams<T1: cornucopia_async::StringSql> {
            pub circle_id: T1,
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Debug)]
        pub struct GetCircleProductNameAscParams<T1: cornucopia_async::StringSql> {
            pub circle_id: T1,
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Debug)]
        pub struct GetCircleProductNameDescParams<T1: cornucopia_async::StringSql> {
            pub circle_id: T1,
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCircleProductReleasedAtAsc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetCircleProductReleasedAtAscBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetCircleProductReleasedAtAscBorrowed<'a>> for GetCircleProductReleasedAtAsc {
            fn from(
                GetCircleProductReleasedAtAscBorrowed {
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
                    circle_name,
                }: GetCircleProductReleasedAtAscBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetCircleProductReleasedAtAscQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetCircleProductReleasedAtAscBorrowed,
            mapper: fn(GetCircleProductReleasedAtAscBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetCircleProductReleasedAtAscQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetCircleProductReleasedAtAscBorrowed) -> R,
            ) -> GetCircleProductReleasedAtAscQuery<'a, C, R, N> {
                GetCircleProductReleasedAtAscQuery {
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
        pub struct GetCircleProductReleasedAtDesc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetCircleProductReleasedAtDescBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetCircleProductReleasedAtDescBorrowed<'a>> for GetCircleProductReleasedAtDesc {
            fn from(
                GetCircleProductReleasedAtDescBorrowed {
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
                    circle_name,
                }: GetCircleProductReleasedAtDescBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetCircleProductReleasedAtDescQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetCircleProductReleasedAtDescBorrowed,
            mapper: fn(GetCircleProductReleasedAtDescBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetCircleProductReleasedAtDescQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetCircleProductReleasedAtDescBorrowed) -> R,
            ) -> GetCircleProductReleasedAtDescQuery<'a, C, R, N> {
                GetCircleProductReleasedAtDescQuery {
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
        pub struct GetCircleProductNameAsc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetCircleProductNameAscBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetCircleProductNameAscBorrowed<'a>> for GetCircleProductNameAsc {
            fn from(
                GetCircleProductNameAscBorrowed {
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
                    circle_name,
                }: GetCircleProductNameAscBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetCircleProductNameAscQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetCircleProductNameAscBorrowed,
            mapper: fn(GetCircleProductNameAscBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetCircleProductNameAscQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetCircleProductNameAscBorrowed) -> R,
            ) -> GetCircleProductNameAscQuery<'a, C, R, N> {
                GetCircleProductNameAscQuery {
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
        pub struct GetCircleProductNameDesc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetCircleProductNameDescBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetCircleProductNameDescBorrowed<'a>> for GetCircleProductNameDesc {
            fn from(
                GetCircleProductNameDescBorrowed {
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
                    circle_name,
                }: GetCircleProductNameDescBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetCircleProductNameDescQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetCircleProductNameDescBorrowed,
            mapper: fn(GetCircleProductNameDescBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetCircleProductNameDescQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetCircleProductNameDescBorrowed) -> R,
            ) -> GetCircleProductNameDescQuery<'a, C, R, N> {
                GetCircleProductNameDescQuery {
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
        pub struct I64Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> i64,
            mapper: fn(i64) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> I64Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'a, C, R, N> {
                I64Query {
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
        pub fn get_circle_product_released_at_asc() -> GetCircleProductReleasedAtAscStmt {
            GetCircleProductReleasedAtAscStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product
  JOIN circle c on product.circle_id = $1 and c.id = product.circle_id 
ORDER BY released_at ASC LIMIT $2 OFFSET $3",
            ))
        }
        pub struct GetCircleProductReleasedAtAscStmt(cornucopia_async::private::Stmt);
        impl GetCircleProductReleasedAtAscStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                circle_id: &'a T1,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetCircleProductReleasedAtAscQuery<'a, C, GetCircleProductReleasedAtAsc, 3>
            {
                GetCircleProductReleasedAtAscQuery {
                    client,
                    params: [circle_id, limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetCircleProductReleasedAtAscBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetCircleProductReleasedAtAsc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                GetCircleProductReleasedAtAscParams<T1>,
                GetCircleProductReleasedAtAscQuery<'a, C, GetCircleProductReleasedAtAsc, 3>,
                C,
            > for GetCircleProductReleasedAtAscStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetCircleProductReleasedAtAscParams<T1>,
            ) -> GetCircleProductReleasedAtAscQuery<'a, C, GetCircleProductReleasedAtAsc, 3>
            {
                self.bind(client, &params.circle_id, &params.limit, &params.offset)
            }
        }
        pub fn get_circle_product_released_at_desc() -> GetCircleProductReleasedAtDescStmt {
            GetCircleProductReleasedAtDescStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product
  JOIN circle c on product.circle_id = $1 and c.id = product.circle_id 
ORDER BY released_at DESC LIMIT $2 OFFSET $3",
            ))
        }
        pub struct GetCircleProductReleasedAtDescStmt(cornucopia_async::private::Stmt);
        impl GetCircleProductReleasedAtDescStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                circle_id: &'a T1,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetCircleProductReleasedAtDescQuery<'a, C, GetCircleProductReleasedAtDesc, 3>
            {
                GetCircleProductReleasedAtDescQuery {
                    client,
                    params: [circle_id, limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetCircleProductReleasedAtDescBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetCircleProductReleasedAtDesc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                GetCircleProductReleasedAtDescParams<T1>,
                GetCircleProductReleasedAtDescQuery<'a, C, GetCircleProductReleasedAtDesc, 3>,
                C,
            > for GetCircleProductReleasedAtDescStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetCircleProductReleasedAtDescParams<T1>,
            ) -> GetCircleProductReleasedAtDescQuery<'a, C, GetCircleProductReleasedAtDesc, 3>
            {
                self.bind(client, &params.circle_id, &params.limit, &params.offset)
            }
        }
        pub fn get_circle_product_name_asc() -> GetCircleProductNameAscStmt {
            GetCircleProductNameAscStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on product.circle_id = $1 and c.id = product.circle_id 
ORDER BY name ASC LIMIT $2 OFFSET $3",
            ))
        }
        pub struct GetCircleProductNameAscStmt(cornucopia_async::private::Stmt);
        impl GetCircleProductNameAscStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                circle_id: &'a T1,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetCircleProductNameAscQuery<'a, C, GetCircleProductNameAsc, 3> {
                GetCircleProductNameAscQuery {
                    client,
                    params: [circle_id, limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetCircleProductNameAscBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetCircleProductNameAsc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                GetCircleProductNameAscParams<T1>,
                GetCircleProductNameAscQuery<'a, C, GetCircleProductNameAsc, 3>,
                C,
            > for GetCircleProductNameAscStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetCircleProductNameAscParams<T1>,
            ) -> GetCircleProductNameAscQuery<'a, C, GetCircleProductNameAsc, 3> {
                self.bind(client, &params.circle_id, &params.limit, &params.offset)
            }
        }
        pub fn get_circle_product_name_desc() -> GetCircleProductNameDescStmt {
            GetCircleProductNameDescStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on product.circle_id = $1 and c.id = product.circle_id 
ORDER BY name DESC LIMIT $2 OFFSET $3",
            ))
        }
        pub struct GetCircleProductNameDescStmt(cornucopia_async::private::Stmt);
        impl GetCircleProductNameDescStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                circle_id: &'a T1,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetCircleProductNameDescQuery<'a, C, GetCircleProductNameDesc, 3> {
                GetCircleProductNameDescQuery {
                    client,
                    params: [circle_id, limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetCircleProductNameDescBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetCircleProductNameDesc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                GetCircleProductNameDescParams<T1>,
                GetCircleProductNameDescQuery<'a, C, GetCircleProductNameDesc, 3>,
                C,
            > for GetCircleProductNameDescStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetCircleProductNameDescParams<T1>,
            ) -> GetCircleProductNameDescQuery<'a, C, GetCircleProductNameDesc, 3> {
                self.bind(client, &params.circle_id, &params.limit, &params.offset)
            }
        }
        pub fn count_circle_product() -> CountCircleProductStmt {
            CountCircleProductStmt(cornucopia_async::private::Stmt::new(
                "SELECT COUNT(*) FROM product WHERE circle_id = $1",
            ))
        }
        pub struct CountCircleProductStmt(cornucopia_async::private::Stmt);
        impl CountCircleProductStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                circle_id: &'a T1,
            ) -> I64Query<'a, C, i64, 1> {
                I64Query {
                    client,
                    params: [circle_id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
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
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGenre {
            pub product_id: String,
            pub genre_id: String,
            pub name: String,
        }
        pub struct GetGenreBorrowed<'a> {
            pub product_id: &'a str,
            pub genre_id: &'a str,
            pub name: &'a str,
        }
        impl<'a> From<GetGenreBorrowed<'a>> for GetGenre {
            fn from(
                GetGenreBorrowed {
                    product_id,
                    genre_id,
                    name,
                }: GetGenreBorrowed<'a>,
            ) -> Self {
                Self {
                    product_id: product_id.into(),
                    genre_id: genre_id.into(),
                    name: name.into(),
                }
            }
        }
        pub struct GetGenreQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetGenreBorrowed,
            mapper: fn(GetGenreBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetGenreQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(GetGenreBorrowed) -> R) -> GetGenreQuery<'a, C, R, N> {
                GetGenreQuery {
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
        pub struct GetGenres {
            pub product_id: String,
            pub genre_id: String,
            pub name: String,
        }
        pub struct GetGenresBorrowed<'a> {
            pub product_id: &'a str,
            pub genre_id: &'a str,
            pub name: &'a str,
        }
        impl<'a> From<GetGenresBorrowed<'a>> for GetGenres {
            fn from(
                GetGenresBorrowed {
                    product_id,
                    genre_id,
                    name,
                }: GetGenresBorrowed<'a>,
            ) -> Self {
                Self {
                    product_id: product_id.into(),
                    genre_id: genre_id.into(),
                    name: name.into(),
                }
            }
        }
        pub struct GetGenresQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetGenresBorrowed,
            mapper: fn(GetGenresBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetGenresQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(GetGenresBorrowed) -> R) -> GetGenresQuery<'a, C, R, N> {
                GetGenresQuery {
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
        pub struct GetUsergenre {
            pub product_id: String,
            pub genre_id: String,
            pub name: String,
        }
        pub struct GetUsergenreBorrowed<'a> {
            pub product_id: &'a str,
            pub genre_id: &'a str,
            pub name: &'a str,
        }
        impl<'a> From<GetUsergenreBorrowed<'a>> for GetUsergenre {
            fn from(
                GetUsergenreBorrowed {
                    product_id,
                    genre_id,
                    name,
                }: GetUsergenreBorrowed<'a>,
            ) -> Self {
                Self {
                    product_id: product_id.into(),
                    genre_id: genre_id.into(),
                    name: name.into(),
                }
            }
        }
        pub struct GetUsergenreQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetUsergenreBorrowed,
            mapper: fn(GetUsergenreBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetUsergenreQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetUsergenreBorrowed) -> R,
            ) -> GetUsergenreQuery<'a, C, R, N> {
                GetUsergenreQuery {
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
        pub struct GetUsergenres {
            pub product_id: String,
            pub genre_id: String,
            pub name: String,
            pub count: i32,
        }
        pub struct GetUsergenresBorrowed<'a> {
            pub product_id: &'a str,
            pub genre_id: &'a str,
            pub name: &'a str,
            pub count: i32,
        }
        impl<'a> From<GetUsergenresBorrowed<'a>> for GetUsergenres {
            fn from(
                GetUsergenresBorrowed {
                    product_id,
                    genre_id,
                    name,
                    count,
                }: GetUsergenresBorrowed<'a>,
            ) -> Self {
                Self {
                    product_id: product_id.into(),
                    genre_id: genre_id.into(),
                    name: name.into(),
                    count,
                }
            }
        }
        pub struct GetUsergenresQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetUsergenresBorrowed,
            mapper: fn(GetUsergenresBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetUsergenresQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetUsergenresBorrowed) -> R,
            ) -> GetUsergenresQuery<'a, C, R, N> {
                GetUsergenresQuery {
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
        pub fn get_genre() -> GetGenreStmt {
            GetGenreStmt(cornucopia_async :: private :: Stmt :: new("SELECT product_id, genre_id, name FROM product_genre JOIN genre g on product_genre.product_id = $1 and g.id = product_genre.genre_id"))
        }
        pub struct GetGenreStmt(cornucopia_async::private::Stmt);
        impl GetGenreStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                genre: &'a T1,
            ) -> GetGenreQuery<'a, C, GetGenre, 1> {
                GetGenreQuery {
                    client,
                    params: [genre],
                    stmt: &mut self.0,
                    extractor: |row| GetGenreBorrowed {
                        product_id: row.get(0),
                        genre_id: row.get(1),
                        name: row.get(2),
                    },
                    mapper: |it| <GetGenre>::from(it),
                }
            }
        }
        pub fn get_genres() -> GetGenresStmt {
            GetGenresStmt(cornucopia_async :: private :: Stmt :: new("SELECT product_id, genre_id, name FROM product_genre JOIN genre g on product_genre.product_id = ANY($1) and g.id = product_genre.genre_id"))
        }
        pub struct GetGenresStmt(cornucopia_async::private::Stmt);
        impl GetGenresStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                genres: &'a T2,
            ) -> GetGenresQuery<'a, C, GetGenres, 1> {
                GetGenresQuery {
                    client,
                    params: [genres],
                    stmt: &mut self.0,
                    extractor: |row| GetGenresBorrowed {
                        product_id: row.get(0),
                        genre_id: row.get(1),
                        name: row.get(2),
                    },
                    mapper: |it| <GetGenres>::from(it),
                }
            }
        }
        pub fn get_usergenre() -> GetUsergenreStmt {
            GetUsergenreStmt(cornucopia_async :: private :: Stmt :: new("SELECT product_id, genre_id, name FROM product_usergenre JOIN genre g on product_usergenre.product_id = $1 and g.id = product_usergenre.genre_id"))
        }
        pub struct GetUsergenreStmt(cornucopia_async::private::Stmt);
        impl GetUsergenreStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                genre: &'a T1,
            ) -> GetUsergenreQuery<'a, C, GetUsergenre, 1> {
                GetUsergenreQuery {
                    client,
                    params: [genre],
                    stmt: &mut self.0,
                    extractor: |row| GetUsergenreBorrowed {
                        product_id: row.get(0),
                        genre_id: row.get(1),
                        name: row.get(2),
                    },
                    mapper: |it| <GetUsergenre>::from(it),
                }
            }
        }
        pub fn get_usergenres() -> GetUsergenresStmt {
            GetUsergenresStmt(cornucopia_async :: private :: Stmt :: new("SELECT product_id, genre_id, name, count FROM product_usergenre JOIN genre g on product_usergenre.product_id = ANY($1) and g.id = product_usergenre.genre_id"))
        }
        pub struct GetUsergenresStmt(cornucopia_async::private::Stmt);
        impl GetUsergenresStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                genres: &'a T2,
            ) -> GetUsergenresQuery<'a, C, GetUsergenres, 1> {
                GetUsergenresQuery {
                    client,
                    params: [genres],
                    stmt: &mut self.0,
                    extractor: |row| GetUsergenresBorrowed {
                        product_id: row.get(0),
                        genre_id: row.get(1),
                        name: row.get(2),
                        count: row.get(3),
                    },
                    mapper: |it| <GetUsergenres>::from(it),
                }
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
        pub struct GetProductReleasedAtAscParams {
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct GetProductReleasedAtDescParams {
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct GetProductNameAscParams {
            pub limit: i64,
            pub offset: i64,
        }
        #[derive(Clone, Copy, Debug)]
        pub struct GetProductNameDescParams {
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
        pub struct GetProduct {
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
        pub struct GetProductBorrowed<'a> {
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
        impl<'a> From<GetProductBorrowed<'a>> for GetProduct {
            fn from(
                GetProductBorrowed {
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
                }: GetProductBorrowed<'a>,
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
        pub struct GetProductQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetProductBorrowed,
            mapper: fn(GetProductBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetProductQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetProductBorrowed) -> R,
            ) -> GetProductQuery<'a, C, R, N> {
                GetProductQuery {
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
        pub struct GetProductReleasedAtAsc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetProductReleasedAtAscBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetProductReleasedAtAscBorrowed<'a>> for GetProductReleasedAtAsc {
            fn from(
                GetProductReleasedAtAscBorrowed {
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
                    circle_name,
                }: GetProductReleasedAtAscBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetProductReleasedAtAscQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetProductReleasedAtAscBorrowed,
            mapper: fn(GetProductReleasedAtAscBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetProductReleasedAtAscQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetProductReleasedAtAscBorrowed) -> R,
            ) -> GetProductReleasedAtAscQuery<'a, C, R, N> {
                GetProductReleasedAtAscQuery {
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
        pub struct GetProductReleasedAtDesc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetProductReleasedAtDescBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetProductReleasedAtDescBorrowed<'a>> for GetProductReleasedAtDesc {
            fn from(
                GetProductReleasedAtDescBorrowed {
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
                    circle_name,
                }: GetProductReleasedAtDescBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetProductReleasedAtDescQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetProductReleasedAtDescBorrowed,
            mapper: fn(GetProductReleasedAtDescBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetProductReleasedAtDescQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetProductReleasedAtDescBorrowed) -> R,
            ) -> GetProductReleasedAtDescQuery<'a, C, R, N> {
                GetProductReleasedAtDescQuery {
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
        pub struct GetProductNameAsc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetProductNameAscBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetProductNameAscBorrowed<'a>> for GetProductNameAsc {
            fn from(
                GetProductNameAscBorrowed {
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
                    circle_name,
                }: GetProductNameAscBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetProductNameAscQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetProductNameAscBorrowed,
            mapper: fn(GetProductNameAscBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetProductNameAscQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetProductNameAscBorrowed) -> R,
            ) -> GetProductNameAscQuery<'a, C, R, N> {
                GetProductNameAscQuery {
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
        pub struct GetProductNameDesc {
            pub id: String,
            pub name: String,
            pub description: Option<String>,
            pub series: Option<String>,
            pub circle_id: String,
            pub actor: Vec<String>,
            pub author: Vec<String>,
            pub illustrator: Vec<String>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: String,
            pub remote_image: Vec<String>,
            pub circle_name: String,
        }
        pub struct GetProductNameDescBorrowed<'a> {
            pub id: &'a str,
            pub name: &'a str,
            pub description: Option<&'a str>,
            pub series: Option<&'a str>,
            pub circle_id: &'a str,
            pub actor: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub author: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub illustrator: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: &'a str,
            pub remote_image: cornucopia_async::ArrayIterator<'a, &'a str>,
            pub circle_name: &'a str,
        }
        impl<'a> From<GetProductNameDescBorrowed<'a>> for GetProductNameDesc {
            fn from(
                GetProductNameDescBorrowed {
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
                    circle_name,
                }: GetProductNameDescBorrowed<'a>,
            ) -> Self {
                Self {
                    id: id.into(),
                    name: name.into(),
                    description: description.map(|v| v.into()),
                    series: series.map(|v| v.into()),
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
                    circle_name: circle_name.into(),
                }
            }
        }
        pub struct GetProductNameDescQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetProductNameDescBorrowed,
            mapper: fn(GetProductNameDescBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetProductNameDescQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(
                self,
                mapper: fn(GetProductNameDescBorrowed) -> R,
            ) -> GetProductNameDescQuery<'a, C, R, N> {
                GetProductNameDescQuery {
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
        pub struct I64Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> i64,
            mapper: fn(i64) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> I64Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'a, C, R, N> {
                I64Query {
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
        pub fn get_product() -> GetProductStmt {
            GetProductStmt(cornucopia_async::private::Stmt::new(
                "SELECT * FROM product WHERE id = $1",
            ))
        }
        pub struct GetProductStmt(cornucopia_async::private::Stmt);
        impl GetProductStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
            ) -> GetProductQuery<'a, C, GetProduct, 1> {
                GetProductQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| GetProductBorrowed {
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
                    mapper: |it| <GetProduct>::from(it),
                }
            }
        }
        pub fn get_product_path() -> GetProductPathStmt {
            GetProductPathStmt(cornucopia_async::private::Stmt::new(
                "SELECT path FROM product WHERE id = $1",
            ))
        }
        pub struct GetProductPathStmt(cornucopia_async::private::Stmt);
        impl GetProductPathStmt {
            pub fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
            ) -> StringQuery<'a, C, String, 1> {
                StringQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn get_product_released_at_asc() -> GetProductReleasedAtAscStmt {
            GetProductReleasedAtAscStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on c.id = product.circle_id 
ORDER BY released_at ASC LIMIT $1 OFFSET $2",
            ))
        }
        pub struct GetProductReleasedAtAscStmt(cornucopia_async::private::Stmt);
        impl GetProductReleasedAtAscStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetProductReleasedAtAscQuery<'a, C, GetProductReleasedAtAsc, 2> {
                GetProductReleasedAtAscQuery {
                    client,
                    params: [limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetProductReleasedAtAscBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetProductReleasedAtAsc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                GetProductReleasedAtAscParams,
                GetProductReleasedAtAscQuery<'a, C, GetProductReleasedAtAsc, 2>,
                C,
            > for GetProductReleasedAtAscStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetProductReleasedAtAscParams,
            ) -> GetProductReleasedAtAscQuery<'a, C, GetProductReleasedAtAsc, 2> {
                self.bind(client, &params.limit, &params.offset)
            }
        }
        pub fn get_product_released_at_desc() -> GetProductReleasedAtDescStmt {
            GetProductReleasedAtDescStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on c.id = product.circle_id 
ORDER BY released_at DESC LIMIT $1 OFFSET $2",
            ))
        }
        pub struct GetProductReleasedAtDescStmt(cornucopia_async::private::Stmt);
        impl GetProductReleasedAtDescStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetProductReleasedAtDescQuery<'a, C, GetProductReleasedAtDesc, 2> {
                GetProductReleasedAtDescQuery {
                    client,
                    params: [limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetProductReleasedAtDescBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetProductReleasedAtDesc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                GetProductReleasedAtDescParams,
                GetProductReleasedAtDescQuery<'a, C, GetProductReleasedAtDesc, 2>,
                C,
            > for GetProductReleasedAtDescStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetProductReleasedAtDescParams,
            ) -> GetProductReleasedAtDescQuery<'a, C, GetProductReleasedAtDesc, 2> {
                self.bind(client, &params.limit, &params.offset)
            }
        }
        pub fn get_product_name_asc() -> GetProductNameAscStmt {
            GetProductNameAscStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on c.id = product.circle_id 
ORDER BY name ASC LIMIT $1 OFFSET $2",
            ))
        }
        pub struct GetProductNameAscStmt(cornucopia_async::private::Stmt);
        impl GetProductNameAscStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetProductNameAscQuery<'a, C, GetProductNameAsc, 2> {
                GetProductNameAscQuery {
                    client,
                    params: [limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetProductNameAscBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetProductNameAsc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                GetProductNameAscParams,
                GetProductNameAscQuery<'a, C, GetProductNameAsc, 2>,
                C,
            > for GetProductNameAscStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetProductNameAscParams,
            ) -> GetProductNameAscQuery<'a, C, GetProductNameAsc, 2> {
                self.bind(client, &params.limit, &params.offset)
            }
        }
        pub fn get_product_name_desc() -> GetProductNameDescStmt {
            GetProductNameDescStmt(cornucopia_async::private::Stmt::new(
                "SELECT product.*, c.name circle_name FROM product 
  JOIN circle c on c.id = product.circle_id 
ORDER BY name DESC LIMIT $1 OFFSET $2",
            ))
        }
        pub struct GetProductNameDescStmt(cornucopia_async::private::Stmt);
        impl GetProductNameDescStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                limit: &'a i64,
                offset: &'a i64,
            ) -> GetProductNameDescQuery<'a, C, GetProductNameDesc, 2> {
                GetProductNameDescQuery {
                    client,
                    params: [limit, offset],
                    stmt: &mut self.0,
                    extractor: |row| GetProductNameDescBorrowed {
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
                        circle_name: row.get(17),
                    },
                    mapper: |it| <GetProductNameDesc>::from(it),
                }
            }
        }
        impl<'a, C: GenericClient>
            cornucopia_async::Params<
                'a,
                GetProductNameDescParams,
                GetProductNameDescQuery<'a, C, GetProductNameDesc, 2>,
                C,
            > for GetProductNameDescStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a GetProductNameDescParams,
            ) -> GetProductNameDescQuery<'a, C, GetProductNameDesc, 2> {
                self.bind(client, &params.limit, &params.offset)
            }
        }
        pub fn count_product() -> CountProductStmt {
            CountProductStmt(cornucopia_async::private::Stmt::new(
                "SELECT COUNT(*) FROM product",
            ))
        }
        pub struct CountProductStmt(cornucopia_async::private::Stmt);
        impl CountProductStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
            ) -> I64Query<'a, C, i64, 0> {
                I64Query {
                    client,
                    params: [],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
    }
    pub mod user {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertUserParams<T1: cornucopia_async::StringSql> {
            pub id: i32,
            pub password: T1,
        }
        #[derive(Debug)]
        pub struct ChangePasswordParams<T1: cornucopia_async::StringSql> {
            pub password: T1,
            pub id: i32,
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUser {
            pub id: i32,
            pub password: String,
        }
        pub struct GetUserBorrowed<'a> {
            pub id: i32,
            pub password: &'a str,
        }
        impl<'a> From<GetUserBorrowed<'a>> for GetUser {
            fn from(GetUserBorrowed { id, password }: GetUserBorrowed<'a>) -> Self {
                Self {
                    id,
                    password: password.into(),
                }
            }
        }
        pub struct GetUserQuery<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> GetUserBorrowed,
            mapper: fn(GetUserBorrowed) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> GetUserQuery<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(GetUserBorrowed) -> R) -> GetUserQuery<'a, C, R, N> {
                GetUserQuery {
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
        pub struct I64Query<'a, C: GenericClient, T, const N: usize> {
            client: &'a C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            stmt: &'a mut cornucopia_async::private::Stmt,
            extractor: fn(&tokio_postgres::Row) -> i64,
            mapper: fn(i64) -> T,
        }
        impl<'a, C, T: 'a, const N: usize> I64Query<'a, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(i64) -> R) -> I64Query<'a, C, R, N> {
                I64Query {
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
        pub fn insert_user() -> InsertUserStmt {
            InsertUserStmt(cornucopia_async::private::Stmt::new(
                "INSERT INTO users(id, password) 
VALUES ($1, $2)
ON CONFLICT (id) DO NOTHING",
            ))
        }
        pub struct InsertUserStmt(cornucopia_async::private::Stmt);
        impl InsertUserStmt {
            pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
                password: &'a T1,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[id, password]).await
            }
        }
        impl<'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                InsertUserParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertUserStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertUserParams<T1>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.id, &params.password))
            }
        }
        pub fn get_user() -> GetUserStmt {
            GetUserStmt(cornucopia_async::private::Stmt::new(
                "SELECT * FROM users WHERE id = $1",
            ))
        }
        pub struct GetUserStmt(cornucopia_async::private::Stmt);
        impl GetUserStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
            ) -> GetUserQuery<'a, C, GetUser, 1> {
                GetUserQuery {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| GetUserBorrowed {
                        id: row.get(0),
                        password: row.get(1),
                    },
                    mapper: |it| <GetUser>::from(it),
                }
            }
        }
        pub fn exist_user() -> ExistUserStmt {
            ExistUserStmt(cornucopia_async::private::Stmt::new(
                "SELECT COUNT(*) FROM users WHERE id = $1",
            ))
        }
        pub struct ExistUserStmt(cornucopia_async::private::Stmt);
        impl ExistUserStmt {
            pub fn bind<'a, C: GenericClient>(
                &'a mut self,
                client: &'a C,
                id: &'a i32,
            ) -> I64Query<'a, C, i64, 1> {
                I64Query {
                    client,
                    params: [id],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it,
                }
            }
        }
        pub fn change_password() -> ChangePasswordStmt {
            ChangePasswordStmt(cornucopia_async::private::Stmt::new(
                "UPDATE users SET password = $1 WHERE id = $2",
            ))
        }
        pub struct ChangePasswordStmt(cornucopia_async::private::Stmt);
        impl ChangePasswordStmt {
            pub async fn bind<'a, C: GenericClient, T1: cornucopia_async::StringSql>(
                &'a mut self,
                client: &'a C,
                password: &'a T1,
                id: &'a i32,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client.execute(stmt, &[password, id]).await
            }
        }
        impl<'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql>
            cornucopia_async::Params<
                'a,
                ChangePasswordParams<T1>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for ChangePasswordStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a ChangePasswordParams<T1>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(client, &params.password, &params.id))
            }
        }
    }
}
