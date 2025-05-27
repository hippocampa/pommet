<div align="center">
<pre>
··············································
:                                            :
:                                       _    :
:  _ __   ___  _ __ ___  _ __ ___   ___| |_  :
: | '_ \ / _ \| '_ ` _ \| '_ ` _ \ / _ \ __| :
: | |_) | (_) | | | | | | | | | | |  __/ |_  :
: | .__/ \___/|_| |_| |_|_| |_| |_|\___|\__| :
: |_|                                        :
:                                            :
··············································
</pre>
<h3 align="center">pommet</h3>

  <p align="center">
    Lightweight Local PHP Development Toolkit
    <br />
    <a href="https://github.com/hippocampa/pommet/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/hippocampa/pommet/issues/new?labels=enhancement&template=contribute-feature---.md">Contribute</a>
  </p>
</div>

> [!NOTE]
> This documentation is written in English. [Click here](../README.md) for the Indonesian version.

---
<div align="center">

![Pommet Terminal Interface](assets/pommet-tui.png)

</div>

## About Pommet

Pommet is a lightweight alternative to XAMPP specifically designed for PHP development on Windows. Unlike similar solutions that tend to be heavy and resource-intensive, Pommet comes with a minimalist approach without sacrificing the necessary functionality.

Pommet manages the following components:
- Apache HTTP Server (v2.4.63)
- PHP (v8.4.7)
- MariaDB (v11.4.7)
- phpMyAdmin (v5.2.2)

This application is built using Rust with a simple yet effective terminal interface to manage all your local development services.

### Development Status

Pommet is currently in early development stage with version 0.x. This means:

- Not yet recommended for production environments
- Suitable for small to medium-scale application development
- Ideal for learning and experimentation
- Stable for daily development needs

---

## Why Choose Pommet

### Efficient Resource Usage
Pommet is designed with minimalist principles. This application only uses resources that are truly needed, unlike other alternatives that often carry unused components.

### Automatic Installation
The entire installation and configuration process is done automatically. You don't need to bother downloading and setting up each component manually.

### Ready-to-Use Configuration
All services have been configured to work together without conflicts. You can immediately start developing applications without having to go through troublesome troubleshooting processes.

### Clean Interface
A simple yet powerful terminal interface allows you to manage all services from one place with intuitive keyboard navigation.

---

## Installation Guide

### System Requirements
- Windows 10 or 11 operating system
- Internet connection to download components
- Administrator access (only for installation)

### Installation Options

Choose one of the following installation methods:

#### Option 1: PowerShell (Automatic)
Open PowerShell as administrator and run:

```powershell
# Create pommet directory
New-Item -ItemType Directory -Force -Path "C:\pommet"

# Download latest executable
$url = "https://github.com/hippocampa/pommet/releases/latest/download/pommet.exe"
Invoke-WebRequest -Uri $url -OutFile "C:\pommet\pommet.exe"

# Add to PATH
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*C:\pommet*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;C:\pommet", "User")
}

Write-Host "Installation complete! Restart terminal and run 'pommet' to start."
```

#### Option 2: Command Prompt (Automatic)
Open Command Prompt as administrator and run:

```cmd
:: Create pommet directory
mkdir C:\pommet

:: Download executable (requires curl available in Windows 10+)
curl -L -o C:\pommet\pommet.exe https://github.com/hippocampa/pommet/releases/latest/download/pommet.exe

:: Add to PATH
setx PATH "%PATH%;C:\pommet" /M

echo Installation complete! Restart terminal and run 'pommet' to start.
```

#### Option 3: Manual Download
1. Visit [GitHub Releases](https://github.com/hippocampa/pommet/releases) page
2. Download `pommet.exe` file from the latest release
3. Create folder `C:\pommet` and move the `pommet.exe` file into it
4. Add `C:\pommet` to PATH environment variable:
   - Open System Properties → Advanced → Environment Variables
   - Select PATH in User variables → Edit
   - Add `C:\pommet` to the list
5. Restart terminal and run `pommet`

### After Installation

After installation is complete:
1. Open a new terminal (Command Prompt or PowerShell) - no need to run as administrator
2. Run the `pommet` command
3. Let Pommet download and install all components automatically
4. Application is ready to use

> **Note**: After installation, Pommet can be run without administrator rights. Admin rights are only required during initial installation.

### Service Access

After services are running, you can access:

| Service | Address | Description |
|---------|---------|-------------|
| Web server | http://localhost/ | Main development server |
| phpMyAdmin | http://localhost/phpmyadmin | Database management interface |
| MariaDB | Port 3306 | Direct database connection |
| Default user | root (no password) | Database credentials |

---

## Configuration

If you need to make adjustments, here are the configuration file locations:

```
C:/pommet/bin/
├── Apache24/conf/httpd.conf
├── php8/php.ini
├── mariadb-11.4.7-winx64/my.ini
└── Apache24/htdocs/phpMyAdmin-5.2.2-english/config.inc.php
```

Make sure to create backups before making changes to configuration files, and restart related services after making modifications.

## Uninstall Guide
To remove Pommet from the system:

### Option 1: PowerShell (Automatic)
```powershell
# Stop all Pommet services if running
taskkill /f /im "httpd.exe" 2>$null
taskkill /f /im "mysqld.exe" 2>$null
taskkill /f /im "pommet.exe" 2>$null

# Remove Pommet directory
Remove-Item -Recurse -Force "C:\pommet" -ErrorAction SilentlyContinue

# Remove from PATH
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$newPath = $currentPath -replace ";C:\\pommet", "" -replace "C:\\pommet;", "" -replace "C:\\pommet", ""
[Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

Write-Host "Pommet successfully removed from system."
```

### Option 2: Manual
1. Stop all Pommet services if running.
2. Delete the `C:\pommet` folder and all its contents.
3. Remove `C:\pommet` from PATH environment variable:
   - Open System Properties → Advanced → Environment Variables
   - Select PATH in User variables → Edit
   - Remove `C:\pommet` from the list
   - Restart terminal to ensure PATH changes are applied

---

## Development

### Building from Source Code

To build Pommet from source code, make sure Rust is installed on your system:

```bash
# Clone repository
git clone https://github.com/hippocampa/pommet.git
cd pommet

# Build project
cargo build --release

# Run application
./target/release/pommet.exe
```

---

## Contributing

This project still needs development help in several areas:

### Areas Needing Help
- Real-time logging implementation like in XAMPP
- Automatic detection for services still running
- Plugin trait refactoring, possibly requiring Toggleable trait

### How to Contribute
1. Fork this repository
2. Create a branch for new features
3. Commit your changes
4. Push to the created branch
5. Create a Pull Request

### Reporting Bugs
If you find a bug, please create a new issue including:
- Operating system version and specifications
- Steps to reproduce the bug
- Expected vs actual behavior
- Screenshots if necessary

---

## License

This project is distributed under the MIT License. See the `LICENSE` file for complete information.

---

## Acknowledgments

Thanks to the open source projects that make Pommet possible:

- [Apache HTTP Server](https://httpd.apache.org/) - Reliable web server
- [PHP](https://www.php.net/) - Powerful web programming language
- [MariaDB](https://mariadb.org/) - Fast database management system
- [phpMyAdmin](https://www.phpmyadmin.net/) - Web-based database management interface
- [Rust](https://www.rust-lang.org/) - Safe and fast programming language

---

<div align="center">
  <p>Give Pommet a <a href="https://github.com/hippocampa/pommet/stargazers">star</a>!</p>
</div>