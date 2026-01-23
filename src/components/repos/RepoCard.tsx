import {
    GitBranch,
    FolderOpen,
    ExternalLink,
    RefreshCw
} from 'lucide-react';
import { RepoInfo } from '../../services/tauri';
import { refreshRepo } from '../../services/tauri';
import { useAppStore } from '../../stores/appStore';
import { useRepoStore } from '../../stores/repoStore';
import './RepoCard.css';

interface RepoCardProps {
    repo: RepoInfo;
}


export function RepoCard({ repo }: RepoCardProps) {
    const fetchRepositories = useRepoStore(state => state.fetchRepositories);
    const selectRepo = useRepoStore(state => state.selectRepo);
    const setCurrentView = useAppStore(state => state.setCurrentView);

    const handleRefresh = async (e: React.MouseEvent) => {
        e.stopPropagation();
        try {
            await refreshRepo(repo.id);
            await fetchRepositories();
        } catch (e) {
            console.error('Failed to refresh repo:', e);
        }
    };

    const handleCardClick = () => {
        selectRepo(repo.id);
        setCurrentView('repo-detail');
    };

    const handleOpenInCode = (e: React.MouseEvent) => {
        e.stopPropagation();
        // Will implement with shell opener
        console.log('Open in code:', repo.path);
    };

    const handleOpenFolder = (e: React.MouseEvent) => {
        e.stopPropagation();
        // Will implement with shell opener
        console.log('Open folder:', repo.path);
    };

    // const statusInfo = statusConfig[repo.status];
    const { health } = repo;

    const getProgressClass = () => {
        if (health.is_dirty) return 'status-fill-dirty';
        if (health.commits_behind > 0) return 'status-fill-behind';
        if (health.commits_ahead > 0) return 'status-fill-ahead';
        return 'status-fill-clean';
    };

    return (
        <div className="repo-card" onClick={handleCardClick}>
            {/* Hover Actions Overlay */}
            <div className="repo-actions-overlay">
                <button className="card-action" onClick={handleRefresh} title="Refresh">
                    <RefreshCw size={14} />
                </button>
                <button className="card-action" onClick={handleOpenFolder} title="Open Folder">
                    <FolderOpen size={14} />
                </button>
                <button className="card-action" onClick={handleOpenInCode} title="Open in Editor">
                    <ExternalLink size={14} />
                </button>
            </div>

            <div className="repo-card-header">
                <div className="repo-name-row">
                    <h3 className="repo-name">{repo.name}</h3>
                </div>
                {/* Description / Path */}
                <p className="repo-path" title={repo.path}>{repo.path}</p>
            </div>

            <div className="repo-card-body">
                {/* Status/Branch Info Row */}
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-end', marginTop: 'auto' }}>
                    <div className="repo-info-row" style={{ margin: 0 }}>
                        <GitBranch size={14} style={{ display: 'inline', marginRight: 6, verticalAlign: 'text-bottom' }} />
                        {repo.current_branch || repo.default_branch || 'HEAD'}
                    </div>

                    {/* Status Text */}
                    <div className="status-badge-minimal">
                        {health.is_dirty ? (
                            <span style={{ color: 'var(--status-dirty)' }}>
                                {health.uncommitted_count} uncommitted
                            </span>
                        ) : health.commits_behind > 0 ? (
                            <span style={{ color: 'var(--error)' }}>
                                {health.commits_behind} behind
                            </span>
                        ) : (
                            <span style={{ color: 'var(--success)' }}>Active</span>
                        )}
                    </div>
                </div>
            </div>

            {/* Footer with ONLY Progress Line */}
            <div className="repo-card-footer">
                <div className="status-progress-bar">
                    <div className={`status-progress-fill ${getProgressClass()}`} style={{ width: '100%' }}></div>
                </div>
            </div>
        </div>
    );
}
