use super::DbPool;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: String,
    pub bio: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub github_id: Option<String>,
    pub zixiao_cloud_id: Option<String>,
}

/// Inserts a new user record into the database.
///
/// Persists all `UserRecord` fields (including optional `github_id` and `zixiao_cloud_id`) into the `users` table.
/// Returns `Ok(())` when the insert succeeds, or an `AppError` if the query fails.
///
/// # Examples
///
/// ```
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # // `get_test_db_pool` is a helper in test harness that returns a `DbPool`.
/// # let pool = get_test_db_pool().await;
/// let user = UserRecord {
///     id: "user-id".into(),
///     username: "alice".into(),
///     email: "alice@example.com".into(),
///     display_name: "Alice".into(),
///     avatar_url: None,
///     bio: None,
///     password_hash: "hashed".into(),
///     is_admin: false,
///     created_at: chrono::Utc::now().timestamp(),
///     updated_at: chrono::Utc::now().timestamp(),
///     github_id: None,
///     zixiao_cloud_id: None,
/// };
/// insert(&pool, &user).await?;
/// # Ok(()) }
/// ```
pub async fn insert(pool: &DbPool, u: &UserRecord) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO users (id, username, email, display_name, avatar_url, bio, password_hash, is_admin, created_at, updated_at, github_id, zixiao_cloud_id) \
         VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)",
    )
    .bind(&u.id)
    .bind(&u.username)
    .bind(&u.email)
    .bind(&u.display_name)
    .bind(&u.avatar_url)
    .bind(&u.bio)
    .bind(&u.password_hash)
    .bind(u.is_admin)
    .bind(u.created_at)
    .bind(u.updated_at)
    .bind(u.github_id.as_deref())
    .bind(u.zixiao_cloud_id.as_deref())
    .execute(pool)
    .await
    .map_err(AppError::from)?;
    Ok(())
}

const SELECT_COLS: &str = "id, username, email, display_name, avatar_url, bio, password_hash, is_admin, created_at, updated_at, github_id, zixiao_cloud_id";

/// Fetches a user matching the given username or email.
///
/// The query returns the first user whose `username` or `email` equals `ident`.
///
/// # Parameters
///
/// - `ident`: the username or email to look up.
///
/// # Returns
///
/// `Some(UserRecord)` if a user with the specified username or email exists, `None` otherwise.
///
/// # Examples
///
/// ```
/// # async fn example() -> anyhow::Result<()> {
/// let pool = /* obtain DbPool */;
/// let user = find_by_username_or_email(&pool, "alice@example.com").await?;
/// if let Some(rec) = user {
///     println!("found user: {}", rec.username);
/// }
/// # Ok(())
/// # }
/// ```
pub async fn find_by_username_or_email(
    pool: &DbPool,
    ident: &str,
) -> Result<Option<UserRecord>, AppError> {
    let row = sqlx::query(&format!(
        "SELECT {SELECT_COLS} FROM users WHERE username = $1 OR email = $1"
    ))
    .bind(ident)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?;
    match row {
        Some(r) => Ok(Some(user_from_row(&r)?)),
        None => Ok(None),
    }
}

/// Fetches a user record by its unique identifier.
///
/// # Returns
///
/// `Some(UserRecord)` if a user with the given `id` exists, `None` otherwise.
///
/// # Examples
///
/// ```rust
/// # async fn example_usage(pool: &DbPool) -> Result<(), AppError> {
/// let maybe_user = find_by_id(pool, "user-id").await?;
/// if let Some(user) = maybe_user {
///     assert_eq!(user.id, "user-id");
/// }
/// # Ok(())
/// # }
/// ```
pub async fn find_by_id(pool: &DbPool, id: &str) -> Result<Option<UserRecord>, AppError> {
    let row = sqlx::query(&format!("SELECT {SELECT_COLS} FROM users WHERE id = $1"))
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(AppError::from)?;
    match row {
        Some(r) => Ok(Some(user_from_row(&r)?)),
        None => Ok(None),
    }
}

