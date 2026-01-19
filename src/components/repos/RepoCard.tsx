import {
    GitBranch,
    FolderOpen,
    ExternalLink,
    RefreshCw,
    CircleDot,
    ArrowUp,
    ArrowDown,
    GitMerge,
    Archive
} from 'lucide-react';
import { RepoInfo, RepoStatus } from '../../services/tauri';
import { refreshRepo } from '../../services/tauri';
import { useAppStore } from '../../stores/appStore';
import { useRepoStore } from '../../stores/repoStore';
import './RepoCard.css';

interface RepoCardProps {
    repo: RepoInfo;
}

const statusConfig: Record<RepoStatus, { icon: React.ReactNode; label: string; className: string }> = {
    clean: { icon: <CircleDot size={14} />, label: 'Clean', className: 'status-clean' },
    dirty: { icon: <CircleDot size={14} />, label: 'Dirty', className: 'status-dirty' },
    ahead: { icon: <ArrowUp size={14} />, label: 'Ahead', className: 'status-ahead' },
    behind: { icon: <ArrowDown size={14} />, label: 'Behind', className: 'status-behind' },
    diverged: { icon: <GitMerge size={14} />, label: 'Diverged', className: 'status-diverged' },
};

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

    const statusInfo = statusConfig[repo.status];
    const { health } = repo;

    return (
        <div className="repo-card">
            <div className="repo-card-header clickable" onClick={handleCardClick}>
                <div className="repo-name-row">
                    <h3 className="repo-name">{repo.name}</h3>
                    <span className={`status-badge ${statusInfo.className}`}>
                        {statusInfo.icon}
                        {statusInfo.label}
                    </span>
                </div>
                <p className="repo-path" title={repo.path}>{repo.path}</p>
            </div>

            <div className="repo-card-body">
                {/* Branch info */}
                <div className="repo-info-row">
                    <GitBranch size={16} />
                    <span>{repo.current_branch || repo.default_branch || 'No branch'}</span>
                </div>

                {/* Health details */}
                {health.is_dirty && (
                    <div className="repo-details">
                        {health.uncommitted_count > 0 && (
                            <span className="detail-chip warning">
                                {health.uncommitted_count} uncommitted
                            </span>
                        )}
                        {health.staged_count > 0 && (
                            <span className="detail-chip info">
                                {health.staged_count} staged
                            </span>
                        )}
                    </div>
                )}

                {(health.commits_ahead > 0 || health.commits_behind > 0) && (
                    <div className="repo-details">
                        {health.commits_ahead > 0 && (
                            <span className="detail-chip info">
                                <ArrowUp size={12} /> {health.commits_ahead} ahead
                            </span>
                        )}
                        {health.commits_behind > 0 && (
                            <span className="detail-chip chip-error">
                                <ArrowDown size={12} /> {health.commits_behind} behind
                            </span>
                        )}
                    </div>
                )}

                {health.stash_count > 0 && (
                    <div className="repo-details">
                        <span className="detail-chip muted">
                            <Archive size={12} /> {health.stash_count} stashed
                        </span>
                    </div>
                )}

                {/* Tags */}
                {repo.tags.length > 0 && (
                    <div className="repo-tags">
                        {repo.tags.map(tag => (
                            <span key={tag} className="tag-chip">{tag}</span>
                        ))}
                    </div>
                )}
            </div>

            <div className="repo-card-footer">
                <button
                    className="card-action"
                    onClick={handleRefresh}
                    title="Refresh status"
                >
                    <RefreshCw size={16} />
                </button>
                <button
                    className="card-action"
                    onClick={handleOpenFolder}
                    title="Open folder"
                >
                    <FolderOpen size={16} />
                </button>
                <button
                    className="card-action"
                    onClick={handleOpenInCode}
                    title="Open in editor"
                >
                    <ExternalLink size={16} />
                </button>
            </div>
        </div>
    );
}
