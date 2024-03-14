use rusqlite;
use rusqlite::{Connection, params};

use crate::ExchangeRate;

pub struct DbConnection {
    pub conn: Connection,
}

impl DbConnection {
    pub fn new() -> DbConnection {
        let db: DbConnection = DbConnection {
            conn: Self::start_db(),
        };
        db.create_schema();
        db
    }

    fn start_db() -> Connection {
        let conn = Connection::open("./exchange_rates.db");
        let result = match conn {
            Ok(c) => c,
            Err(E) => panic!("{:?}", E.sqlite_error()),
        };
        result
    }

    fn create_schema(&self) {
        _ = &self.conn
            .execute("CREATE TABLE IF NOT EXISTS exchange_rates ( \
        iso_symbol string primary key, \
        rate real )", [])
            .expect("Table should always be created.");

        _ = &self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS metadata ( \
        timestamp primary key)",
                [],
            )
            .expect("Table should always be created.");
    }

    pub fn update_rates(&self, new_values: ExchangeRate) {
        self.clear_tables();
        self.insert_metadata(new_values.timestamp);
        let mut stmt = self
            .conn
            .prepare(
                "INSERT INTO exchange_rates \
        (iso_symbol, rate) VALUES  \
        (?1, ?2)",
            )
            .expect("Invalid data passed to database!");

        for values in new_values.rates {
            stmt.execute(params![values.0, values.1])
                .expect("TODO: panic message");
        }
    }

    pub fn clear_tables(&self) {
        self.conn
            .execute("DELETE FROM exchange_rates WHERE TRUE", ())
            .expect("TODO: panic message");
        self.conn
            .execute("DELETE FROM metadata WHERE TRUE", ())
            .expect("TODO: panic message");
    }

    pub fn get_data_timestamp(&self) -> u64 {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT timestamp FROM metadata \
            WHERE true",
            )
            .unwrap();
        let mut rows = stmt.query([]);
        let mut row = rows.unwrap();
        match row.next().expect("unwrapping result") {
            Some(v) => v.get(0).expect("unwrapping f64 to return"),
            None => 0u64,
        }
    }

    fn insert_metadata(&self, p: u64) {
        self.conn
            .execute(
                "INSERT INTO metadata (timestamp) \
        VALUES (?1)",
                (p,),
            )
            .unwrap();
    }

    pub fn find_rates(&self, from: &str, to: &str) -> (f64, f64) {
        let from = self.get_rate(from);
        let to = self.get_rate(to);
        (from, to)
    }

    fn get_rate(&self, iso_symbol: &str) -> f64 {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT rate \
                     FROM exchange_rates \
                     WHERE iso_symbol = (?1)",
            )
            .unwrap();

        let mut rows = stmt.query([iso_symbol.to_uppercase()]);
        let mut row = rows.unwrap();
        match row.next().expect("unwrapping result") {
            Some(v) => v.get(0).expect("unwrapping f64 to return"),
            None => panic!("There is no matching ISO symbol."),
        }
    }
}
