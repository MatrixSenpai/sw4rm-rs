use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// XML Object
///
/// A metadata object that allows for more fine-tuned XML model definitions.
///
/// When using arrays, XML element names are not inferred (for singular/plural forms) and
/// the name property should be used to add that information. See examples for expected behavior.
#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(default, rename_all = "camelCase")]
pub struct XML {
    /// Replaces the name of the element/attribute used for the described schema property. When
    /// defined within the Items Object (items), it will affect the name of the individual XML
    /// elements within the list. When defined alongside type being array (outside the items), it
    /// will affect the wrapping element and only if wrapped is true. If wrapped is false, it
    /// will be ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the namespace definition. Value SHOULD be in the form of a URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The prefix to be used for the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    /// MAY be used only for an array definition. Signifies whether the array is wrapped (for
    /// example, <books><book/><book/></books>) or unwrapped (<book/><book/>). Default value is
    /// false. The definition takes effect only when defined alongside type being array (outside
    /// the items).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,

    /// Allows extensions to the Swagger Schema. The field name MUST begin with x-, for example,
    /// x-internal-id. The value can be null, a primitive, an array or an object. See Vendor
    /// Extensions for further details.
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub x_fields: HashMap<String, Value>,
}
