
fn main() {
    let data = "
fn main() {
    let data = \"aaa\";
    let mut recursed_data = String::new();
    for line in data.lines() {
        for word in line.split(' ') {
            if word == String::from(\"\\\"aa\") + \"a\\\";\" {
                recursed_data.push('\"');
                for char in data.chars() {
                    if (char == '\"') | (char == '\\\\')  {
                        recursed_data.push('\\\\');
                    }
                    recursed_data.push(char);
                }
                recursed_data.push_str(\"\\\";\");
            } else {
                recursed_data.push_str(word);
            }
            recursed_data.push(' ');
        }
        recursed_data = String::from(recursed_data.trim_end());
        recursed_data.push('\\n');
    }
    println!(\"{}\", recursed_data.trim_end());
}";
    let mut recursed_data = String::new();
    for line in data.lines() {
        for word in line.split(' ') {
            if word == String::from("\"aa") + "a\";" {
                recursed_data.push('"');
                for char in data.chars() {
                    if (char == '"') | (char == '\\')  {
                        recursed_data.push('\\');
                    }
                    recursed_data.push(char);
                }
                recursed_data.push_str("\";");
            } else {
                recursed_data.push_str(word);
            }
            recursed_data.push(' ');
        }
        recursed_data = String::from(recursed_data.trim_end());
        recursed_data.push('\n');
    }
    println!("{}", recursed_data.trim_end());
}
