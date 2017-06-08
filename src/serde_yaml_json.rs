// https://github.com/serde-rs/serde/issues/927

use std::collections::HashMap;
use std::collections::BTreeMap;

extern crate yaml_rust;
extern crate argparse;
#[macro_use] extern crate log;
#[macro_use] extern crate from_variants;

extern crate serde;
#[macro_use] extern crate serde_json;
extern crate serde_yaml;
#[macro_use] extern crate serde_derive;

#[derive(Clone, Serialize, Deserialize, Debug)]
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
    Array(Vec<ArgValue>),
    Hash(BTreeMap<String, ArgValue>),
}

impl ArgValue {
    fn to_json(&self, n: &str) -> String {
        match *self {
            ArgValue::None             =>   "".to_string(),
            ArgValue::Str(ref v)       =>   format!("\"{}\":{}", n, v ),
            ArgValue::Bool(ref v)      =>   format!("\"{}\":{}", n, v ),
            ArgValue::Byte(ref v)      =>   format!("\"{}\":{}", n, v ),
            ArgValue::Int16(ref v)     =>   format!("\"{}\":{}", n, v ),
            ArgValue::Int32(ref v)     =>   format!("\"{}\":{}", n, v ),
            ArgValue::Int64(ref v)     =>   format!("\"{}\":{}", n, v ),
            ArgValue::UInt16(ref v)    =>   format!("\"{}\":{}", n, v ),
            ArgValue::UInt32(ref v)    =>   format!("\"{}\":{}", n, v ),
            ArgValue::UInt64(ref v)    =>   format!("\"{}\":{}", n, v ),
            ArgValue::Double(ref v)    =>   format!("\"{}\":{}", n, v ),
            ArgValue::Array(ref v)     =>   format!("\"{}\":{:?}", n, v ),
            ArgValue::Hash(ref v)      =>   format!("\"{}\":{:?}", n, v ),
        }
    }
}

trait FromVariant: Sized {
    fn from_variant(variant: &ArgValue) -> Option<Self>;
}

// String, Array, Hash differ, for all other variants use macros
impl FromVariant for String {
    fn from_variant(variant: &ArgValue) -> Option<Self> {
        match *variant {
            ArgValue::Str(ref v) => Some((*v).clone()),
            _ => None
        }
    }
}

impl<T> FromVariant for Vec<T> where T: FromVariant {
    fn from_variant(variant: &ArgValue) -> Option<Self> {
        match *variant {
            ArgValue::Array(ref v) => {
                let mut ret = Vec::<T>::new();
                for el in v {
                    let val: T = FromVariant::from_variant(&el).expect("type mismatch");
                    ret.push(val);
                }
                Some(ret)
            }
            _ => None
        }
    }
}

macro_rules! from_variant_inst {
    [$t: tt, $i: ident] => {
        impl FromVariant for $t {
            fn from_variant(variant: &ArgValue) -> Option<Self> {
                match *variant {
                    ArgValue::$i(v) => Some(v),
                    _ => None
                }
            }
        }
    };
}

from_variant_inst![bool, Bool];
from_variant_inst![u8,   Byte];
from_variant_inst![i16,  Int16];
from_variant_inst![i32,  Int32];
from_variant_inst![i64,  Int64];
from_variant_inst![u16,  UInt16];
from_variant_inst![u32,  UInt32];
from_variant_inst![u64,  UInt64];
from_variant_inst![f64,  Double];

#[derive(Clone, Serialize, Deserialize, Debug)]
enum ArgsTag {
    Args,
    Rets,
}

#[derive(Serialize, Deserialize, Debug)]
enum DbusMessageType {
    Method,
    Signal,
}

type NamedArgs = HashMap<String, ArgValue>;
type NamedArgs2 = HashMap<String, serde_json::value::Value>;

#[derive(Serialize, Deserialize)]
struct DbusMessage {
    api_name: String,
    dbus_name: String,
    message_type: DbusMessageType,
    args: NamedArgs,
}

#[derive(Serialize, Deserialize)]
struct DbusMessageRouting(String, String, String, Vec<DbusMessage>);

#[derive(Serialize, Deserialize)]
struct ConfYaml(Vec<DbusMessageRouting>);

#[derive(Serialize, Deserialize, Debug)]
struct ClientRequest {
    #[serde(rename = "type")]
    tp: DbusMessageType,
    name: String,
    args: Option<NamedArgs2>,
    rets: Option<NamedArgs2>,
}

