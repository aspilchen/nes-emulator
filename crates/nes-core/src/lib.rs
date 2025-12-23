mod bus;
pub mod cartridge;
mod cpu;
pub mod error;
pub mod input;
pub mod nes;
pub mod observers;
pub mod ppu;

pub use nes::Nes;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{
        bus::Bus,
        cartridge::cartridge::Cartridge,
        cpu::{
            self,
            addressing::{AddressMode, AddressResult},
            cpu6502::Cpu6502,
            instruction::Instruction,
        },
        observers::NestTestNesObserver,
        Nes,
    };

    fn print_trace(details: &NestTestNesObserver) {
        for (cpu, ppu) in details.cpu_traces.iter().zip(details.ppu_traces.iter()) {
            let mut bytes = vec![cpu.instruction.opcode];

            for i in 0..cpu.instruction.size - 1 {
                if let AddressResult::Memory(addr_result) = &cpu.operand {
                    let op_bytes = addr_result.operand.to_le_bytes();
                    bytes.push(op_bytes[i as usize]);
                }
            }

            let byte_str = bytes
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(" ");

            let addr_str = match &cpu.operand {
                AddressResult::Memory(addr_result) => match addr_result.mode {
                    AddressMode::Absolute => {
                        format!("${:04X}", addr_result.effective_address)
                    }
                    AddressMode::AbsoluteX => {
                        format!("${:04X},X", addr_result.effective_address)
                    }
                    AddressMode::AbsoluteY => {
                        format!("${:04X},Y", addr_result.effective_address)
                    }
                    AddressMode::Immediate => {
                        format!(
                            "${:02X}",
                            if let AddressResult::Memory(addr_result) = &cpu.operand {
                                addr_result.operand as u8
                            } else {
                                0
                            }
                        )
                    }
                    AddressMode::Indirect => {
                        format!("(${:04X})", addr_result.operand)
                    }
                    AddressMode::IndirectX => {
                        format!(" (${:02X},X)", addr_result.operand)
                    }
                    AddressMode::IndirectY => {
                        format!("(${:02X}),Y", addr_result.operand)
                    }
                    AddressMode::Relative => {
                        format!("${:04X}", addr_result.effective_address)
                    }
                    AddressMode::ZeroPage => {
                        format!("${:02X}", addr_result.effective_address)
                        // match instruction.name {
                        //     cpu::instruction::Op::STX => format!("{} = {:02X}", result, cpu.cpu.x),
                        //     _ => result,
                        // }
                    }
                    AddressMode::ZeroPageX => {
                        format!("${:02X},X", addr_result.effective_address)
                    }
                    AddressMode::ZeroPageY => {
                        format!("${:02X},Y", addr_result.effective_address)
                    }
                    _ => "".to_string(),
                },
                _ => "".to_string(),
            };
            println!(
            "{:04X}  {: <8}  {:?}  {: <31} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            cpu.cpu_snapshot.pc,
            byte_str,
            cpu.instruction.name,
            addr_str,
            cpu.cpu_snapshot.a,
            cpu.cpu_snapshot.x,
            cpu.cpu_snapshot.y,
            cpu.cpu_snapshot.status.bits(),
            cpu.cpu_snapshot.sp,
            cpu.cpu_snapshot.cycles,
        )
        }
    }

    #[test]
    fn test_nestest_trace() {
        let filepath = format!("{}/test/nestest.nes", env!("CARGO_MANIFEST_DIR"));
        let rom = fs::read(filepath).expect("cannot open file");
        let mut nes = Nes::new(&rom).expect("failed to create NES");
        nes.observer = Some(Box::new(NestTestNesObserver::new()));
        for _ in 0..100 {
            nes.step();
        }

        // let observer = nes.observer.take().unwrap().downcast::<NestTestNesObserver>().unwrap();
        print_trace(nes.observer.as_ref().unwrap().as_any().downcast_ref::<NestTestNesObserver>().unwrap());

        // nes_observer
        //     .traces
        //     .iter()
        //     .for_each(|trace| cpu_trace(trace));
        // // Compare to nestest.log
        // // let expected = fs::read_to_string("nestest.log").unwrap();
        // // let expected_lines: Vec<&str> = expected.lines().collect();
        // // for (i, trace) in traces.iter().enumerate() {
        // //     assert_eq!(trace, expected_lines[i]);
        // // }
    }
}
