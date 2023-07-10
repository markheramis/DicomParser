use dicom_core::value::PrimitiveValue;
use dicom_core::value::Value as DicomValue;
use dicom_core::DataDictionary;
use dicom_core::VR;
use dicom_dictionary_std::StandardDataDictionary;
use dicom_object::open_file;
use dicom_object::Tag;
use serde::Serialize;
use serde_json::json;

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize)]
struct DicomProps(HashMap<String, Option<String>>);

#[no_mangle]
pub fn parse(path: &Path, show_tag: bool) -> String {
    let allowed_vrs: Vec<VR> = vec![
        // Strings
        VR::AE,
        VR::AS,
        VR::AT,
        VR::CS,
        VR::DA,
        VR::DS,
        VR::DT,
        VR::IS,
        VR::LO,
        VR::LT,
        VR::PN,
        VR::SH,
        VR::ST,
        VR::TM,
        VR::UC,
        VR::UI,
        VR::UR,
        VR::UT,
        // Double Values
        VR::FL,
        VR::FD,
        // Long Values
        VR::SV,
        VR::UV,
        // UInt Values
        VR::UL,
    ];
    let object: dicom_object::FileDicomObject<dicom_object::InMemDicomObject> =
        open_file(&path).unwrap();
    let dicom_props: DicomProps = DicomProps(
        object
            .iter()
            .filter(
                |element: &&dicom_core::DataElement<dicom_object::InMemDicomObject, Vec<u8>>| {
                    allowed_vrs.contains(&element.header().vr())
                },
            )
            .map(
                |element: &dicom_core::DataElement<dicom_object::InMemDicomObject, Vec<u8>>| {
                    let header: &dicom_core::DataElementHeader = element.header();
                    let key: String;
                    if show_tag {
                        key = get_tag_name(header.tag);
                    } else {
                        key = header.tag.to_string();
                    }
                    let value: Option<String> = get_value(element.value());
                    (format!("{}", key), value)
                },
            )
            .collect(),
    );
    let json: JsonValue = json!(dicom_props);
    return json.to_string();
}

fn get_tag_name(tag: Tag) -> String {
    let dictionary: StandardDataDictionary = StandardDataDictionary::default();
    return {
        let this: Option<&str> = dictionary
            .by_tag(tag)
            .map(|entry: &dicom_core::dictionary::DictionaryEntryRef<'_>| entry.alias);
        match this {
            Some(x) => x,
            None => (|| "Unknown")(),
        }
    }
    .to_string();
}
fn get_value(value: &DicomValue<dicom_object::InMemDicomObject, Vec<u8>>) -> Option<String> {
    match value {
        // &PrimitiveValue
        DicomValue::Primitive(primitive_value) => match primitive_value {
            PrimitiveValue::Str(s) => Some(s.to_owned()),
            PrimitiveValue::Date(d) => Some(
                d.iter()
                    .map(|date: &dicom_core::value::DicomDate| date.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            PrimitiveValue::Time(t) => Some(
                t.iter()
                    .map(|time: &dicom_core::value::DicomTime| time.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            PrimitiveValue::DateTime(dt) => Some(
                dt.iter()
                    .map(|dt: &dicom_core::value::DicomDateTime| dt.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            PrimitiveValue::I32(i) => Some(
                i.iter()
                    .map(|i: &i32| i.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            PrimitiveValue::U16(u) => Some(
                u.iter()
                    .map(|u: &u16| u.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            PrimitiveValue::Strs(ss) => Some(
                ss.iter()
                    .map(|s: &String| s.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            ),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]

mod tests {
    use serde_json::Error as JsonError;
    use super::*;
    fn parse_json(json_string: &str) -> Result<JsonValue, JsonError> {
        let parsed: JsonValue = serde_json::from_str(json_string)?;
        Ok(parsed)
    }
    #[test]
    fn test_parse_no_tag_should_not_be_null_with_ext() {
        let path: &Path = Path::new("./assets/dicom-with-ext.dcm");
        let result: String = parse(path, false);
        assert!(!result.is_empty());
    }
    #[test]
    fn test_parse_with_tag_should_not_be_null_with_ext() {
        let path: &Path = Path::new("./assets/dicom-with-ext.dcm");
        let result: String = parse(path, true);
        assert!(!result.is_empty());
    }
    #[test]
    fn test_parse_with_tag_should_have_correct_study_instance_uid_with_ext() {
        let path: &Path = Path::new("./assets/dicom-with-ext.dcm");
        let result: String = parse(path, true);
        match parse_json(result.as_str()) {
            Ok(parsed_json) => {
                if let Some(parsed_value) = parsed_json["StudyInstanceUID"].as_str() {
                    let expected_value =
                        "1.2.826.0.1.3680043.8.1055.1.20111102150758591.92402465.76095170";
                    //let expected_value = "1.2.826.0.1.3680043.2.594.20385.9727.6138.15049.24610\0";
                    assert_eq!(parsed_value, expected_value);
                } else {
                    panic!("StudyInstanceUID is not found in the parsed json");
                }
            }
            Err(e) => println!("Could not parse JSON: {}", e),
        }
    }
    #[test]
    fn test_parse_no_tag_should_have_correct_study_instance_uid_with_ext() {
        let path: &Path = Path::new("./assets/dicom-with-ext.dcm");
        let result: String = parse(path, false);
        match parse_json(result.as_str()) {
            Ok(parsed_json) => {
                if let Some(parsed_value) = parsed_json["(0020,000D)"].as_str() {
                    let expected_value =
                        "1.2.826.0.1.3680043.8.1055.1.20111102150758591.92402465.76095170";
                    //let expected_value = "1.2.826.0.1.3680043.2.594.20385.9727.6138.15049.24610\0";
                    assert_eq!(parsed_value, expected_value);
                } else {
                    panic!("(0020,0052) is not found in the parsed json");
                }
            }
            Err(e) => println!("Could not parse JSON: {}", e),
        }
    }
    #[test]
    fn test_parse_with_tag_should_have_correct_study_instance_uid_on_no_ext() {
        let path: &Path = Path::new("./assets/dicom-no-ext");
        let result: String = parse(path, true);
        match parse_json(result.as_str()) {
            Ok(parsed_json) => {
                if let Some(parsed_value) = parsed_json["StudyInstanceUID"].as_str() {
                    let expected_value =
                        "1.3.46.670589.11.3540642177.2867929537.1763690001.2563942908";
                    assert_eq!(parsed_value, expected_value);
                } else {
                    panic!("(0020,0052) is not found in the parsed json");
                }
            }
            Err(e) => println!("Could not parse JSON: {}", e),
        }
    }
    #[test]
    fn test_parse_no_tag_should_have_correct_study_instance_uid_on_no_ext() {
        let path: &Path = Path::new("./assets/dicom-no-ext");
        let result: String = parse(path, false);
        match parse_json(result.as_str()) {
            Ok(parsed_json) => {
                if let Some(parsed_value) = parsed_json["(0020,000D)"].as_str() {
                    let expected_value =
                        "1.3.46.670589.11.3540642177.2867929537.1763690001.2563942908";
                    assert_eq!(parsed_value, expected_value);
                } else {
                    panic!("(0020,0052) is not found in the parsed json");
                }
            }
            Err(e) => println!("Could not parse JSON: {}", e),
        }
    }
}
