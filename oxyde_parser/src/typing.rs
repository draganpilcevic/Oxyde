use syn::{Block, Ident, __private::Span};

#[derive(Debug, PartialEq, Eq)]
pub enum Mutability {
    Mutable,
    View,
    Pure,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
}

impl Visibility {
    pub fn from_syn(input: syn::Visibility) -> Visibility {
        return match input {
            syn::Visibility::Public(_) => Visibility::Public,
            _ => Visibility::Private,
        };
    }
}

pub enum NativeTypes {
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
}

impl NativeTypes {
    pub fn to_solidity_id(&self) -> String {
        match self {
            NativeTypes::Uint8 => "uint8".into(),
            NativeTypes::Uint16 => "uint16".into(),
            NativeTypes::Uint32 => "uint32".into(),
            NativeTypes::Uint64 => "uint64".into(),
            NativeTypes::Uint128 => "uint128".into(),
            NativeTypes::Uint256 => "uint256".into(),
            NativeTypes::Int8 => "int8".into(),
            NativeTypes::Int16 => "int16".into(),
            NativeTypes::Int32 => "int32".into(),
            NativeTypes::Int64 => "int64".into(),
            NativeTypes::Int128 => "int128".into(),
            NativeTypes::Int256 => "int256".into(),

            NativeTypes::Boolean => "bool".into(),
            NativeTypes::Address => "address".into(),

            NativeTypes::String => "string".into(),

            NativeTypes::Bytes(nb_bytes) => format!("bytes{}", nb_bytes), // from 1 to 32 bytes
        }
        //.to_string()
    }

    pub fn to_rust_id(&self) -> String {
        match self {
            NativeTypes::Uint8 => "u8",
            NativeTypes::Uint16 => "u16",
            NativeTypes::Uint32 => "u32",
            NativeTypes::Uint64 => "u64",
            NativeTypes::Uint128 => "u128",
            NativeTypes::Uint256 => "u256",
            NativeTypes::Int8 => "i8",
            NativeTypes::Int16 => "i16",
            NativeTypes::Int32 => "i32",
            NativeTypes::Int64 => "i64",
            NativeTypes::Int128 => "i128",
            NativeTypes::Int256 => "i256",

            NativeTypes::Boolean => "bool",
            NativeTypes::Address => "Address",

            NativeTypes::String => "String",

            NativeTypes::Bytes(u8) => "Vec<u8>", // from 1 to 32 bytes
        }
        .to_string()
    }
}

pub enum ArgType {
    Native(NativeTypes),
    Array(Box<ArgType>),
    Mapping(Box<ArgType>, Box<ArgType>),
}

impl ArgType {
    pub fn to_rust_id(&self) -> String {
        match self {
            ArgType::Native(native_arg) => native_arg.to_rust_id(),
            ArgType::Mapping(type1, type2) => {
                format!("StorageMap<{},{}>", type1.to_rust_id(), type2.to_rust_id())
            }
            ArgType::Array(_) => panic!(),
        }
    }

    pub fn to_ident(&self) -> Ident {
        return Ident::new(&self.to_rust_id(), Span::mixed_site());
    }
}

pub struct Arg {
    pub name: Option<Ident>,
    pub r#type: syn::Type,
}

pub trait FromBlock {
    fn from_block(block: Block) -> Self;
}

pub struct ParsedFunction<T>
where
    T: FromBlock,
{
    pub name: Ident,
    pub visibility: Visibility,
    pub mutability: Mutability,
    pub args: Vec<Arg>,
    pub output: syn::ReturnType, // rust is single type return, what about solidity?
    pub inner: T,
}

pub struct EmptyInner {}

impl FromBlock for EmptyInner {
    fn from_block(_block: Block) -> Self {
        return EmptyInner {};
    }
}

pub struct ParsedInner {}

impl FromBlock for ParsedInner {
    fn from_block(_block: Block) -> Self {
        panic!("not implemented");
    }
}
