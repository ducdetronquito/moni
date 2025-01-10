use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::PgPool;

#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>
}


#[async_trait]
pub trait UserRepo {
    async fn add_user(&self, id: Uuid, name: &str, email: &str) -> anyhow::Result<User>;
}

pub struct PostgresUserRepo {
    pg_pool: PgPool
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> PostgresUserRepo {
        PostgresUserRepo { pg_pool: pool}
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn add_user(&self, id: Uuid, name: &str, email: &str) -> anyhow::Result<User> {
        let user = sqlx::query_file_as!(User, "src/queries/user/add_user.sql", id, name, email).fetch_one(&self.pg_pool).await?;
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[sqlx::test]
    async fn add_user(pool: PgPool) {
        let user_repo = PostgresUserRepo::new(pool);
        let user_id = Uuid::from_str("019450c3-984d-7145-855e-195a8e3bba25").unwrap();

        let user  = user_repo.add_user(user_id, "Doofie", "doofie@doof.us").await.unwrap();

        assert_eq!(user.id, user_id);
        assert_eq!(user.name, "Doofie");
        assert_eq!(user.email, "doofie@doof.us");
        assert!(user.created < user.updated);
    }

    #[sqlx::test]
    async fn add_user_with_invalid_email(pool: PgPool) {
        let user_repo = PostgresUserRepo::new(pool);
        let user_id = Uuid::from_str("019450c3-984d-7145-855e-195a8e3bba25").unwrap();

        let error  = user_repo.add_user(user_id, "Doofie", "INVALID EMAIL").await.unwrap_err();
        assert_eq!(format!("{}", error), "error returned from database: value for domain email violates check constraint \"email_check\"")
    }
}
