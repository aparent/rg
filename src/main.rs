extern crate tcod;
extern crate rg;

use tcod::console;
use tcod::console::Console;
use tcod::console::Root;
use tcod::input;

use rg::util::Vec3;

fn render(con: &mut console::Root, pos : &Vec3) {
    con.clear();
    con.put_char(pos.x, pos.y, '@', console::BackgroundFlag::Set);
    con.flush();
}

fn init_console(con_x : i32, con_y: i32) -> Root {
    return console::Root::initializer()
               .size(con_x,con_y)
               .title("rg")
               .fullscreen(false)
               .init();
}

fn main() {
    let con_x = 160;
    let con_y = 120;
    let mut char_pos = Vec3{x:25,y:0,z:0} + Vec3{x:0,y:25,z:0};
    let mut exit = false;
    let mut con = init_console(con_x, con_y);
    while !(con.window_closed() || exit) {
        render(&mut con, &char_pos);
        let keypress = con.wait_for_keypress(true);
        match keypress.key {
            input::Key::Special(input::KeyCode::Escape) => exit = true,
            input::Key::Special(input::KeyCode::Up) => {
                if char_pos.y > 0 {
                    char_pos.y -= 1;
                }
            },
            input::Key::Special(input::KeyCode::Down) => {
                if char_pos.y < con_y - 1 {
                    char_pos.y += 1;
                }
            },
            input::Key::Special(input::KeyCode::Left) => {
                if char_pos.x > 0 {
                    char_pos.x -= 1;
                }
            },
            input::Key::Special(input::KeyCode::Right) => {
                if char_pos.x < con_x - 1 {
                    char_pos.x += 1;
                }
            },
            _ => {}
        }
    }
}

/*
mod item;

fn main() {
    let mut type_table : Vec<item::ItemType> = Vec::new();
    type_table.push(item::ItemType{name: String::from("foo")});
    type_table.push(item::ItemType{name: String::from("bar")});

    let lt_builder = item::LTBuilder::new(&type_table);
    println!("{:?}",type_table);
}*/
