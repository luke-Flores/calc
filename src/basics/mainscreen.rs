/*
    calc - terminal based calculator
    Copyright (C) 2024  Luke Flores

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
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

    'inploop : loop{
        let option = mainscreen.getchar();
        match option{
            KEY_DOWN => mainscreen.change_hl(mainscreen.highligh_num+1),
            KEY_UP => mainscreen.change_hl(mainscreen.highligh_num-1),
            //q
            113 => break 'inploop,
            10 => { //enter
                match mainscreen.highligh_num{
                    //calc
                    0 => {
                        endwin();
                        crate::modules::text::textcalc();
                        initscr();
                    },
                    1 => todo!(),
                    _ => panic!("Somehow highlighted item was not an actual item present"),
                }
            },
            _ => (),
        }
        mainscreen.print_list();
    }
    mainscreen.delete();
    endwin();
}
