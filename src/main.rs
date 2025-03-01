mod ui;

fn main() {
    println!("Hello, world!");

    if let Err(e) = ui::board_ui::run() {
        eprintln!("Failed to start GUI: {}", e);
    }
}
