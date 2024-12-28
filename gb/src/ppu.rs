#[derive(Debug)]
pub struct Ppu {
    // Picture processing unit.
    // The Game Boy has a 160x144 LCD.
    pub screen: [Color; 160 * 144],

    pub cgb: bool,

    // Top left coordinates of the visible background from
    // the 256x256 background tilemap.
    pub top_left_background: Coord,

    // The window is sort of a second background on top of the background.
    // It is fairly limited: it has no transparency, it's always a rectangle,
    // and only the top-left pixel coordinate can be controlled.
    //
    // Possible usage include a fixed status bar in an otherwise scrolling game like Mario.
    //
    // The window is visible (if enabled) when both coordinates are in the range
    // x: [0, 166], y: [0, 143],
    // thus (7, 0) place the window at the top left of the screen,
    // completely covering the background.
    pub top_left_window: Coord,

    pub scanline: Coord,

    // 0xff40: LCD control.

    // Controls whether the LCD is on and the PPU is active.
    // Setting it to zero turns both off,
    // which grants immediate and full access to VRAM, OAM, etc.
    pub on: bool,

    // Controls which background map the window uses for rendering.
    pub window_tile_map: u16,
    pub bg_tile_map: u16,

    // Controls whether the window shall be displayed or not,
    // overriden by `on`.
    pub window_enable: bool,

    // Controls which addressing mode the background and window use to pick tiles.
    pub bg_window_addressing_mode: u16,

    pub obj_size: Coord,
    pub obj_enable: bool,

    // Enable in regular mode, priority in CGB.
    pub bg_window_enable_priority: bool,

    // Let's just deal with the original Game Boy palette first.
    pub palette: [Color; 4],
}

#[derive(Debug)]
pub struct Coord {
    x: u8,
    y: u8,
}

impl Coord {
    pub fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Color {
    pub fn from(num: u8) -> Color {
        match num {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Invalid color"),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            screen: [Color::White; 160 * 144],

            cgb: false,
            top_left_background: Coord::default(),
            top_left_window: Coord::default(),

            scanline: Coord::default(),

            on: false,

            window_tile_map: 0x9800,
            bg_tile_map: 0x9800,

            window_enable: false,

            bg_window_addressing_mode: 0x8800,

            obj_size: Coord { x: 8, y: 8 },
            obj_enable: false,

            bg_window_enable_priority: false,

            palette: [Color::White; 4],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xff42 => self.top_left_background.y,
            0xff43 => self.top_left_background.x,
            0xff4a => self.top_left_window.y,
            0xff4b => self.top_left_window.x,
            0xff44 => self.scanline.x,
            _ => todo!("PPU read {:#X}", addr),
        }
    }

    pub fn handle(&mut self, addr: u16, byte: u8) {
        match addr {
            0xff40 => {
                // Set up LCD control register values.
                self.on = byte & 0x80 != 0;

                self.window_tile_map = match byte & 0x40 {
                    0 => 0x9800,
                    0x40 => 0x9c00,
                    _ => unreachable!(),
                };

                self.bg_tile_map = match byte & 0x08 {
                    0 => 0x9800,
                    0x08 => 0x9c00,
                    _ => unreachable!(),
                };

                self.window_enable = byte & 0x20 != 0;

                self.bg_window_addressing_mode = match byte & 0x10 {
                    0 => 0x8800,
                    0x10 => 0x8000,
                    _ => unreachable!(),
                };

                self.obj_size = match byte & 0x04 {
                    0 => Coord { x: 8, y: 8 },
                    0x04 => Coord { x: 8, y: 16 },
                    _ => unreachable!(),
                };

                self.obj_enable = byte & 0x02 != 0;

                self.bg_window_enable_priority = byte & 0x01 != 0;

                dbg!(self);
            }
            0xff42 => self.top_left_background.y = byte,
            0xff43 => self.top_left_background.x = byte,
            0xff4a => self.top_left_window.y = byte,
            0xff4b => self.top_left_window.x = byte,
            0xff47 => {
                self.palette[0] = Color::from((byte & 0xc0) >> 6);
                self.palette[1] = Color::from((byte & 0x30) >> 4);
                self.palette[2] = Color::from((byte & 0xc) >> 2);
                self.palette[3] = Color::from(byte & 0x03);
            }
            _ => todo!("PPU address {:#X}", addr),
        }
    }
}
