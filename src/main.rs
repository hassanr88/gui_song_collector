/*
By: Hassan Rana
Date: 2026-10-14
Program Details: a program to help you to keep track of your Song collection.
*/

mod ui;
mod utils;

use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::text_button::TextButton;
use crate::ui::text_input::TextInput;
use macroquad::prelude::*;
use crate::ui::listview::ListView;
use crate::ui::messagebox::MessageBox;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Song Collector".to_string(),
        window_width: 1000,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let lbl_songs = Label::new("Your Songs", 400.0, 100.0, 50);
    let mut songs = vec!["Thriller - Michael Jackson".to_string(), "Stayin' Alive - Bee Gees".to_string(), "Bohemian Rhapsody - Queen".to_string(), "Like a Rolling Stone - Bob Dylan".to_string(), "I Will Always Love You - Whitney Houston".to_string()];
    let mut song_list = ListView::new(&songs, 100.0, 200.0, 30);
    let mut txt_song = TextInput::new(100.0, 400.0, 400.0, 50.0, 25.0);
    let mut btn_exit = TextButton::new(750.0, 700.0, 200.0, 50.0, "Exit", RED, ORANGE, 30);
    let btn_addsong = TextButton::new(100.0, 500.0, 200.0, 50.0, "Add Song", GRAY, GREEN, 30);
    let btn_removesong = TextButton::new(350.0, 500.0, 200.0, 50.0, "Remove Song", GRAY, RED, 30);
    let mut error_box = MessageBox::info("ERROR", "Please Type a Song Name.");
    song_list.with_max_visible_items(5);
    txt_song.set_max_chars(50);
    loop {
        clear_background(WHITE);
        draw_grid(50.0, BROWN);
        if btn_addsong.click() {
            if !txt_song.get_text().is_empty() {
                songs.push(txt_song.get_text().to_string());
                song_list.clear();
                song_list.add_items(&songs);
            }
            else {
                error_box.show();
            }
        }
        if btn_removesong.click() {
            let mut found = false;
            if !txt_song.get_text().is_empty() {
                for i in 0..songs.len() {
                    if songs[i] == txt_song.get_text() {
                        songs.remove(i);
                        found == true;
                        break;
                    }
                }
                if found == false {
                    
                }
                song_list.clear();
                song_list.add_items(&songs);
            }
            else {
                error_box.show();
            }
        }
        if btn_exit.click() {
            break;
        }
        txt_song.draw();
        lbl_songs.draw();
        song_list.draw();
        error_box.draw();
        next_frame().await;
    }
}
