use pgrust_core::config::DataSource;
use pgrust_core::tasks::backup_db;

pub fn backup(ds: &DataSource, db_name: &String, schema_name: &String) {
    backup_db(ds, db_name, schema_name, |line| {
        println!("Progress: {}", line);
    });
}
