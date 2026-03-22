import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{value?: string}>('plugin:shizuku|ping', {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export interface ShizukuStatus {
  serviceAvailable: boolean
  preV11: boolean
  permissionGranted: boolean
  shouldShowRequestRationale: boolean
  canRequestPermission: boolean
  serverUid?: number
  serverVersion?: number
  message?: string
}

export interface RequestPermissionRequest {
  requestCode?: number
}

export interface RequestPermissionResponse {
  requested: boolean
  granted: boolean
  message?: string
}

export interface OpenShizukuResponse {
  opened: boolean
  message?: string
}

export interface RunAdbCommandRequest {
  command: string
  timeoutMs?: number
}

export interface RunAdbCommandResponse {
  exitCode: number
  stdout: string
  stderr: string
  durationMs: number
}

export interface GetSystemPropertyResponse {
  key: string
  value?: string
}

export interface ListPackagesRequest {
  includeSystem?: boolean
  limit?: number
}

export interface ListPackagesResponse {
  packages: string[]
}

export async function getStatus(): Promise<ShizukuStatus> {
  return await invoke<ShizukuStatus>('plugin:shizuku|get_status')
}

export async function requestPermission(
  payload?: RequestPermissionRequest
): Promise<RequestPermissionResponse> {
  return await invoke<RequestPermissionResponse>('plugin:shizuku|request_permission', {
    payload,
  })
}

export async function openShizuku(): Promise<OpenShizukuResponse> {
  return await invoke<OpenShizukuResponse>('plugin:shizuku|open_shizuku')
}

export async function runAdbCommand(
  payload: RunAdbCommandRequest
): Promise<RunAdbCommandResponse> {
  return await invoke<RunAdbCommandResponse>('plugin:shizuku|run_adb_command', {
    payload,
  })
}

export async function getSystemProperty(key: string): Promise<GetSystemPropertyResponse> {
  return await invoke<GetSystemPropertyResponse>('plugin:shizuku|get_system_property', {
    payload: { key },
  })
}

export async function listPackages(payload?: ListPackagesRequest): Promise<ListPackagesResponse> {
  return await invoke<ListPackagesResponse>('plugin:shizuku|list_packages', {
    payload,
  })
}
