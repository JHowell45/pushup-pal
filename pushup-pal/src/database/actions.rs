use chrono::{NaiveDateTime, Utc};
use diesel::{dsl::sum, prelude::*};
use uuid::Uuid;

use crate::database::models::PushupSession;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_pushup_session(
    conn: &mut SqliteConnection,
    uuid: Uuid,
) -> Result<Option<PushupSession>, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let pushup_session = pushup_sessions
        .filter(id.eq(uuid.to_string()))
        .first::<PushupSession>(conn)
        .optional()?;

    Ok(pushup_session)
}

pub fn get_pushup_sessions(conn: &mut SqliteConnection) -> Result<Vec<PushupSession>, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let sessions = pushup_sessions.load::<PushupSession>(conn)?;

    Ok(sessions)
}

pub fn get_first_pushup_session(
    conn: &mut SqliteConnection,
) -> Result<Option<PushupSession>, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let latest = pushup_sessions.first::<PushupSession>(conn).optional()?;
    Ok(latest)
}

pub fn get_latest_pushup_session(
    conn: &mut SqliteConnection,
) -> Result<Option<PushupSession>, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let latest = pushup_sessions
        .order_by(created_at.desc())
        .first::<PushupSession>(conn)
        .optional()?;
    Ok(latest)
}

pub fn get_todays_pushup_total(
    conn: &mut SqliteConnection,
    start_date: NaiveDateTime,
) -> Result<i64, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let count_query = pushup_sessions
        .filter(created_at.ge(start_date))
        .select(sum(amount))
        .get_result::<Option<i64>>(conn)?;
    Ok(match count_query {
        Some(count) => count,
        None => 0,
    })
}

pub fn insert_new_pushup_session(
    conn: &mut SqliteConnection,
    total: i32,
) -> Result<PushupSession, DbError> {
    use crate::database::schema::pushup_sessions::dsl::*;

    let new_pushup_session = PushupSession {
        id: Uuid::new_v4().to_string(),
        amount: total,
        created_at: Utc::now().naive_local(),
    };

    diesel::insert_into(pushup_sessions)
        .values(&new_pushup_session)
        .execute(conn)?;

    Ok(new_pushup_session)
}
