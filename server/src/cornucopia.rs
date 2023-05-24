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
    pub mod insert_product {
        use cornucopia_async::GenericClient;
        use futures;
        use futures::{StreamExt, TryStreamExt};
        #[derive(Debug)]
        pub struct InsertProductParams<
            T1: cornucopia_async::StringSql,
            T2: cornucopia_async::StringSql,
            T3: cornucopia_async::StringSql,
            T4: cornucopia_async::StringSql,
            T5: cornucopia_async::StringSql,
            T6: cornucopia_async::StringSql,
            T7: cornucopia_async::ArraySql<Item = T6>,
            T8: cornucopia_async::StringSql,
            T9: cornucopia_async::ArraySql<Item = T8>,
            T10: cornucopia_async::StringSql,
            T11: cornucopia_async::ArraySql<Item = T10>,
            T12: cornucopia_async::StringSql,
        > {
            pub id: T1,
            pub name: T2,
            pub description: Option<T3>,
            pub series: Option<T4>,
            pub circle_id: T5,
            pub actor: T7,
            pub author: T9,
            pub illustrator: T11,
            pub price: i32,
            pub sale_count: i32,
            pub age: super::super::types::public::Age,
            pub released_at: time::Date,
            pub rating: Option<f64>,
            pub rating_count: i32,
            pub comment_count: i32,
            pub path: T12,
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
        pub fn exist_product() -> ExistProductStmt {
            ExistProductStmt(cornucopia_async::private::Stmt::new(
                "SELECT id FROM product WHERE id = ANY($1)",
            ))
        }
        pub struct ExistProductStmt(cornucopia_async::private::Stmt);
        impl ExistProductStmt {
            pub fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::ArraySql<Item = T1>,
            >(
                &'a mut self,
                client: &'a C,
                ids: &'a T2,
            ) -> StringQuery<'a, C, String, 1> {
                StringQuery {
                    client,
                    params: [ids],
                    stmt: &mut self.0,
                    extractor: |row| row.get(0),
                    mapper: |it| it.into(),
                }
            }
        }
        pub fn insert_product() -> InsertProductStmt {
            InsertProductStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO product(id, name, description, series, circle_id, actor, author, illustrator, price, sale_count, age, released_at, rating, rating_count, comment_count, path) 
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"))
        }
        pub struct InsertProductStmt(cornucopia_async::private::Stmt);
        impl InsertProductStmt {
            pub async fn bind<
                'a,
                C: GenericClient,
                T1: cornucopia_async::StringSql,
                T2: cornucopia_async::StringSql,
                T3: cornucopia_async::StringSql,
                T4: cornucopia_async::StringSql,
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::ArraySql<Item = T6>,
                T8: cornucopia_async::StringSql,
                T9: cornucopia_async::ArraySql<Item = T8>,
                T10: cornucopia_async::StringSql,
                T11: cornucopia_async::ArraySql<Item = T10>,
                T12: cornucopia_async::StringSql,
            >(
                &'a mut self,
                client: &'a C,
                id: &'a T1,
                name: &'a T2,
                description: &'a Option<T3>,
                series: &'a Option<T4>,
                circle_id: &'a T5,
                actor: &'a T7,
                author: &'a T9,
                illustrator: &'a T11,
                price: &'a i32,
                sale_count: &'a i32,
                age: &'a super::super::types::public::Age,
                released_at: &'a time::Date,
                rating: &'a Option<f64>,
                rating_count: &'a i32,
                comment_count: &'a i32,
                path: &'a T12,
            ) -> Result<u64, tokio_postgres::Error> {
                let stmt = self.0.prepare(client).await?;
                client
                    .execute(
                        stmt,
                        &[
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
                T5: cornucopia_async::StringSql,
                T6: cornucopia_async::StringSql,
                T7: cornucopia_async::ArraySql<Item = T6>,
                T8: cornucopia_async::StringSql,
                T9: cornucopia_async::ArraySql<Item = T8>,
                T10: cornucopia_async::StringSql,
                T11: cornucopia_async::ArraySql<Item = T10>,
                T12: cornucopia_async::StringSql,
            >
            cornucopia_async::Params<
                'a,
                InsertProductParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>,
                std::pin::Pin<
                    Box<
                        dyn futures::Future<Output = Result<u64, tokio_postgres::Error>>
                            + Send
                            + 'a,
                    >,
                >,
                C,
            > for InsertProductStmt
        {
            fn params(
                &'a mut self,
                client: &'a C,
                params: &'a InsertProductParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>,
            ) -> std::pin::Pin<
                Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
            > {
                Box::pin(self.bind(
                    client,
                    &params.id,
                    &params.name,
                    &params.description,
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
    }
}
