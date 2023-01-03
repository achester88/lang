pub struct ErrorHandler {
    cur_line: u32,
    pub cur_char: u32,
    lines_len: Vec<u32>,
    stage: String,
    pub context: Vec<String>,
}

/* TODO
* add fn to make error from line:char
* keep copy of og text
* add fn to reset errorhandler
* pass errorhandler to all struct in main
*/
impl ErrorHandler {
    pub fn new(l: u32, c: u32, name: String, context: Vec<String>) -> Self {
        Self {
            cur_line: l,
            cur_char: c,
            lines_len: vec![],
            stage: name,
            context: context,
        }
    }

    pub fn forwards(&mut self, size: u32) {
        self.cur_char += size;
    }

    pub fn backwards(&mut self, size: u32) {
        if self.cur_char == 0 {
            self.cur_char = 1
        }
        self.cur_char -= size;
    }

    pub fn next_line(&mut self) {
        self.cur_line += 1;
        self.lines_len.push(self.cur_line);
        self.cur_char = 0;
    }
    pub fn last_line(&mut self) {
        self.lines_len.pop();
        if self.lines_len.len() == 0 {
            //iff edgy caseing
            self.lines_len.push(0);
        }
        let at_char = self.lines_len.len();
        self.cur_char = self.lines_len[at_char - 1];
        self.cur_line -= 1;
    }

    pub fn throw_error(&mut self, ft: String) {
        //https://en.wikipedia.org/wiki/ANSI_escape_code
        let l = self.cur_line;
        let c = self.cur_char;
        println!("\x1b[31mError\x1b[0m: {} at {}:{}", ft, l, c);
        println!("");
        println!("{}", self.context[(l - 1) as usize]);
        println!("{: <1$}^", "", (c as usize));
        panic!();
    }

    pub fn throw_error_untraced(&mut self, ft: String) {
        println!("\x1b[31mError\x1b[0m: {} at {}", ft, self.stage);
        panic!();
    }

    pub fn get_pos(&mut self) -> (u32, u32) {
        return (self.cur_line, self.cur_char);
    }
}

/*
*Add file name and location
* ^^^ all part of term
*
*/