use chrono::{DateTime, Utc};
use diesel_derives::Insertable;
use serde::{Deserialize, Serialize};

use crate::database::schema::pushup_sessions;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = pushup_sessions)]
pub struct PushupSession {
    pub id: String,
    pub amount: i32,
    pub created_at: DateTime<Utc>,
}
