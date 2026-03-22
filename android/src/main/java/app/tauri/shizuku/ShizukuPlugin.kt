package app.tauri.shizuku

import android.app.Activity
import android.content.Intent
import android.content.pm.PackageManager
import android.os.SystemClock
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.util.concurrent.TimeUnit
import kotlin.concurrent.thread
import rikka.shizuku.Shizuku

private const val DEFAULT_REQUEST_CODE = 9322
private const val DEFAULT_TIMEOUT_MS = 15_000L
private const val SHIZUKU_PACKAGE = "moe.shizuku.privileged.api"
private const val SHIZUKU_DOWNLOAD_URL = "https://shizuku.rikka.app/download/"

@InvokeArg
class PingArgs {
  var value: String? = null
}

@InvokeArg
class RequestPermissionArgs {
  var requestCode: Int? = null
}

@InvokeArg
class RunAdbCommandArgs {
  var command: String = ""
  var timeoutMs: Long? = null
}

@InvokeArg
class GetSystemPropertyArgs {
  var key: String = ""
}

@InvokeArg
class ListPackagesArgs {
  var includeSystem: Boolean? = null
  var limit: Int? = null
  var timeoutMs: Long? = null
}

data class CommandExecutionResult(
  val exitCode: Int,
  val stdout: String,
  val stderr: String,
  val durationMs: Long,
)

@TauriPlugin
class ShizukuPlugin(private val activity: Activity) : Plugin(activity) {
  @Command
  fun ping(invoke: Invoke) {
    val args = invoke.parseArgs(PingArgs::class.java)

    val ret = JSObject()
    ret.put("value", args.value ?: "pong")
    invoke.resolve(ret)
  }

  @Command
  fun getStatus(invoke: Invoke) {
    val serviceAvailable = runCatching { Shizuku.pingBinder() }.getOrDefault(false)
    val preV11 = runCatching { Shizuku.isPreV11() }.getOrDefault(false)
    val permissionGranted = serviceAvailable &&
      runCatching { Shizuku.checkSelfPermission() == PackageManager.PERMISSION_GRANTED }.getOrDefault(false)
    val shouldShowRequestRationale = serviceAvailable &&
      runCatching { Shizuku.shouldShowRequestPermissionRationale() }.getOrDefault(false)

    val message = when {
      !isShizukuInstalled() -> "Shizuku app is not installed."
      !serviceAvailable -> "Shizuku service is not running."
      preV11 -> "Shizuku version is below v11 and unsupported."
      permissionGranted -> "Shizuku is ready."
      shouldShowRequestRationale -> "Permission denied with rationale required."
      else -> "Permission not granted."
    }

    val ret = JSObject()
    ret.put("serviceAvailable", serviceAvailable)
    ret.put("preV11", preV11)
    ret.put("permissionGranted", permissionGranted)
    ret.put("shouldShowRequestRationale", shouldShowRequestRationale)
    ret.put("canRequestPermission", serviceAvailable && !preV11 && !permissionGranted && !shouldShowRequestRationale)
    if (serviceAvailable) {
      ret.put("serverUid", runCatching { Shizuku.getUid() }.getOrNull())
      ret.put("serverVersion", runCatching { Shizuku.getVersion() }.getOrNull())
    }
    ret.put("message", message)

    invoke.resolve(ret)
  }

  @Command
  fun requestPermission(invoke: Invoke) {
    val args = invoke.parseArgs(RequestPermissionArgs::class.java)
    val requestCode = args.requestCode ?: DEFAULT_REQUEST_CODE

    val ret = JSObject()

    if (!runCatching { Shizuku.pingBinder() }.getOrDefault(false)) {
      ret.put("requested", false)
      ret.put("granted", false)
      ret.put("message", "Shizuku service is not running.")
      invoke.resolve(ret)
      return
    }

    if (runCatching { Shizuku.isPreV11() }.getOrDefault(false)) {
      ret.put("requested", false)
      ret.put("granted", false)
      ret.put("message", "Shizuku version is below v11 and unsupported.")
      invoke.resolve(ret)
      return
    }

    val granted = runCatching { Shizuku.checkSelfPermission() == PackageManager.PERMISSION_GRANTED }.getOrDefault(false)
    if (granted) {
      ret.put("requested", false)
      ret.put("granted", true)
      ret.put("message", "Permission already granted.")
      invoke.resolve(ret)
      return
    }

    val shouldShowRationale = runCatching { Shizuku.shouldShowRequestPermissionRationale() }.getOrDefault(false)
    if (shouldShowRationale) {
      ret.put("requested", false)
      ret.put("granted", false)
      ret.put("message", "Permission denied previously. Explain to user before requesting again.")
      invoke.resolve(ret)
      return
    }

    runCatching { Shizuku.requestPermission(requestCode) }
      .onFailure {
        invoke.reject(it.message ?: "Failed to request Shizuku permission")
        return
      }

    ret.put("requested", true)
    ret.put("granted", false)
    ret.put("message", "Permission request sent. Call getStatus() after user action.")
    invoke.resolve(ret)
  }

