# 🔨 cargo-forge

A powerful scaffolding tool for Rust projects. Generate production-ready project structures in seconds.

## ✨ Features

- **🚀 Fast Project Generation**: Go from empty directory to working project in seconds
- **📦 Built-in Templates**: CLI apps, web services, and more
- **🔧 Production Ready**: Templates include best practices and common dependencies
- **💾 Embedded Templates**: No network required - templates are bundled with the binary
- **🎯 Cargo Integration**: Works as a cargo subcommand (`cargo forge`)

## 🚀 Quick Start

### Installation

```bash
# Install from source (for now)
git clone https://github.com/your-username/cargo-forge
cd cargo-forge
cargo install --path .
```

### Usage

Create a new CLI application:
```bash
cargo forge new my-cli-app
```

Create a web API with Axum:
```bash
cargo forge new my-api --template axum-api
```

## 📚 Available Templates

### 🖥️ CLI Template (`cli`)
Perfect for command-line tools and utilities.

**Includes:**
- `clap` for argument parsing
- Basic CLI structure with options and help
- Ready-to-run example

**Example:**
```bash
cargo forge new my-tool --template cli
cd my-tool
cargo run -- --name "World" --count 3
```

### 🌐 Axum API Template (`axum-api`)
Production-ready web service template using Axum.

**Includes:**
- `axum` for the web framework
- `tokio` for async runtime
- `serde` and `serde_json` for JSON handling
- `tracing` for logging
- Health check endpoint
- Example CRUD endpoint structure
- Proper error handling

**Example:**
```bash
cargo forge new my-api --template axum-api
cd my-api
cargo run
# Visit http://127.0.0.1:3000/health
```

## 🛠️ Development Roadmap

### ✅ Milestone 1: MVP (Current)
- [x] Basic CLI interface
- [x] Template system with string replacement
- [x] Embedded templates using `include_dir`
- [x] CLI and Axum API templates

### 🎯 Milestone 2: Enhanced Templating
- [ ] Full Tera templating engine support
- [ ] Conditional features (add logging, database, Docker, etc.)
- [ ] Interactive template selection
- [ ] Git integration (auto-init repos)

### 🎯 Milestone 3: More Templates
- [ ] TUI application template (Ratatui)
- [ ] WASM library template
- [ ] Desktop GUI template (Tauri/egui)
- [ ] Game template (Bevy)

### 🎯 Milestone 4: Advanced Features
- [ ] Custom template support from Git repositories
- [ ] Template discovery and sharing
- [ ] Configuration file support
- [ ] Plugin system

## 🏗️ Project Structure

```
cargo-forge/
├── src/
│   ├── main.rs          # Main entry point
│   └── cli.rs           # CLI argument parsing
├── templates/
│   ├── cli/             # CLI application template
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── axum-api/        # Web API template
│       ├── Cargo.toml
│       └── src/main.rs
└── Cargo.toml
```

## 🤝 Contributing

Contributions are welcome! Here are some ways you can help:

1. **Add new templates**: Create templates for different project types
2. **Improve existing templates**: Add features, fix bugs, improve documentation
3. **Enhance the CLI**: Add new features, improve UX
4. **Documentation**: Improve READMEs, add examples, write guides

### Adding a New Template

1. Create a new directory in `templates/`
2. Add your template files with `{{ project_name }}` placeholders
3. Update the template selection logic in `main.rs`
4. Test your template thoroughly

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by tools like `create-react-app`, `rails new`, and `cargo generate`
- Built with the amazing Rust ecosystem
- Thanks to the Rust community for feedback and suggestions

---

**Happy forging! 🔨✨**
