#[derive(Debug, Clone, PartialEq)]
pub enum DbEngine {
    Postgres,
    DuckDb,
    ArrowFlight,
}

pub struct DbearerConnection {
    pub engine: DbEngine,
    pub connection_string: String,
}

impl DbearerConnection {
    pub fn new(engine: DbEngine, conn: impl Into<String>) -> Self {
        Self { engine, connection_string: conn.into() }
    }

    pub fn is_connected(&self) -> bool {
        !self.connection_string.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbearer_connection() {
        // Arrange & Act
        let conn = DbearerConnection::new(DbEngine::DuckDb, "duckdb://local.db");
        // Assert
        assert!(conn.is_connected());
    }
}
