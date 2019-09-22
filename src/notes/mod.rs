mod all;
mod create;
mod delete;
mod get;
mod update;

pub use self::all::get_all_notes;
pub use self::create::create_note;
pub use self::delete::delete_note;
pub use self::get::get_note;
pub use self::update::update_note;

pub const NOTES: &str = "/api/notes/";
