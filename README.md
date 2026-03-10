# Budget Stats GUI

A Tauri-based desktop application for analyzing and visualizing financial data from WealthSimple. Built with Vue 3, TypeScript, and Rust, this application provides a powerful platform for running custom analysis scripts on financial data with support for both built-in and user-imported scripts (Python, R, etc.).

## Project Overview

Budget Stats GUI is a personal financial analysis tool that allows you to:
- Import and manage financial data from WealthSimple
- Run built-in analysis scripts for statistical insights
- Import and execute custom scripts (Python, R) to analyze your data
- Maintain a local SQLite database with read/write pool separation for performance and safety

### Technology Stack
- **Frontend**: Vue 3 with TypeScript and Vite
- **Desktop Framework**: Tauri 2.x
- **Backend**: Rust with async/await support (Tokio runtime)
- **Database**: SQLite with WAL mode and connection pooling via SQLx
- **Scripting Support**: Python 3, RScript

## Architecture

### High-Level Flow

```
Vue.js Frontend (TypeScript)
    ↓ (Tauri Commands)
Rust Backend (Controlled Functions)
    ↓ (Async/Await with Tokio)
Database Layer (SQLx with Pooled Connections)
    ↓
SQLite Database (Read/Write Pools)
```

### Component Structure

#### Frontend (`src/`)
- **App.vue**: Root component with router and menu bar
- **components/**: Reusable Vue components
  - `menuBar.vue`: Navigation menu
  - `scriptAddBar.vue`: UI for adding imported scripts
- **views/**: Page components organized by feature
  - `Landing Pages/`: Initial user interface and data views
  - `Work Pages/`: Feature-specific pages (WealthSimple, Settings)
- **controller/**: Business logic and data transformation
- **router/**: Vue Router configuration for SPA navigation

#### Backend (`src-tauri/src/`)
- **controlled_functions/**: Tauri commands exposed to the frontend
  - `wealthsimple_data_view_scripts.rs`: Script execution and management
  - `carousel_alerts_data_landing_page.rs`: Dashboard alerts
  - `csv_data_import_apple_script.rs`: Data import automation
- **db_modules/**: Database operations and pooling
  - `db_pool_opener.rs`: SQLx pool initialization and management
  - `import_script_to_db.rs`: Importing scripts to the database
  - `delete_script_from_db.rs`: Removing scripts from the database
- **independent_functions/**: Utility functions and helpers

## Data Flow Architecture

### 1. Frontend to Backend Communication

All communication between Vue.js frontend and Rust backend happens through **Tauri Commands**:

```typescript
// Frontend (Vue/TypeScript)
const result = await invoke('command_name', { param1: value1 });
```

```rust
// Backend (Rust)
#[tauri::command]
pub async fn command_name(param: Type) -> Result<ReturnType, String> {
    // Implementation
}
```

Commands are registered in `lib.rs`:
```rust
.invoke_handler(tauri::generate_handler![
    wealthsimple_data_get_imported_scripts,
    wealthsimple_data_run_imported_script,
    import_script_to_db,
    // ...
])
```

### 2. Database Connection Management

The application uses **SQLx connection pooling** with separate read and write pools to handle concurrent requests safely:

#### Connection Pool Setup (app startup in `lib.rs`):

1. **Pool Initialization**: On app startup, `db_pool_opener::open_pools()` creates two pools:
   - **Read Pool**: 5 concurrent connections in read-only mode
   - **Write Pool**: 1 connection in read-write mode (SQLite limitation)

2. **Pool Sharing**: Pools are wrapped in `Arc` (atomic reference counting) for thread-safe sharing:
   ```rust
   pub struct ReadPool(Arc<SqlitePool>);
   pub struct WritePool(Arc<SqlitePool>);
   ```

3. **App State Management**: Pools are registered with Tauri's app state:
   ```rust
   app.manage(read_pool);
   app.manage(write_pool);
   ```

#### Connection Pool Usage:

Commands access pools through Tauri's `State` dependency injection:

```rust
#[tauri::command]
pub async fn get_scripts(pool: tauri::State<'_, ReadPool>) -> Result<Vec<Script>, String> {
    let rows = sqlx::query("SELECT * FROM imported_scripts")
        .fetch_all(pool.as_ref())  // Get connection from read pool
        .await?;
    Ok(scripts)
}
```

#### Connection Lifecycle:

- **Automatic Checkout**: When executing a query, SQLx automatically obtains a connection from the pool
- **Automatic Return**: Once the query completes, the connection is automatically returned to the pool
- **Timeout/Wait**: If all connections are in use, new requests wait for an available connection
- **No Manual Cleanup**: You never manually return connections; the pool handles this automatically

### 3. Database State Management

The database stores:
- **imported_scripts table**: Metadata about user-imported scripts
  - path: Full path to the script file
  - name: User-friendly name
  - description: Script purpose
  - date_added: When the script was imported
  - extension: File extension (py, R, etc.)

## How Data Flows for Imported Scripts

### Overview
Imported scripts are user-provided Python or R scripts that can analyze data from the database. They receive structured JSON payloads containing context about the database and validation information.

### Step-by-Step Flow

#### 1. User Imports a Script (Frontend → Backend)

**Frontend** (`scriptAddBar.vue`):
```typescript
// User selects a script file
const payload = {
    name: "My Analysis Script",
    path: "/Users/owen/script.py",
    description: "Analyzes balance trends",
    extension: "py"
};

// Send to backend
await invoke('import_script_to_db', payload);
```

#### 2. Script Stored in Database (Backend)

**Backend** (`import_script_to_db.rs`):
```rust
#[tauri::command]
pub async fn import_script_to_db(
    app: AppHandle,
    pool: tauri::State<'_, WritePool>,  // Write pool for INSERT
    payload: ImportPayload
) -> Result<(), String> {
    // 1. Copy script to app data directory for safety
    let dest_path = app.path().app_data_dir()?.join("imported_scripts");
    fs::copy(&payload.path, &dest_path)?;
    
    // 2. Store metadata in database
    sqlx::query("INSERT INTO imported_scripts ...")
        .bind(dest_path)
        .bind(&payload.name)
        .execute(pool.as_ref())  // Use write pool
        .await?;
}
```

#### 3. User Runs an Imported Script (Frontend calls Backend)

**Frontend** calls:
```typescript
const result = await invoke('wealthsimple_data_run_imported_script', {
    handler: "/path/to/script.py"
});
```

#### 4. Payload is Created and Script is Executed (Backend)

**Backend** (`wealthsimple_data_view_scripts.rs`):

```rust
#[tauri::command]
pub async fn wealthsimple_data_run_imported_script(
    pool: tauri::State<'_, ReadPool>,  // Read pool for SELECT
    handler: String,
    db_path_state: tauri::State<'_, std::path::PathBuf>
) -> Result<String, String> {
    // 1. Query database for script metadata
    let script_row = sqlx::query("SELECT name FROM imported_scripts WHERE path = ?")
        .bind(&handler)
        .fetch_one(pool.as_ref())  // Use read pool
        .await?;
    
    // 2. Build payload JSON with context
    let payload = serde_json::json!({
        "name": script_row.get::<String, _>("name"),
        "db_parent_path": db_parent_path,
        "caller": "wealthsimple_data_view",
        "expected_table_name": "wealthsimple",
        "time": Utc::now().to_rfc3339()  // ISO 8601 format
    });
    
    // 3. Hash the payload for verification
    let hash = sha256_hash(&payload);
    payload["hash"] = hash;
    
    // 4. Spawn external process to run script
    let child = Command::new("python3")
        .arg(&handler)
        .arg(payload.to_string())  // Pass as JSON string argument
        .spawn()?;
    
    // 5. Wait for script completion and capture output
    let output = child.wait_with_output().await?;
    Ok(String::from_utf8(output.stdout)?)
}
```

#### 5. Script Receives Data and Processes It

**External Script** (`script.py` or `script.R`):
```python
import sys
import json
from pathlib import Path

# Argument 1: JSON payload with context
payload_json = json.loads(sys.argv[1])

name = payload_json["name"]
db_path = Path(payload_json["db_parent_path"]) / "db" / "budget-stats-gui.db"
hash_value = payload_json["hash"]

# Connect to database and analyze data
# sqlite3.connect(db_path)
# ... run analysis ...
# print(results_as_json)
```

### Payload Structure (JSON passed to scripts)

Every imported script receives this JSON payload:

```json
{
    "name": "Script Name",
    "db_parent_path": "/Users/owenhoekstra/Library/Application Support/ca.owenhoekstra.budget-stats-gui",
    "caller": "wealthsimple_data_view",
    "expected_db_path_extension": "db/budget-stats-gui.db",
    "expected_table_name": "wealthsimple",
    "time": "2026-03-10T14:30:00.123456Z",
    "hash": "sha256_hash_of_payload_for_verification"
}
```

### SQL Injection Prevention

SQLx uses **parameterized queries** (prepared statements) to prevent SQL injection:

```rust
// SAFE: Parameter is bound separately, not interpolated into SQL
sqlx::query("SELECT * FROM imported_scripts WHERE path = ?")
    .bind(&user_provided_path)  // Bound parameter
    .fetch_all(pool.as_ref())
    .await?

// NOT SAFE (don't do this):
// sqlx::query(&format!("SELECT * FROM imported_scripts WHERE path = '{}'", user_path))
```

Parameters are bound left-to-right matching `?` placeholders in the query string.

## Key Features

### Read/Write Pool Separation
- **5 Read Connections**: Allows concurrent queries without blocking
- **1 Write Connection**: SQLite limitation (W-A-L mode mitigation)
- **Automatic Connection Management**: Pooling handled by SQLx
- **Async-First**: All database operations are async with Tokio runtime

### Built-In Scripts
Currently includes:
- **General Statistics**: Computes mean, standard deviation, median, and month-over-month balance changes

### User-Imported Scripts
- Support for Python 3 and R scripts
- Scripts receive structured JSON context
- SHA256 payload verification for integrity
- Script files copied to app data directory for persistence
- Full database access for analysis

### Security & Safety
- Read-only connection pool for queries
- Separate write pool with limited concurrency
- SQLx parameterized queries prevent SQL injection
- WAL (Write-Ahead Logging) mode for better concurrency
- Scripts run in isolated processes with context validation

## Database

### Location
- **macOS**: `/Users/<username>/Library/Application Support/ca.owenhoekstra.budget-stats-gui/db/budget-stats-gui.db`

### Schema
```sql
CREATE TABLE imported_scripts (
    id TEXT PRIMARY KEY,
    path TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    date_added TEXT,
    extension TEXT
);
```

### WAL Mode
The database uses Write-Ahead Logging (WAL) for improved concurrency:
- Readers don't block writers
- Multiple readers can run concurrently
- Single writer operates independently

## Development Status

⚠️ **In Active Development** - This project is a personal tool in active development and is not production-ready. Features and APIs may change.

### Planned Features
- Additional built-in analysis scripts
- Scheduled script execution
- Data visualization and charting
- Export functionality
- Advanced filtering and querying UI

## Building and Running

### Prerequisites
- Rust 1.77.2+
- Node.js with pnpm
- Python 3 (for script support)

### Development
```bash
pnpm install
cargo tauri dev
```

### Build
```bash
pnpm build
cargo tauri build
```

