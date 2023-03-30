use std::fs;

pub fn exec() {
    // read current directory
    let entries = fs::read_dir(".").unwrap();

    for entry in entries.into_iter().flatten() {
        let path = entry.path();

        let name = path.file_name().unwrap().to_str().unwrap();

        let new_name: String = whitespace_to_underline(name);

        if name == new_name {
            continue;
        }

        let res = fs::rename(name, &new_name);
        if res.is_err() {
            eprint!("Error: {} doesnt rename to {}", name, new_name);
            continue;
        }

        println!("Rename {} to {}", name, new_name);
    }
}

fn whitespace_to_underline(file_name: &str) -> String {
    file_name
        .chars()
        .map(|x| match x {
            ' ' => '_',
            '　' => '_',
            _ => x,
        })
        .collect()
}