/// Fetches a user record by its GitHub ID.
///
/// Queries the users store for a row whose `github_id` matches the provided value and returns the corresponding `UserRecord` when found.
///
/// # Returns
///
/// `Some(UserRecord)` if a user with the given GitHub ID exists, `None` otherwise.
///
/// # Examples
///
/// ```no_run
/// # use yuxu_server::db::users::find_by_github_id;
/// # use yuxu_server::db::DbPool;
/// # async fn example(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
/// let user = find_by_github_id(pool, "12345").await?;
/// if let Some(u) = user {
///     assert_eq!(u.github_id.as_deref(), Some("12345"));
/// }
/// # Ok(()) }
/// ```
pub async fn find_by_github_id(
    pool: &DbPool,
    github_id: &str,
) -> Result<Option<UserRecord>, AppError> {
    let row = sqlx::query(&format!(
        "SELECT {SELECT_COLS} FROM users WHERE github_id = $1"
    ))
    .bind(github_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?;
    match row {
        Some(r) => Ok(Some(user_from_row(&r)?)),
        None => Ok(None),
    }
}

/// Fetches a user record matching the given Zixiao Cloud identifier.
///
/// # Returns
///
/// `Ok(Some(UserRecord))` if a user with the provided `zixiao_cloud_id` exists, `Ok(None)` if no match is found, or an `Err(AppError)` if the query fails.
///
/// # Examples
///
/// ```no_run
/// # use yuxu_server::db::users::find_by_zixiao_cloud_id;
/// # async fn example(pool: &yuxu_server::DbPool) {
/// let user = find_by_zixiao_cloud_id(pool, "cloud-id-123").await?;
/// if let Some(u) = user {
///     println!("found user: {}", u.username);
/// }
/// # Ok::<(), yuxu_server::AppError>(())
/// # }
/// ```
pub async fn find_by_zixiao_cloud_id(
    pool: &DbPool,
    zixiao_cloud_id: &str,
) -> Result<Option<UserRecord>, AppError> {
    let row = sqlx::query(&format!(
        "SELECT {SELECT_COLS} FROM users WHERE zixiao_cloud_id = $1"
    ))
    .bind(zixiao_cloud_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::from)?;
    match row {
        Some(r) => Ok(Some(user_from_row(&r)?)),
        None => Ok(None),
    }
}

// Unused until the settings-based "link GitHub to my account" endpoint is
// wired up; see the TODO in routes/auth.rs. Keeping it here (rather than
// reinventing the guard at callsite) so the github_id update always goes
// through the same conditional WHERE.
/// Link a GitHub account ID to a user, refusing to overwrite a different existing link.
///
/// This operation sets the user's `github_id` and updates `updated_at`. It is idempotent:
/// calling it with the same `github_id` again has no effect. If the user is already
/// linked to a different GitHub account, the function returns `AppError::Conflict`.
///
/// # Examples
///
/// ```no_run
/// # async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
/// let pool: DbPool = /* obtain pool */ unimplemented!();
/// // Attempts to link GitHub ID "12345" to user "user-uuid".
/// // Succeeds when github_id is null or already "12345"; fails with Conflict otherwise.
/// link_github_id(&pool, "user-uuid", "12345").await?;
/// # Ok(()) }
/// ```
#[allow(dead_code)]
pub async fn link_github_id(pool: &DbPool, user_id: &str, github_id: &str) -> Result<(), AppError> {
    // Only bind when the row has no github_id yet, or is already bound to the
    // same id (idempotent retries). Prevents an attacker-controlled callback
    // from silently re-binding a user who is already tied to a different
    // GitHub account.
    let result = sqlx::query(
        "UPDATE users SET github_id = $1, updated_at = $2 \
         WHERE id = $3 AND (github_id IS NULL OR github_id = $1)",
    )
    .bind(github_id)
    .bind(chrono::Utc::now().timestamp())
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(AppError::from)?;
    if result.rows_affected() == 0 {
        return Err(AppError::Conflict(
            "user is already linked to a different github account".into(),
        ));
    }
    Ok(())
}

/// Links a Zixiao Cloud account identifier to the specified user, setting `zixiao_cloud_id`
/// only if it is currently null or already equal to the provided value.
///
/// On success, updates the user's `zixiao_cloud_id` and `updated_at`.
///
/// # Returns
///
/// `Ok(())` on success; `AppError::Conflict` if the user is already linked to a different Zixiao Cloud account.
///
/// # Examples
///
/// ```no_run
/// # use crates::yuxu_server::db::users::link_zixiao_cloud_id;
/// # async fn example(pool: &crates::yuxu_server::db::DbPool) {
/// let res = link_zixiao_cloud_id(pool, "user-id", "zixiao-123").await;
/// assert!(res.is_ok() || matches!(res.unwrap_err(), crate::AppError::Conflict(_)));
/// # }
/// ```
#[allow(dead_code)]
pub async fn link_zixiao_cloud_id(
    pool: &DbPool,
    user_id: &str,
    zixiao_cloud_id: &str,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE users SET zixiao_cloud_id = $1, updated_at = $2 \
         WHERE id = $3 AND (zixiao_cloud_id IS NULL OR zixiao_cloud_id = $1)",
    )
    .bind(zixiao_cloud_id)
    .bind(chrono::Utc::now().timestamp())
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(AppError::from)?;
    if result.rows_affected() == 0 {
        return Err(AppError::Conflict(
            "user is already linked to a different zixiao cloud account".into(),
        ));
    }
    Ok(())
}

