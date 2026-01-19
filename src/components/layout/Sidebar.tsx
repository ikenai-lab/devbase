/**
 * Sidebar navigation component.
 */

import {
    LayoutDashboard,
    Search,
    PieChart,
    Trash2,
    ChevronLeft,
    ChevronRight,
    Code2
} from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import './Sidebar.css';

export function Sidebar() {
    const {
        currentView,
        setCurrentView,
        version,
        sidebarCollapsed,
        toggleSidebar
    } = useAppStore();

    const navItems = [
        { id: 'dashboard', label: 'Dashboard', icon: LayoutDashboard },
        { id: 'search', label: 'Search', icon: Search },
        { id: 'analytics', label: 'Analytics', icon: PieChart },
        { id: 'hygiene', label: 'Hygiene', icon: Trash2 },
    ];

    return (
        <aside className={`sidebar ${sidebarCollapsed ? 'collapsed' : ''}`}>
            <div className="sidebar-header">
                {!sidebarCollapsed && (
                    <div className="logo">
                        <Code2 size={24} />
                        <span className="logo-text">DevBase</span>
                    </div>
                )}
                <button
                    className="collapse-btn"
                    onClick={toggleSidebar}
                    aria-label="Toggle Sidebar"
                >
                    {sidebarCollapsed ? <ChevronRight size={20} /> : <ChevronLeft size={20} />}
                </button>
            </div>

            <nav className="sidebar-nav">
                {navItems.map((item) => (
                    <button
                        key={item.id}
                        className={`nav-item ${currentView === item.id ? 'active' : ''}`}
                        onClick={() => setCurrentView(item.id as any)}
                        title={sidebarCollapsed ? item.label : ''}
                    >
                        <span className="nav-icon">
                            <item.icon size={20} />
                        </span>
                        {!sidebarCollapsed && <span className="nav-label">{item.label}</span>}
                    </button>
                ))}
            </nav>

            <div className="sidebar-footer">
                {!sidebarCollapsed && (
                    <div className="version-info">v{version}</div>
                )}
            </div>
        </aside>
    );
}
