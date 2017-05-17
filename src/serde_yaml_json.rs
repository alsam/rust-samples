use std::collections::HashMap;

extern crate yaml_rust;
extern crate argparse;
#[macro_use] extern crate log;
#[macro_use] extern crate from_variants;

extern crate serde;
extern crate serde_json;
extern crate serde_yaml;

#[macro_use] extern crate serde_derive;

#[derive(Clone, Serialize, Deserialize)]
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

type NamedArgs<'a> = HashMap<&'a str, ArgValue>;

#[derive(Serialize, Deserialize)]
struct DbusMessage<'a> {
    api_name: String,
    dbus_name: String,
    message_type: DbusMessageType,
    args: NamedArgs<'a>,
}

#[derive(Serialize, Deserialize)]
struct DbusMessageRouting<'a>(String, String, String, Vec<DbusMessage<'a>>);

#[derive(Serialize, Deserialize)]
struct ConfYaml<'a>(Vec<DbusMessageRouting<'a>>);

#[derive(Serialize, Deserialize)]
struct ClientRequest<'a>(DbusMessageType, String, NamedArgs<'a>);

#[derive(Serialize, Deserialize)]
struct ClientRequestBody<'a>(DbusMessageType, NamedArgs<'a>);

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
                                args: [ 
                                    ("param1", ArgValue::Str("Hi".to_string())),
                                    ("param2", ArgValue::Bool(true)),
                                    ("param3", ArgValue::Int64(17i64)),
                                    ("param4", ArgValue::Double(2.718281828f64)),
                                ].iter().cloned().collect() 
                              }
            ]
        )
    ];

    let s = serde_yaml::to_string(&conf).unwrap();
    println!("s : {}", s);

    let r = ClientRequest(DbusMessageType::Method, "api_name".to_string(), [
                                        ("param1", ArgValue::Str("Hi".to_string())),            
                                        ("param2", ArgValue::Bool(true)),            
                                        ("param3", ArgValue::Int64(17i64)),            
                                        ("param4", ArgValue::Double(2.718281828f64)),        
                                ].iter().cloned().collect()
                          );
    let rs = serde_json::to_string(&r).unwrap();
    println!("rs : {}", rs);

    let rb = ClientRequestBody(DbusMessageType::Method, [
                                        ("param1", ArgValue::Str("Hi".to_string())),            
                                        ("param2", ArgValue::Bool(true)),            
                                        ("param3", ArgValue::Int64(17i64)),            
                                        ("param4", ArgValue::Double(2.718281828f64)),        
                                ].iter().cloned().collect()
                          );

    let mut a_request = HashMap::new();
    a_request.insert("api_name", rb);

    let rsq = serde_json::to_string(&a_request).unwrap();
    println!("rsq : {}", rsq);

}
