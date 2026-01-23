use crate::cartridge::{header::InesHeader, mapper::MapperType, ChrBank};

pub struct InesRom {
    pub header: InesHeader,
    pub prg_rom: Vec<u8>,
    // pub chr_rom: Vec<u8>,
    pub chr_rom: ChrBank,
}

impl InesRom {
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let prg_bank_size_bytes = 16 * 1024;
        let chr_bank_size_bytes = 8 * 1024;
        let header = InesHeader::parse(data).expect("invalid iNes file");
        let prg_size = prg_bank_size_bytes * header.prg_banks as usize;
        let chr_size = chr_bank_size_bytes * header.chr_banks as usize;
        let skip_trainer = data[6] & 0b100 != 0;
        let prg_begin = 16 + if skip_trainer { 512 } else { 0 };
        let prg_end = prg_begin + prg_size;
        let chr_begin = prg_end;
        let chr_end = chr_begin + chr_size;
        let prg_rom = data[prg_begin..prg_end].to_vec();
        let chr_rom = if chr_size > 0 {
            data[chr_begin..chr_end].to_vec()
        } else {
            vec![0; chr_bank_size_bytes]
        };
        Ok(Self {
            header,
            prg_rom,
            chr_rom: ChrBank { data: chr_rom },
        })
    }
}
