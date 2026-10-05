use pgrust_core::config::DataSource;
use pgrust_core::tasks::{RestoreOptions, backup_db, restore};

pub fn backup(ds: &DataSource, db_name: &String, schema_name: &String) {
    backup_db(ds, db_name, schema_name, |line| {
        println!("Progress: {}", line);
    });
}
pub fn restore_db(ds: &DataSource, opt: &RestoreOptions) {
    restore(&ds, &opt, |line| {
        println!("Progress: {}", line);
    });
}
