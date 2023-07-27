//pub mod stuff;

use oxyde_parser::{
    parse_method,
    typing::{Arg, EmptyInner, ParsedFunction},
};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{
    parse_macro_input, FnArg, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Signature, Type,
    Visibility,
};

fn collect_args_from_signature(signature: Signature) -> proc_macro2::TokenStream {
    let mut curr_id = 0usize;
    let method_name = signature.ident; //.to_string();

    let tuple_types /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(path) => {
                        Some(path.path.segments[0].ident.clone())
                    },
                    _ => None // let's say is unsupported
                }
            },
            _ => None,
        }
    }); //.collect();

    let args = quote! {(#(#tuple_types,)*)};

    // dispatch Tuple fields to the function call
    let tupled: Vec<proc_macro2::TokenStream> /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(_path) => {
                        //Some(path.path.segments[0].ident.clone())
                        let i = syn::Index::from(curr_id);
                        let res = Some(
                            quote!{
                                temp.#i
                            }
                        );
                        curr_id += 1;
                        res
                    },
                    _ => None // this should be the self argument
                }
            },
            _ => None,
        }
    }).collect();

    let branch = quote! {
        stringify!(#method_name) => {
            //println!("Called to: {}", stringify!(#method_name));
            // here try to deserialize input vector
            //let temp: (#(#tuple_types,)*) = bincode::deserialize(&data).unwrap();
            let temp: #args = deserialize(&data).unwrap();
            //println!("args: {}", stringify!(#args));
            //println!("{:?}", temp);

            return serialize(&self.#method_name(#(#tupled,)*)).unwrap();
            //return bincode::serialize(&#type_name::#method_name(#(#tupled,)*)).unwrap();

        },
    };

    return branch;
}

