// https://github.com/serde-rs/serde/issues/927

use std::collections::HashMap;

extern crate yaml_rust;
extern crate argparse;
#[macro_use] extern crate log;
#[macro_use] extern crate from_variants;

extern crate serde;
#[macro_use] extern crate serde_json;
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

#[derive(Serialize, Deserialize, Debug)]
enum DbusMessageType {
    Method,
    Signal,
}

type NamedArgs<'a> = HashMap<&'a str, ArgValue>;
type NamedArgs2<'a> = HashMap<&'a str, serde_json::value::Value>;

#[derive(Serialize, Deserialize)]
struct DbusMessage<'a> {
    api_name: String,
    dbus_name: String,
    message_type: DbusMessageType,
    #[serde(borrow)]
    args: NamedArgs<'a>,
}

#[derive(Serialize, Deserialize)]
struct DbusMessageRouting<'a>(String, String, String, #[serde(borrow)] Vec<DbusMessage<'a>>);

#[derive(Serialize, Deserialize)]
struct ConfYaml<'a>(#[serde(borrow)] Vec<DbusMessageRouting<'a>>);

#[derive(Serialize, Deserialize, Debug)]
struct ClientRequest<'a> {
    #[serde(rename = "type")]
    tp: DbusMessageType,
    name: String,
    #[serde(borrow)]
    args: NamedArgs2<'a>,
}

#[derive(Serialize, Deserialize)]
struct ClientRequestBody<'a>(DbusMessageType, #[serde(borrow)] NamedArgs<'a>);

type CRequest<'a> = HashMap<&'a str, ClientRequestBody<'a>>;

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

    let r = ClientRequest { tp: DbusMessageType::Method, name: "api_name".to_string(), args:
                                [       ("param1", json!("Hi")),
                                        ("param2", json!(true)),
                                        ("param3", json!(17i64)),
                                        ("param4", json!(2.718281828f64)),
                                ].iter().cloned().collect()
                          };
    let rs = serde_json::to_string(&r).unwrap();
    println!("rs : {}", rs);

    let d: ClientRequest = serde_json::from_str(&rs).expect("json deserialization error");
    println!("d: {:?}", d);


    let rb = ClientRequestBody(DbusMessageType::Method, [
                                        ("param1", ArgValue::Str("Hi".to_string())),
                                        ("param2", ArgValue::Bool(true)),
                                        ("param3", ArgValue::Int64(17i64)),
                                        ("param4", ArgValue::Double(2.718281828f64)),
                                ].iter().cloned().collect()
                          );

    let mut a_request: CRequest = HashMap::new();
    a_request.insert("api_name", rb);

    let rsq = serde_json::to_string(&a_request).unwrap();
    println!("rsq : {}", rsq);

    let de: CRequest = serde_json::from_str(&rsq).expect("json deserialization error");

    let rsq2 = serde_json::to_string(&de).expect("json serialization error");
    println!("rsq2 : {}", rsq2);

    for key in de.keys() {
        println!("key {}", key);
    }
    println!("key[0] : {}", de.keys().nth(0).unwrap());

}
