/**
 * Dashboard component - main view showing repository health.
 */

import { useEffect, useState } from 'react';
import { GitBranch, RefreshCw, FolderGit2, AlertCircle, CheckCircle } from 'lucide-react';
import { healthCheck, HealthStatus } from '../../services/tauri';
import './Dashboard.css';

export function Dashboard() {
    const [health, setHealth] = useState<HealthStatus | null>(null);
    const [isLoading, setIsLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const fetchHealth = async () => {
        try {
            setIsLoading(true);
            setError(null);
            const status = await healthCheck();
            setHealth(status);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to connect to backend');
        } finally {
            setIsLoading(false);
        }
    };

    useEffect(() => {
        fetchHealth();
    }, []);

    return (
        <div className="dashboard">
            <div className="dashboard-header">
                <div className="welcome-section">
                    <h2>Welcome to DevBase</h2>
                    <p>Your local repository command center</p>
                </div>
                <button className="refresh-btn" onClick={fetchHealth} disabled={isLoading}>
                    <RefreshCw size={18} className={isLoading ? 'spinning' : ''} />
                    Refresh
                </button>
            </div>

            <div className="stats-grid">
                <div className="stat-card">
                    <div className="stat-icon">
                        <FolderGit2 size={24} />
                    </div>
                    <div className="stat-content">
                        <span className="stat-value">0</span>
                        <span className="stat-label">Repositories</span>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon warning">
                        <AlertCircle size={24} />
                    </div>
                    <div className="stat-content">
                        <span className="stat-value">0</span>
                        <span className="stat-label">Uncommitted Changes</span>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon success">
                        <CheckCircle size={24} />
                    </div>
                    <div className="stat-content">
                        <span className="stat-value">0</span>
                        <span className="stat-label">Up to Date</span>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon">
                        <GitBranch size={24} />
                    </div>
                    <div className="stat-content">
                        <span className="stat-value">0</span>
                        <span className="stat-label">Active Branches</span>
                    </div>
                </div>
            </div>

            <div className="status-section">
                <h3>System Status</h3>
                {isLoading ? (
                    <div className="status-loading">Checking backend status...</div>
                ) : error ? (
                    <div className="status-error">
                        <AlertCircle size={20} />
                        <span>{error}</span>
                    </div>
                ) : health ? (
                    <div className="status-ok">
                        <CheckCircle size={20} />
                        <span>Backend connected (v{health.version})</span>
                        <span className="db-status">
                            Database: {health.database_connected ? 'Connected' : 'Disconnected'}
                        </span>
                    </div>
                ) : null}
            </div>

            <div className="empty-state">
                <FolderGit2 size={64} />
                <h3>No Repositories Yet</h3>
                <p>Add a scan path in Settings to start discovering your local repositories.</p>
                <button className="primary-btn">Configure Scan Paths</button>
            </div>
        </div>
    );
}
