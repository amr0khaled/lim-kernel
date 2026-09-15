pub static PIC1: u16 = 0x20;
pub static PIC2: u16 = 0xA0;
pub static PIC1_COMMAND: u16 = PIC1;
pub static PIC1_DATA: u16 = PIC1 + 1;
pub static PIC2_COMMAND: u16 = PIC2;
pub static PIC2_DATA: u16 = PIC2 + 1;
pub static PIC1_INTERRUPT: u8 = 0x20;
pub static PIC2_INTERRUPT: u8 = 0x28;
pub static PIT_COMMAND: u16 = 0x43;
pub static PIT_CHANNEL_0: u16 = 0x40;

/// COMMANDS
/// End of Intterupt command
pub static PIC_EOI: u8 = 0x20;
/// Initialisation command
pub static PIC_INIT: u8 = 0x11;
/// Vector offset ICW2
/// Wiring Master and slave ICW3
/// Give info about env ICW4

/// Present ICW4
pub static ICW1_ICW4: u8 = 0b1;
/// Single (casecade) mode
pub static ICW1_SINGLE: u8 = 0b10;
/// Call address interval 4 (8)
pub static ICW1_INTERVAL4: u8 = 0b100;
/// Level triggered (edge) mode
pub static ICW1_LEVEL: u8 = 0b1000;
/// Initialization
pub static ICW1_INIT: u8 = 0b10000;

/// 8086/88 mode
pub static ICW4_8086: u8 = 0b1;
/// Auto (normal) EOI
pub static ICW4_AUTO: u8 = 0b10;
/// Buffered mode/slave
pub static ICW4_BUF_SLAVE: u8 = 0b1000;
/// Buffered mode/master
pub static ICW4_BUF_MASTER: u8 = 0b1100;
/// Special fully nested (not)
pub static ICW4_SFNM: u8 = 0b10000;

pub static CASECADE_IRQ: u8 = 2;
