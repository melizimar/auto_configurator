use std::process::Command;

pub fn install_fonts(font_folder: &str) {
    // Comando PowerShell para instalar as fontes TTF
    let install_fonts_command = format!(
        r#"Get-ChildItem "{}\*.ttf" | ForEach-Object {{ Install-Font $_.FullName -Scope AllUsers }}"#,
        font_folder
    );

    // Executando o comando PowerShell para instalar as fontes
    let status = Command::new("powershell")
        .args(&["-Command", &install_fonts_command])
        .status()
        .expect("Falha ao executar o comando PowerShell para instalar fontes");

    if status.success() {
        println!("Fontes instaladas com sucesso!");
    } else {
        eprintln!("Falha ao instalar fontes");
    }
}