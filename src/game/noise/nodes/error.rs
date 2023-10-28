use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoiseError {
    // Node related errors
    #[error("Invalid node name: {0}")]
    InvalidNodeName(String),

    #[error("Node '{0}' not found")]
    NodeNotFound(String),

    #[error("Duplicate node name detected: {0}")]
    DuplicateNodeName(String),

    #[error("Invalid node definition: {0}")]
    InvalidNodeDefinition(String),

    #[error("Node '{0}' has missing required input")]
    MissingRequiredInput(String),

    #[error("Unexpected input provided for node '{0}'")]
    UnexpectedInput(String),

    // Constant related errors
    #[error("Constant '{0}' not found")]
    ConstantNotFound(String),

    #[error("Invalid value for constant '{0}': {1}")]
    InvalidConstantValue(String, String),

    #[error("Unresolved constant reference: {0}")]
    UnresolvedConstant(String),

    // Template related errors
    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("Invalid structure for template '{0}'")]
    InvalidTemplateStructure(String),

    #[error("Missing input '{0}' for template")]
    MissingTemplateInput(String),

    #[error("Invalid template input '{0}'")]
    InvalidTemplateInput(String),

    #[error("Invalid template definition: {0}")]
    InvalidTemplateDefinition(String),

    // Combiner and other specific node type errors
    #[error("Incompatible node types for '{0}'")]
    IncompatibleNodeTypes(String),

    // Serialization and Deserialization errors
    #[error("Error during serialization: {0}")]
    SerializationError(String),

    #[error("Error during deserialization: {0}")]
    DeserializationError(String),

    // Miscellaneous errors
    #[error("Invalid conversion attempted: {0}")]
    InvalidConversion(String),

    #[error("Circular reference detected involving '{0}'")]
    CircularReference(String),

    #[error("Missing output node: {0}")]
    MissingOutputNode(String),

    #[error("Validation failed for '{0}' due to: {1}")]
    ValidationFailed(String, String),

    #[error("General error: {0}")]
    General(String),

    // Node instance specific error
    #[error("Invalid node instance reference: {0}")]
    InvalidNodeInstance(String),
}
