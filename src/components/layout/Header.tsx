/**
 * Header component for the main content area.
 */

import { Bell, User } from 'lucide-react';
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
    const { currentView } = useAppStore();

    return (
        <header className="main-header">
            <div className="header-left">
                <h1 className="page-title">{viewTitles[currentView]}</h1>
            </div>

            <div className="header-right">
                <button className="header-action" title="Notifications">
                    <Bell size={20} />
                    <span className="notification-badge">3</span>
                </button>
                <button className="header-action profile" title="Profile">
                    <User size={20} />
                </button>
            </div>
        </header>
    );
}
