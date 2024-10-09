#[derive(PartialEq, Debug, Clone)]
pub struct AttributeIntegerValueType {
    pub minimum: i32,
    pub maximum: i32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeHexValueType {
    pub minimum: i32,
    pub maximum: i32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeFloatValueType {
    pub minimum: f64,
    pub maximum: f64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeStringValueType {
    pub value: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttributeEnumValueType {
    pub values: Vec<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AttributeValueType {
    Integer(AttributeIntegerValueType),
    Hex(AttributeHexValueType),
    Float(AttributeFloatValueType),
    String(AttributeStringValueType),
    Enum(AttributeEnumValueType),
}

#[derive(PartialEq, Debug, Clone)]
pub struct NetworkAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct NodeAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MessageAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SignalAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct EnvironmentVariableAttribute {
    pub attribute_name: String,
    pub attribute_value_type: AttributeValueType,
}

#[derive(PartialEq, Debug, Clone)]
pub enum AttributeDefinition {
    Network(NetworkAttribute),
    Node(NodeAttribute),
    Message(MessageAttribute),
    Signal(SignalAttribute),
    EnvironmentVariable(EnvironmentVariableAttribute),
}
