#![allow(dead_code, unused_variables)]

/*
IN-FILE MODULE DECLARATION: all modules are declared in the same file as the root module.

mod database {
    pub enum Status {
        Connected,
        Interrupted
    }

    pub fn connect_to_database() -> Status {
        return Status::Connected;
    }
    
    pub fn get_user() {

    }
}

mod auth_utils {
    pub mod models {
        pub struct Credentials {
          username: String,
          password: String,
        }
    }

    // absolute path starting from the module root, fully qualified name is necessary to indicate the item scope.
    pub fn login(creds: models::Credentials) {
        // absolute path starting from the crate root, fully qualified name is necessary to indicate the item scope.
        crate::database::get_user()
    }
}
*/

// EXTERNAL MODULE DECLARATION: modules are declared in separate files.
// tells to rust compiler that rust_auth crate has 2 submodules: database and auth_utils.
// database module is a single file module, so rust compiler will look for database.rs file.
mod database;
// auth_utils module is a directory module, so rust compiler will look for auth_utils/mod.rs file.
// subsequent modules declared in auth_utils/mod.rs will be declared in the scope of auth_utils module.
mod auth_utils;
// Rust compiled will first look for a rs file, if it doesn't find it, it will look for a directory with the same name.

// bringing modules into the scope in order to don't use fully qualified names.
use auth_utils::models::Credentials;
// by prefixing a use clause with pub, we can re-export the item so it can be used in other modules.
pub use database::Status;

// using the modules
pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds)
    }
}
