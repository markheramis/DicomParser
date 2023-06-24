use dicom_core::VR;
use dicom_core::value::Value;
use dicom_core::value::PrimitiveValue;
use dicom_object::open_file;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;
use dicom_dictionary_std::StandardDataDictionary;
use dicom_core::DataDictionary;
use dicom_object::Tag;

#[derive(Serialize)]
struct DicomProps(HashMap<String, Option<String>>);



pub fn parse(path: &Path, show_tag: bool) -> String {
    let allowed_vrs: Vec<VR> = vec![
        // Strings
        VR::AE, VR::AS, VR::AT, VR::CS, VR::DA, VR::DS, VR::DT, VR::IS, VR::LO, VR::LT, VR::PN, VR::SH, VR::ST, VR::TM, VR::UC, VR::UI, VR::UR, VR::UT,
        // Double Values
        VR::FL, VR::FD,
        // Long Values
        VR::SV, VR::UV,
        // UInt Values
        VR::UL,
    ];
    let object = open_file(&path).unwrap();
    let dicom_props: DicomProps = DicomProps(
        object
            .iter()
            .filter(|element| allowed_vrs.contains(&element.header().vr()))
            .map(|element| {
                let header = element.header();
                let key: String;
                if show_tag {
                    key = header.tag.to_string();
                } else {
                    key = get_tag_name(header.tag);
                }
                let value = get_value(element.value());
                (format!("{}", key), value)
            })
            .collect(),
    );
    let json: serde_json::Value = json!(dicom_props);
    return json.to_string();
}

fn get_tag_name(
    tag: Tag
) -> String {
    let dictionary: StandardDataDictionary = StandardDataDictionary::default();
    return {
        let this = dictionary
        .by_tag(tag)
        .map(|entry: &dicom_core::dictionary::DictionaryEntryRef<'_> | {
            entry.alias
        });
        match this {
            Some(x) => x,
            None => (|| "Unknown")(),
        }
    }
        .to_string();
}
fn get_value(
    value: &Value<dicom_object::InMemDicomObject, Vec<u8>>,
) -> Option<String> {
    match value {
        // &PrimitiveValue
        Value::Primitive(primitive_value) => match primitive_value {
            PrimitiveValue::Str(s) => Some(s.to_owned()),
            PrimitiveValue::Date(d) => {
                Some(d.iter().map(|date: &dicom_core::value::DicomDate| date.to_string()).collect::<Vec<String>>().join(","))
            }
            PrimitiveValue::Time(t) => {
                Some(t.iter().map(|time: &dicom_core::value::DicomTime| time.to_string()).collect::<Vec<String>>().join(","))
            }
            PrimitiveValue::DateTime(dt) => {
                Some(dt.iter().map(|dt: &dicom_core::value::DicomDateTime| dt.to_string()).collect::<Vec<String>>().join(","))
            }
            PrimitiveValue::I32(i) => {
                Some(i.iter().map(|i: &i32| i.to_string()).collect::<Vec<String>>().join(","))
            }
            PrimitiveValue::U16(u) => {
                Some(u.iter().map(|u: &u16| u.to_string()).collect::<Vec<String>>().join(","))
            }
            PrimitiveValue::Strs(ss) => {
                Some(ss.iter().map(|s: &String| s.to_string()).collect::<Vec<String>>().join(","))
            }
            _ => None,
        },
        _ => None,
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
*/