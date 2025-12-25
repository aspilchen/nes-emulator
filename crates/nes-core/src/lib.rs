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
    use crate::nes::{self, Nes};
    use crate::observers::ppu_observer::NestestPpuObserver;
    // use crate::observers::cpu_observer::NestestCpuStepTrace;
    use crate::{cpu, observers::*, ppu};
    use std::io::BufRead;
    use std::{fs, io};

    #[test]
    fn test_nestest_trace() {
        let filepath = format!("{}/test_roms/nestest.nes", env!("CARGO_MANIFEST_DIR"));
        let rom = fs::read(filepath).expect("cannot open file");
        let mut nes = Nes::new(&rom).expect("failed to create NES");
        nes.cpu_observer = Some(Box::new(NestestCpuObserver::new()));
        nes.ppu_observer = Some(Box::new(NestestPpuObserver::new()));
        for _ in 0..10 {
            nes.step();
        }

        let cpu_binding = nes.cpu_observer.unwrap();
        let cpu_iter = cpu_binding
            .as_any()
            .downcast_ref::<NestestCpuObserver>()
            .unwrap()
            .traces
            .iter();

        let ppu_binding = nes.ppu_observer.unwrap();
        let ppu_iter = ppu_binding
            .as_any()
            .downcast_ref::<NestestPpuObserver>()
            .unwrap()
            .traces
            .iter();

        let log = fs::read_to_string(format!(
            "{}/test_roms/nestest.log",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("err");
        let nestest_log: Vec<String> = log.lines().map(String::from).collect();

        for ((ppu, cpu), log) in cpu_iter.zip(ppu_iter).zip(nestest_log.iter()) {
            let trace = NestestStepTrace { cpu, ppu };
            println!("{}", trace);
        }
    }
}
