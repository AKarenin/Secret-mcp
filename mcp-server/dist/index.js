#!/usr/bin/env node
import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { CallToolRequestSchema, ListToolsRequestSchema, } from "@modelcontextprotocol/sdk/types.js";
import Database from "better-sqlite3";
import * as fs from "fs";
import * as path from "path";
import * as os from "os";
function getDbPath() {
    const platform = os.platform();
    let dataDir;
    if (platform === "darwin") {
        dataDir = path.join(os.homedir(), "Library", "Application Support");
    }
    else if (platform === "win32") {
        dataDir = process.env.APPDATA || path.join(os.homedir(), "AppData", "Roaming");
    }
    else {
        dataDir = process.env.XDG_DATA_HOME || path.join(os.homedir(), ".local", "share");
    }
    return path.join(dataDir, "secret-mcp", "secrets.db");
}
function openDatabase(dbPath) {
    const db = new Database(dbPath, { readonly: false });
    return db;
}
function searchSecrets(db, query) {
    const pattern = `%${query.toLowerCase()}%`;
    const stmt = db.prepare(`
    SELECT name, description FROM secrets
    WHERE LOWER(name) LIKE ? OR LOWER(COALESCE(description, '')) LIKE ?
    ORDER BY name
  `);
    return stmt.all(pattern, pattern);
}
function writeEnvFile(db, keys, envPath) {
    // Validate path is absolute
    if (!path.isAbsolute(envPath)) {
        throw new Error("Path must be absolute");
    }
    // Get values for requested keys
    const stmt = db.prepare("SELECT name, value FROM secrets WHERE name = ?");
    const found = new Map();
    const missing = [];
    for (const key of keys) {
        const row = stmt.get(key);
        if (row) {
            found.set(row.name, row.value);
        }
        else {
            missing.push(key);
        }
    }
    // Build .env content
    let content = "";
    for (const key of keys) {
        const value = found.get(key);
        if (value !== undefined) {
            // Escape value if needed
            if (value.includes(" ") || value.includes('"') || value.includes("'") || value.includes("\n")) {
                content += `${key}="${value.replace(/"/g, '\\"')}"\n`;
            }
            else {
                content += `${key}=${value}\n`;
            }
        }
    }
    // Ensure directory exists
    const dir = path.dirname(envPath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    // Write file with restricted permissions
    fs.writeFileSync(envPath, content, { mode: 0o600 });
    return {
        success: true,
        written: found.size,
        missing,
    };
}
async function main() {
    const server = new Server({
        name: "secret-mcp-server",
        version: "0.1.0",
    }, {
        capabilities: {
            tools: {},
        },
    });
    // Get database path
    const dbPath = getDbPath();
    if (!fs.existsSync(dbPath)) {
        console.error("Error: Database not found at", dbPath);
        console.error("Please run the Secret MCP app first to create the database.");
        process.exit(1);
    }
    let db;
    try {
        db = openDatabase(dbPath);
    }
    catch (error) {
        console.error(`Error opening database: ${error}`);
        process.exit(1);
    }
    // List available tools
    server.setRequestHandler(ListToolsRequestSchema, async () => {
        return {
            tools: [
                {
                    name: "search_secrets",
                    description: "Search for secrets by name or description. Returns names and descriptions only, never values. Use this to find secrets before writing them to a .env file.",
                    inputSchema: {
                        type: "object",
                        properties: {
                            query: {
                                type: "string",
                                description: "Search query to match against secret names and descriptions",
                            },
                        },
                        required: ["query"],
                    },
                },
                {
                    name: "write_env",
                    description: "Write specified secrets to a .env file. Values are retrieved securely and never exposed to the AI. The file is created with restricted permissions (600).",
                    inputSchema: {
                        type: "object",
                        properties: {
                            keys: {
                                type: "array",
                                items: { type: "string" },
                                description: "Secret names to include in the .env file",
                            },
                            path: {
                                type: "string",
                                description: "Absolute path where the .env file should be written",
                            },
                        },
                        required: ["keys", "path"],
                    },
                },
            ],
        };
    });
    // Handle tool calls
    server.setRequestHandler(CallToolRequestSchema, async (request) => {
        const { name, arguments: args } = request.params;
        try {
            if (name === "search_secrets") {
                const query = args.query;
                const results = searchSecrets(db, query);
                return {
                    content: [
                        {
                            type: "text",
                            text: JSON.stringify(results, null, 2),
                        },
                    ],
                };
            }
            if (name === "write_env") {
                const { keys, path: envPath } = args;
                const result = writeEnvFile(db, keys, envPath);
                let message = `Successfully wrote ${result.written} secret(s) to ${envPath}`;
                if (result.missing.length > 0) {
                    message += `\nMissing secrets (not found): ${result.missing.join(", ")}`;
                }
                return {
                    content: [
                        {
                            type: "text",
                            text: message,
                        },
                    ],
                };
            }
            throw new Error(`Unknown tool: ${name}`);
        }
        catch (error) {
            return {
                content: [
                    {
                        type: "text",
                        text: `Error: ${error instanceof Error ? error.message : String(error)}`,
                    },
                ],
                isError: true,
            };
        }
    });
    // Start the server
    const transport = new StdioServerTransport();
    await server.connect(transport);
    // Handle cleanup
    process.on("SIGINT", () => {
        db.close();
        process.exit(0);
    });
    process.on("SIGTERM", () => {
        db.close();
        process.exit(0);
    });
}
main().catch((error) => {
    console.error("Fatal error:", error);
    process.exit(1);
});