  @Command
  fun openShizuku(invoke: Invoke) {
    val ret = JSObject()
    val launchIntent = activity.packageManager.getLaunchIntentForPackage(SHIZUKU_PACKAGE)

    val opened = if (launchIntent != null) {
      launchIntent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      activity.startActivity(launchIntent)
      true
    } else {
      val intent = Intent(Intent.ACTION_VIEW).apply {
        data = android.net.Uri.parse(SHIZUKU_DOWNLOAD_URL)
        addFlags(Intent.FLAG_ACTIVITY_NEW_TASK)
      }
      activity.startActivity(intent)
      true
    }

    ret.put("opened", opened)
    ret.put("message", if (launchIntent != null) "Shizuku app opened." else "Shizuku not installed. Opened download page.")
    invoke.resolve(ret)
  }

  @Command
  fun runAdbCommand(invoke: Invoke) {
    val args = invoke.parseArgs(RunAdbCommandArgs::class.java)
    val command = args.command.trim()

    if (command.isEmpty()) {
      invoke.reject("command cannot be empty")
      return
    }

    ensureShizukuReady(invoke) ?: return

    val timeoutMs = args.timeoutMs ?: DEFAULT_TIMEOUT_MS
    runCatching {
      executeProcess(arrayOf("sh", "-c", command), timeoutMs)
    }.onSuccess { result ->
      invoke.resolve(toResultObject(result))
    }.onFailure {
      invoke.reject(it.message ?: "Failed to execute adb command")
    }
  }

  @Command
  fun getSystemProperty(invoke: Invoke) {
    val args = invoke.parseArgs(GetSystemPropertyArgs::class.java)
    val key = args.key.trim()

    if (key.isEmpty()) {
      invoke.reject("key cannot be empty")
      return
    }

    ensureShizukuReady(invoke) ?: return

    runCatching {
      executeProcess(arrayOf("getprop", key), DEFAULT_TIMEOUT_MS)
    }.onSuccess { result ->
      val ret = JSObject()
      ret.put("key", key)
      ret.put("value", result.stdout.trim().ifEmpty { null })
      invoke.resolve(ret)
    }.onFailure {
      invoke.reject(it.message ?: "Failed to read system property")
    }
  }

  @Command
  fun listPackages(invoke: Invoke) {
    val args = invoke.parseArgs(ListPackagesArgs::class.java)
    ensureShizukuReady(invoke) ?: return

    val includeSystem = args.includeSystem ?: true
    val timeoutMs = args.timeoutMs ?: DEFAULT_TIMEOUT_MS
    val command = if (includeSystem) {
      "cmd package list packages"
    } else {
      "cmd package list packages -3"
    }

    runCatching {
      executeProcess(arrayOf("sh", "-c", command), timeoutMs)
    }.onSuccess { result ->
      if (result.exitCode != 0) {
        invoke.reject("Command failed: ${result.stderr}")
        return
      }

      val packages = result.stdout
        .lineSequence()
        .map { it.trim() }
        .filter { it.startsWith("package:") }
        .map { it.removePrefix("package:") }
        .filter { it.isNotBlank() }
        .let { seq ->
          val limit = args.limit
          if (limit != null && limit > 0) seq.take(limit).toList() else seq.toList()
        }

      val ret = JSObject()
      ret.put("packages", JSArray(packages))
      invoke.resolve(ret)
    }.onFailure {
      invoke.reject(it.message ?: "Failed to list installed packages")
    }
  }

  private fun ensureShizukuReady(invoke: Invoke): Boolean? {
    val alive = runCatching { Shizuku.pingBinder() }.getOrDefault(false)
    if (!alive) {
      invoke.reject("Shizuku service is not running")
      return null
    }

    val granted = runCatching { Shizuku.checkSelfPermission() == PackageManager.PERMISSION_GRANTED }.getOrDefault(false)
    if (!granted) {
      invoke.reject("Shizuku permission is not granted")
      return null
    }

    return true
  }

  private fun executeProcess(command: Array<String>, timeoutMs: Long): CommandExecutionResult {
    val started = SystemClock.elapsedRealtime()
    val process = Shizuku.newProcess(command, null, null)

    var stdoutText = ""
    var stderrText = ""

    val stdoutReader = thread(start = true) {
      stdoutText = process.inputStream.bufferedReader().use { it.readText() }
    }

    val stderrReader = thread(start = true) {
      stderrText = process.errorStream.bufferedReader().use { it.readText() }
    }

    // ShizukuRemoteProcess may throw IllegalThreadStateException on waitFor(timeout).
    // Use a dedicated waiter thread to avoid calling exitValue on a running process.
    val waiter = thread(start = true) {
      runCatching { process.waitFor() }
    }

    waiter.join(timeoutMs)
    val finished = !waiter.isAlive
    if (!finished) {
      process.destroyForcibly()
      waiter.join(1_000)
    }

    stdoutReader.join(1_000)
    stderrReader.join(1_000)

    val exitCode = runCatching { process.exitValue() }.getOrDefault(-1)
    val duration = SystemClock.elapsedRealtime() - started

    return CommandExecutionResult(
      exitCode = exitCode,
      stdout = stdoutText,
      stderr = stderrText,
      durationMs = duration,
    )
  }

  private fun toResultObject(result: CommandExecutionResult): JSObject {
    val ret = JSObject()
    ret.put("exitCode", result.exitCode)
    ret.put("stdout", result.stdout)
    ret.put("stderr", result.stderr)
    ret.put("durationMs", result.durationMs)
    return ret
  }

  @Suppress("DEPRECATION")
  private fun isShizukuInstalled(): Boolean {
    return runCatching {
      activity.packageManager.getPackageInfo(SHIZUKU_PACKAGE, 0)
      true
    }.getOrDefault(false)
  }
}
