use std::{
    fs::{self, File},
    io::Read,
};

use oxyde_transpiler::{typing::*, utils};
use syn::{
    AngleBracketedGenericArguments, GenericArgument, ImplItem, Item, ItemImpl, ItemStruct,
    PathArguments, Type,
};

fn main() {
    let filename = "lib";
    let path = "sample_contracts/src/lib.rs";

    //let path = "src/lib.rs";
    let mut file = File::open(path).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let ast = syn::parse_file(&content).unwrap();
    /*
    if let Some(shebang) = ast.shebang {
        println!("{}", shebang);
    }
    */
    println!("{} items", ast.items.len());
    //println!("{} attrs", ast.attrs.len());

    // try to extract struct info and convert to Contract
    let mut contract = Contract::default();

    let struct_def = ast
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Struct(item_struct) => Some(item_struct.clone()),
            _ => None,
        })
        .collect::<Vec<ItemStruct>>()
        .get(0)
        .expect("No struct definition in file")
        .clone();

    contract.name = struct_def.ident.to_string();

    for field in struct_def.fields.iter() {
        let field_type = match field.ty.clone() {
            Type::Path(p) => {
                //Some(p.path.segments[0].ident)
                //p.path.segments[0].arguments[0].

                // can expect all to be storage item or storage map?
                let val = p.path.segments[0].ident.to_string();
                match val.as_str() {
                    "StorageItem" => match &p.path.segments[0].arguments {
                        PathArguments::AngleBracketed(stuff) => match &stuff.args[0] {
                            GenericArgument::Type(t) => match t {
                                Type::Path(p2) => p2.path.segments[0].ident.to_string(),
                                _ => panic!(),
                            },
                            _ => panic!("nope again"),
                        },
                        _ => panic!("nooooope"),
                    },
                    "StorageMap" => match &p.path.segments[0].arguments {
                        PathArguments::AngleBracketed(stuff) => match &stuff.args[0] {
                            GenericArgument::Type(t) => match t {
                                Type::Path(p2) => p2.path.segments[0].ident.to_string(),
                                _ => panic!(),
                            },
                            _ => panic!("nope again"),
                        },
                        _ => panic!("nooooope"),
                    },
                    _ => panic!("unknown"),
                }

                //p.path.segments[0].ident.to_string()
            }
            _ => panic!(),
        };

        let field_data = FieldData {
            visibility: Visibility::from_syn(&field.vis),
            name: field.ident.clone().unwrap().to_string(),
            r#type: field_type,
        };

        contract.fields.push(field_data);
    }

    let implementations = ast
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Impl(item_impl) => Some(item_impl.clone()),
            _ => None,
        })
        .collect::<Vec<ItemImpl>>()
        .get(0)
        .expect("no impl given")
        .clone();

    let methods: Vec<Method> = implementations
        .items
        .iter()
        .filter_map(|elem| match elem {
            ImplItem::Fn(impl_item_fn) => Some(Method::parse_syn(impl_item_fn)),
            _ => None,
        })
        .collect();

    contract.methods = methods;

    println!("{:?}", contract);

    print!("{:?}", contract.compile());

    let output = utils::render_contract(&contract);

    //fs::write("generated.sol", contract.compile()).expect("Unable to write file");
    fs::write(format!("{}.sol", filename), output.unwrap()).expect("Unable to write file");
}
