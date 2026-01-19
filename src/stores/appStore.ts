/**
 * Application store using Zustand.
 */

import { create } from 'zustand';

/** Navigation views in the app */
export type View = 'dashboard' | 'search' | 'analytics' | 'hygiene' | 'settings' | 'repo-detail';

/** Application state interface */
interface AppState {
    /** Current active view */
    currentView: View;
    /** Current theme */
    theme: 'light' | 'dark';
    /** Application version */
    version: string;
    /** Sidebar collapsed state */
    sidebarCollapsed: boolean;
    /** Loading state */
    isLoading: boolean;

    // Actions
    setCurrentView: (view: View) => void;
    setTheme: (theme: 'light' | 'dark') => void;
    toggleTheme: () => void;
    setVersion: (version: string) => void;
    toggleSidebar: () => void;
    setLoading: (loading: boolean) => void;
}

/** Global application store */
export const useAppStore = create<AppState>((set) => ({
    currentView: 'dashboard',
    theme: 'dark', // Default to Dark
    version: '0.1.0',
    sidebarCollapsed: false,
    isLoading: true,

    setCurrentView: (view) => set({ currentView: view }),
    setTheme: (theme) => set({ theme }),
    toggleTheme: () => set((state) => ({ theme: state.theme === 'light' ? 'dark' : 'light' })),
    setVersion: (version) => set({ version }),
    toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
    setLoading: (isLoading) => set({ isLoading }),
}));
