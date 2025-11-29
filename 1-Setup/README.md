# Setup von Toolchain und IDE

## Installation von VS Code unter Windows

1. Laden Sie sich die Installationsdatei herunter: [VS Code Downloads](https://code.visualstudio.com/download) in der Variante "Windows 10/11 User Installer".
2. Starten Sie die Installation und akzeptieren Sie alle Default-Werte im Wizzard abgesehen vom der Lizenz-Vereinbarung, die Sie akzeptieren müssen.

## Installation von rustup unter Windows

1. Öffnen Sie eine Powershell
2. Geben Sie folgendes Kommando ein, um den Linker für Windows herunterzuladen und zu installieren.
   ```
   winget install `
   --id Microsoft.VisualStudio.2022.BuildTools `
   --override "--add Microsoft.VisualStudio.Workload.VCTools `
   --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 `
   --add Microsoft.VisualStudio.Component.Windows11SDK.26100 `
   --quiet --wait --norestart --nocache"
   ```
4. Wechsel Sie in das Verzeichnis des Linkers mit dem Kommando
   ```
   cd "C:\Program Files (x86)\Microsoft Visual Studio"
   cd "2022\BuildTools\VC\Tools\MSVC\*\bin\Hostx64\x64"
   ```
   und fügen es dem Suchpfad zu mit dem Kommando
   ```
   [Environment]::SetEnvironmentVariable(`
   "Path", $env:Path + ';' + ${pwd}, "User")
   ```
6. Geben Sie das Kommando `winget install rustlang.rustup` ein. Rustup wird dann installiert. Zusätzlich wird die Rust-Toolchain heruntergeladen, mit der Sie Rust-Programme für Ihr Laptop übersetzen können.


## Installation des Rust Language Server für VS Code

1. Starten Sie *Visual Studio Code*
2. Drücken Sie [Strg] + [Shift] + [P] und geben das Kommando "Install Extensions" ein
3. Suchen Sie im Textfeld "Search Extensions in Marketplace" nach der Extension "rust-analyzer".
4. Klicken Sie bei der Extension "rust-analyzer" auf die Schaltfläche [Install].
5. Bestätigen Sie das sich öffnende Popup mit "Trust Publisher & Install"

## Testen der Installation

1. Erstellen Sie an beliebiger Stelle auf Ihrem Rechner einen Ordner mit dem Namen "rust-test"
2. Öffnen Sie diesen Ordner mit *Visual Studio Code* über die Menüleiste *File -> Open Folder*
3. Bestätigen Sie das Popup mit "Yes, I trust the authors".
4. Öffnen Sie eine Konsole mit der Tasten-Kombination [Strg] + [Shift] + [P] und dem Kommando "Create New Terminal". Als Profil wählen Sie "PowerShell".
5. Geben Sie in die Shell folgendes Kommando ein:
   ```
   cargo init
   ```
   Ein minimales Projekt wird erstellt.
6. Geben Sie in die Shell folgendes Kommando ein:
   ```
   cargo run
   ```
   Das Projekt sollte die Zeichenkette "Hello, world!" ausgeben.

## Installation von probe-rs

1. Öffnen Sie eine PowerShell.
2. Geben Sie folgendes Kommando ein:
   ```
   cargo install probe-rs-tools --locked
   ```
   Der Source-Code von probe-rs wird heruntergeladen und übersetzt. Dieser Vorgang kann einige Zeit in Anspruch nehmen.
4. Testen Sie die Installation durch das Kommando
   ```
   probe-rs --version
   ```
6. Laden Sie folgendes Tool herunter und halten es bereit, um während des Workshops dem USB-Gerät den korrekten Treiber zuzuordnen: [zadig](https://github.com/pbatard/libwdi/releases/download/v1.5.1/zadig-2.9.exe)
   **Achtung: Dieses Tool benötigt Admin-Rechte!**

## Installation der Cross-Toolchain

1. Öffnen Sie eine PowerShell.
2. Geben Sie folgendes Kommando ein:
   ```
   rustup target add riscv32imc-unknown-none-elf
   ```
   Es installiert die Cross-Toolchain für das Devboard.

## Download der Workshop-Unterlagen

Laden Sie die Workshop-Unterlage herunter:
https://github.com/SSV-embedded/rust-workshop/
