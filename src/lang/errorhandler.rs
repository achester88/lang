pub struct ErrorHandler {
    cur_line: u32,
    pub cur_char: u32,
    lines_len: Vec<u32>,
    stage: String,
}

impl ErrorHandler {
    pub fn new(l: u32, c: u32, stage: String) -> Self {
        return Self { // both line and char start at 1 to fix
            cur_line: l,
            cur_char: c,
            lines_len: vec!(),
            stage: stage,
        };
    }

    pub fn forwards(&mut self, size: u32) {
        self.cur_char += size;
    }

    pub fn backwards(&mut self, size: u32) {
      if self.cur_char == 0 { self.cur_char = 1}
      self.cur_char -= size;
    }

    pub fn next_line(&mut self) {
        self.cur_line += 1;
        self.lines_len.push(self.cur_line);
        self.cur_char = 0;
    }
    pub fn last_line(&mut self) {
        self.lines_len.pop();
        if self.lines_len.len() == 0 { //iff edgy caseing
          self.lines_len.push(0);
        }
        let at_char = self.lines_len.len();
        self.cur_char = self.lines_len[at_char-1];
        self.cur_line -= 1;
    }

    pub fn throw_error(&mut self, ft: String) {
        //https://en.wikipedia.org/wiki/ANSI_escape_code
      let l = self.cur_line;
      let c = self.cur_char;
      println!(
            "\x1b[31mError\x1b[0m: {} at {}:{}",ft, l, c);
      panic!();
    }
}
