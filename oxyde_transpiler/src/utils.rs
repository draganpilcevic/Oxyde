use crate::typing::{Contract, FieldData, Input, Method, SolidityTypes};

pub fn render_contract(contract: &Contract) -> Result<String, ()> {
    /*
    let mut output = vec![];
    let mut curr_depth = 0usize;
    */

    let license = "UNLICENSED";
    let pragma_solidity = "pragma solidity >=0.8.2 <0.9.0;";

    let val = format!(
        "// SPDX-License-Identifier: {}
{}

contract {} {{

{}

{}
}}
",
        license,
        pragma_solidity,
        contract.name,
        render_fields(&contract.fields, 1).unwrap(),
        render_methods(&contract.methods, 1).unwrap() //contract.render_methods(&contract.methods)
    );

    return Ok(val);

    /*
    output.push("// SPDX-License-Identifier: UNLICENSED\n".into());
    output.push("pragma solidity >=0.8.2 <0.9.0;\n".into());
    output.push(format!("contract {} {{", contract.name));
    curr_depth += 1;


    output.push(render_fields(&contract.fields, curr_depth).unwrap());

    output.push("}".into());

    return Ok(output.join("\n"));
    */
}

fn render_fields(fields: &Vec<FieldData>, curr_depth: usize) -> Result<String, ()> {
    let mut output = vec![];

    for field in fields.iter() {
        output.push(format!(
            "{}{} {} {};",
            "   ".repeat(curr_depth),
            SolidityTypes::convert(&field.r#type), //::from_string(&field.r#type).to_string(),
            field.visibility.to_string(),
            field.name
        ));
    }

    return Ok(output.join("\n"));
}

fn render_method_inputs(args: &Vec<Input>) -> Result<String, ()> {
    let mut output: Vec<String> = vec![];

    for arg in args {
        output.push(format!("{} {}", arg.r#type.to_string(), arg.name))
    }

    return Ok(output.join(", "));
}

fn render_methods(methods: &Vec<Method>, curr_depth: usize) -> Result<String, ()> {
    let mut output = vec![];

    for method in methods.iter() {
        if method.name == "constructor" {
            output.push(format!(
                "{spacing}constructor({args}) {{}}",
                spacing = "   ".repeat(curr_depth),
                args = render_method_inputs(&method.inputs).unwrap(),
                //SolidityTypes::convert(&field.r#type),  //::from_string(&field.r#type).to_string(),
            ));
        } else {
            output.push(format!(
                "{spacing}function {function_name}({args}) {visibility} {{}}",
                spacing = "   ".repeat(curr_depth),
                function_name = method.name,
                args = render_method_inputs(&method.inputs).unwrap(), //"uint64 placeholder",
                visibility = method.visibility.to_string() //SolidityTypes::convert(&field.r#type),  //::from_string(&field.r#type).to_string(),
            ));
        }
    }

    return Ok(output.join("\n\n"));
}
