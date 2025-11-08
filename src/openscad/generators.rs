use std::fmt::Debug;

pub fn generate_matrix_source<T>(name: String, data: Vec<Vec<T>>) -> String
where
    T: ToString,
    T: Debug,
{
    let mut builder = String::new();

    builder.push_str("// To be used in other OpenScad source file with\n");
    builder.push_str(format!("// include <{}.scad>\n", name).as_str());
    builder.push_str(format!("{} = [\n", name).as_str());
    for row in data.iter() {
        builder.push_str(format!("    {:?},\n", row).as_str());
    }
    builder.push_str("];\n");

    return builder;
}
