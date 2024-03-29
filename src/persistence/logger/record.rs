use crc32fast::Hasher;

const PAGE_SIZE: usize = 4096;

#[derive(Copy, Clone)]
pub enum RecordType {
    FULL = 0,
    FIRST = 1,
    MIDDLE = 2,
    LAST = 3,
}

pub struct Record {
    record_type: RecordType,
    len: usize,
    data: Vec<u8>,
    crc: u32,
}

impl Record {
    pub fn new(data: Vec<u8>) -> Vec<Record> {
        let len = data.len();

        if data.len() <= PAGE_SIZE - 32 + 4 {
            return vec![Record {
                record_type: RecordType::FULL,
                len,
                data: data.clone(),
                crc: Record::calculate_crc(data),
            }];
        }

        let first_data = data[..PAGE_SIZE - 32].to_vec();
        let mut remaining_data = &data[PAGE_SIZE - 32..];

        let mut records = vec![Record {
            record_type: RecordType::FIRST,
            len: first_data.len(),
            data: first_data.clone(),
            crc: Record::calculate_crc(first_data),
        }];

        while remaining_data.len() > PAGE_SIZE - 32 {
            let middle_data = remaining_data[..PAGE_SIZE - 32].to_vec();
            remaining_data = &remaining_data[PAGE_SIZE - 32..];

            records.push(Record {
                record_type: RecordType::MIDDLE,
                len: middle_data.len(),
                data: middle_data.clone(),
                crc: Record::calculate_crc(middle_data),
            });
        }

        records.push(Record {
            record_type: RecordType::LAST,
            len: remaining_data.len(),
            data: remaining_data.to_vec().clone(),
            crc: Record::calculate_crc(remaining_data.to_vec()),
        });

        records
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.record_type as u8);
        buffer.extend(&(self.len as u32).to_be_bytes());
        buffer.extend(&self.data);
        buffer.extend(&self.crc.to_be_bytes());

        buffer
    }

    fn calculate_crc(data: Vec<u8>) -> u32 {
        let mut hasher = Hasher::new();

        hasher.update(&data);
        hasher.finalize()
    }

    pub fn from_bytes(mut data: Vec<u8>) -> Record {
        let record_type_byte = data.remove(0);
        let record_type = match record_type_byte {
            0 => RecordType::FULL,
            1 => RecordType::FIRST,
            2 => RecordType::MIDDLE,
            3 => RecordType::LAST,
            _ => panic!("Invalid record type"),
        };

        let len_bytes = [data[0], data[1], data[2], data[3]];
        data = data[4..].to_vec();

        let len = u32::from_be_bytes(len_bytes) as usize;

        let crc_bytes = [
            data[data.len() - 4],
            data[data.len() - 3],
            data[data.len() - 2],
            data[data.len() - 1],
        ];
        data = data[..data.len() - 4].to_vec();

        if data.len() != len {
            panic!("Data length does not match the length in the record");
        }

        let crc = u32::from_be_bytes(crc_bytes);

        let calculated_crc = Record::calculate_crc(data.clone());

        if crc != calculated_crc {
            panic!("CRC does not match the calculated CRC");
        }

        Record {
            record_type,
            len,
            data,
            crc,
        }
    }
}
