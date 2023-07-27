use syn::{FnArg, ImplItemFn, ItemFn, ImplItem,};
use typing::{Arg, FromBlock, Mutability, ParsedFunction, Visibility};

pub mod typing;
pub mod visitors;

pub fn parse_function<T>(item: ItemFn) -> ParsedFunction<T>
where
    T: FromBlock,
{
    let mut mutability: Mutability = Mutability::Pure;

    let mut args = vec![];
    for arg in item.sig.inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    mutability = Mutability::Mutable;
                } else {
                    mutability = Mutability::View;
                }
            }
            FnArg::Typed(pat_type) => {
                args.push(Arg {
                    name: match *pat_type.pat {
                        syn::Pat::Ident(ident) => Some(ident.ident.clone()),
                        _ => panic!("unknown"),
                    },
                    r#type: (*pat_type.ty).clone(),
                });
            }
        }
    }

    return ParsedFunction {
        name: item.sig.ident,
        visibility: Visibility::from_syn(item.vis),
        mutability: mutability,
        args: args,
        output: item.sig.output,
        inner: T::from_block(*item.block),
    };
}

pub fn parse_method<T>(item: ImplItemFn) -> ParsedFunction<T>
where
    T: FromBlock,
{
    let mut mutability: Mutability = Mutability::Pure;

    let mut args = vec![];
    for arg in item.sig.inputs {
        match arg {
            FnArg::Receiver(receiver) => {
                if receiver.mutability.is_some() {
                    mutability = Mutability::Mutable;
                } else {
                    mutability = Mutability::View;
                }
            }
            FnArg::Typed(pat_type) => {
                args.push(Arg {
                    name: match *pat_type.pat {
                        syn::Pat::Ident(ident) => Some(ident.ident.clone()),
                        _ => panic!("unknown"),
                    },
                    r#type: (*pat_type.ty).clone(),
                });
            }
        }
    }

    return ParsedFunction {
        name: item.sig.ident,
        visibility: Visibility::from_syn(item.vis),
        mutability: mutability,
        args: args,
        output: item.sig.output,
        inner: T::from_block(item.block),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
