use serde_json::Value;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BasicLogEntry {
    pub npid: String,
    pub parent: String,
    pub timestamp: String,
    pub protocol: String,
    pub source: String,
    pub destination: String,
    pub length: String,
    pub info: String,
    pub interface: String,
    pub payload: Vec<u8>,
}

impl TryFrom<Value> for BasicLogEntry {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or("Expected JSON object")?;

        macro_rules! get_str {
            ($key:expr) => {
                obj.get($key)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| format!("Missing or invalid '{}'", $key))?
                    .to_string()
            };
        }

        let payload = match obj.get("payload") {
            Some(Value::Array(arr)) => {
                arr.iter()
                    .map(|v| v.as_u64().ok_or("Payload must be array of u8"))
                    .collect::<Result<Vec<u64>, _>>()?
                    .into_iter()
                    .map(|n| n as u8)
                    .collect()
            }
            _ => return Err("Missing or invalid 'payload'".into()),
        };

        Ok(BasicLogEntry {
            npid: get_str!("npid"),
            parent: get_str!("parent"),
            timestamp: get_str!("timestamp"),
            protocol: get_str!("protocol"),
            source: get_str!("source"),
            destination: get_str!("destination"),
            length: get_str!("length"),
            info: get_str!("info"),
            interface: get_str!("interface"),
            payload,
        })
    }
}
