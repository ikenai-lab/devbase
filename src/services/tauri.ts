/**
 * Tauri API service - Type-safe wrappers for Tauri commands.
 */

import { invoke } from '@tauri-apps/api/core';

// ========== Types ==========

export interface HealthStatus {
    status: string;
    version: string;
    database_connected: boolean;
}

export interface DiscoveredRepo {
    path: string;
    name: string;
    remote_url: string | null;
    default_branch: string | null;
    current_branch: string | null;
}

export interface RepoHealth {
    is_dirty: boolean;
    uncommitted_count: number;
    staged_count: number;
    commits_ahead: number;
    commits_behind: number;
    stash_count: number;
    current_branch: string | null;
    is_detached: boolean;
}

export type RepoStatus = 'clean' | 'dirty' | 'ahead' | 'behind' | 'diverged';

export interface RepoInfo {
    id: number;
    path: string;
    name: string;
    remote_url: string | null;
    default_branch: string | null;
    current_branch: string | null;
    health: RepoHealth;
    status: RepoStatus;
    tags: string[];
}

export interface ScanPath {
    id: number;
    path: string;
    enabled: boolean;
    max_depth: number;
}

export interface Tag {
    id: number;
    name: string;
    color: string;
}

export interface IpcError {
    code: string;
    message: string;
}

// ========== Health Commands ==========

export async function healthCheck(): Promise<HealthStatus> {
    return invoke<HealthStatus>('health_check');
}

export async function getVersion(): Promise<string> {
    return invoke<string>('get_version');
}

// ========== Scan Commands ==========

export async function startScan(): Promise<DiscoveredRepo[]> {
    return invoke<DiscoveredRepo[]>('start_scan');
}

export async function scanPath(path: string, maxDepth?: number): Promise<DiscoveredRepo[]> {
    return invoke<DiscoveredRepo[]>('scan_path', { path, maxDepth });
}

// ========== Repository Commands ==========

export async function getRepositories(): Promise<RepoInfo[]> {
    return invoke<RepoInfo[]>('get_repositories');
}

export async function getRepoHealth(path: string): Promise<RepoHealth> {
    return invoke<RepoHealth>('get_repo_health', { path });
}

export async function refreshRepo(repoId: number): Promise<RepoInfo> {
    return invoke<RepoInfo>('refresh_repo', { repoId });
}

// ========== Settings Commands ==========

export async function getScanPaths(): Promise<ScanPath[]> {
    return invoke<ScanPath[]>('get_scan_paths');
}

export async function addScanPath(path: string, maxDepth?: number): Promise<ScanPath> {
    return invoke<ScanPath>('add_scan_path', { path, maxDepth });
}

export async function removeScanPath(id: number): Promise<void> {
    return invoke<void>('remove_scan_path', { id });
}

export async function updateScanPath(
    id: number,
    enabled?: boolean,
    maxDepth?: number
): Promise<void> {
    return invoke<void>('update_scan_path', { id, enabled, maxDepth });
}

export async function getSetting(key: string): Promise<string | null> {
    return invoke<string | null>('get_setting', { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
    return invoke<void>('set_setting', { key, value });
}

// ========== Tag Commands ==========

export async function getTags(): Promise<Tag[]> {
    return invoke<Tag[]>('get_tags');
}

export async function createTag(name: string, color?: string): Promise<Tag> {
    return invoke<Tag>('create_tag', { name, color });
}

export async function deleteTag(id: number): Promise<void> {
    return invoke<void>('delete_tag', { id });
}

export async function assignTag(repoId: number, tagId: number): Promise<void> {
    return invoke<void>('assign_tag', { repoId, tagId });
}

export async function removeTag(repoId: number, tagId: number): Promise<void> {
    return invoke<void>('remove_tag', { repoId, tagId });
}

// ========== History Commands ==========

export interface CommitLogEntry {
    oid: string;
    short_oid: string;
    message: string;
    author_name: string;
    author_email: string;
    date: number;
    parents: string[];
    refs: string[];
}

export async function getCommitLog(path: string, limit?: number): Promise<CommitLogEntry[]> {
    return invoke<CommitLogEntry[]>('get_commit_log', { path, limit });
}
