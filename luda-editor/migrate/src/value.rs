use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub yvalue: yrs::types::Value,
}
impl Value {
    pub fn into_string(self) -> String {
        if let yrs::types::Value::Any(any) = self.yvalue {
            if let lib0::any::Any::String(string) = any {
                string.to_string()
            } else {
                panic!("Expected string, got {:?}", any);
            }
        } else {
            unreachable!("Expected Any, got {:?}", self.yvalue);
        }
    }
    pub fn from_yrs_value(value: yrs::types::Value) -> Self {
        Self { yvalue: value }
    }
    pub fn into_any(&self) -> lib0::any::Any {
        if let yrs::types::Value::Any(any) = &self.yvalue {
            any.clone()
        } else {
            unreachable!("Expected Any, got {:?}", self.yvalue);
        }
    }
    pub fn deserialize<T: serde::de::DeserializeOwned + std::fmt::Debug>(self) -> T {
        let mut buf = "".to_string();
        self.yvalue.to_json().to_json(&mut buf);

        let result = serde_json::from_str(&buf);
        result.unwrap()
    }
}
impl Into<i32> for Value {
    fn into(self) -> i32 {
        if let yrs::types::Value::Any(any) = self.yvalue {
            if let lib0::any::Any::Number(number) = any {
                number as i32
            } else {
                panic!("Expected number, got {:?}", any);
            }
        } else {
            unreachable!("Expected Any, got {:?}", self.yvalue);
        }
    }
}
// impl From<&i32> for Value {
//     fn from(value: &i32) -> Self {
//         Value {
//             yvalue: yrs::types::Value::Any(lib0::any::Any::Number(*value as f64)),
//         }
//     }
// }
impl Into<String> for Value {
    fn into(self) -> String {
        self.into_string()
    }
}
// impl From<&String> for Value {
//     fn from(value: &String) -> Self {
//         Value {
//             yvalue: yrs::types::Value::Any(lib0::any::Any::String(value.clone().into())),
//         }
//     }
// }
impl<T: History> Into<List<T>> for Value {
    fn into(self) -> List<T> {
        if let yrs::types::Value::YArray(array) = self.yvalue {
            List::new(
                array
                    .iter()
                    .map(|value| T::from_value(Value::from_yrs_value(value))),
            )
        } else {
            panic!("Expected YArray, got {:?}", self.yvalue);
        }
    }
}
impl<T: History> Into<Map<T>> for Value {
    fn into(self) -> Map<T> {
        if let yrs::types::Value::YMap(map) = self.yvalue {
            Map::new(
                map.iter().map(|(key, value)| {
                    (key.to_string(), T::from_value(Value::from_yrs_value(value)))
                }),
            )
        } else {
            panic!("Expected YMap, got {:?}", self.yvalue);
        }
    }
}
impl<T: History> Into<Single<T>> for Value {
    fn into(self) -> Single<T> {
        Single::new(T::from_value(Value::from_yrs_value(self.yvalue)))
    }
}
impl<T: serde::Serialize> From<T> for Value {
    fn from(t: T) -> Self {
        let yvalue = yrs::types::Value::from(
            lib0::any::Any::from_json(&serde_json::to_string(&t).unwrap()).unwrap(),
        );
        Self { yvalue }
    }
}
