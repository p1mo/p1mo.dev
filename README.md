# p1mo.dev

### Personal Portfolio & Full-Stack Web Application

<p align="center">
  <a href="https://p1mo.dev">Live Website</a> ·
  <a href="https://github.com/p1mo/p1mo.dev">Source Code</a>
</p>

---

## About

**p1mo.dev** is my personal developer portfolio and full-stack web application, built from the ground up with Rust and modern web technologies.

The project combines a Rust-powered backend with a lightweight, client-side frontend built using Vite, HTML, HTMX and CSS (tailwindcss).

The goal is to create a fast, responsive and maintainable personal space to showcase my work, projects and technical interests while keeping the underlying architecture simple and efficient.

## Tech Stack

### Backend
- **[Rust](https://www.rust-lang.org/)** – Core application and server-side logic.
- **[Salvo](https://salvo.rs/)** – Asynchronous web framework.
- **[Tokio](https://tokio.rs/)** – Asynchronous runtime.
- **Serde** – Data serialization and deserialization.

### Frontend
- **[Vite](https://vite.dev/)** – Frontend development and build tooling.
- **HTML5** – Semantic markup and structure.
- **[HTMX](https://htmx.org/)** – Dynamic interactions with minimal JavaScript.
- **CSS / [Tailwind CSS](https://tailwindcss.com/)** – Styling and layout.
- **TypeScript** – Frontend build-time type checking.

## Project Structure

```text
p1mo.dev/
├── app/            # Vite frontend application
├── src/            # Rust backend source code
├── database/       # Database-related files
├── gen/            # Generated project files
├── Cargo.toml      # Rust dependencies and configuration
├── config.json     # Application configuration
├── build.rs        # Rust build script
└── README.md
```

## Getting Started

### Prerequisites

Make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- [Node.js](https://nodejs.org/)
- npm

### Clone the Repository

```bash
git clone https://github.com/p1mo/p1mo.dev.git
cd p1mo.dev
```

### Start the Backend

```bash
cargo run
```

The backend uses `config.json` for its host and port configuration.

### Start the Frontend

In a separate terminal:

```bash
cd app
npm install
npm run dev
```

### Build for Production

Build the frontend:

```bash
cd app
npm install
npm run build
```

Build the Rust application:

```bash
cd ..
cargo build --release
```

## Configuration

The application can be configured through `config.json`.

The following environment variables are also supported:

| Variable | Description | Default |
|---|---|---|
| `P1MO_CONFIG` | Path to the configuration file | `config.json` |
| `P1MO_LOG_LEVEL` | Logging verbosity | `info` |

## License

This project is licensed under the **Apache License 2.0**. See the [LICENSE](LICENSE) file for details.

---

<p align="center">Built with Rust, curiosity and a little bit of ☕</p>