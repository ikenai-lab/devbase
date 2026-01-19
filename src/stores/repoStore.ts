/**
 * Repository store - Zustand store for repositories and scanning state.
 */

import { create } from 'zustand';
import { RepoInfo, ScanPath, Tag, CommitLogEntry, getRepositories, getScanPaths, getTags, startScan, getCommitLog } from '../services/tauri';

interface RepoState {
    // Data
    repositories: RepoInfo[];
    scanPaths: ScanPath[];
    tags: Tag[];

    // UI State
    isScanning: boolean;
    isLoading: boolean;
    lastScanTime: Date | null;
    error: string | null;

    // Filters
    searchQuery: string;
    selectedStatus: string | null;
    selectedTags: string[];

    // Actions
    fetchRepositories: () => Promise<void>;
    fetchScanPaths: () => Promise<void>;
    fetchTags: () => Promise<void>;
    runScan: () => Promise<void>;
    setSearchQuery: (query: string) => void;
    setSelectedStatus: (status: string | null) => void;
    toggleTag: (tag: string) => void;
    clearFilters: () => void;

    // Selection & History
    selectedRepoId: number | null;
    activeCommitLog: CommitLogEntry[];
    selectRepo: (id: number | null) => void;
    loadCommitLog: (repoId: number) => Promise<void>;
}

export const useRepoStore = create<RepoState>((set, get) => ({
    // Initial state
    repositories: [],
    scanPaths: [],
    tags: [],
    isScanning: false,
    isLoading: false,
    lastScanTime: null,
    error: null,
    searchQuery: '',
    selectedStatus: null,
    selectedTags: [],

    fetchRepositories: async () => {
        set({ isLoading: true, error: null });
        try {
            const repos = await getRepositories();
            set({ repositories: repos, isLoading: false });
        } catch (e) {
            set({ error: String(e), isLoading: false });
        }
    },

    fetchScanPaths: async () => {
        try {
            const paths = await getScanPaths();
            set({ scanPaths: paths });
        } catch (e) {
            set({ error: String(e) });
        }
    },

    fetchTags: async () => {
        try {
            const tags = await getTags();
            set({ tags });
        } catch (e) {
            set({ error: String(e) });
        }
    },

    runScan: async () => {
        set({ isScanning: true, error: null });
        try {
            await startScan();
            const repos = await getRepositories();
            set({
                repositories: repos,
                isScanning: false,
                lastScanTime: new Date()
            });
        } catch (e) {
            set({ error: String(e), isScanning: false });
        }
    },

    setSearchQuery: (query) => set({ searchQuery: query }),

    setSelectedStatus: (status) => set({ selectedStatus: status }),

    toggleTag: (tag) => {
        const current = get().selectedTags;
        if (current.includes(tag)) {
            set({ selectedTags: current.filter(t => t !== tag) });
        } else {
            set({ selectedTags: [...current, tag] });
        }
    },

    clearFilters: () => set({
        searchQuery: '',
        selectedStatus: null,
        selectedTags: []
    }),

    // Selection & History
    selectedRepoId: null,
    activeCommitLog: [],

    selectRepo: (id) => set({ selectedRepoId: id, activeCommitLog: [] }),

    loadCommitLog: async (repoId) => {
        const repo = get().repositories.find(r => r.id === repoId);
        if (!repo) return;

        set({ isLoading: true, error: null });
        try {
            const logs = await getCommitLog(repo.path, 100); // Limit 100 for now
            set({ activeCommitLog: logs, isLoading: false });
        } catch (e) {
            set({ error: String(e), isLoading: false });
        }
    },
}));

// Selector for filtered repositories
export const useFilteredRepos = () => {
    const { repositories, searchQuery, selectedStatus, selectedTags } = useRepoStore();

    return repositories.filter(repo => {
        // Search filter
        if (searchQuery) {
            const query = searchQuery.toLowerCase();
            const matchesName = repo.name.toLowerCase().includes(query);
            const matchesPath = repo.path.toLowerCase().includes(query);
            if (!matchesName && !matchesPath) return false;
        }

        // Status filter
        if (selectedStatus && repo.status !== selectedStatus) {
            return false;
        }

        // Tag filter
        if (selectedTags.length > 0) {
            const hasAllTags = selectedTags.every(tag => repo.tags.includes(tag));
            if (!hasAllTags) return false;
        }

        return true;
    });
};
