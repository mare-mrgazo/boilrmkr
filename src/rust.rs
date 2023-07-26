pub fn rust() {
    let vscode_profile_name: &str = "roost";

    println!(
        "
    Thank you for choosing rust! Here are some shortcuts:

        • f1: compile and run
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

    Command::new("cargo")
        .arg("init")
        .arg(arg)
        .spawn()
        .expect("cargo command failed to start");

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
