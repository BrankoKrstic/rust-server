use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf>  {
    pub data: HashMap<&'buf str, Value<'buf>>
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

impl <'buf> From<&'buf str> for QueryString<'buf> {
    fn from(query_str: &'buf str) -> Self {
        let mut map = HashMap::new();
        for item in query_str.split("&") {
            let mut key = item;
            let mut val = "";
            if let Some(i) = item.find('=') {
                key = &item[..i];
                val = &item[i + 1..];
            }
            map.entry(key)
            .and_modify(|e| {
                 match e {
                    Value::Single(v) => *e = Value::Multiple(vec![v, val]),
                    Value::Multiple(vc) => vc.push(val)
,
                };
            })
            .or_insert(Value::Single(val));
        }
        QueryString { data: map }
    }
}