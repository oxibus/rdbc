#[derive(PartialEq, Debug, Clone)]
pub enum AttributeValue {
    UnsignedInteger(u32),
    SignedInteger(u32),
    Double(f64),
    String(String),
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeDefault {
    attribute_name: String,
    attribute_value: AttributeValue,
}
