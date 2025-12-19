use crate::cartridge::mapper::MapperType;

pub struct InesRom {
    pub header: InesHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

pub struct InesHeader {
    pub ascii: Vec<u8>,
    pub prg_banks: u8,
    pub chr_banks: u8,
    pub mapper: MapperType,
    pub mirroring: Mirroring,
}

#[derive(Debug, Copy, Clone)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}

impl InesRom {
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let prg_bank_size_bytes = 16 * 1024;
        let chr_bank_size_bytes = 8 * 1024;
        let header = InesHeader::parse(data).expect("invalid iNes file");
        let prg_size = prg_bank_size_bytes * header.prg_banks as usize;
        let chr_size = chr_bank_size_bytes * header.chr_banks as usize;
        let mut offset = 16;
        let prg_rom = data[offset..offset + prg_size].to_vec();
        offset += prg_size;
        let chr_rom = if chr_size > 0 {
            data[offset..offset + chr_size].to_vec()
        } else {
            vec![0; chr_bank_size_bytes]
        };
        Ok(Self {
            header,
            prg_rom,
            chr_rom,
        })
    }
}

impl InesHeader {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 16 {
            return Err("header".into());
        }

        let ascii = data[0..4].to_vec();
        let prg_banks = data[4];
        let chr_banks = data[5];
        let mapper_low = data[6] >> 4;
        let mapper_high = data[7] >> 4;
        let mapper_id = mapper_low | (mapper_high << 4);
        let mapper = match mapper_id {
            0 => MapperType::Mapper0,
            _ => MapperType::Unsupported(mapper_id),
        };

        let mirroring = if data[6] & 1 == 1 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };
        if ascii != b"NES\x1A" {
            return Err("header".into());
        }
        Ok(Self {
            ascii,
            prg_banks,
            chr_banks,
            mapper,
            mirroring,
        })
    }
}
