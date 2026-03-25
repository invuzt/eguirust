use macroquad::prelude::*;

#[macroquad::main("OdfizApp")]
async fn main() {
    loop {
        // Coba warna hijau dulu (Green) untuk menandai versi baru
        clear_background(GREEN);
        
        draw_text("ODFIZ BERHASIL!", 40.0, 100.0, 50.0, BLACK);
        draw_text("Tekan layar untuk keluar", 40.0, 160.0, 30.0, DARKGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            break;
        }

        next_frame().await
    }
}
