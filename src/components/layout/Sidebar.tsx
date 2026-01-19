/**
 * Sidebar navigation component.
 */

import {
    LayoutDashboard,
    Search,
    BarChart3,
    Trash2,
    Settings,
    ChevronLeft,
    ChevronRight,
    GitBranch
} from 'lucide-react';
import { useAppStore, View } from '../../stores/appStore';
import './Sidebar.css';

interface NavItem {
    id: View;
    label: string;
    icon: React.ReactNode;
}

const navItems: NavItem[] = [
    { id: 'dashboard', label: 'Dashboard', icon: <LayoutDashboard size={20} /> },
    { id: 'search', label: 'Global Search', icon: <Search size={20} /> },
    { id: 'analytics', label: 'Analytics', icon: <BarChart3 size={20} /> },
    { id: 'hygiene', label: 'Disk Hygiene', icon: <Trash2 size={20} /> },
    { id: 'settings', label: 'Settings', icon: <Settings size={20} /> },
];

export function Sidebar() {
    const { currentView, setCurrentView, sidebarCollapsed, toggleSidebar } = useAppStore();

    return (
        <aside className={`sidebar ${sidebarCollapsed ? 'collapsed' : ''}`}>
            <div className="sidebar-header">
                <div className="logo">
                    <GitBranch size={28} />
                    {!sidebarCollapsed && <span className="logo-text">DevBase</span>}
                </div>
                <button className="collapse-btn" onClick={toggleSidebar} title="Toggle sidebar">
                    {sidebarCollapsed ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
                </button>
            </div>

            <nav className="sidebar-nav">
                {navItems.map((item) => (
                    <button
                        key={item.id}
                        className={`nav-item ${currentView === item.id ? 'active' : ''}`}
                        onClick={() => setCurrentView(item.id)}
                        title={item.label}
                    >
                        <span className="nav-icon">{item.icon}</span>
                        {!sidebarCollapsed && <span className="nav-label">{item.label}</span>}
                    </button>
                ))}
            </nav>

            <div className="sidebar-footer">
                {!sidebarCollapsed && (
                    <div className="version-info">
                        <span>v{useAppStore.getState().version}</span>
                    </div>
                )}
            </div>
        </aside>
    );
}
