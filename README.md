# Secret MCP

A desktop app for managing secrets with an MCP server that lets AI coding assistants write `.env` files without ever seeing the secret values.

## Why?

Generating .env file is a pain, especially because there is no vibing out of it. But its an even stronger pain to leak secrets to AI coding assistants run on the cloud. 

## Features

- **Desktop App**: Simple window for managing secrets (name, description, value)
- **MCP Server**: Two tools for AI assistants:
  - `search_secrets`: Find secrets by name/description (never exposes values)
  - `write_env`: Write secrets to `.env` files (values go straight to file, never to AI)
- **Local Storage**: All secrets stored locally in SQLite
- **npm Package**: Just `npx secret-mcp` - no build required

## Installation

### Desktop App

Download from [Releases](https://github.com/AKarenin/Secret-mcp/releases) or build from source:

```bash
npm install
npm run tauri build
```

### MCP Server Setup

Add to your MCP client config:

```json
"secret-mcp": {
  "command": "npx",
  "args": ["secret-mcp"]
}
```


## Usage

1. Open Secret MCP app
2. Add your secrets (API keys, tokens, etc.)
3. When coding with AI, it will automatically use `search_secrets` and `write_env` to set up your `.env` files
(Note: the name of the secret is the variable name in the `.env` file)

## MCP Tools

### search_secrets

Search for secrets by name or description. Returns names and descriptions only - **values are never exposed**.

```typescript
// Input
{ query: "openai" }

// Output
[
  { name: "OPENAI_API_KEY", description: "OpenAI API key" }
]
```

### write_env

Write secrets to a `.env` file. Values go directly from your local database to the file - **never passed through the AI**.

```typescript
// Input
{
  keys: ["OPENAI_API_KEY", "DATABASE_URL"],
  path: "/Users/you/project/.env"
}

// Output
"Successfully wrote 2 secret(s) to /Users/you/project/.env"
```

## Data Storage

Secrets are stored locally:
- **macOS**: `~/Library/Application Support/secret-mcp/secrets.db`
- **Linux**: `~/.local/share/secret-mcp/secrets.db`
- **Windows**: `%APPDATA%/secret-mcp/secrets.db`

## Security

- Secret values never leave your machine (except to `.env` files you specify)
- MCP server only returns secret names and descriptions to the AI
- `.env` files written with `600` permissions (owner read/write only)

## Tech Stack

- **Desktop**: Tauri 2.0 + Svelte 5 + TypeScript
- **MCP Server**: Node.js + @modelcontextprotocol/sdk + better-sqlite3

## License

MIT
