use std::collections::HashMap;
use crate::cpu::cpu::CPU;
use crate::cpu::instructions::{Instruction, INSTRUCTION_MAP};
use crate::cpu::addressing::Addressing;
use crate::ppu::cartridge::Cartridge;

pub fn trace(cpu: &mut CPU) -> String {
    let code = cpu.read(cpu.prog_counter);
    let ops: &Instruction = INSTRUCTION_MAP.get(&code).unwrap();

    let begin = cpu.prog_counter;
    let mut hex_dump = vec![];
    hex_dump.push(code);

    let (mem_addr, stored_value) = match ops.mode {
        Addressing::Immediate | Addressing::None => (0, 0),
        _ => {
            let (addr, _) = cpu.get_param_address(&ops.mode, begin + 1);
            (addr, cpu.read(addr))
        }
    };

    let tmp = match ops.bytes {
        1 => match ops.address {
            0x0a | 0x4a | 0x2a | 0x6a => format!("A "),
            _ => String::from(""),
        },
        2 => {
            let address: u8 = cpu.read(begin + 1);
            // let value = cpu.mem_read(address));
            hex_dump.push(address);

            match ops.mode {
                Addressing::Immediate => format!("#${:02x}", address),
                Addressing::ZeroPage => format!("${:02x} = {:02x}", mem_addr, stored_value),
                Addressing::ZeroPageX => format!(
                    "${:02x},X @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                Addressing::ZeroPageY => format!(
                    "${:02x},Y @ {:02x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                Addressing::IndirectX => format!(
                    "(${:02x},X) @ {:02x} = {:04x} = {:02x}",
                    address,
                    (address.wrapping_add(cpu.x.value())),
                    mem_addr,
                    stored_value
                ),
                Addressing::IndirectY => format!(
                    "(${:02x}),Y = {:04x} @ {:04x} = {:02x}",
                    address,
                    (mem_addr.wrapping_sub(cpu.y.value() as u16)),
                    mem_addr,
                    stored_value
                ),
                Addressing::None => {
                    // assuming local jumps: BNE, BVS, etc....
                    let address: usize =
                        (begin as usize + 2).wrapping_add((address as i8) as usize);
                    format!("${:04x}", address)
                }

                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 2. code {:02x}",
                    ops.mode, ops.address
                ),
            }
        }
        3 => {
            let address_lo = cpu.read(begin + 1);
            let address_hi = cpu.read(begin + 2);
            hex_dump.push(address_lo);
            hex_dump.push(address_hi);

            let address = cpu.read_u16(begin + 1);

            match ops.mode {
                Addressing::None => {
                    if ops.address == 0x6c {
                        //jmp indirect
                        let jmp_addr = if address & 0x00FF == 0x00FF {
                            let lo = cpu.read(address);
                            let hi = cpu.read(address & 0xFF00);
                            (hi as u16) << 8 | (lo as u16)
                        } else {
                            cpu.read_u16(address)
                        };

                        // let jmp_addr = cpu.mem_read_u16(address);
                        format!("(${:04x}) = {:04x}", address, jmp_addr)
                    } else {
                        format!("${:04x}", address)
                    }
                }
                Addressing::Absolute => format!("${:04x} = {:02x}", mem_addr, stored_value),
                Addressing::AbsoluteX => format!(
                    "${:04x},X @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                Addressing::AbsoluteY => format!(
                    "${:04x},Y @ {:04x} = {:02x}",
                    address, mem_addr, stored_value
                ),
                _ => panic!(
                    "unexpected addressing mode {:?} has ops-len 3. code {:02x}",
                    ops.mode, ops.address
                ),
            }
        }
        _ => String::from(""),
    };

    let hex_str = hex_dump
        .iter()
        .map(|z| format!("{:02x}", z))
        .collect::<Vec<String>>()
        .join(" ");
    let asm_str = format!("{:04x}  {:8}  {: >4} {}", begin, hex_str, ops.name, tmp)
        .trim_end()
        .to_string();

    format!(
        "{:47} A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x}",
        asm_str, cpu.a.value(), cpu.x.value(), cpu.y.value(), cpu.status.value, cpu.stack_pointer,
    )
        .to_ascii_uppercase()
}

struct TestRom {
    header: Vec<u8>,
    trainer: Option<Vec<u8>>,
    pgp_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}
fn create_rom(rom: TestRom) -> Vec<u8> {
    let mut result = Vec::with_capacity(
        rom.header.len()
            + rom.trainer.as_ref().map_or(0, |t| t.len())
            + rom.pgp_rom.len()
            + rom.chr_rom.len(),
    );

    result.extend(&rom.header);
    if let Some(t) = rom.trainer {
        result.extend(t);
    }
    result.extend(&rom.pgp_rom);
    result.extend(&rom.chr_rom);

    result
}

pub fn test_rom() -> Cartridge {
    let test_rom = create_rom(TestRom {
        header: vec![
            0x4E, 0x45, 0x53, 0x1A, 0x02, 0x01, 0x31, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        ],
        trainer: None,
        pgp_rom: vec![1; 2 * 16384],
        chr_rom: vec![2; 1 * 8192],
    });

    Cartridge::new(test_rom).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cpu::bus::Bus;
    use crate::ppu::ppu::PPU;

    #[test]
    fn test_format_trace() {
        let mut bus = Bus::new(test_rom(), |ppu: &PPU| {});
        bus.write(100, 0xa2);
        bus.write(101, 0x01);
        bus.write(102, 0xca);
        bus.write(103, 0x88);
        bus.write(104, 0x00);

        let mut cpu = CPU::new(bus);
        cpu.prog_counter = 0x64;
        cpu.a.set(1);
        cpu.x.set(2);
        cpu.y.set(3);
        let mut result: Vec<String> = vec![];
        cpu.interpret_callback(|cpu| {
            result.push(trace(cpu));
        });

        assert_eq!(
            "0064  A2 01     LDX #$01                        A:01 X:02 Y:03 P:24 SP:FD",
            result[0]
        );
        assert_eq!(
            "0066  CA        DEX                             A:01 X:01 Y:03 P:24 SP:FD",
            result[1]
        );
        assert_eq!(
            "0067  88        DEY                             A:01 X:00 Y:03 P:26 SP:FD",
            result[2]
        );
    }

    #[test]
    fn test_format_mem_access() {
        let mut bus = Bus::new(test_rom(), |ppu: &PPU| {});
        // ORA ($33), Y
        bus.write(100, 0x11);
        bus.write(101, 0x33);

        //data
        bus.write(0x33, 00);
        bus.write(0x34, 04);

        //target cell
        bus.write(0x400, 0xAA);

        let mut cpu = CPU::new(bus);
        cpu.prog_counter = 0x64;
        cpu.y.set(0);
        let mut result: Vec<String> = vec![];
        cpu.interpret_callback(|cpu| {
            result.push(trace(cpu));
        });
        assert_eq!(
            "0064  11 33     ORA ($33),Y = 0400 @ 0400 = AA  A:00 X:00 Y:00 P:24 SP:FD",
            result[0]
        );
    }
}
