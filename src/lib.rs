// modules
pub mod cli;
pub mod create_dir;
pub mod create_file;
pub mod create_subfiles;

// structs
pub use cli::Cli;

// error handeling
mod errors {
    pub mod create_error;
    pub mod split_error;
}

// utils
mod utils {
    pub mod split_string;
}
