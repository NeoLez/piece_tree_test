use unicode_segmentation::UnicodeSegmentation;

pub struct FixedSizeString{
    fixed_string : Box<[u8]>,
    bytes_left : usize,
    current_index : usize,
    line_starts : Vec<usize>,
    length : usize,
    length_graphemes : usize,
}

impl FixedSizeString {
    fn update_line_starts_with_appended_str(&mut self, s : &str) {
        for (i, c) in s.char_indices() {
            if c == '\n' {
                self.line_starts.push(i+self.len()-s.len()+1)
            }
        }
    }
    fn update_length_graphemes_with_appended_str(&mut self, s : &str) {
        self.length_graphemes += s.graphemes(true).count();
    }
    fn update_length_with_appended_str(&mut self, s : &str) {
        self.length += s.chars().count();
    }
    fn update_range_with_appended_str(&mut self, s : &str) {
        self.bytes_left -= s.len();
        self.current_index += s.len();
    }

    pub fn new(capacity : usize) -> Self {
        FixedSizeString {
            fixed_string : vec![0 as u8; capacity].into_boxed_slice(),
            bytes_left : capacity,
            current_index : 0,
            line_starts : vec![],
            length : 0,
            length_graphemes : 0,
        }
    }
    
    pub fn from_string(string : String) -> Self {
        let mut r = FixedSizeString::new(string.len());
        r.append(&string);
        r
    }
    pub fn append_whole(&mut self, string : &String) -> Result<(), ()> {
        let length = string.len();
        if length <= self.bytes_left {
            self.fixed_string[self.current_index..self.current_index+length].copy_from_slice(string.as_bytes());
            self.update_length_graphemes_with_appended_str(&string);
            self.update_length_with_appended_str(&string);
            self.update_line_starts_with_appended_str(&string);
            self.update_range_with_appended_str(&string);
            return Result::Ok(());
        }
        Result::Err(())
    }
    pub fn append(&mut self, string : &String) -> usize {
        let length = string.len();
        if length <= self.bytes_left {
            self.fixed_string[self.current_index..self.current_index+length].copy_from_slice(string.as_bytes());
            self.update_length_graphemes_with_appended_str(&string);
            self.update_length_with_appended_str(&string);
            self.update_line_starts_with_appended_str(&string);
            self.update_range_with_appended_str(&string);
            return length;
        }else {
            let mut until = 0;
            for (i, _) in string.char_indices() {
                if i > self.bytes_left {
                    break;
                }
                until = i;
            }
            let u8_slice = &string.as_bytes()[0..until];
            self.fixed_string[self.current_index..self.current_index+until].copy_from_slice(u8_slice);

            let str_slice;
            unsafe {
                str_slice = std::str::from_utf8_unchecked(u8_slice);
            }

            self.update_length_graphemes_with_appended_str(str_slice);
            self.update_length_with_appended_str(str_slice);
            self.update_line_starts_with_appended_str(str_slice);
            self.update_range_with_appended_str(str_slice);
            until
        }
    }
    pub fn full_string(&self) -> &str{
        unsafe {
            let slice = std::slice::from_raw_parts(self.fixed_string.as_ptr(), self.current_index);
            std::str::from_utf8_unchecked(slice)
        }
    }
    /// Slices the string without splitting graphemes. The range is (start, length), in graphemes
    pub fn string_slice_graphemes(&self, range : (usize, usize)) -> &str
    {
        let s = self.full_string();

        let mut graphemes = s.grapheme_indices(true);
        
        if let Some((i, s0)) = graphemes.nth(range.0) {
            return match range.1 {
                0 => "",
                1 => s0,
                n => {
                    let res;
                    if let Some((i2, s2)) = graphemes.nth(n - 2) {
                        res = &s[i..(i2+s2.len())]
                    }else {
                        panic!();
                    }
                    res
                }
            };
        }
        panic!()
    }
    /// Slices the string, can split graphemes. The range is (start, length), in chars
    pub fn string_slice_chars(&self, range : (usize, usize)) -> &str {
        let s = self.full_string();

        let mut chars = s.char_indices();
        
        if let Some((i, s0)) = chars.nth(range.0) {
            return match range.1 {
                0 => "",
                1 => &s[i..(i+s0.len_utf8())],
                n => {
                    let res;
                    if let Some((i2, s2)) = chars.nth(n - 2) {
                        res = &s[i..(i2+s2.len_utf8())]
                    }else {
                        panic!();
                    }
                    res
                }
            };
        }
        panic!()
    }
    /*fn calculate_line_starts(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new(); 
        let s = self.full_string();
        for (i, c) in s.char_indices() {
            if c == '\n' {
                res.push(i+c.len_utf8());
            }
        }
        res
    }*/
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn len_graphemes(&self) -> usize {
        self.length_graphemes
    }
    pub fn line_starts(&self) -> &Vec<usize> {
        &self.line_starts
    }
}