use macroquad::prelude::*;

#[macroquad::main("OdfizApp")]
async fn main() {
    loop {
        // Isi layar dengan warna merah terang
        clear_background(RED);

        // Gambar teks di tengah layar
        draw_text("HALO ODFIZ!", 20.0, 100.0, 60.0, WHITE);
        draw_text("Rust is Running!", 20.0, 180.0, 30.0, YELLOW);

        next_frame().await
    }
}
