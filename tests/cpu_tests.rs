use rust_byte::cpu::cpu::CPU;
use rust_byte::cpu::cpu_status::Status;

#[cfg(test)]
mod test {
    use crate::Status;
    use crate::CPU;

    #[test]
    fn test_0xa9_lda_load() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.a.value(), 0x05);
        assert!(!cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);

        assert_eq!(cpu.a.value(), 0);
        assert!(cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.a.set(69);
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.x.value(), 69);
        assert!(!cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xe8_inx_simple() {
        let mut cpu = CPU::new();
        cpu.x.set(19);
        cpu.interpret(vec![0xe8, 0x00]);

        assert_eq!(cpu.x.value(), 20);
        assert!(!cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xe8_inx_wrap_zero() {
        let mut cpu = CPU::new();
        cpu.x.set(0xfe);
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.x.value(), 0);
        assert!(cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_clear_functions() {
        let mut cpu = CPU::new();
        cpu.status.add(Status::Carry);
        cpu.status.add(Status::Decimal);
        cpu.status.add(Status::InterruptDisable);
        cpu.status.add(Status::Overflow);
        cpu.interpret(vec![0x18, 0xd8, 0x58, 0xb8, 0x00]);

        assert!(!cpu.status.is_set(Status::Carry));
        assert!(!cpu.status.is_set(Status::Decimal));
        assert!(!cpu.status.is_set(Status::InterruptDisable));
        assert!(!cpu.status.is_set(Status::Overflow));
    }

    // TODO: Write tests for all instructions
}