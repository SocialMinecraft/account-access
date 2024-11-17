use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Store {
    db: PgPool
}

struct RowDef {
    // fields
}

impl Store {
    pub fn new(db: PgPool) -> Self {
        Store { db }
    }
}