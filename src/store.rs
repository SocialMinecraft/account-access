use chrono::{Duration, Utc};
use rand::{thread_rng, Rng};
use sqlx::PgPool;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Store {
    db: PgPool
}

struct RowDef {
    //id: i64,
    account_id: String,
    token: String,
    //expires_at: NaiveDateTime,
}

impl Store {
    pub fn new(db: PgPool) -> Self {
        Store { db }
    }

    pub async fn create_token(&self, account_id: &String, ttl: Duration) -> anyhow::Result<String> {

        let re : sqlx::Result<RowDef> = sqlx::query_as!(
            RowDef,
            r#"
            INSERT INTO tokens (
                account_id,
                token,
                expires_at
            ) VALUES ($1, $2, $3)
            RETURNING
                account_id,
                token
            ;"#,
            account_id,
            Self::generate_token(),
            Utc::now().naive_utc() + ttl,
        )
            .fetch_one(&self.db)
            .await;

        let re = re?;
        Ok(re.token)
    }

    pub async fn token_to_account(&self, token: &String) -> anyhow::Result<Option<String>> {
        let re : sqlx::Result<Option<RowDef>> = sqlx::query_as!(
            RowDef,
            r#"
            SELECT
                account_id,
                token
            FROM
                tokens
            WHERE
                token = $1 AND
                expires_at >= CURRENT_TIMESTAMP
            ;"#,
            token
        )
            .fetch_optional(&self.db)
            .await;

        let re = re?;
        match re {
            None => Ok(None),
            Some(t) => Ok(Some(t.account_id))
        }
    }

    pub fn generate_token() -> String {
        let mut rng = thread_rng();
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let token = (0..50)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        info!("Token generated: {}", token);
        token
    }

}