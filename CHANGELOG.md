# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-05-27

### Added
- **Initial release** of Pommet - Lightweight Local PHP Development Toolkit
- **Terminal User Interface (TUI)** built with Ratatui for managing PHP development stack
- **Automated installation system** for all required components
- **Plugin architecture** for managing individual services with Plugin trait
- **Apache HTTP Server v2.4.63** integration and management
- **PHP v8.4.7** installation and configuration
- **MariaDB v11.4.7** database server management
- **phpMyAdmin v5.2.2** web-based database administration interface
- **Service management** with start/stop functionality for all components
- **Pre-configured setup** with optimized settings for local development
- **Windows compatibility** with PowerShell and Command Prompt installation scripts
- **PATH environment variable** automatic configuration
- **Automatic component download** and extraction from official sources
- **Ready-to-use configuration files** for all services:
  - Apache httpd.conf
  - PHP php.ini  
  - MariaDB my.ini
  - phpMyAdmin config.inc.php
- **Clean uninstall process** with automatic cleanup
- **Keyboard navigation** for TUI interface
- **MIT/Apache-2.0 dual licensing**
- **Comprehensive documentation** in both Indonesian and English
- **GitHub repository** with issue templates and contribution guidelines

### Technical Features
- **Rust-based implementation** for performance and safety
- **Async/await support** with Tokio runtime for downloads
- **Cross-platform HTTP client** using reqwest with streaming support
- **Archive handling** for ZIP, TAR, and BZIP2 formats
- **Crossterm integration** for terminal control
- **Modular plugin system** with toggleable services
- **Error handling** throughout the application stack
- **Resource-efficient design** with minimal system footprint

### Service Access Points
- Web server: `http://localhost/`
- phpMyAdmin: `http://localhost/phpmyadmin`
- MariaDB: Port 3306 (root user, no password)
- Installation directory: `C:\pommet\`

### Documentation
- Comprehensive README with installation guides
- English documentation in `docs/README_EN.md`
- Installation options: PowerShell script, Command Prompt script, and manual download
- Uninstall guides for both automated and manual removal
- Configuration file locations and backup recommendations
- Development setup instructions for building from source

### Development Notes
- Early development stage (0.x version)
- Suitable for small to medium-scale development projects
- Ideal for learning and experimentation
- Stable for daily development needs
- Not recommended for production environments

### Known Limitations
- Windows-only support in this release
- Real-time logging not yet implemented
- Automatic service detection needs improvement
- Plugin trait architecture may require refactoring for better toggleable support

[0.1.0]: https://github.com/hippocampa/pommet/releases/tag/v0.1.0
