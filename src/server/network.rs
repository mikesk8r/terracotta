#[derive(Debug)]
pub enum NetworkValue {
    Array(Box<[u8]>),
    #[allow(dead_code, non_camel_case_types)]
    bool(bool),
    #[allow(non_camel_case_types)]
    u8(u8),
    #[allow(dead_code, non_camel_case_types)]
    u16(u16),
    String(String),
}

impl PartialEq for NetworkValue {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub fn read_string(buffer: &mut std::slice::Iter<'_, u8>) -> Result<String, &'static str> {
    let mut length = read_varint(buffer)?;
    let mut string = vec![];
    while length > 0 {
        string.push(buffer.next().expect("").clone());
        length -= 1;
    }
    let string = String::from_utf8(string).expect("");
    Ok(string)
}

pub fn read_varint(buffer: &mut std::slice::Iter<'_, u8>) -> Result<i32, &'static str> {
    let mut value = 0;

    let mut current_byte = buffer.next().expect("").to_owned();
    for i in 0..5 {
        value |= ((current_byte & 0b01111111) as i32) << (7 * i);
        if current_byte & 0b10000000 == 0 {
            break;
        }
        current_byte = buffer.next().expect("").to_owned();
    }

    Ok(value)
}

pub fn write_string(buffer: &mut Vec<u8>, value: String) {
    let mut data = value.into_bytes();
    write_varint(buffer, data.len() as i32);
    buffer.append(&mut data);
}

pub fn write_varint(buffer: &mut Vec<u8>, mut int: i32) {
    if int == 0 {
        buffer.push(0);
        return;
    }
    let mut temp = [0];
    while int != 0 {
        temp[0] = (int & 0b01111111) as u8;
        int = (int >> 7) & (i32::MAX >> 6);
        if int != 0 {
            temp[0] |= 0b10000000;
        }
        buffer.push(temp[0]);
    }
}
