extern crate yaml_rust;
extern crate argparse;
#[macro_use] extern crate log;
#[macro_use] extern crate from_variants;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
enum ArgValue {
    None,
    Str(String),
    Bool(bool),
    Byte(u8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Double(f64),
}

#[derive(Serialize, Deserialize)]
enum DbusMessageType {
    Method,
    Signal,
}

#[derive(Serialize, Deserialize)]
struct DbusMessage {
    api_name: String,
    dbus_name: String,
    message_type: DbusMessageType,
    args: Vec<(String,ArgValue)>,
}

#[derive(Serialize, Deserialize)]
struct DbusMessageRouting(String, String, String, Vec<DbusMessage>);

#[derive(Serialize, Deserialize)]
struct ConfYaml(Vec<DbusMessageRouting>);

fn main()
{
    let conf = vec![
        DbusMessageRouting(
            "com.a.b.servicetest".to_string(),
            "/com/a/b/servicetest".to_string(),
            "com.a.b.IServiceTest".to_string(),
            vec![ DbusMessage { api_name: "api_name".to_string(),
                                dbus_name: "dbus_name".to_string(),
                                message_type: DbusMessageType::Method,
                                args: vec![
                                    ("param1".to_string(), ArgValue::Str("Hi".to_string())),            
                                    ("param2".to_string(), ArgValue::Bool(true)),            
                                    ("param3".to_string(), ArgValue::Int64(17i64)),            
                                    ("param4".to_string(), ArgValue::Double(2.718281828f64)),            
                                ] 
                              }
            ]
        )
    ];

    let s = serde_yaml::to_string(&conf).unwrap();
    println!("s : {}", s);

}
