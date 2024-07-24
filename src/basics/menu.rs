use ncurses::*;

pub struct Menu{
    start_coord: (i32, i32),
    end_coord: (i32, i32),
    window: WINDOW,

    pub word_list: Vec<String>,
    pub highligh_num: i32,
}

// many methods here assume that the screen has enough space to print all items in the list. For
// most screens this should be fine. Maybe fix later
impl Menu {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Menu{
        let temp_win = newwin(end.0 - start.0, end.1 - end.0, start.0, start.1);
        let temp = Menu{
            start_coord: (start.0, start.1), end_coord: (end.0, end.1),
            highligh_num: 0,
            window: temp_win,
            word_list: Vec::new(),
        };
        keypad(temp_win, true);
        refresh();
        return temp;
    }
    // I don't belive there are deconstructors in rust but if so then this needs to be one
    // can also be used to make the window dormant
    pub fn delete(&self){
        delwin(self.window);
        refresh();
    }

    pub fn reconstruct(&mut self){
        self.window=newwin(self.end_coord.0 - self.start_coord.0, self.end_coord.1 - self.start_coord.1, self.end_coord.0, self.end_coord.1);
        refresh();
    }

    pub fn print_list(&self){
        let mut line_num: i32 = 0;
        for word in self.word_list.iter(){
            wmove(self.window, line_num, 0);
            wclrtoeol(self.window);
            if line_num == self.highligh_num{
                wattron(self.window, A_REVERSE);
                let _ = waddstr(self.window, word);
                wattroff(self.window, A_REVERSE);
            }
            else{
                let _ = waddstr(self.window, word);
            }
            line_num+=1;
        }
        wrefresh(self.window);
    }

    pub fn getchar(&self) -> i32{
        wgetch(self.window)
    }

    pub fn change_hl(&mut self, mut new_hl_num: i32){
        //fix boundaries to be correct
        if new_hl_num>=self.word_list.len() as i32{
            new_hl_num = 0;
        }
        else if new_hl_num == -1{
            new_hl_num=self.word_list.len() as i32 -1 ;
        }

        //reset old highlighted word
        mv(self.highligh_num as i32, self.start_coord.1);
        wclrtoeol(self.window);
        let _ = mvwprintw(self.window, self.highligh_num, 0, self.word_list[self.highligh_num as usize].as_str());

        // set new highlighted word
        self.highligh_num = new_hl_num;
        mv(self.highligh_num, self.start_coord.1);
        wclrtoeol(self.window);
        wattron(self.window, A_REVERSE);
        let _ = mvwprintw(self.window, self.highligh_num, 0, self.word_list[self.highligh_num as usize].as_str());
        wattroff(self.window, A_REVERSE);
    }
}
