use std::io;

use non_sucking_todo_list::app::{ui::start_ui, App};
fn main() -> Result<(), io::Error> {
    let mut app = App::new();
    start_ui(&mut app)?;
    Ok(())
}
