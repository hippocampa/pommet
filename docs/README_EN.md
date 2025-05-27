# Pommet

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
    php local mini development toolkit
    <br />
    <a href="https://github.com/hippocampa/pommet">View Demo</a>
    &middot;
    <a href="https://github.com/hippocampa/pommet/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>
    &middot;
    <a href="https://github.com/hippocampa/pommet/issues/new?labels=enhancement&template=contribute-feature---.md">Contribute</a>
  </p>
</div>

> [!NOTE]
> This documentation is in English. [Click here](../README.md) for the Indonesian version.

## About Pommet

Pommet is a lightweight PHP development environment manager for Windows, similar to XAMPP but with a minimal footprint. It manages:

- Apache HTTP Server (v2.4.63)
- PHP (v8.4.7)
- MariaDB (v11.4.7)
- phpMyAdmin (v5.2.2)

Built with Rust, Pommet features a terminal-based user interface for managing your local development services with minimal resource usage.

## Features

- **Lightweight**: Requires significantly less system resources than alternatives
- **Automatic setup**: Downloads and configures all components automatically
- **Pre-configured**: All services are pre-configured to work together seamlessly
- **Terminal UI**: Clean, intuitive interface for managing services
- **Integrated management**: Start, stop, and monitor services from a single interface
- **Minimal installation**: Only installs what you need, no bloatware

## Getting Started

### Prerequisites

- Windows OS (10/11 recommended)
- Internet connection (for initial download of components)

### Installation

1. Download the latest release from the [releases page](https://github.com/hippocampa/pommet/releases)
2. Run the executable with administrator privileges
3. Pommet will automatically download and install all required components to `C:/pommet`

### Usage

Run Pommet and use the keyboard shortcuts to control your services:


When services are running:
- Web server: http://localhost/
- phpMyAdmin: http://localhost/phpmyadmin
- MariaDB port: 3306
- Default MariaDB user: root (no password)

### Configuration Files

Pommet uses pre-configured files for optimal development experience:

- **Apache**: C:/pommet/bin/Apache24/conf/httpd.conf
- **PHP**: C:/pommet/bin/php8/php.ini
- **MariaDB**: C:/pommet/bin/mariadb-11.4.7-winx64/my.ini
- **phpMyAdmin**: C:/pommet/bin/Apache24/htdocs/phpMyAdmin-5.2.2-english/config.inc.php

## Development

### Building from Source

To build Pommet from source, you'll need Rust installed on your system.

```sh
# Clone the repository
git clone https://github.com/hippocampa/pommet.git
cd pommet

# Build the project
cargo build --release

# Run the application
./target/release/pommet.exe
```

## Help Needed

We welcome contributions! Here are some areas where help is needed:

- [ ] Realtime log implementation similar to XAMPP (requires TUI update)
- [ ] Auto-detect if a plugin is still running
- [ ] Refactor `Plugin` trait - Maybe we need a `Toggleable` trait for plugins that can be toggled

## License

Distributed under the MIT License. See `LICENSE` file for more information.

## Acknowledgments

- [Apache HTTP Server](https://httpd.apache.org/)
- [PHP](https://www.php.net/)
- [MariaDB](https://mariadb.org/)
- [phpMyAdmin](https://www.phpmyadmin.net/)
- [Rust](https://www.rust-lang.org/)
