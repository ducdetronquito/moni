use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Project {
    id: Uuid,
    name: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    owner_id: Uuid,
}

#[async_trait]
pub trait ProjectRepo {
    async fn list_projects(&self, owner_id: Uuid) -> anyhow::Result<Vec<Project>>;
    async fn add_project(&self, id: Uuid, name: &str, owner: Uuid) -> anyhow::Result<Project>;
}

#[allow(dead_code)]
struct PostgresProjectRepo {
    pg_pool: PgPool
}

#[allow(dead_code)]
impl PostgresProjectRepo {
    pub fn new(pg_pool: PgPool) -> PostgresProjectRepo {
        PostgresProjectRepo { pg_pool: pg_pool}
    }
}

#[async_trait]
impl ProjectRepo for PostgresProjectRepo {
    async fn list_projects(&self, owner_id: Uuid) -> anyhow::Result<Vec<Project>> {
        let projects = sqlx::query_file_as!(Project, "src/queries/list_projects.sql", owner_id).fetch_all(&self.pg_pool).await?;
        Ok(projects)
    }

    async fn add_project(&self, id: Uuid, name: &str, owner_id: Uuid) -> anyhow::Result<Project> {
        let project = sqlx::query_file_as!(Project, "src/queries/add_project.sql", id, name, owner_id).fetch_one(&self.pg_pool).await?;
        Ok(project)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::user::{PostgresUserRepo, UserRepo};
    use super::*;

    #[sqlx::test]
    async fn test_add_project(pool: PgPool) {
        let user_repo = PostgresUserRepo::new(pool.clone());
        let user_id = Uuid::from_str("01945150-5a8b-7559-b3c8-a1fa5f53e997").unwrap();
        let user = user_repo.add_user(user_id, "Doofie", "doofie@doof.us").await.unwrap();

        let project_repo = PostgresProjectRepo::new(pool);
        let project_id = Uuid::from_str("019450c3-984d-7145-855e-195a8e3bba25").unwrap();
        let project_name = "Groceries";

        let project = project_repo.add_project(project_id, project_name, user.id).await.unwrap();
        assert_eq!(project.id, project_id);
        assert_eq!(project.name, project_name);
        assert_eq!(project.owner_id, user.id);
        assert!(project.created < project.updated);
    }

    #[sqlx::test]
    async fn test_list_projects_when_no_projects_exists(pool: PgPool) {
        let user_repo = PostgresUserRepo::new(pool.clone());
        let user_id = Uuid::from_str("01945150-5a8b-7559-b3c8-a1fa5f53e997").unwrap();
        let user = user_repo.add_user(user_id, "Doofie", "doofie@doof.us").await.unwrap();

        let project_repo = PostgresProjectRepo::new(pool);
        let projects = project_repo.list_projects(user.id).await.unwrap();
        assert_eq!(projects.len(), 0);
    }

    #[sqlx::test]
    async fn test_list_projects_when_projects_exist(pool: PgPool) {
        let user_repo = PostgresUserRepo::new(pool.clone());
        let user_id = Uuid::from_str("01945150-5a8b-7559-b3c8-a1fa5f53e997").unwrap();
        let user = user_repo.add_user(user_id, "Doofie", "doofie@doof.us").await.unwrap();

        let project_repo = PostgresProjectRepo::new(pool);
        let project_id = Uuid::from_str("019450c3-984d-7145-855e-195a8e3bba25").unwrap();
        let project_name = "Groceries";
        let _ = project_repo.add_project(project_id, &project_name, user.id).await.unwrap();

        let projects = project_repo.list_projects(user.id).await.unwrap();
        assert_eq!(projects.len(), 1);
        assert_eq!(projects[0].id, project_id);
        assert_eq!(projects[0].name, project_name);
        assert_eq!(projects[0].owner_id, user.id);
        assert!(projects[0].created < projects[0].updated);
    }
}