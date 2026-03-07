# so-ref — MCP Reference Library Server

## Overview

An MCP server that manages a local library of cloned git repositories for use as reference material. Eliminates the manual friction of cloning repos, telling agents where they are, and keeping them up to date.

## Problem

When working with AI agents, a common workflow is:
1. Find an open source project to use as a reference
2. Clone it locally
3. Tell the agent where it lives
4. Have the agent explore it

This is repetitive. The agent should be able to manage references itself.

## Solution

A minimal MCP server (stdio transport) that exposes three tools for managing a local reference library. The agent calls these tools directly — no manual cloning or path management needed.

Once a reference is added, the agent retrieves its local path via `get_reference` and then uses its own file/search tools to explore the codebase.

## Tools

### `add_reference`

Adds a new repo to the local reference library.

- **Input:** Full git repository URL (e.g. `https://github.com/drizzle-team/drizzle-orm`)
- **Behaviour:**
  - Clones the repo into the managed data directory
  - Derives a short name from the repo URL (e.g. `drizzle-orm`)
  - Stores metadata in the manifest (URL, short name, local path, timestamp)
- **Output:** Confirmation with the assigned short name
- **Errors:** Clone failure, repo already exists

### `get_reference`

Retrieves a reference by short name, ensuring it's fresh.

- **Input:** Short name (e.g. `drizzle-orm`)
- **Behaviour:**
  - Looks up the repo in the manifest
  - If last fetch was >5 days ago: runs `git fetch origin && git reset --hard origin/HEAD`
  - Returns the absolute local path to the repo
- **Output:** Absolute path to the repo on disk
- **Errors:** Reference not found, fetch/reset failure

### `list_references`

Lists all references in the library.

- **Input:** None
- **Output:** List of references with short name, URL, and last fetched timestamp

## Architecture

- **Language:** Rust
- **Transport:** MCP over stdio (JSON-RPC)
- **Lifecycle:** Long-lived process, spawned by the MCP client. Idle between tool calls. Manifest loaded into memory at startup, flushed to disk on writes.

## State & Storage

- **Data directory:** `~/.local/share/so-ref/` (or equivalent XDG path)
  - `manifest.json` — reference metadata
  - `repos/` — cloned repositories
- **Manifest schema:**
  ```json
  {
    "references": [
      {
        "name": "drizzle-orm",
        "url": "https://github.com/drizzle-team/drizzle-orm",
        "path": "/home/user/.local/share/so-ref/repos/drizzle-orm",
        "last_fetched": "2026-03-07T12:00:00Z"
      }
    ]
  }
  ```
- State lives with the server, not tied to any specific MCP client.

## Design Decisions

| Decision | Choice | Rationale |
|---|---|---|
| Name resolution | Full URL required on add, short name for retrieval | Keep v0.1 simple, smart resolution can come later |
| Staleness threshold | 5 days | Conservative default, revisit based on real usage |
| Freshness strategy | `git fetch && git reset --hard origin/HEAD` | Repos are read-only references, avoids merge conflicts |
| Name collisions | Not handled | Punt to future version |
| Config/state location | Server-owned data directory | MCP is portable across clients, state shouldn't live inside any one client's config |
| Transport | stdio | Simplest, widest client support |

## Future Considerations (not in v0.1)

- Smart name resolution (GitHub API search from partial name)
- Configurable staleness threshold
- Name collision handling (org/repo keys, alias support)
- `remove_reference` tool
- Shallow clones for large repos

## Version

v0.1.0
