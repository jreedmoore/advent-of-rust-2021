mod puzzle {
  // These parsers would probably be easier to express with nom
  // https://github.com/Geal/nom 
  enum Packet {
    Lit(Literal),
    Op(Operator)
  }
  impl Packet {
    fn maybe_parse(input: &mut HexStringReader) -> Option<Packet> {
      let version = input.read_bits(3);
      let typ = input.read_bits(3);
      match typ {
        4 => Some(Packet::Lit(Literal::parse(version, input))),
        _ => Some(Packet::Op(Operator::parse(version, typ, input)))
      }
    }
    fn version(&self) -> u8 {
      match self {
        Packet::Lit(l) => l.version,
        Packet::Op(o) => o.version
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
      // or take version as parameter and start reader after tag?
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
      todo!();
    }
  }

  #[derive(Clone)]
  struct HexString {
    bytes: Vec<u8>
  }
  impl HexString {
    fn parse(input: &str) -> Option<HexString> {
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
        f << first_len | s
      } else {
        let vec_idx = offset / 8;
        let byte_start_offset = vec_idx * 8;
        let offset_into_byte = offset - byte_start_offset;
        let shift = 8 - offset_into_byte - len;
        let mask_with_offset: u8 = HexString::mask(len) << shift;
        (self.bytes[vec_idx] & mask_with_offset) >> shift
      }
    }
  }

  struct HexStringReader {
    hex_string: HexString,
    offset: usize
  }
  impl HexStringReader {
    fn new(hex_string: HexString) -> HexStringReader {
      HexStringReader { hex_string: hex_string, offset: 0 }
    }
    fn read_bits(&mut self, len: usize) -> u8 {
      let res = HexString::read_bits(&self.hex_string, self.offset, len);
      self.offset += len;
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
    fn test_hex_string() {
      let hex_string = HexString::parse("D2FE28").unwrap();
      assert_eq!(hex_string.read_bits(0,3), 6);
      assert_eq!(hex_string.read_bits(3,3), 4);
      assert_eq!(hex_string.read_bits(6,1), 1);
      assert_eq!(hex_string.read_bits(8,3), 7);
      assert_eq!(hex_string.read_bits(7,4), 7);
    }
  }
}

pub fn part_one(input: &str) -> Option<u64> {
  todo!()
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