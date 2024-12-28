use crate::util::masked;

#[derive(Debug)]
pub struct Apu {
    pub on: bool,
    pub channel_one: ChannelOne,
    pub channel_two: ChannelTwo,
    pub channel_three: ChannelThree,
    pub channel_four: ChannelFour,

    // External channels.
    pub vin_left: bool,
    pub vin_right: bool,

    // Master volume, indexed at zero.
    pub left_volume: u8,
    pub right_volume: u8,
}

#[derive(Debug)]
struct ChannelOne {
    wave_duty: u8,
    length_timer: u8,

    initial_volume: u8,

    // 0 = decrease volume over time,
    // 1 = increase volume over time.
    envelope_direction: bool,

    // The envelope ticks at 64Hz,
    // and the channel's envelope will be increased/decreased
    // every sweep pace of these ticks.
    //
    // 0 = disable envelope
    sweep_pace: u8,

    pan_left: bool,
    pan_right: bool,
}

#[derive(Debug)]
struct ChannelTwo {
    pan_left: bool,
    pan_right: bool,
}

#[derive(Debug)]
struct ChannelThree {
    pan_left: bool,
    pan_right: bool,
}

#[derive(Debug)]
struct ChannelFour {
    pan_left: bool,
    pan_right: bool,
}

impl ChannelOne {
    fn init() -> Self {
        Self {
            wave_duty: 0,
            length_timer: 0,

            initial_volume: 0,
            envelope_direction: false,
            sweep_pace: 0,

            pan_left: false,
            pan_right: false,
        }
    }
}

impl ChannelTwo {
    fn init() -> Self {
        Self {
            pan_left: false,
            pan_right: false,
        }
    }
}

impl ChannelThree {
    fn init() -> Self {
        Self {
            pan_left: false,
            pan_right: false,
        }
    }
}

impl ChannelFour {
    fn init() -> Self {
        Self {
            pan_left: false,
            pan_right: false,
        }
    }
}

impl Apu {
    pub fn new() -> Self {
        Self {
            on: false,
            channel_one: ChannelOne::init(),
            channel_two: ChannelTwo::init(),
            channel_three: ChannelThree::init(),
            channel_four: ChannelFour::init(),

            vin_left: false,
            vin_right: false,

            left_volume: 0,
            right_volume: 0,
        }
    }

    pub fn handle(&mut self, addr: u16, byte: u8) {
        match addr {
            0xff11 => {
                self.channel_one.wave_duty = byte >> 6;
                self.channel_one.length_timer = byte << 2;
            }
            0xff12 => {
                self.channel_one.initial_volume = byte >> 4;
                self.channel_one.envelope_direction = masked(byte, 3) == 1;
                self.channel_one.sweep_pace = byte << 5;
            }
            0xff24 => {
                self.vin_left = byte & 0x80 != 0;
                self.vin_right = masked(byte, 3) != 0;

                self.left_volume = (byte >> 4) & 0x7;
                self.right_volume = byte & 0x7;

                dbg!(byte, self);
            }
            0xff25 => {
                self.channel_four.pan_left = byte & 0x80 != 0;
                self.channel_three.pan_left = byte & 0x40 != 0;
                self.channel_two.pan_left = byte & 0x20 != 0;
                self.channel_one.pan_left = byte & 0x10 != 0;

                self.channel_four.pan_right = byte & 0x08 != 0;
                self.channel_three.pan_right = byte & 0x04 != 0;
                self.channel_two.pan_right = byte & 0x02 != 0;
                self.channel_one.pan_right = byte & 0x01 != 0;
            }
            0xff26 => self.on = true,
            _ => todo!("{:#X}", addr),
        }
    }
}
