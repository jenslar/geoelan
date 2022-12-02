//! EAF constraint.

use serde::{
    Serialize,
    Deserialize,
    Serializer,
};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct Constraint {
    pub description: String,
    // pub stereotype: String,
    // #[serde(rename = "$attribute")] // DOES NOT EXIST. ISSUE: BECOMES VALUE ON SERIALIZE NOT ATTRIBUTE
    // https://github.com/tafia/quick-xml/issues/203#issuecomment-727233825
    // or
    // https://github.com/tafia/quick-xml/issues/326
    // #[serde(flatten)] // Err: no variant of enum StereoType found in flattened data
    pub stereotype: StereoType,
}

impl Constraint {
    pub fn from_stereotype(stereotype: &StereoType) -> Self {
        stereotype.to_constraint()
    }

    pub fn from_string(stereotype: &String) -> Self {
        StereoType::from_string(stereotype).to_constraint()
    }
}

// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[derive(Debug, Clone, Deserialize, PartialEq)]
// #[serde(untagged)] // Err: data did not match any variant of untagged enum StereoType
pub enum StereoType {
    #[serde(rename = "Included_In")]
    IncludedIn, // time alignable: true
    #[serde(rename = "Symbolic_Association")]
    SymbolicAssociation, // time alignable: true
    #[serde(rename = "Symbolic_Subdivision")]
    SymbolicSubdivision, // time alignable: true
    #[serde(rename = "Time_Subdivision")]
    TimeSubdivision, // time alignable: true
}

// #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
// #[derive(Debug, Clone, Deserialize, PartialEq)]
// // #[derive(Debug, Clone, PartialEq)]
// pub enum StereoType {
//     IncludedIn, // time alignable: true
//     SymbolicAssociation, // time alignable: true
//     SymbolicSubdivision, // time alignable: true
//     TimeSubdivision, // time alignable: true
// }

// trying to get around quick-xml untagged bug
// https://github.com/tafia/quick-xml/issues/203#issuecomment-727233825
// works for eaf + json so far
impl Serialize for StereoType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Self::IncludedIn => "Included_In",
            Self::SymbolicAssociation => "Symbolic_Association",
            Self::SymbolicSubdivision => "Symbolic_Subdivision",
            Self::TimeSubdivision => "Time_Subdivision",
        })
    }
}

// struct StrVisitor;

// impl<'de> Visitor<'de> for StrVisitor {
// impl<'de> Visitor<'de> for StereoType {
//     type Value = StereoType;

//     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//         formatter.write_str("one of Included_In, Symbolic_Association, Symbolic_Subdivision ,Time_Subdivision")
//     }
    
//     fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//     where E: serde::de::Error,
//     {
//         match value {
//             "Included_In" => Ok(Self::IncludedIn),
//             "Symbolic_Association" => Ok(Self::SymbolicAssociation),
//             "Symbolic_Subdivision" => Ok(Self::SymbolicSubdivision),
//             "Time_Subdivision" => Ok(Self::TimeSubdivision),
//         }
//         // Ok(value.to_owned())
//     }
// }

// // trying to get around quick-xml untagged bug
// // https://github.com/tafia/quick-xml/issues/203#issuecomment-727233825
// impl<'de> Deserialize<'de> for StereoType {
//     fn deserialize<D>(&self, deserializer: D) -> Result<String, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_str(match self {
//             "Included_In" => Self::IncludedIn,
//             "Symbolic_Association" => Self::SymbolicAssociation,
//             "Symbolic_Subdivision" => Self::SymbolicSubdivision,
//             "Time_Subdivision" => Self::TimeSubdivision,
//         })
//     }
// }

impl From<String> for StereoType {
    fn from(stereotype: String) -> Self {
        match stereotype.as_str() {
            "Included_In" => Self::IncludedIn,
            "Time_Subdivision" => Self::TimeSubdivision,
            "Symbolic_Subdivision" => Self::SymbolicSubdivision,
            "Symbolic_Association" => Self::SymbolicAssociation,
            s => panic!("(!) No such stereotype '{}'", s) // TODO return Result instead?
        }
    }
}

impl Into<String> for StereoType {
    fn into(self) -> String {
        match &self {
            Self::IncludedIn=> "Included_In".to_owned(),
            Self::TimeSubdivision=> "Time_Subdivision".to_owned(),
            Self::SymbolicSubdivision=> "Symbolic_Subdivision".to_owned(),
            Self::SymbolicAssociation=> "Symbolic_Association".to_owned(),
        }
    }
}

impl StereoType {
    pub fn to_constraint(&self) -> Constraint {
        match &self {
            Self::IncludedIn => Constraint{
                description: "Time alignable annotations within the parent annotation's time interval, gaps are allowed".to_owned(),
                stereotype: StereoType::IncludedIn},
                // stereotype: "Included_In".to_owned()},
            Self::SymbolicSubdivision => Constraint{
                description: "Symbolic subdivision of a parent annotation. Annotations refering to the same parent are ordered".to_owned(),
                stereotype: StereoType::SymbolicSubdivision},
                // stereotype: "Symbolic_Subdivision".to_owned()},
            Self::SymbolicAssociation => Constraint{
                description: "1-1 association with a parent annotation".to_owned(),
                stereotype: StereoType::SymbolicAssociation},
                // stereotype: "Symbolic_Association".to_owned()},
            Self::TimeSubdivision => Constraint{
                description: "Time subdivision of parent annotation's time interval, no time gaps allowed within this interval".to_owned(),
                stereotype: Self::TimeSubdivision},
                // stereotype: "Time_Subdivision".to_owned()},
        }
    }

    /// Checks whether a constraint is time alignable.
    pub fn time_alignable(&self) -> bool {
        match &self {
            StereoType::IncludedIn | StereoType::TimeSubdivision => true,
            StereoType::SymbolicAssociation | StereoType::SymbolicSubdivision => false,
        }
    }

    pub fn from_string(stereotype: &str) -> Self {
        match stereotype {
            "Included_In" => Self::IncludedIn,
            "Time_Subdivision" => Self::TimeSubdivision,
            "Symbolic_Subdivision" => Self::SymbolicSubdivision,
            "Symbolic_Association" => Self::SymbolicAssociation,
            s => panic!("(!) No such stereotype '{}'", s) // TODO return Result instead?
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            Self::IncludedIn => "Included_In".to_owned(),
            Self::TimeSubdivision => "Time_Subdivision".to_owned(),
            Self::SymbolicSubdivision => "Symbolic_Subdivision".to_owned(),
            Self::SymbolicAssociation => "Symbolic_Association".to_owned(),
        }
    }
}

