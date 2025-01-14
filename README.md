# Pact for Zed

A zed extension that brings full Pact language support to the [Zed](https://github.com/zed-industries/zed) editor.

## ✨ Features

- 🎨 Syntax highlighting
- 📑 Smart outline view for quick navigation
- 📦 Code folding
- ⚡️ Built-in tasks for running Pact files and tests
- 🚀 Intelligent code assistance:
  - Auto-completion
  - Real-time error checking
  - Hover documentation
  - Go to definition
  - And more!

## 🚀 Getting Started

### Option 1: Using Pactup (Recommended)

Pactup is the easiest way to manage your Pact installation:

follow the instructions to install Pactup: https://github.com/kadena-community/pactup then run the following commands:

```bash
# Install the latest nightly build
pactup install nightly --force

# Set it as your active version
pactup use nightly
```

Quick one-liner (no installation required):

```bash
npx pactup install nightly --force && npx pactup which nightly
# /home/salama/.local/share/pactup/pact-versions/nightly
```

Copy the path to the pact installation and append `/bin/pact` then paste it in your Zed settings

```json
{
  "lsp": {
    "pact": {
      "path": "/home/salama/.local/share/pactup/pact-versions/nightly/bin/pact",
      "arguments": ["--lsp"]
    }
  }
}
```

### Option 2: Manual Configuration

You can specify a custom Pact binary in your Zed settings:

1. Open Zed Settings
2. Add the following configuration:

```json
{
  "lsp": {
    "pact": {
      "path": "/path/to/your/pact",
      "arguments": ["--lsp"]
    }
  }
}
```

## 🔍 How It Works

The extension will look for a Pact binary in this order:

1. System PATH
2. Custom path specified in Zed settings
3. Automatically downloaded nightly build (cached for future use)

> 💡 **Tip**: Using Pactup is recommended as it helps you manage and update your Pact installation easily.

## 🔄 Keeping Pact Updated

To update to the latest nightly build:

```bash
pactup install nightly --force
pactup use nightly
```

You can also specify a Pact version for your project by creating a `.pact-version` file in your project root:

```
nightly
```

## 🛠️ Development

Want to contribute? Here's how to set up the extension locally:

1. Clone the repository:

```bash
git clone https://github.com/kadena-community/pact-zed
```

2. Open Zed
3. Go to `Settings` → `Extensions`
4. Click `Install Dev Extension` and select the `pact-zed` folder

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Open issues for bugs or feature requests
- Submit pull requests
- Share feedback
