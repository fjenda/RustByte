
pub enum InterruptType {
    NMI,
}

pub struct Interrupt {
    pub interrupt_type: InterruptType,
    pub cycles: u8,
    pub address: u16,
    pub flag_mask: u8,
}

pub const NMI: Interrupt = Interrupt {
    interrupt_type: InterruptType::NMI,
    cycles: 2,
    address: 0xFFFA,
    flag_mask: 0x20,
};