#[cfg(test)]
mod tests {
    use crate::cpu;
    use crate::nes::{Nes, StepResult};
    use cpu::{AddressMode, AddressResult, Op};
    use std::fmt::{self};
    use std::fs;

    #[test]
    fn nestest() {
        let test_folder = format!("{}/src/test/test_data", env!("CARGO_MANIFEST_DIR"));
        let filepath = format!("{}/nestest.nes", test_folder);
        let rom = fs::read(filepath).expect("cannot open file");
        let mut nes = Nes::new(&rom).expect("failed to create NES");
        let mut step_results: Vec<String> = Vec::new();

        loop {
            let result = nes.step();
            match result.cpu.op_name {
                Op::BRK => break,
                _ => step_results.push(format!("{}", result)),
            }
        }

        let log = fs::read_to_string(format!("{}/nestest.log", test_folder)).expect("err");
        let nestest_log: Vec<String> = log.lines().map(String::from).collect();

        for (step, log) in step_results.iter().zip(nestest_log.iter()) {
            assert_eq!(step, log);
        }
    }

    impl<'a> fmt::Display for StepResult {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let byte_str = generate_byte_str(&self.cpu.bytes_fetched);
            let name_str = generate_name_string(&self.cpu);
            let addr_str = match &self.cpu.address_result {
                AddressResult::Memory(mem) => match mem.mode {
                    AddressMode::Absolute => format_absolute(mem.effective_address, &self.cpu),
                    AddressMode::AbsoluteX => {
                        format_absolute_x(mem.base_address, mem.effective_address, &self.cpu)
                    }
                    AddressMode::AbsoluteY => {
                        format_absolute_y(mem.base_address, mem.effective_address, &self.cpu)
                    }
                    AddressMode::Immediate => format_immediate(&self.cpu.bytes_read),
                    AddressMode::Indirect => format_indirect(
                        mem.base_address,
                        mem.effective_address,
                        &self.cpu.bytes_read,
                    ),
                    AddressMode::IndirectX => format_indirect_x(
                        mem.base_address,
                        mem.effective_address,
                        mem.indexed_offset.unwrap(),
                        &self.cpu,
                    ),
                    AddressMode::IndirectY => {
                        format_indirect_y(mem.base_address, mem.effective_address, &self.cpu)
                    }
                    AddressMode::ZeroPage => format_zero_page(mem.effective_address, &self.cpu),
                    AddressMode::ZeroPageX => {
                        format_zero_page_x(mem.base_address, mem.effective_address, &self.cpu)
                    }
                    AddressMode::ZeroPageY => {
                        format_zero_page_y(mem.base_address, mem.effective_address, &self.cpu)
                    }
                    AddressMode::Relative => format_relative(mem.effective_address),
                    _ => "".into(),
                },
                AddressResult::Accumulator => format_accumulator(),
                _ => "".into(),
            };
            write!(
            f,
            "{:04X}  {: <8} {:>4} {: <27} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} PPU:{:>3},{:>3} CYC:{}",
            self.cpu.cpu_snapshot.pc,
            byte_str,
            name_str,
            addr_str,
            self.cpu.cpu_snapshot.a,
            self.cpu.cpu_snapshot.x,
            self.cpu.cpu_snapshot.y,
            self.cpu.cpu_snapshot.status,
            self.cpu.cpu_snapshot.sp,
            self.ppu_result.scanline,
            self.ppu_result.cycles,
            self.cpu.cpu_snapshot.cycles,
        )
        }
    }

    fn generate_byte_str(bytes_fetched: &Vec<cpu::MemoryAccess>) -> String {
        bytes_fetched
            .iter()
            .map(|b| format!("{:02X}", b.value))
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn generate_name_string(collector: &cpu::Collector) -> String {
        if collector.undocumented {
            format!("*{:?}", collector.op_name)
        } else {
            format!("{:?}", collector.op_name)
        }
    }

    fn format_absolute(effective_address: u16, collector: &cpu::Collector) -> String {
        if !collector.bytes_overwrite.is_empty() {
            format!(
                "${:04X} = {:02X}",
                effective_address, collector.bytes_overwrite[0].value
            )
        } else if !collector.bytes_read.is_empty() {
            format!(
                "${:04X} = {:02X}",
                effective_address, collector.bytes_read[0].value
            )
        } else {
            format!("${:04X}", effective_address)
        }
    }

    fn format_absolute_x(
        base_address: u16,
        effective_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        let value = if !collector.bytes_overwrite.is_empty() {
            collector.bytes_overwrite[0].value
        } else {
            collector.bytes_read[0].value
        };
        format!(
            "${:04X},X @ {:04X} = {:02X}",
            base_address, effective_address, value
        )
    }

    fn format_absolute_y(
        base_address: u16,
        effective_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        let value = if !collector.bytes_overwrite.is_empty() {
            collector.bytes_overwrite[0].value
        } else {
            collector.bytes_read[0].value
        };
        format!(
            "${:04X},Y @ {:04X} = {:02X}",
            base_address, effective_address, value
        )
    }

    fn format_accumulator() -> String {
        "A".into()
    }

    fn format_immediate(bytes_read: &Vec<cpu::MemoryAccess>) -> String {
        format!("#${:02X}", bytes_read[0].value)
    }

    fn format_indirect(
        base_address: u16,
        effective_address: u16,
        bytes_read: &Vec<cpu::MemoryAccess>,
    ) -> String {
        if bytes_read.is_empty() {
            format!("(${:04X}) = {:04X}", base_address, effective_address)
        } else {
            let value = u16::from_le_bytes([bytes_read[0].value, bytes_read[1].value]);
            format!("(${:04X}) = {:04X}", base_address, value)
        }
    }

    fn format_indirect_x(
        base_address: u16,
        effective_address: u16,
        indexed_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        if !collector.bytes_overwrite.is_empty() {
            format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                base_address,
                indexed_address,
                effective_address,
                collector.bytes_overwrite[0].value
            )
        } else if !collector.bytes_read.is_empty() {
            format!(
                "(${:02X},X) @ {:02X} = {:04X} = {:02X}",
                base_address, indexed_address, effective_address, collector.bytes_read[2].value
            )
        } else {
            "".into()
        }
    }

    fn format_indirect_y(
        base_address: u16,
        effective_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        let operand = collector.bytes_fetched[1].value;
        if !collector.bytes_overwrite.is_empty() {
            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                operand, base_address, effective_address, collector.bytes_overwrite[0].value,
            )
        } else if !collector.bytes_read.is_empty() {
            format!(
                "(${:02X}),Y = {:04X} @ {:04X} = {:02X}",
                operand, base_address, effective_address, collector.bytes_read[2].value,
            )
        } else {
            "".into()
        }
    }

    fn format_zero_page(effective_address: u16, collector: &cpu::Collector) -> String {
        let value = if !collector.bytes_overwrite.is_empty() {
            collector.bytes_overwrite[0].value
        } else if !collector.bytes_write.is_empty() {
            collector.bytes_write[0].value
        } else if !collector.bytes_read.is_empty() {
            collector.bytes_read[0].value
        } else {
            0
        };
        format!("${:02X} = {:02X}", effective_address, value)
    }

    fn format_zero_page_x(
        base_address: u16,
        effective_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        let value = if !collector.bytes_overwrite.is_empty() {
            collector.bytes_overwrite[0].value
        } else {
            collector.bytes_read[0].value
        };
        format!(
            "${:02X},X @ {:02X} = {:02X}",
            base_address, effective_address, value
        )
    }

    fn format_zero_page_y(
        base_address: u16,
        effective_address: u16,
        collector: &cpu::Collector,
    ) -> String {
        let value = if !collector.bytes_overwrite.is_empty() {
            collector.bytes_overwrite[0].value
        } else {
            collector.bytes_read[0].value
        };
        format!(
            "${:02X},Y @ {:02X} = {:02X}",
            base_address, effective_address, value
        )
    }

    fn format_relative(effective_address: u16) -> String {
        format!("${:04X}", effective_address)
    }
}
