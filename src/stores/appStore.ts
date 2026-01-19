/**
 * Application store using Zustand.
 */

import { create } from 'zustand';

/** Navigation views in the app */
export type View = 'dashboard' | 'search' | 'analytics' | 'hygiene' | 'settings';

/** Application state interface */
interface AppState {
    /** Current active view */
    currentView: View;
    /** Application version */
    version: string;
    /** Sidebar collapsed state */
    sidebarCollapsed: boolean;
    /** Loading state */
    isLoading: boolean;

    // Actions
    setCurrentView: (view: View) => void;
    setVersion: (version: string) => void;
    toggleSidebar: () => void;
    setLoading: (loading: boolean) => void;
}

/** Global application store */
export const useAppStore = create<AppState>((set) => ({
    currentView: 'dashboard',
    version: '0.1.0',
    sidebarCollapsed: false,
    isLoading: true,

    setCurrentView: (view) => set({ currentView: view }),
    setVersion: (version) => set({ version }),
    toggleSidebar: () => set((state) => ({ sidebarCollapsed: !state.sidebarCollapsed })),
    setLoading: (isLoading) => set({ isLoading }),
}));
