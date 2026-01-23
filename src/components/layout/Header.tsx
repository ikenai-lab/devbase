/**
 * Header component for the main content area.
 */

import { Settings, Sun, Moon } from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import './Header.css';

const viewTitles: Record<string, string> = {
    dashboard: 'Repository Dashboard',
    search: 'Global Code Search',
    analytics: 'Local Analytics',
    hygiene: 'Disk Hygiene',
    settings: 'Settings',
};

export function Header() {
    const { currentView, setCurrentView, theme, toggleTheme } = useAppStore();

    return (
        <header className="main-header">
            <div className="header-left">
                <h1 className="page-title">{viewTitles[currentView]}</h1>
            </div>

            <div className="header-right">
                <button
                    className="header-action"
                    title={theme === 'light' ? 'Switch to Dark Mode' : 'Switch to Light Mode'}
                    onClick={toggleTheme}
                >
                    {theme === 'light' ? <Moon size={20} /> : <Sun size={20} />}
                </button>

                <button
                    className={`header-action ${currentView === 'settings' ? 'active' : ''}`}
                    title="Settings"
                    onClick={() => setCurrentView('settings')}
                >
                    <Settings size={20} />
                </button>

            </div>
        </header>
    );
}
