pub struct U16Visitor;

impl<'de> serde::de::Visitor<'de> for U16Visitor {
    type Value = u16;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 0 and 65535")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(v)
    }
}


pub struct U32Visitor;

impl<'de> serde::de::Visitor<'de> for U32Visitor {
    type Value = u32;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between 0 and 4294967295")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(v)
    }
}

