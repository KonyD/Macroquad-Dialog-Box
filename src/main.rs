use macroquad::prelude::*;
use substring::Substring;

struct DialogBox {
    x: f32,
    y: f32,
    dialog: Vec<String>,
    img: Texture2D,
    strIndex: usize,
}
impl DialogBox {
    fn update(&mut self) {
        if is_key_pressed(KeyCode::Enter) {
            if self.strIndex < self.dialog.len() {
                self.strIndex += 1;
            }
            if self.strIndex >= self.dialog.len() {
                self.strIndex = 0;
            }
        }

        draw_texture(self.img, self.x, self.y, WHITE);
        draw_text(&self.dialog[self.strIndex], 50.0, 390.0, 40.0, WHITE);
    }
}

#[macroquad::main("dialog box")]
async fn main() {
    
    let mut dialogBox = DialogBox {
        x: 25.0,
        y: 340.0,
        dialog: vec!["Hello!".to_string(), "I'm TravelerInTheDark.".to_string()],
        img: load_texture("assets/images/dialog_box.png").await.unwrap(),
        strIndex: 0,
    };

    loop {
        clear_background(Color {r: 0.40, g: 0.40, b: 1.0, a: 1.0});
        
        dialogBox.update();

        next_frame().await
    }
}