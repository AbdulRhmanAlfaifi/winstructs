use serde::{ser};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::fmt::{Display,Debug};
use std::fmt;
use serialize::{
    serialize_u64
};

// Option to display references as nested
pub static mut NESTED_REFERENCE: bool = false;

#[derive(Serialize, Debug)]
pub struct MftEnumReference{
    #[serde(serialize_with = "serialize_u64")]
    reference: u64,
    entry: u64,
    sequence: u16
}

// Represents a MFT Reference struct
// https://msdn.microsoft.com/en-us/library/bb470211(v=vs.85).aspx
// https://jmharkness.wordpress.com/2011/01/27/mft-file-reference-number/
pub struct MftReference(pub u64);
impl MftReference{
    fn get_enum_ref(&self)->MftEnumReference{
        let mut raw_buffer = vec![];
        raw_buffer.write_u64::<LittleEndian>(self.0).unwrap();
        MftEnumReference{
            reference: LittleEndian::read_u64(&raw_buffer[0..8]),
            entry: LittleEndian::read_u64(
                &[&raw_buffer[0..6], &[0,0]].concat()
            ),
            sequence: LittleEndian::read_u16(&raw_buffer[6..8])
        }
    }
}
impl Display for MftReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl Debug for MftReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)
    }
}
impl ser::Serialize for MftReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ser::Serializer
    {
        // If NESTED_REFERENCE we need to serialize the enumerated structure
        if unsafe{NESTED_REFERENCE} {
            serializer.serialize_newtype_struct("mft_reference",&self.get_enum_ref())
        } else {
            // Just serialize the u64 version
            serialize_u64(&self.0,serializer)
        }
    }
}

#[test]
fn test_reference() {
    let raw_reference: &[u8] = &[0x73,0x00,0x00,0x00,0x00,0x00,0x68,0x91];

    let mft_reference = MftReference(
        LittleEndian::read_u64(&raw_reference[0..8])
    );
    assert_eq!(mft_reference.0,10477624533077459059);
    assert_eq!(format!("{}", mft_reference),"10477624533077459059");
    // assert_eq!(mft_reference.sequence,37224);
}
