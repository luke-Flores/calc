use ncurses::*;
use crate::basics::menu;


pub fn ui(){
    let mut y: i32 = 0;
    let mut x: i32 = 0;

    initscr();
    cbreak();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    getmaxyx(stdscr(), &mut y, &mut x);
    keypad(stdscr(), true);

    let mut mainscreen = menu::Menu::new((0, 0), (y, x));
    mainscreen.word_list = vec!(String::from("calc"), String::from("equations"));
    mainscreen.print_list();
    refresh();

    loop{
        let option = mainscreen.getchar();
        match option{
            KEY_DOWN => mainscreen.change_hl(mainscreen.highligh_num+1),
            KEY_UP => mainscreen.change_hl(mainscreen.highligh_num-1),
            //q
            113 => break,
            10 => { //enter
                match mainscreen.highligh_num{
                    0 => todo!(),
                    1 => todo!(),
                    _ => panic!("Somehow highlighted item was not an actual item present"),
                }
            },
            _ => (),
        }
    }


    mainscreen.delete();
    endwin();
}
