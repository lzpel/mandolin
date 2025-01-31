pub mod templates;
mod utils;

use openapiv3::{Content, MediaType, OpenAPI, Operation, ReferenceOr, RequestBody, Schema, SchemaKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::LazyLock;
use crate::utils::capitalize;

#[derive(Serialize, Deserialize)]
pub struct LappedMediaType {
    pub media_type: String,
    #[serde(flatten)]
    pub media: MediaType,
}
impl TryFrom<&Content> for LappedMediaType {
    type Error = ();
    fn try_from(content: &Content) -> Result<Self, Self::Error> {
        if content.is_empty() {
            Err(())
        } else {
            let v = content
                .iter()
                .filter(|(k, _)| k.contains("json"))
                .next()
                .unwrap_or(content.iter().next().expect("No content"));
            Ok(Self {
                media_type: v.0.to_string(),
                media: v.1.clone(),
            })
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct LappedRequestBody {
    pub default_content: LappedMediaType,
    pub identifier: String,
    #[serde(flatten)]
    pub request_body: RequestBody,
    //required
    //content
}
#[derive(Serialize, Deserialize)]
pub struct LappedOperation {
    pub path: String,
    pub method: String,
    pub function: String,
    pub request_identifier: String,
    pub response_identifier: String,
    //Option<ReferenceOr<RequestBody>>だがReference<>を許容しないのでOption<RequestBody>
    //Option<RequestBody>だがcontext_oneを追加してOption<LappedRequestBody>
    pub request_body: Option<LappedRequestBody>,
    #[serde(flatten)]
    pub operation: Operation,
}
impl LappedOperation {
    pub fn new(path: &str, method: &str, operation: &Operation) -> Self {
        let function = match &operation.operation_id {
            None => format!("{}{}", method, path).replace("/", "_"),
            Some(v) => v.clone(),
        };
        let body = operation.request_body.as_ref().and_then(|v| {
            let request_body = v.as_item().expect("Referenced request body is not allowd");
            //jsonを含めば、json、それ以外ならbytes
            Some(LappedRequestBody {
                request_body: request_body.clone(),
                default_content: LappedMediaType::try_from(&request_body.content).unwrap(),
                identifier: utils::camel_case(
                    format!("request_body_{}", function.as_str()).as_str(),
                ),
            })
        });
        Self {
            path: path.to_string(),
            method: method.to_string(),
            operation: operation.clone(),
            function: function.clone(),
            request_identifier: utils::camel_case(
                format!("request_{}", function.as_str()).as_str(),
            ),
            response_identifier: utils::camel_case(
                format!("response_{}", function.as_str()).as_str(),
            ),
            request_body: body,
        }
    }
}
#[derive(Serialize, Deserialize)]
pub struct Empty {}
static EMPTY_OBJECT: LazyLock<minijinja::Value> =
    LazyLock::new(|| minijinja::Value::from_serialize(Empty {}));

pub struct Mandolin {
    api: OpenAPI,
    templates: Vec<String>,
}
impl Mandolin {
    pub fn new(api: OpenAPI) -> Self {
        Self {
            api,
            templates: vec![],
        }
    }
    pub fn template<T: AsRef<str>>(&mut self, template: T) -> &mut Self {
        self.templates.push(template.as_ref().to_string());
        self
    }
    fn decode<S: AsRef<str>>(content: S) -> String {
        content.as_ref().replace("~0", "~").replace("~1", "/") // RFC6901
    }
    fn decode_list<S: AsRef<str>>(content: S) -> Vec<String> {
        content.as_ref().split("/").map(|v | Self::decode(v)).collect()
    }
    fn encode<S: AsRef<str>>(content: S) -> String {
        content.as_ref().replace("~", "~0").replace("/", "~1") // RFC6901
    }
    fn snake_case<S: AsRef<str>>(s: S)->String{
        let mut snake_case = String::new();
        for (i, c) in s.as_ref().chars().enumerate() {
            if c.is_uppercase() {
                (i!=0).then(|| snake_case.push('_'));
                snake_case.push(c.to_ascii_lowercase());
            } else {
                snake_case.push(c);
            }
        }
        snake_case
    }
    fn p(
        api: minijinja::Value,
        path: &str,
        no_err: bool,
    ) -> Result<minijinja::Value, minijinja::Error> {
        let default = if no_err {
            Ok(EMPTY_OBJECT.clone())
        } else {
            Err(minijinja::Error::new(
                minijinja::ErrorKind::NonKey,
                format!("p: {path} not found"),
            ))
        };
        let mut parent = api;
        for p0 in path.split("/").skip(1) {
            let p = Self::decode(&p0);
            parent = if let Some(map) = parent.as_object() {
                match map.get_value(&minijinja::Value::from(&p)) {
                    None => match p
                        .parse::<usize>()
                        .ok()
                        .and_then(|i| map.get_value(&minijinja::Value::from(i)))
                    {
                        None => return default,
                        Some(latest) => latest,
                    },
                    Some(latest) => latest,
                }
            } else {
                return default;
            }
        }
        Ok(parent)
    }
    fn r_base(api: &minijinja::Value, value: minijinja::Value, no_err: bool) -> Result<minijinja::Value, minijinja::Error> {
        match ReferenceOr::<()>::deserialize(&value) {
            Ok(ReferenceOr::Reference { reference }) => Self::p(api.clone(), reference.as_str(), no_err),
            _ => Ok(value)
        }
    }
    fn r(api: &minijinja::Value, value: minijinja::Value, no_err: bool) -> Result<minijinja::Value, minijinja::Error> {
        let v=Self::r_base(api, value, no_err)?;
        if let Ok(v)=BTreeMap::<minijinja::Value, minijinja::Value>::deserialize(&v) {
            return v.into_iter().map(|(k, v)| Self::r_base(api, v, no_err).map(|v| (k, v))).collect();
        } else if let Ok(v) = Vec::<minijinja::Value>::deserialize(&v) {
            return v.into_iter().map(|v| Self::r_base(api, v, no_err)).collect();
        }
        Ok(v)
    }
    fn pr<'a>(
        api: minijinja::Value,
        path: &str,
        no_err: bool,
    ) -> Result<minijinja::Value, minijinja::Error> {
        let v = Self::p(api.clone(), path, no_err)?;
        Self::r(&api, v, false)
    }
    fn ls(
        api: minijinja::Value,
        path: &str,
        no_err: bool,
    ) -> Result<Vec<(String, minijinja::Value)>, minijinja::Error> {
        let v = Self::pr(api, path, no_err)?;
        if let Some(v) = v.as_object() {
            if let Some(v) = v.try_iter_pairs() {
                return Ok(v.map(|(k, v)| (format!("{path}/{}", Self::encode(k.to_string())), v,)).collect())
            } else if let Some(v) = v.try_iter() {
                return Ok(v.enumerate().map(|(k, v)| (format!("{path}/{}", k), v)).collect())
            }
        }
        if no_err {
            Ok(Default::default())
        } else {
            Err(minijinja::Error::new(minijinja::ErrorKind::NonKey, format!("ls {}", path), ))
        }
    }
    fn lsop(
        api: minijinja::Value,
        path: &str,
        no_err: bool,
    ) -> Result<Vec<(String, minijinja::Value)>, minijinja::Error> {
        let v = Self::ls(api.clone(), path, no_err)?;
        let methods = [
            "get", "put", "post", "delete", "options", "head", "patch", "trace",
        ];
        let w = v
            .iter()
            .map(|(k, v_path)| {
                Self::ls(api.clone(), k, no_err)
                    .unwrap_or_default()
                    .into_iter()
                    .map(move |(k, v)| (k, v_path, v))
            })
            .flatten()
            .filter_map(|(k, _, v)| {
                let method=methods.into_iter().find(|w| k.ends_with(w))?;
                let operation=Operation::deserialize(v).ok()?;
                let decoded=Self::decode_list(&k).into_iter().nth_back(1).unwrap_or_default();
                let lapped=LappedOperation::new(decoded.as_str(), method, &operation);
                Some((k, minijinja::Value::from_serialize(lapped)))
            })
            .collect();
        Ok(w)
    }
    fn recursive_pointed_objects(path: String, value: &minijinja::Value, output: &mut Vec<(String, minijinja::Value)>){
        output.push((path.clone(), value.clone()));//注目箇所を追加
        if let Some(v) = value.as_object() {//子を検索
            if let Some(v) = v.try_iter_pairs() {
                v.for_each(|(k,v)| Self::recursive_pointed_objects(format!("{path}/{}", Self::encode(k.to_string())), &v, output));
            } else if let Some(v) = v.try_iter() {
                v.enumerate().for_each(|(k,v)| Self::recursive_pointed_objects(format!("{path}/{k}"), &v, output));
            }
        }
    }
    pub fn render(&self) -> Result<String, minijinja::Error> {
        let mut env = minijinja::Environment::new();
        let api = minijinja::Value::from_serialize(&self.api);
        let map_pointed_objects: Vec<(String, minijinja::Value)> = {
            let mut output=Default::default();
            Self::recursive_pointed_objects("#".to_string(), &api, &mut output);
            output
        };
        env.add_filter("snake_case", |value: &str|{
            Self::snake_case(value)
        });
        env.add_filter("ident", |value: &str| {
            let mut v: String=Default::default();
            for i in value.split("/").skip(1){
                for j in Self::decode(i).split("/"){
                    if !j.is_empty(){
                        v.push_str(capitalize(j).as_str())
                    }
                }
            }
            Ok(v)
        });
        env.add_filter("pointer_decode", |value: &str| {
           if value.starts_with("#/"){
               Ok(minijinja::Value::from_serialize(Self::decode_list(value)))
           }else{
               Ok(minijinja::Value::from_serialize(Self::decode(value)))
           }
        });
        {
            let api = api.clone();
            env.add_filter(
                "p",
                move |value: &minijinja::Value| {
                    Self::p(api.clone(), value.as_str().unwrap_or_default(), true)
                },
            );
        }
        {
            let api = api.clone();
            env.add_filter(
                "r",
                move |value: minijinja::Value| {
                    Self::r(&api, value, false)
                },
            );
        }
        {
            let api = api.clone();
            env.add_filter(
                "pr",
                move |value: &minijinja::Value| {
                    Self::pr(api.clone(), value.as_str().unwrap_or_default(), true)
                },
            );
        }
        {
            let api = api.clone();
            env.add_filter(
                "ls",
                move |value: &minijinja::Value| {
                    Self::ls(api.clone(), value.as_str().unwrap_or_default(), true).map(|v| minijinja::Value::from_serialize(v))
                },
            );
        }
        {
            let api = api.clone();
            env.add_filter(
                "lsop",
                move |value: &minijinja::Value| {
                    Self::lsop(api.clone(), value.as_str().unwrap_or_default(), true).map(|v| minijinja::Value::from_serialize(v))
                },
            );
        }
        {
            env.add_filter(
                "ref",
                |value: &minijinja::Value| {
                    match ReferenceOr::<()>::deserialize(value) {
                        Ok(ReferenceOr::Reference { reference }) => minijinja::Value::from(Self::decode_list(&reference).into_iter().nth_back(1).unwrap_or_default()),
                        _ => minijinja::Value::UNDEFINED
                    }
                },
            );
        }
        {
            let map_pointed_objects=map_pointed_objects.clone();
            env.add_function("ls_operation",move || {
                let o: Vec<(String, minijinja::Value)>=map_pointed_objects.iter()
                    .filter(|(k, _)|{
                        let mut v=k.split("/");
                        v.next().is_some_and(|v| v.eq("#"))
                            && v.next().is_some_and(|v| v.eq("paths"))
                            && v.next().is_some()
                            && v.next().is_some_and(|v| ["get", "put", "post", "delete", "options", "head", "patch", "trace"].iter().any(|i| i.eq(&v)))
                            && v.next().is_none()
                    })
                    .cloned()
                    .collect();
                Ok(minijinja::Value::from_serialize(o))
            })
        }
        {
            let map_pointed_objects=map_pointed_objects.clone();
            env.add_function("ls_schema",move || {
                let o: Vec<(String, Schema)>=map_pointed_objects.iter()
                    .filter_map(|(k, v)| Schema::deserialize(v).ok().map(|v| (k, v)))
                    .filter(|(_, v)| match &v.schema_kind{
                        SchemaKind::Type(_)=>true,
                        _ => false
                    })
                    .map(|(k, v)| (k.clone(), v))
                    .collect();
                Ok(minijinja::Value::from_serialize(o))
            })
        }
        {
            let map_pointed_objects=map_pointed_objects.clone();
            env.add_function("ls_all",move || {
                Ok(minijinja::Value::from_serialize(map_pointed_objects.clone()))
            })
        }
        let v=self.templates.join("\n");
        env.add_template("main", v.as_str())?;
        let template = env.get_template("main")?;
        template.render(&self.api)
    }
}
#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;
    fn apis() -> HashMap<String, OpenAPI> {
        fs::read_dir(&Path::new(".").join("openapi"))
            .unwrap()
            .filter_map(Result::ok)
            .filter_map(|entry| {
                entry
                    .path()
                    .to_str()
                    .unwrap_or_default()
                    .contains("yaml")
                    .then(|| {
                        (
                            entry.file_name().to_str().unwrap_or_default().to_string(),
                            serde_yaml::from_reader(std::io::BufReader::new(File::open(entry.path()).unwrap(), ))
                            .unwrap(),
                        )
                    })
            })
            .collect()
    }
    fn write<P: AsRef<Path>, S: AsRef<str>>(path: P, content: S) -> std::io::Result<()> {
        let mut writer = std::io::BufWriter::new(File::create(path)?);
        println!("{}", content.as_ref());
        writeln!(writer, "{}", content.as_ref())
    }
    #[test]
    fn test_filter() {
        let v = apis().get("openapi.yaml").unwrap().clone();
        let r = Mandolin::new(v)
            .template("{{'#'|p|tojson}}\n{{'#/paths'|p|tojson}}\n{{'#/servers/0'|p|tojson}}\n{{'#'|ls|tojson}}{{'#/servers'|ls|tojson}}\n{{'#/paths'|lsop|tojson}}")
            .render()
            .unwrap();
        println!("{}", r)
    }
    #[test]
    fn test_ls() {
        let v = apis().get("openapi.yaml").unwrap().clone();
        let r = Mandolin::new(v)
            .template("{{'#'|p}}\n{{'#'|pr}}\n{% for k, v in '#'|ls %}{{k}}={{v}}\n{%endfor%}")
            .render()
            .unwrap();
        println!("{}", r)
    }
    #[test]
    fn test_ls_operation() {
        let r = Mandolin::new(apis().remove("openapi.yaml").unwrap())
            .template("lsop\n{% for k, v in '#/paths'|lsop %}{{k}}={{v}}\n{%endfor%}\nls_operation()\n{% for k, v in ls_operation() %}{{k}}={{v}}\n{%endfor%}")
            .render()
            .unwrap();
        println!("{}", r)
    }
    #[test]
    fn test_render_schema() {
        let r = Mandolin::new(apis().remove("openapi_nesting.yaml").unwrap())
            .template(templates::SCHEMA)
            .template(templates::DUMP)
            .render()
            .unwrap();
        write("examples/test_render_schema.out.rs", r).unwrap()
    }
    #[test]
    fn test_render_trait() {
        let r = Mandolin::new(apis().remove("openapi_nesting.yaml").unwrap())
            .template(templates::HEADER)
            .template(templates::SCHEMA)
            .template(templates::TRAIT)
            .template(templates::DUMP)
            .render()
            .unwrap();
        write("examples/test_render_trait.out.rs", r).unwrap()
    }
    #[test]
    fn test_render() {
        for entry in fs::read_dir(&Path::new(".").join("openapi"))
            .unwrap()
            .filter_map(Result::ok)
        {
            if entry
                .path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .contains("yaml")
            {
                println!("{}", entry.path().to_str().unwrap_or_default());
                let v = Mandolin::new(serde_yaml::from_reader(std::io::BufReader::new(File::open(entry.path()).unwrap())).unwrap(), )
                .template(templates::MAIN)
                .render()
                .unwrap();
                println!("{}", v)
            }
        }
    }
    #[test]
    fn test_macro(){
        let r = Mandolin::new(apis().remove("openapi.yaml").unwrap())
            .template("{%- macro SCHEMA(schema) -%}{{schema}}{%- endmacro -%}{{ SCHEMA(openapi) }}")
            .render()
            .unwrap();
        println!("{}", r)
    }
    #[test]
    fn test_try_iter_pairs(){
        #[derive(Serialize, Deserialize)]
        struct Point{ x: i32, y: i32, }
        let v0=minijinja::Value::from_serialize("abc".chars().map(|v| v.to_string()).collect::<Vec<String>>());
        let v1=minijinja::Value::from_serialize("abc".chars().map(|v| (format!("key_{v}"), format!("value_{v}"))).collect::<HashMap<String, String>>());
        let v2 = minijinja::Value::from_serialize(Point{x:0, y:1});
        let v3=minijinja::Value::from_serialize("abc");
        let detector=|v: minijinja::Value|{
            if let Ok(v)=BTreeMap::<minijinja::Value, minijinja::Value>::deserialize(&v) {
                return v.into_iter().map(|(k, v)| format!("{k}={v}")).collect::<String>();
            } else if let Ok(v) = Vec::<minijinja::Value>::deserialize(&v) {
                return v.into_iter().map(|v| format!("{v}!")).collect::<String>();
            }
            return ["this is just value".to_string()].into_iter().collect::<String>();
        };
        println!("{}", detector(v0));
        println!("{}", detector(v1));
        println!("{}", detector(v2));
        println!("{}", detector(v3));
    }
}
