<script>
  import {
    getStatus,
    requestPermission,
    openShizuku,
    runAdbCommand,
    getSystemProperty,
    listPackages,
  } from 'tauri-plugin-shizuku-api'

  let command = $state('id')
  let systemPropertyKey = $state('ro.build.version.release')
  let packageLimit = $state(20)
  let includeSystem = $state(false)
  let status = $state(null)
  let logs = $state([])

  function addLog(value) {
    logs = [
      `[${new Date().toLocaleTimeString()}] ${typeof value === 'string' ? value : JSON.stringify(value)}`,
      ...logs,
    ].slice(0, 60)
  }

  async function refreshStatus() {
    try {
      status = await getStatus()
      addLog(status)
    } catch (e) {
      addLog(`getStatus error: ${e}`)
    }
  }

  async function doRequestPermission() {
    try {
      const ret = await requestPermission()
      addLog(ret)
      await refreshStatus()
    } catch (e) {
      addLog(`requestPermission error: ${e}`)
    }
  }

  async function doOpenShizuku() {
    try {
      const ret = await openShizuku()
      addLog(ret)
    } catch (e) {
      addLog(`openShizuku error: ${e}`)
    }
  }

  async function doRunCommand() {
    try {
      const ret = await runAdbCommand({
        command,
        timeoutMs: 15000,
      })
      addLog(ret)
    } catch (e) {
      addLog(`runAdbCommand error: ${e}`)
    }
  }

  async function doGetProp() {
    try {
      const ret = await getSystemProperty(systemPropertyKey)
      addLog(ret)
    } catch (e) {
      addLog(`getSystemProperty error: ${e}`)
    }
  }

  async function doListPackages() {
    try {
      const ret = await listPackages({
        includeSystem,
        limit: Number(packageLimit),
      })
      addLog({ packageCount: ret.packages.length, packages: ret.packages })
    } catch (e) {
      addLog(`listPackages error: ${e}`)
    }
  }
</script>

<main class="container">
  <h1>Tauri Shizuku Plugin Test Console</h1>
  <p>Android only. Steps: open Shizuku -> start service -> request permission -> run command.</p>

  <section class="card">
    <h2>Shizuku Status</h2>
    <div class="actions">
      <button onclick={refreshStatus}>Refresh Status</button>
      <button onclick={doOpenShizuku}>Open Shizuku</button>
      <button onclick={doRequestPermission}>Request Permission</button>
    </div>
    <pre>{status ? JSON.stringify(status, null, 2) : 'No status yet.'}</pre>
  </section>

  <section class="card">
    <h2>ADB Command</h2>
    <input bind:value={command} placeholder="command, e.g. id" />
    <button onclick={doRunCommand}>Run Command</button>
  </section>

  <section class="card">
    <h2>System Property</h2>
    <input bind:value={systemPropertyKey} placeholder="ro.build.version.release" />
    <button onclick={doGetProp}>Get Property</button>
  </section>

  <section class="card">
    <h2>Installed Packages</h2>
    <label>
      <input type="checkbox" bind:checked={includeSystem} /> Include system apps
    </label>
    <input type="number" min="1" max="500" bind:value={packageLimit} />
    <button onclick={doListPackages}>List Packages</button>
  </section>

  <section class="card">
    <h2>Logs</h2>
    <div class="logs">
      {#each logs as log}
        <div>{log}</div>
      {/each}
    </div>
  </section>
</main>
