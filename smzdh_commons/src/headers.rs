use iron::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use iron::headers::ContentType;
use iron::modifiers::Header;
use std::collections::BTreeMap;
use iron::status;
use iron::status::Status;
use std::convert::Into;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::json::ToJson;

#[derive(Debug)]
pub struct JsonResponse {
    pub data:BTreeMap<String,Json>,
}

impl JsonResponse {
    pub fn new() -> Self {
        JsonResponse {
            data:BTreeMap::new(),
        }
    }

    pub fn new_with<E:Into<String>,R:ToJson>(code:i64,error:E,result:R) -> Self {
        let mut tmp = JsonResponse {
            data:BTreeMap::new(),
        };
        tmp.set_code(code);
        tmp.set_error(error);
        tmp.set_result(result);
        tmp
    }


    pub fn set_code(&mut self,code:i64) -> Option<Json> {
        self.data.insert(String::from("code"),Json::I64(code))
    }

    pub fn set_error<E:Into<String>>(&mut self,error:E) -> Option<Json> {
        let mut e = Json::Null;
        let es = error.into();
        if !es.is_empty() {
            e = Json::String(es);
        }
        self.data.insert(String::from("error"),e)
    }

    pub fn set_result<R:ToJson>(&mut self,result:R) -> Option<Json> {
        self.data.insert(String::from("result"),result.to_json())
    }

    pub fn insert<K:Into<String>,V:ToJson>(&mut self,key:K,value:V) -> Option<Json> {
        self.data.insert(key.into(),value.to_json())
    }

    pub fn get_btmap(self) -> BTreeMap<String,Json> {
        self.data
    }

    pub fn clone_from_brmap(&mut self,othre:BTreeMap<String,Json>) {
        self.data = othre;
    }
}

impl ToJson for JsonResponse {
    fn to_json(&self) -> Json {
        Json::Object(self.data.clone())
    }
}

pub fn json_headers() -> Header<ContentType> {
    Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                            vec![(Attr::Charset, Value::Utf8)])))
}


pub fn success_json_response(jr:&JsonResponse) -> (Status,Header<ContentType>,String) {
    (
        status::Ok,
        Header(ContentType(Mime(TopLevel::Application, SubLevel::Json,
                                vec![(Attr::Charset, Value::Utf8)]))),
        json::encode(&jr.data).unwrap_or(String::new()),
    )
}