/// Constructs a UserRecord from a Postgres `PgRow`.
///
/// Attempts to extract each expected column from the provided row and maps any
/// extraction error into `AppError`.
///
/// # Examples
///
/// ```
/// // given a `row: sqlx::postgres::PgRow` obtained from a query:
/// // let user = user_from_row(&row)?;
/// ```
#[cfg(feature = "postgres")]
fn user_from_row(row: &sqlx::postgres::PgRow) -> Result<UserRecord, AppError> {
    Ok(UserRecord {
        id: row.try_get("id")?,
        username: row.try_get("username")?,
        email: row.try_get("email")?,
        display_name: row.try_get("display_name")?,
        avatar_url: row.try_get("avatar_url")?,
        bio: row.try_get("bio")?,
        password_hash: row.try_get("password_hash")?,
        is_admin: row.try_get("is_admin")?,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        github_id: row.try_get("github_id")?,
        zixiao_cloud_id: row.try_get("zixiao_cloud_id")?,
    })
}

/// Constructs a `UserRecord` from a SQLite query row.
///
/// Reads each expected column from `row`, converting SQLite's integer `is_admin`
/// into a `bool` and mapping nullable text columns (`github_id`, `zixiao_cloud_id`)
/// into `Option<String>`.
///
/// # Parameters
///
/// - `row`: A SQLite query row containing the selected user columns.
///
/// # Returns
///
/// `Ok(UserRecord)` populated from the row, `Err(AppError)` if any required column
/// cannot be extracted or converted.
///
/// # Examples
///
/// ```
/// // Assume `row` was obtained from a `sqlx::query(...).fetch_one(&pool).await.unwrap()`
/// // and contains the selected user columns.
/// let user = user_from_row(&row).unwrap();
/// assert_eq!(user.username, row.try_get::<String, _>("username").unwrap());
/// ```
#[cfg(all(feature = "sqlite", not(feature = "postgres")))]
fn user_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<UserRecord, AppError> {
    Ok(UserRecord {
        id: row.try_get("id")?,
        username: row.try_get("username")?,
        email: row.try_get("email")?,
        display_name: row.try_get("display_name")?,
        avatar_url: row.try_get("avatar_url")?,
        bio: row.try_get("bio")?,
        password_hash: row.try_get("password_hash")?,
        is_admin: row.try_get::<i64, _>("is_admin")? != 0,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
        github_id: row.try_get("github_id")?,
        zixiao_cloud_id: row.try_get("zixiao_cloud_id")?,
    })
}