impl ClientRequest {
    fn to_json(&self) -> String {
        fn serialize_args_or_rets(tag: &str, ar: &Option<NamedArgs2>) -> String {
            match ar {
                &Some(ref a) => {
                    let args_serialized: Vec<_> = a.iter().map( |ret| {
                        let (n, val) = ret;
                        //val.to_json(n)
                        format!("\"{}\":{}", n, val)
                        } ).collect();
                    format!("\"{}\": {{ {}", tag, args_serialized.join(","))
                },
                &None =>  "".to_string(),
            }
        }
        let json_reply_prefix = format!("{{\"type\":\"{:?}\",\"name\":\"{}\",", &self.tp, &self.name);
        let args_serialized = serialize_args_or_rets("args", &self.args);
        let rets_serialized = serialize_args_or_rets("rets", &self.rets);
        format!("{}{}{} }} }}",json_reply_prefix, args_serialized, rets_serialized).to_string()
    }
}

#[derive(Serialize, Deserialize)]
struct ClientRequestBody(DbusMessageType, NamedArgs);

type CRequest = HashMap<String, ClientRequestBody>;

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
                                    ("param1".to_string(), ArgValue::Str("Hi".to_string())),
                                    ("param2".to_string(), ArgValue::Bool(true)),
                                    ("param3".to_string(), ArgValue::Int64(17i64)),
                                    ("param4".to_string(), ArgValue::Double(2.718281828f64)),
                                    ("param5".to_string(), ArgValue::Array( vec![ArgValue::Double(1.1), ArgValue::Double(2.2), ArgValue::Double(3.3), ] )),
                                ].iter().cloned().collect()
                              }
            ]
        )
    ];

    let s = serde_yaml::to_string(&conf).unwrap();
    println!("s : {}", s);

    let r = ClientRequest { tp: DbusMessageType::Method, name: "api_name".to_string(),
                            args: Some([    ("param1".to_string(), json!("Hi")),
                                            ("param2".to_string(), json!(true)),
                                            ("param3".to_string(), json!(17i64)),
                                            ("param4".to_string(), json!(2.718281828f64)),
                                            ("param5".to_string(), json!( vec![ json!(1.1), json!(2.2), json!(3.3), ] )),
                                       ].iter().cloned().collect()),
                            rets: None,
                          };
    let rs = serde_json::to_string(&r).unwrap();
    println!("rs : {}", rs);

    let d: ClientRequest = serde_json::from_str(&rs).expect("json deserialization error");
    println!("d: {:?}", d);

    let ds = serde_json::to_string(&d).unwrap();
    println!(" => ds: {}", ds);

    let data = "{\"type\":\"Method\",\"name\":\"utest_4p2r\",\"rets\":{\"parm1\":71,\"parm2\":12.12 } }";
    let d2: ClientRequest = serde_json::from_str(&data).expect("json deserialization error");
    println!("d2: {:?}", d2);
    println!("d2.to_json(): {}", d2.to_json());


    let rb = ClientRequestBody(DbusMessageType::Method, [
                                        ("param1".to_string(), ArgValue::Str("Hi".to_string())),
                                        ("param2".to_string(), ArgValue::Bool(true)),
                                        ("param3".to_string(), ArgValue::Int64(17i64)),
                                        ("param4".to_string(), ArgValue::Double(2.718281828f64)),
                                ].iter().cloned().collect()
                          );

    let mut a_request: CRequest = HashMap::new();
    a_request.insert("api_name".to_string(), rb);

    let rsq = serde_json::to_string(&a_request).unwrap();
    println!("rsq : {}", rsq);

    let de: CRequest = serde_json::from_str(&rsq).expect("json deserialization error");

    let rsq2 = serde_json::to_string(&de).expect("json serialization error");
    println!("rsq2 : {}", rsq2);

    for key in de.keys() {
        println!("key {}", key);
    }
    println!("key[0] : {}", de.keys().nth(0).unwrap());

    let arr = ArgValue:: Array( vec![ArgValue::Double(1.1), ArgValue::Double(2.2), ArgValue::Double(3.3),] );

    let from_arr_variant: Vec<f64> = FromVariant::from_variant(&arr).expect("array expected");

    println!("arr: {:?} from_arr_variant: {:?}", arr, from_arr_variant);

    let arr2 = ArgValue::Array( vec![ArgValue::Array(vec![ArgValue::Double(1.1), ArgValue::Double(2.2)]),
                                     ArgValue::Array(vec![ArgValue::Double(3.3), ArgValue::Double(4.4)]) ] );
    let from_arr2_variant: Vec<Vec<f64>> = FromVariant::from_variant(&arr2).expect("array of array expected");

    println!("arr2: {:?} from_arr2_variant: {:?}", arr2, from_arr2_variant);

    let bad_arr = ArgValue:: Array( vec![ArgValue::Double(1.1), ArgValue::Int32(2), ArgValue::Double(3.3),] );
    println!("bad_arr: {:?}", bad_arr);

    // expect here runtime error `thread 'main' panicked at 'type mismatch',`
    let from_bad_arr_variant: Vec<f64> = FromVariant::from_variant(&bad_arr).expect("array expected");
}
