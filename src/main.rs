use std::env;

fn decode_bencoded_value(encoded_value: &str) -> anyhow::Result<serde_json::Value> {
    let value: serde_bencode::value::Value = serde_bencode::from_str(encoded_value)
        .unwrap_or_else(|_| panic!("Failed to decode bencoded value: {}", encoded_value));

    decode(value)
}

fn decode(value: serde_bencode::value::Value) -> anyhow::Result<serde_json::Value> {
    match value {
        serde_bencode::value::Value::Bytes(b) => {
            let string = String::from_utf8(b)?;
            Ok(serde_json::Value::String(string))
        }
        serde_bencode::value::Value::Int(i) => Ok(serde_json::Value::Number(i.into())),
        serde_bencode::value::Value::List(l) => Ok(serde_json::Value::Array(
            l.into_iter()
                .map(|item| decode(item))
                .collect::<anyhow::Result<Vec<serde_json::Value>>>()?,
        )),
        serde_bencode::value::Value::Dict(d) => Ok(serde_json::Value::Object(
            d.into_iter()
                .map(|(key, item)| {
                    let key = String::from_utf8(key)?;
                    let value = decode(item)?;
                    Ok((key, value))
                })
                .collect::<anyhow::Result<serde_json::Map<String, serde_json::Value>>>()?,
        )),
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value)?;
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }

    Ok(())
}