fn collect_args_from_signature_pure(
    type_name: Ident,
    signature: Signature,
) -> proc_macro2::TokenStream {
    let mut curr_id = 0usize;
    let method_name = signature.ident; //.to_string();

    let tuple_types /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(path) => {
                        Some(path.path.segments[0].ident.clone())
                    },
                    _ => None // let's say is unsupported
                }
            },
            _ => None,
        }
    }); //.collect();

    let args = quote! {(#(#tuple_types,)*)};

    // dispatch Tuple fields to the function call
    let tupled: Vec<proc_macro2::TokenStream> /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(_path) => {
                        //Some(path.path.segments[0].ident.clone())
                        let i = syn::Index::from(curr_id);
                        let res = Some(
                            quote!{
                                temp.#i
                            }
                        );
                        curr_id += 1;
                        res
                    },
                    _ => None // this should be the self argument
                }
            },
            _ => None,
        }
    }).collect();

    let branch = quote! {
        stringify!(#method_name) => {
            println!("Called to: {}", stringify!(#method_name));
            // here try to deserialize input vector
            //let temp: (#(#tuple_types,)*) = bincode::deserialize(&data).unwrap();
            let temp: #args = bincode::deserialize(&data).unwrap();
            //println!("args: {}", stringify!(#args));
            //println!("{:?}", temp);

            return bincode::serialize(&#type_name::#method_name(#(#tupled,)*)).unwrap();
        },
    };

    return branch;
}

fn extract_constructor(type_name: Ident, signature: Signature) -> proc_macro2::TokenStream {
    let mut curr_id = 0usize;
    let method_name = signature.ident; //.to_string();

    let tuple_types /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(path) => {
                        Some(path.path.segments[0].ident.clone())
                    },
                    _ => None // let's say is unsupported
                }
            },
            _ => None,
        }
    }); //.collect();

    let args = quote! {(#(#tuple_types,)*)};

    // dispatch Tuple fields to the function call
    let tupled: Vec<proc_macro2::TokenStream> /*: Vec<Ident>*/ = signature.inputs.iter().filter_map(|elem| {
        match elem {
            FnArg::Typed(pat_type) => {
                match *(pat_type.ty).clone() {
                    Type::Path(_path) => {
                        //Some(path.path.segments[0].ident.clone())
                        let i = syn::Index::from(curr_id);
                        let res = Some(
                            quote!{
                                temp.#i
                            }
                        );
                        curr_id += 1;
                        res
                    },
                    _ => None // this should be the self argument
                }
            },
            _ => None,
        }
    }).collect();

    let branch = quote! {
            // here try to deserialize input vector
            //let temp: (#(#tuple_types,)*) = bincode::deserialize(&data).unwrap();
            let temp: #args = bincode::deserialize(&data).unwrap();
            //println!("args: {}", stringify!(#args));
            //println!("{:?}", temp);

            return Box::new(#type_name::#method_name(#(#tupled,)*));
    };

    return branch;
}

fn args_to_tuple(args: &Vec<Arg>) -> proc_macro2::TokenStream {
    let arg_types: Vec<Type> = args.iter().map(|elem| elem.r#type.to_owned()).collect();
    return quote! {
        (#(#arg_types,)*)
    }
    .into();
}

#[proc_macro_attribute]
pub fn oxyde_entrypoints2(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let _input_stream = input.clone();

    let input = parse_macro_input!(input as ItemImpl);
    let struct_name = match *input.self_ty {
        Type::Path(path) => path.path.segments[0].ident.clone(),
        _ => panic!(),
    };

    // extract public methods
    let mut methods: Vec<ParsedFunction<EmptyInner>> = input
        .items
        .into_iter()
        .filter_map(|impl_item| match impl_item {
            ImplItem::Fn(item) => {
                let parsed = parse_method(item);
                if parsed.visibility == oxyde_parser::typing::Visibility::Public {
                    Some(parsed)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    // extract constructor
    let pos_constructor = methods
        .iter()
        .position(|elem| elem.name.to_string() == "constructor");
    let constructor = match pos_constructor {
        Some(pos) => Some(methods.remove(pos)),
        None => None,
    };

    // generate the trait components
    // start with mut
    let mut_methods: Vec<proc_macro2::TokenStream> = methods
        .iter()
        .filter(|elem| elem.mutability == oxyde_parser::typing::Mutability::Mutable)
        .map(|parsed| {
            let args = args_to_tuple(&parsed.args);
            let method_name = parsed.name.clone();

            let function_call_args: Vec<proc_macro2::TokenStream> = (0..parsed.args.len())
                .into_iter()
                .map(|id| {
                    let i = syn::Index::from(id);
                    quote! {a.#i}
                })
                .collect();

            quote! {
                stringify!(#(parsed.name)) => {
                    let temp: #args = deserialize(&data).unwrap();
                    self.#method_name(#(#function_call_args,)*);
                }
            }
        })
        .collect();

    return TokenStream::new();
}

#[proc_macro_attribute] //(Entrypoints)]
pub fn oxyde_entrypoints(_attr: TokenStream, input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    //let ast: DeriveInput = syn::parse(input).unwrap();
    //ast.attrs[0].
    let _input_stream = input.clone();

    let input = parse_macro_input!(input as ItemImpl);
    let input2 = input.clone();

    let name = match *input.self_ty {
        Type::Path(path) => path.path.segments[0].ident.clone(),
        _ => panic!(),
    };

    // get public methods
    let public_methods: Vec<ImplItemFn> = input
        .items
        .iter()
        .filter_map(|elem| match elem {
            ImplItem::Fn(method) => match method.vis {
                Visibility::Public(_) => Some(method.to_owned()),
                _ => None,
            },
            _ => None,
        })
        .collect();

    // extract mutable, view and pure functions seperately
    // NEED SPECIAL CASE FOR CONSTRUCTOR!
    let mut mutable_methods: Vec<proc_macro2::TokenStream> = vec![];
    let mut view_methods: Vec<proc_macro2::TokenStream> = vec![];
    let mut pure_methods: Vec<proc_macro2::TokenStream> = vec![];

    let mut constructor: Option<proc_macro2::TokenStream> = None;

    for method in public_methods {
        if method.sig.ident.to_string() == "constructor" {
            //panic!("found constructor");
            constructor = Some(extract_constructor(name.clone(), method.sig));
            continue;
        }

        if method.sig.inputs.len() == 0 {
            // is pure method for sure
            // return a branch?
            pure_methods.push(collect_args_from_signature_pure(name.clone(), method.sig));
        } else {
            match &method.sig.inputs[0] {
                // it's a pure function
                FnArg::Typed(_) => {
                    pure_methods.push(collect_args_from_signature_pure(name.clone(), method.sig));
                }
                // it's mutable or view
                // i don't really care which actually here?
                // actually I do need to dispatch to correct vector
                FnArg::Receiver(receiver) => match receiver.mutability {
                    // means it's mutable
                    Some(_) => {
                        mutable_methods.push(collect_args_from_signature(method.sig));
                    }
                    // it's a view function
                    None => {
                        view_methods.push(collect_args_from_signature(method.sig));
                    }
                },
            }
        }
    }

    let expanded = match constructor {
        Some(val) => {
            quote! {
                #input2

                impl Constructable for #name {
                    fn _constructor(data: Vec<u8>) -> Box<dyn Deref> {
                        #val
                    }
                }

                impl Entrypoint for #name {
                    fn execute(&mut self, method: &str, data: Vec<u8>) -> Vec<u8> {
                        match method {
                            #(#mutable_methods)*
                            _ => panic!("Unknown execute method: {}", method)
                        }
                    }

                    fn query(&self, method: &str, data: Vec<u8>) -> Vec<u8> {
                        match method {
                            #(#view_methods)*
                            #(#pure_methods)*
                            _ => panic!("Unknown query method: {}", method) //panic!(format!("Unknown query method: {}", method))
                        }
                    }
                }
            }
        }
        None => {
            quote! {
                #input2

                impl Entrypoint for #name {
                    fn execute(&mut self, method: &str, data: Vec<u8>) {
                        match method {
                            #(#mutable_methods)*
                            _ => println!("Unknown execute method: {}", method)
                        }
                    }

                    fn query(&self, method: &str, data: Vec<u8>) {
                        match method {
                            #(#view_methods)*
                            #(#pure_methods)*
                            _ => println!("Unknown query method: {}", method)
                        }
                    }
                }
            }
        }
    };

    return proc_macro::TokenStream::from(expanded);
}

#[proc_macro_derive(Deref)]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = input.ident;

    /*
    for field in input.fields {
        field.ident.unwrap()
    }
    */

    let fields = input.fields.iter().map(|item| {
        let field_name = item.ident.clone().unwrap();
        quote! {
            #field_name: self.#field_name.clone()
        }
    });

    let fields2 = input.fields.iter().map(|item| {
        let field_name = item.ident.clone().unwrap();
        quote! {
            self.#field_name.load_mut()
        }
    });

    quote! {
        impl Deref for #name {
            //fn deref(&self) -> Box<dyn DerefEntrypoints<Entrypoint>> {
            fn deref(&self) -> Box<dyn Entrypoint> {
                return Box::new(#name {
                    #(#fields,)*
                });
            }

            fn load_from_store(&mut self) {
                #(#fields2;)*
            }
        }
    }
    .into()
}
