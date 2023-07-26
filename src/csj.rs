pub fn csj() {
    let vscode_profile_name: &str = "client side javascript";

    println!(
        "
    Thank you for choosing client side javascript!

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

    let mut file_name = "index.html";
    let mut file_path = Path::new(arg).join(file_name);
    let mut file = File::create(&file_path).unwrap();

    file_name = "style.css";
    file_path = Path::new(arg).join(file_name);
    let _ = File::create(&file_path).unwrap();

    file_name = "app.js";
    file_path = Path::new(arg).join(file_name);
    let _ = File::create(&file_path).unwrap();

    let content = "
<!DOCTYPE html>
<html lang='en'>
    
<head>
    <meta charset='UTF-8'>
    <meta name='viewport' content='width=device-width, initial-scale=1.0'>
    <meta http-equiv='X-UA-Compatible' content='ie=edge'>
    <title>My Website</title>
    <link rel='stylesheet' href='./style.css'>
</head>
    
<body>
    <main>
            <h1>Welcome to My Website</h1>
    </main>
    <script src='app.js'></script>
</body>
    
</html>
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
