use serde::{Deserialize, Serialize};
use syn::{FnArg, Pat, Type};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct FieldData {
    pub visibility: Visibility,
    pub name: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Contract {
    pub name: String,
    pub fields: Vec<FieldData>, //Vec<(String, String)>, //SolidityTypes)>,
    pub methods: Vec<Method>,
}

impl Default for Contract {
    fn default() -> Self {
        return Contract {
            name: "".into(),
            fields: vec![],
            methods: vec![],
        };
    }
}

impl Contract {
    pub fn compile(&self) -> String {
        let mut output = vec![];
        let mut curr_depth = 0usize;

        output.push("// SPDX-License-Identifier: UNLICENSED\n".into());
        output.push("pragma solidity >=0.8.2 <0.9.0;\n".into());
        output.push(format!("contract {} {{", self.name));
        curr_depth += 1;

        for field in self.fields.iter() {
            output.push(format!(
                "{} {} {} {};",
                "   ".repeat(curr_depth),
                SolidityTypes::from_string(&field.r#type).to_string(),
                field.visibility.to_string(),
                field.name
            ));
        }

        output.push("}".into());

        return output.join("\n");
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

impl Visibility {
    pub fn to_string(&self) -> &str {
        return match self {
            Visibility::Public => "public",
            Visibility::Private => "private",
        };
    }
}

impl Visibility {
    pub fn from_syn(syn_visibility: &syn::Visibility) -> Self {
        return match syn_visibility {
            syn::Visibility::Public(_) => Visibility::Public,
            _ => Visibility::Private,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Input {
    pub name: String,
    pub r#type: SolidityTypes,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum Mutability {
    Pure,
    Mutable,
    NonMutable,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Method {
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub name: String,
    pub inputs: Vec<Input>,
}

impl Method {
    pub fn parse_syn(input: &syn::ImplItemFn) -> Self {
        Method {
            name: input.sig.ident.to_string(),
            visibility: Visibility::from_syn(&input.vis),
            mutability: match input.sig.inputs[0].clone() {
                FnArg::Receiver(receiver) => {
                    match receiver.mutability {
                        None => Mutability::NonMutable, // not sure
                        Some(_) => Mutability::Mutable,
                    }
                }
                _ => Mutability::Pure,
            },
            inputs: input
                .sig
                .inputs
                .iter()
                .filter_map(|elem| match elem {
                    FnArg::Typed(val) => {
                        let type_val = match *val.ty.clone() {
                            Type::Path(p) => p.path.segments[0].ident.to_string(),
                            _ => return None,
                        };

                        let name = match *val.pat.clone() {
                            Pat::Ident(ident) => ident.ident.to_string(),
                            _ => return None,
                        };

                        Some(Input {
                            name: name,
                            r#type: SolidityTypes::from_string(&type_val),
                        })
                    }
                    _ => None,
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum OxydeToken {
    Contract {
        name: String,
        fields: Vec<SolidityTypes>,
        methods: Vec<Method>,
    },
    Method(Method),
    Field {
        visibility: Visibility,
        r#type: SolidityTypes,
    },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub enum SolidityTypes {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Uint256,
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Int256,

    Boolean,
    Address,

    String,

    Bytes(u8), // from 1 to 32 bytes 

    Array(Box<SolidityTypes>),

    Mapping,

    Unknown(String),
}

impl SolidityTypes {
    pub fn convert(input: &str) -> String {
        return SolidityTypes::from_string(input).to_string().to_owned();
    }

    pub fn from_string(input: &str) -> SolidityTypes {
        println!("type from_string: {}", input);
        return match input {
            "u8" => SolidityTypes::Uint8, 
            "u16" => SolidityTypes::Uint16, 
            "u32" => SolidityTypes::Uint32, 
            "u64" => SolidityTypes::Uint64, 
            "u128" => SolidityTypes::Uint128, 
            "u256" => SolidityTypes::Uint256, 
            "U256" => SolidityTypes::Uint256,
            "i8" => SolidityTypes::Int8, 
            "i16" => SolidityTypes::Int16, 
            "i32" => SolidityTypes::Int32, 
            "i64" => SolidityTypes::Int64, 
            "i128" => SolidityTypes::Int128, 
            "i256" => SolidityTypes::Int256,
            "bool" => SolidityTypes::Boolean,
            "Address" => SolidityTypes::Address,
            "String" => SolidityTypes::String,
            _ => SolidityTypes::Unknown("unknown".into()), 
            //_ => panic!("not implemented"),
        };
    }

    pub fn to_string(&self) -> &str {
        return match self {
            SolidityTypes::Uint8 => "uint8",
            SolidityTypes::Uint16 => "uint16",
            SolidityTypes::Uint32 => "uint32",
            SolidityTypes::Uint64 => "uint64",
            SolidityTypes::Uint128 => "uint128",
            SolidityTypes::Uint256 => "uint256",
            SolidityTypes::Int8 => "int8",
            SolidityTypes::Int16 => "int16",
            SolidityTypes::Int32 => "int32",
            SolidityTypes::Int64 => "int64",
            SolidityTypes::Int128 => "int128",
            SolidityTypes::Int256 => "int256",
            SolidityTypes::Boolean => "bool",
            SolidityTypes::Address => "address",
            SolidityTypes::String => "string",
            //_ => panic!(),
            SolidityTypes::Unknown(inner) => &inner,
            _ => panic!(),
        };
    }
}




#[cfg(test)] 
mod tests {
    #[test]
    fn test_storage_element() {

    }

}