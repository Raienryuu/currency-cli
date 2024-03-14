use currency_cli::{get_arguments, match_args};
use currency_cli::exchange_rates::database::DbConnection;


fn main() {
    let db = DbConnection::new();
    let args: Vec<String> = get_arguments();
    match_args(args, db);
}
