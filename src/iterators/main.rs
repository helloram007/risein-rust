mod sports;

use sports::{ football, FootballPlayer };
fn main() {
    let my_player = FootballPlayer {
        name: "Simon".to_string(),
        age: 26,
    };

    football();
}