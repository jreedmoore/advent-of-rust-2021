mod puzzle {
  // These parsers would probably be easier to express with nom
  // https://github.com/Geal/nom 
  pub enum Packet {
    Lit(Literal),
    Op(Operator)
  }
  impl Packet {
    pub fn maybe_parse(input: &mut HexStringReader) -> Option<Packet> {
      let version = input.read_bits(3);
      let typ = input.read_bits(3);
      //println!("v {} t {}", version, typ);
      match typ {
        4 => Some(Packet::Lit(Literal::parse(version, input))),
        _ => Some(Packet::Op(Operator::parse(version, typ, input)))
      }
    }
    pub fn version(&self) -> u8 {
      match self {
        Packet::Lit(l) => l.version,
        Packet::Op(o) => o.version
      }
    }
    pub fn vec(&self) -> Vec<&Packet> {
      match self {
        l @ Packet::Lit(_) => vec![l.clone()],
        o @ Packet::Op(op) => {
          let mut base = vec![o.clone()];
          base.append(&mut op.sub_packets.iter().flat_map(|p| p.vec()).collect::<Vec<&Packet>>());
          base
        }
      }
    }
  }

  #[derive(Debug, PartialEq)]
  struct Literal {
    version: u8,
    value: u64
  }
  impl Literal {
    fn maybe_parse(input: &mut HexStringReader) -> Option<Literal> {
      let version = input.read_bits(3);
      let typ = input.read_bits(3);
      if typ == 4 {
        Some(Literal::parse(version, input))
      } else {
        None
      }
    }
    fn parse(version: u8, input: &mut HexStringReader) -> Literal {
      let mut nybbles : Vec<u8> = Vec::new();
      loop {
        let stop_flag = input.read_bits(1) == 0;
        let next = input.read_bits(4);
        nybbles.push(next);
        if stop_flag { break; }
      }
      let mut value : u64 = 0;
      let mut shift : usize = 0;
      for nybble in nybbles.iter().rev() {
        value = value | (*nybble as u64) << shift;
        shift = shift + 4;
      }
      Literal { version: version, value: value }
    }
  }

  struct Operator {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Packet>
  }
  impl Operator {
    fn maybe_parse(input: &mut HexStringReader) -> Option<Operator> {
      let version = input.read_bits(3);
      let typ = input.read_bits(3);
      if typ != 4 {
        Some(Operator::parse(version, typ, input))
      } else {
        None
      }
    }
    fn parse(version: u8, type_id: u8, input: &mut HexStringReader) -> Operator {
      let length_type_id = input.read_bits(1);
      if length_type_id == 0 {
        let bit_length = input.read_bits_u16(15);
        let packets = Operator::sub_parse_by_bits(bit_length, input);
        Operator { version: version, type_id: type_id, sub_packets: packets }
      } else {
        let packet_length = input.read_bits_u16(11);
        let packets = Operator::sub_parse_by_packets(packet_length, input);
        Operator { version: version, type_id: type_id, sub_packets: packets }
      }
    }

    fn sub_parse_by_bits(bit_length: u16, input: &mut HexStringReader) -> Vec<Packet> {
      let mut packets: Vec<Packet> = Vec::new();
      let starting_offset = input.offset.clone();
      while input.offset - starting_offset < bit_length as usize {
        packets.push(Packet::maybe_parse(input).unwrap())
      }
      packets
    }

    fn sub_parse_by_packets(packet_length: u16, input: &mut HexStringReader) -> Vec<Packet> {
      let mut packets: Vec<Packet> = Vec::with_capacity(packet_length as usize);
      for _ in 0..packet_length {
        packets.push(Packet::maybe_parse(input).unwrap())
      }
      packets
    }
  }

  #[derive(Clone)]
  pub struct HexString {
    bytes: Vec<u8>
  }
  impl HexString {
    pub fn parse(input: &str) -> Option<HexString> {
      let mut bytes : Vec<u8> = Vec::with_capacity(input.len() / 2);
      for i in 0..input.len() / 2 {
        let start = i*2;
        let slice = &input[start..start+2];
        let b = u8::from_str_radix(slice,16).ok()?;
        bytes.push(b)
      }
      Some(HexString { bytes: bytes })
    }
    fn mask(len: usize) -> u8 {
      match len {
        0 => 0x00,
        1 => 0x01,
        2 => 0x03,
        3 => 0x07,
        4 => 0x0f,
        5 => 0x1f,
        6 => 0x3f,
        7 => 0x7f,
        8 => 0xff,
        _ => panic!("len is too large")
      }
    }
    fn read_bits(&self, offset: usize, len: usize) -> u8 {
      if len == 0 {
        panic!("can't read zero bits");
      }
      if len > 8 {
        panic!("output larger than u8");
      }
      let next_byte_offset = ((offset + 8) / 8) * 8;
      if next_byte_offset < (offset + len) {
        let first_len = next_byte_offset - offset;
        let next_len = len - first_len;

        let f = self.read_bits(offset, first_len);
        let s = self.read_bits(next_byte_offset, next_len);
        println!("read_bits splitting {:x?} {:x?}", f, s);
        f << next_len | s
      } else {
        let vec_idx = offset / 8;
        let byte_start_offset = vec_idx * 8;
        let offset_into_byte = offset - byte_start_offset;
        let shift = 8 - offset_into_byte - len;
        let mask_with_offset: u8 = HexString::mask(len) << shift;
        (self.bytes[vec_idx] & mask_with_offset) >> shift
      }
    }
    fn read_bits_u16(&self, offset: usize, len: usize) -> u16 {
      if len > 8 {
        let f = self.read_bits(offset, 8) as u16;
        let s = self.read_bits(offset+8, len-8) as u16;
        println!("off {} {:x?} {:x?} shift {}", offset, f, s, len-8);
        f << (len-8) | s
      } else {
        self.read_bits(offset, len) as u16
      }
    }
  }

  pub struct HexStringReader {
    hex_string: HexString,
    offset: usize
  }
  impl HexStringReader {
    pub fn new(hex_string: HexString) -> HexStringReader {
      HexStringReader { hex_string: hex_string, offset: 0 }
    }
    fn read_bits(&mut self, len: usize) -> u8 {
      let res = HexString::read_bits(&self.hex_string, self.offset, len);
      self.offset += len;
      res
    }

    fn read_bits_u16(&mut self, len: usize) -> u16 {
      if len > 16 {
        panic!("len too large");
      }
      let res = self.hex_string.read_bits_u16(self.offset, len);
      self.offset = self.offset + len;
      res
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_literal() {
      let hex_string = HexString::parse("D2FE28").unwrap();
      assert_eq!(Literal::maybe_parse(&mut HexStringReader::new(hex_string.clone())), Some(Literal { version: 6, value: 2021 }));
      assert_eq!(Packet::maybe_parse(&mut HexStringReader::new(hex_string.clone())).unwrap().version(), 6);
    }

    #[test]
    fn test_parse_operator() {
      let bit_length_operator_string = HexString::parse("38006F45291200").unwrap();
      let op = Operator::maybe_parse(&mut HexStringReader::new(bit_length_operator_string.clone())).unwrap();
      assert_eq!(op.version, 1);
      assert_eq!(op.sub_packets.len(), 2);

      let op_packet = Packet::maybe_parse(&mut HexStringReader::new(bit_length_operator_string.clone())).unwrap();
      assert_eq!(op_packet.version(), 1);

      let packet_length_operator_string = HexString::parse("EE00D40C823060").unwrap();
      let op_p = Operator::maybe_parse(&mut HexStringReader::new(packet_length_operator_string)).unwrap();
      assert_eq!(op_p.version, 7);
      assert_eq!(op_p.sub_packets.len(), 3);
    }

    #[test]
    fn test_hex_string_read_bits_u16() {
      let hex_string = HexString::parse("ffff").unwrap();
      assert_eq!(hex_string.read_bits_u16(0, 16), 0xffff);
      assert_eq!(hex_string.read_bits_u16(0, 12), 0x0fff);

      let hex_string = HexString::parse("1234").unwrap();
      assert_eq!(hex_string.read_bits_u16(0, 16), 0x1234);
      assert_eq!(hex_string.read_bits_u16(0, 12), 0x123);

      // 80      02       F4
      // 1000000 00000010 11110100
      // 0123567 89012345 678
      //    ^start         ^end
      // 0xb = 1011
      let hex_string = HexString::parse("8002F4").unwrap();
      assert_eq!(hex_string.read_bits_u16(3, 15), 0xb);
    }

    #[test]
    fn test_hex_string_reader_read_bits_u16() {
      let hex_string = HexString::parse("38006F45291200").unwrap();
      let mut reader = HexStringReader::new(hex_string);
      reader.read_bits(3);
      reader.read_bits(3);
      reader.read_bits(1);
      assert_eq!(reader.read_bits_u16(15), 27);

      let hex_string = HexString::parse("EE00D40C823060").unwrap();
      let mut reader = HexStringReader::new(hex_string);
      reader.read_bits(3);
      reader.read_bits(3);
      reader.read_bits(1);
      assert_eq!(reader.read_bits_u16(11), 3);

      let hex_string = HexString::parse("1A8002FA78").unwrap();
      let mut reader = HexStringReader::new(hex_string.clone());
      reader.read_bits(4);
      reader.read_bits(3);
      reader.read_bits(3);
      assert_eq!(reader.read_bits(1), 0);
      assert_eq!(hex_string.read_bits_u16(11, 15), 11);
      assert_eq!(reader.read_bits_u16(15), 11);
    }

    #[test]
    fn test_hex_string_read_bits_u8() {
      let hex_string = HexString::parse("D2FE28").unwrap();
      assert_eq!(hex_string.read_bits(0,3), 6);
      assert_eq!(hex_string.read_bits(3,3), 4);
      assert_eq!(hex_string.read_bits(6,1), 1);
      assert_eq!(hex_string.read_bits(8,3), 7);
      assert_eq!(hex_string.read_bits(7,4), 7);

      let hex_string = HexString::parse("8002F4").unwrap();
      assert_eq!(hex_string.read_bits(3+8, 15-8), 0xb);
    }
  }
}

pub fn part_one(input: &str) -> Option<u64> {
  let hex_string = puzzle::HexString::parse(input)?;
  let mut reader = puzzle::HexStringReader::new(hex_string);
  let packet = puzzle::Packet::maybe_parse(&mut reader)?;
  Some(packet.vec().iter().fold(0u64, |acc, p| acc + p.version() as u64))
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLE_1: &'static str = "8A004A801A8002F478";
  const EXAMPLE_2: &'static str = "620080001611562C8802118E34";
  const EXAMPLE_3: &'static str = "C0015000016115A2E0802F182340";
  const EXAMPLE_4: &'static str = "A0016C880162017C3686B18A3D4780";

  #[test]
  fn test_part_one_examples() {
    assert_eq!(part_one(EXAMPLE_1), Some(16));
    assert_eq!(part_one(EXAMPLE_2), Some(12));
    assert_eq!(part_one(EXAMPLE_3), Some(23));
    assert_eq!(part_one(EXAMPLE_4), Some(31));
  }
}