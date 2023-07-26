pub fn node() {
    let vscode_profile_name: &str = "node";

    println!(
        "
    Thank you for choosing rust! Here are some shortcuts:

        • f1: run (electronmonw)
        • f2: kill terminal
        • f3: toggle explorer

    Next its time to give your project a name:
        "
    );

    let mut input = String::new();

    print!("> ");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let arg: &str = input.trim();

    let _ = fs::create_dir(arg);

    let file_name = "package.json";
    let file_path = Path::new(arg).join(file_name);
    let mut file = File::create(&file_path).unwrap();

    let base_string = String::from(
        r#"
    {
      "name": "",
      "version": "",
      "description": "",
      "main": "scripts/main.js",
      "scripts": {
        "start": "cls && npx electronmon ."
      },
      "author": "",
      "license": "ISC"
    }
                      "#,
    );
    let to_insert = arg;
    let index = 22;

    let base_string: String = base_string
        .chars()
        .take(index)
        .chain(to_insert.chars())
        .chain(base_string.chars().skip(index))
        .collect();

    let base_str: &str = &base_string;

    let _ = file.write_all(base_str.as_bytes());

    Command::new("cmd")
        .args(&["/C", "npm", "install", "--prefix", arg, "electron"])
        .spawn()
        .expect("Failed to execute npm");

    Command::new("cmd")
        .args(&["/C", "npm", "install", "--prefix", arg, "electronmon"])
        .spawn()
        .expect("Failed to execute npm");

    let _ = fs::create_dir(Path::new(arg).join("scripts"));

    let file_name = "main.js";
    let file_path = Path::new(arg).join("scripts").join(file_name);
    let mut file = File::create(&file_path).unwrap();

    let content = "
console.log('Hello World!')
                  ";

    let _ = file.write_all(content.as_bytes());

    Command::new("cmd")
        .args(&["/C", "code", arg, "--profile", vscode_profile_name])
        .spawn()
        .expect("Failed to open Visual Studio Code.");

    println!(
        "
    Boilerplate created!
        "
    );
}
