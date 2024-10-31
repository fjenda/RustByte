use rust_byte::cpu::cpu::CPU;
use rust_byte::cpu::cpu_status::Status;

#[cfg(test)]
mod test {
    use crate::Status;
    use crate::CPU;

    #[test]
    fn test_0xa9_lda_load() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xa9, 0x05, 0x00]).expect("Failed to load program");
        cpu.interpret();

        assert_eq!(cpu.a.value(), 0x05);
        assert!(!cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xa9, 0x00, 0x00]).expect("Failed to load program");
        cpu.interpret();

        assert_eq!(cpu.a.value(), 0);
        assert!(cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_0xaa_tax() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xaa, 0x00]).expect("Failed to load program");
        cpu.a.set(69);
        cpu.interpret();

        assert_eq!(cpu.x.value(), 69);
        assert!(!cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_increase() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xc8, 0xe8, 0x00]).expect("Failed to load program");
        cpu.x.set(19);
        cpu.y.set(29);
        cpu.interpret();

        assert_eq!(cpu.x.value(), 20);
        assert_eq!(cpu.y.value(), 30);
    }

    #[test]
    fn test_increase_wrap_zero() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xc8, 0xc8, 0x00]).expect("Failed to load program");
        cpu.y.set(0xfe);
        cpu.interpret();

        assert_eq!(cpu.y.value(), 0);
        assert!(cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_decrease() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xca, 0x88, 0x00]).expect("Failed to load program");
        cpu.x.set(21);
        cpu.y.set(31);
        cpu.interpret();

        assert_eq!(cpu.x.value(), 20);
        assert_eq!(cpu.y.value(), 30);
    }

    #[test]
    fn test_decrease_wrap_zero() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0x88, 0x88, 0x00]).expect("Failed to load program");
        cpu.y.set(2);
        cpu.interpret();

        assert_eq!(cpu.y.value(), 0);
        assert!(cpu.status.is_set(Status::Zero));
        assert!(!cpu.status.is_set(Status::Negative));
    }

    #[test]
    fn test_clear_functions() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0x18, 0xd8, 0x58, 0xb8, 0x00]).expect("Failed to load program");
        cpu.status.add(Status::Carry);
        cpu.status.add(Status::Decimal);
        cpu.status.add(Status::InterruptDisable);
        cpu.status.add(Status::Overflow);
        cpu.interpret();

        assert!(!cpu.status.is_set(Status::Carry));
        assert!(!cpu.status.is_set(Status::Decimal));
        assert!(!cpu.status.is_set(Status::InterruptDisable));
        assert!(!cpu.status.is_set(Status::Overflow));
    }

    #[test]
    fn test_set_functions() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0x38, 0xf8, 0x78, 0x00]).expect("Failed to load program");
        cpu.interpret();

        assert!(cpu.status.is_set(Status::Carry));
        assert!(cpu.status.is_set(Status::Decimal));
        assert!(cpu.status.is_set(Status::InterruptDisable));
    }

    // TODO: Write tests for all instructions


    // TODO: Write tests for memory
    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.load_program(vec![0xa5, 0x10, 0x00]).expect("Failed to load program");
        cpu.memory.write(0x10, 0x55);
        cpu.interpret();

        assert_eq!(cpu.a.value(), 0x55);
    }
}
