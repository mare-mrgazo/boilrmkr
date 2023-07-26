pub fn py() {
    let vscode_profile_name: &str = "python";

    println!(
        "
    Thank you for choosing python! Here are some shortcuts:

    • f1: run
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

    let root = Path::new(arg).join("env");

    Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(root)
        .spawn()
        .expect("python command failed to start");

    let _ = fs::create_dir(Path::new(arg).join("project"));

    let file_name = "main.py";
    let file_path = Path::new(arg).join("project").join(file_name);
    let mut file = File::create(&file_path).unwrap();

    let content = "
print('Hello World!')
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
