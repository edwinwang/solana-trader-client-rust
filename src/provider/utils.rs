// temporary solution to handle project string to integer conversions.
// TODO: the string / enum mapping should ideally be handled in the build step of solana-trader-proto.

use serde_json::{json, Value};
use solana_trader_proto::api::Project;

/// Convert string enum values in JSON to their corresponding integer or string representations.
/// This includes:
/// - Project enums ("P_JUPITER" -> 2)
/// - Infinity values in priceImpactPercent ("INF_NOT" -> "NOT")
pub fn convert_string_enums(value: &mut Value) {
    match value {
        Value::Object(map) => {
            // Handle any object that has these fields
            for (key, val) in map {
                match key.as_str() {
                    // Convert project fields to integers
                    "project" => {
                        if val.is_string() {
                            if let Some(project_str) = val.as_str() {
                                if let Some(project_enum) = Project::from_str_name(project_str) {
                                    *val = json!(project_enum as i32);
                                }
                            }
                        } else {
                            // Recurse into project object if it's not a string
                            convert_string_enums(val);
                        }
                    }
                    "tradeFeeRate" => {
                        if val.is_string() {
                            if let Some(trade_fee_rate) = val.as_str() {
                                *val = json!(trade_fee_rate.parse::<u64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "height" => {
                        if val.is_string() {
                            if let Some(trade_fee_rate) = val.as_str() {
                                *val = json!(trade_fee_rate.parse::<u64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "slot" => {
                        if val.is_string() {
                            if let Some(slot) = val.as_str() {
                                *val = json!(slot.parse::<i64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "time" => {
                        if val.is_string() {
                            if let Some(slot) = val.as_str() {
                                *val = json!(slot.parse::<i64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "token1Reserves" => {
                        if val.is_string() {
                            if let Some(slot) = val.as_str() {
                                *val = json!(slot.parse::<i64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "token2Reserves" => {
                        if val.is_string() {
                            if let Some(slot) = val.as_str() {
                                *val = json!(slot.parse::<i64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "openTime" => {
                        if val.is_string() {
                            if let Some(slot) = val.as_str() {
                                *val = json!(slot.parse::<i64>().unwrap())
                            }
                        } else {
                            convert_string_enums(val);
                        }
                    }
                    "infinity" => {
                        if let Some(infinity_str) = val.as_str() {
                            let mapped = match infinity_str {
                                "INF_NOT" => 0,
                                "INF" => 1,
                                "INF_NEG" => 2,
                                _ => continue,
                            };
                            *val = json!(mapped);
                        }
                    }
                    // Recurse into other fields
                    _ => convert_string_enums(val),
                }
            }
        }
        Value::Array(arr) => {
            // Recurse into arrays
            for item in arr {
                convert_string_enums(item);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_conversion() {
        let mut value = json!({
            "project": "P_JUPITER",
            "nested": {
                "project": "P_RAYDIUM"
            },
            "array": [
                {"project": "P_OPENBOOK"}
            ]
        });

        convert_string_enums(&mut value);

        assert_eq!(value["project"], 2); // P_JUPITER
        assert_eq!(value["nested"]["project"], 3); // P_RAYDIUM
        assert_eq!(value["array"][0]["project"], 5); // P_OPENBOOK
    }

    #[test]
    fn test_infinity_conversion() {
        let mut value = json!({
            "priceImpactPercent": {
                "infinity": "INF_NOT",
                "percent": 0.0
            },
            "steps": [{
                "priceImpactPercent": {
                    "infinity": "INF",
                    "percent": 0.5
                }
            }]
        });

        convert_string_enums(&mut value);

        assert_eq!(value["priceImpactPercent"]["infinity"], 0);
        assert_eq!(value["steps"][0]["priceImpactPercent"]["infinity"], 1);
    }

    #[test]
    fn test_complex_nested_structure() {
        let mut value = json!({
            "quotes": [{
                "project": "P_JUPITER",
                "routes": [{
                    "steps": [{
                        "project": "P_RAYDIUM",
                        "priceImpactPercent": {
                            "infinity": "INF_NOT"
                        }
                    }]
                }]
            }]
        });

        convert_string_enums(&mut value);

        assert_eq!(value["quotes"][0]["project"], 2); // P_JUPITER
        assert_eq!(value["quotes"][0]["routes"][0]["steps"][0]["project"], 3); // P_RAYDIUM
        assert_eq!(
            value["quotes"][0]["routes"][0]["steps"][0]["priceImpactPercent"]["infinity"],
            0
        );
    }
}
