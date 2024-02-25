#[allow(dead_code)]
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode{
    fn new(fg:u8,bg:u8) -> ColorCode{
        ColorCode(bg << 4 | fg)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char:u8,
    color:ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer{
    chars:[[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    column_position: usize,
    color_code :ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_str(&mut self, string:&str){
        for byte in string.bytes(){
            match byte{
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
    pub fn write_byte(&mut self, byte:u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position > BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color = self.color_code;

                self.buffer.chars[row][col] = ScreenChar{
                    ascii_char: byte,
                    color
                };
                self.column_position += 1;
            }
        }
    }
    fn new_line(&mut self){

    }
}

pub fn print(){
    let mut wr =  Writer {
        column_position:0,
        color_code:ColorCode::new(Color::White as u8,Color::Black as u8),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer)}
    };

    wr.write_byte(b'H');
    wr.write_str("ELLO ");
    wr.write_str("Wôrld!");
}