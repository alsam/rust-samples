#[cfg(test)]

mod tests {

    use serde::de::value::{Error as ValueError, StrDeserializer};
    use serde::de::IntoDeserializer;

    //use evread::event::deserialize_hex_str;
    use evread::event::*;

    #[test]
    fn test_hex_str_deserialize() {
        let deserializer: StrDeserializer<ValueError> = "0x7d76d5fe14".into_deserializer();
        let result = deserialize_hex_str(deserializer).unwrap();
        assert_eq!(result[0], 538864647700);
    }

    #[test]
    #[should_panic(expected = "hex is not prefixed with 0x or 0X")]
    fn test_hex_str_deserialize_bad_prefix() {
        let deserializer: StrDeserializer<ValueError> = "Xy7d76d".into_deserializer();
        let result = deserialize_hex_str(deserializer).unwrap();
        println!("result: {:?}", &result); // not reached
    }

    #[test]
    #[should_panic(expected = "ParseIntError { kind: InvalidDigit }")]
    fn test_hex_str_deserialize_invalid_digit() {
        let deserializer: StrDeserializer<ValueError> = "0x712t8".into_deserializer();
        let result = deserialize_hex_str(deserializer).unwrap();
        println!("result: {:?}", &result); // not reached
    }

    #[test]
    fn test_hex_str_deserialize_vec() {
        let deserializer: StrDeserializer<ValueError> = "0x7d76d5fe:fd7d7e".into_deserializer();
        let result = deserialize_hex_str(deserializer).unwrap();
        assert_eq!(result[0], 2104940030);
        assert_eq!(result[1], 16612734);
    }
}
