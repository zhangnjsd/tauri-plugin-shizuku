const COMMANDS: &[&str] = &[
  "ping",
  "get_status",
  "request_permission",
  "open_shizuku",
  "run_adb_command",
  "get_system_property",
  "list_packages",
];

fn main() {
  let result = tauri_plugin::Builder::new(COMMANDS)
    .android_path("android")
    .ios_path("ios")
    .try_build();

  if !(cfg!(docsrs) && std::env::var("TARGET").unwrap().contains("android")) {
    result.unwrap();
  }
}
