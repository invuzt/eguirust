use macroquad::prelude::*;

#[macroquad::main("OdfizApp")]
async fn main() {
    loop {
        clear_background(RED);

        draw_text("BERHASIL!", 40.0, 100.0, 60.0, WHITE);
        draw_text("Rust + Macroquad", 40.0, 160.0, 30.0, YELLOW);
        
        next_frame().await
    }
}
