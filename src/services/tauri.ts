/**
 * Tauri IPC service for type-safe command invocation.
 */

import { invoke } from '@tauri-apps/api/core';

/** Health status response from backend */
export interface HealthStatus {
    status: string;
    version: string;
    database_connected: boolean;
}

/** IPC error response structure */
export interface IpcError {
    code: string;
    message: string;
}

/**
 * Check application health status.
 */
export async function healthCheck(): Promise<HealthStatus> {
    return invoke<HealthStatus>('health_check');
}

/**
 * Get application version.
 */
export async function getVersion(): Promise<string> {
    return invoke<string>('get_version');
}
