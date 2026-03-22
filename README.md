# Tauri Plugin Shizuku

Android Tauri v2 plugin for communicating with Shizuku.

Current capabilities:

- Shizuku service status detection
- Shizuku permission request trigger
- Open Shizuku app (or download page when not installed)
- Execute shell/adb style commands through Shizuku
- Query Android system properties
- List installed packages

## Install

Rust side:

```toml
[dependencies]
tauri-plugin-shizuku = { path = "../tauri-plugin-shizuku" }
```

JS side:

```bash
bun tauri add shizuku
```

Register plugin:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_shizuku::init())
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
```

## Rust API (Out-of-the-box)

After plugin registration, call directly from Rust:

```rust
use tauri_plugin_shizuku::ShizukuExt;

let status = app.shizuku().get_status()?;
let _ = app.shizuku().request_permission(Default::default())?;
let cmd = app.shizuku().run_adb_command(tauri_plugin_shizuku::RunAdbCommandRequest {
	command: "id".to_string(),
	timeout_ms: Some(15_000),
})?;
```

No extra manifest edits are required by plugin users. The Android provider and native code are shipped inside this plugin.

## JavaScript API

```ts
import {
	getStatus,
	requestPermission,
	openShizuku,
	runAdbCommand,
	getSystemProperty,
	listPackages,
} from 'tauri-plugin-shizuku-api'

const status = await getStatus()
if (!status.permissionGranted) {
	await openShizuku()
	await requestPermission()
}

const commandResult = await runAdbCommand({ command: 'id', timeoutMs: 15000 })
const release = await getSystemProperty('ro.build.version.release')
const packages = await listPackages({ includeSystem: false, limit: 30 })
```

## Android Behavior Notes

- This plugin depends on `dev.rikka.shizuku:api` and `dev.rikka.shizuku:provider`.
- End users still need to install and start Shizuku (or Sui) because this is required by Shizuku's runtime model.
- `runAdbCommand` currently uses `Shizuku.newProcess` for command execution.

## Example App

An interactive test console is provided at `examples/tauri-app`.

Run steps:

```bash
cd examples/tauri-app
bun tauri android dev
```

In app:

1. Tap `Open Shizuku`
2. Start Shizuku service in Shizuku app
3. Tap `Request Permission`
4. Tap `Refresh Status`
5. Run command/property/package tests
