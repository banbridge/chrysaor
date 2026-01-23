use common::error::{AppErrorBuilt, AppResult};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::OnConflict;
use sea_orm::{Condition, ConnectionTrait, EntityTrait, PaginatorTrait, QueryFilter};
use std::collections::HashMap;

#[allow(dead_code)]
pub async fn count_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let count = PaginatorTrait::count(query, db).await.map_err(|e| {
        AppErrorBuilt::db_query_failed(format!("count query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;
    Ok(count)
}

#[allow(dead_code)]
pub async fn list_by_cond<T>(
    db: &impl ConnectionTrait,
    cond: &Condition,
    page_size: u64,
    page: u64,
) -> AppResult<Vec<T::Model>>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let res = PaginatorTrait::paginate(query, db, page_size)
        .fetch_page(page)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_query_failed(format!("list query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;
    Ok(res)
}

#[allow(dead_code)]
pub async fn get_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<T::Model>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::find().filter(cond.to_owned());

    let res = query
        .one(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_query_failed(format!("get query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?
        .ok_or(AppErrorBuilt::db_not_found("record not found".to_string()).print_stack())?;

    Ok(res)
}

#[allow(dead_code)]
pub async fn delete_by_cond<T>(db: &impl ConnectionTrait, cond: &Condition) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::delete_many().filter(cond.to_owned());

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_delete_failed(format!("delete query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(res.rows_affected)
}

#[allow(dead_code)]
pub async fn update_with_model<T>(
    db: &impl ConnectionTrait,
    m: T::ActiveModel,
    cond: &Condition,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let query = T::update_many().filter(cond.to_owned()).set(m);

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_update_failed(format!("update query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;

    Ok(res.rows_affected)
}

#[allow(dead_code)]
pub async fn update_by_cond<T>(
    db: &impl ConnectionTrait,
    m: HashMap<String, Expr>,
    cond: &Condition,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let mut query = T::update_many().filter(cond.to_owned());
    for (column, expr) in m {
        query = query.col_expr(column, expr);
    }

    let res = query.exec(db).await.map_err(|e| {
        AppErrorBuilt::db_update_failed(format!("update query failed: {:?}", e))
            .with_base(e.into())
            .print_stack()
    })?;
    Ok(res.rows_affected)
}

// 泛型插入接口
#[allow(dead_code)]
pub async fn insert_one<T>(db: &impl ConnectionTrait, model: T::ActiveModel) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert(model)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;

    println!("insert one success: {:?}", res);
    Ok(res)
}

#[allow(dead_code)]
pub async fn insert_many<T>(
    db: &impl ConnectionTrait,
    models: Vec<T::ActiveModel>,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert_many(models)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;
    Ok(res)
}

#[allow(dead_code)]
pub async fn insert_many_with_conflict<T>(
    db: &impl ConnectionTrait,
    models: Vec<T::ActiveModel>,
    conflict: OnConflict,
) -> AppResult<u64>
where
    T: EntityTrait,
    <T as EntityTrait>::Model: Sync,
{
    let res = T::insert_many(models)
        .on_conflict(conflict)
        .exec_without_returning(db)
        .await
        .map_err(|e| {
            AppErrorBuilt::db_insert_failed(format!("insert query failed: {:?}", e))
                .with_base(e.into())
                .print_stack()
        })?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::entity::user;
    use crate::{conn, dbop};
    use common::id_gen::gen_uniq_id;
    use dotenv;
    use sea_orm::{ColumnTrait, Condition};
    use std::sync::Arc;
    use tokio;

    #[tokio::test]
    async fn test_insert() {
        dotenv::dotenv().ok();
        let dsn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = conn::get_postgresql_db(dsn.as_str()).await.unwrap();

        let db = Arc::new(db);

        let user_id = gen_uniq_id();
        //
        // let u = User {
        //     id: 0,
        //     username: "test".to_string(),
        //     user_id: user_id.clone(),
        //     nickname: Some("test".to_string()),
        //     created_at: Local::now().fixed_offset(),
        //     updated_at: Local::now().fixed_offset(),
        // };
        //
        // println!("user_id: {:?}", user_id);
        //
        // let res = dbop::insert_one::<user::Entity>(&db, u.into())
        //     .await
        //     .unwrap();
        //
        // println!("insert one success: {:?}", res);

        let cond = Condition::all().add(user::Column::UserId.eq(user_id.clone()));
        let user_query = dbop::get_by_cond::<user::Entity>(db.get_db(), &cond).await;

        println!("user_query: {:?}", user_query);
    }
}
