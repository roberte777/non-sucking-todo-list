use std::io;

use non_sucking_todo_list::app::ui::start_ui;
fn main() -> Result<(), io::Error> {
    start_ui()?;
    Ok(())
}
