use syn::{FnArg, ImplItemFn, ItemFn, Pat, Type};

use crate::typing::{SolidityTypes, Visibility};

pub struct Method<T> {
    pub name: String,
    pub is_mutable: bool,
    pub args: Vec<T>,
    pub outputs: Vec<T>,
    pub inner: (),
}

pub struct FreeFunction<T> {
    pub name: String,
    pub args: Vec<u8>,
    pub outputs: Vec<T>,
    pub inner: (),
}

pub enum Mutability {
    Mutable,
    View,
    Pure,
}

pub struct ParsedFunction<T> {
    pub name: String,
    pub is_public: bool,
    pub mutability: Mutability,
    pub args: Vec<(String, T)>,
    pub outputs: Vec<T>,
    pub inner: (),
}

impl<T> ParsedFunction<T> {
    pub fn new() -> Self {
        return ParsedFunction {
            name: "".into(),
            is_public: true,
            mutability: Mutability::Pure,
            args: vec![],
            outputs: vec![],
            inner: (),
        };
    }
}

pub fn parse_method(item: ImplItemFn) -> ParsedFunction<SolidityTypes> {
    let mut output: ParsedFunction<SolidityTypes> = ParsedFunction::new();

    output.name = item.sig.ident.to_string();
    output.is_public = match item.vis {
        syn::Visibility::Public(_) => true,
        _ => false,
    };

    if item.sig.inputs.len() == 0 {
        output.mutability = Mutability::Pure;
    } else {
        for arg in item.sig.inputs {
            match arg {
                FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_some() {
                        output.mutability = Mutability::Mutable;
                    } else {
                        output.mutability = Mutability::View;
                    }
                }
                FnArg::Typed(pat_type) => {
                    let out_arg_name = match *pat_type.pat {
                        Pat::Ident(ident) => ident.ident.to_string(),
                        _ => panic!("unknown"),
                    };

                    let out_arg_type = match *pat_type.ty {
                        Type::Path(p) => {
                            SolidityTypes::from_string(&p.path.segments[0].ident.to_string())
                        }
                        _ => panic!("not implemented"),
                    };

                    output.args.push((out_arg_name, out_arg_type));
                }
            }
        }
    }

    return output;
}

pub fn parse_function(_item: ItemFn) {
    panic!("not relevant");
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    #[test]
    fn test_parse_function_signature() {
        let target = "pub fn my_func(val_1: u64, val_2: String) {}";

        let val = parse_str::<syn::ItemFn>(target).unwrap();
    
        
    }
}
