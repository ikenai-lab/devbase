import { useEffect } from 'react';
import { ArrowLeft, GitGraph, FileCode, GitPullRequest } from 'lucide-react';
import { useAppStore } from '../../stores/appStore';
import { useRepoStore } from '../../stores/repoStore';
import { CommitGraph } from '../graph/CommitGraph';
import './RepoDetail.css';

export function RepoDetail() {
    const { setCurrentView } = useAppStore();
    const { selectedRepoId, repositories, activeCommitLog, loadCommitLog, isLoading, error } = useRepoStore();

    const repo = repositories.find(r => r.id === selectedRepoId);

    useEffect(() => {
        if (selectedRepoId) {
            loadCommitLog(selectedRepoId);
        }
    }, [selectedRepoId, loadCommitLog]);

    if (!repo) {
        return (
            <div className="repo-detail-error">
                <h2>Repository not found</h2>
                <button onClick={() => setCurrentView('dashboard')}>Back to Dashboard</button>
            </div>
        );
    }

    return (
        <div className="repo-detail-container">
            <div className="repo-detail-header">
                <button className="back-button" onClick={() => setCurrentView('dashboard')}>
                    <ArrowLeft size={20} />
                    Back
                </button>
                <div className="repo-title">
                    <h2>{repo.name}</h2>
                    <span className="repo-path">{repo.path}</span>
                </div>
                <div className="repo-actions">
                    <button className="icon-button active" title="Commit Graph">
                        <GitGraph size={20} />
                    </button>
                    <button className="icon-button" title="Files">
                        <FileCode size={20} />
                    </button>
                    <button className="icon-button" title="Pull Requests">
                        <GitPullRequest size={20} />
                    </button>
                </div>
            </div>

            <div className="repo-content">
                {isLoading && <div className="loading">Loading history...</div>}
                {error && <div className="error">{error}</div>}

                {!isLoading && !error && activeCommitLog.length > 0 && (
                    <CommitGraph commits={activeCommitLog} />
                )}

                {!isLoading && !error && activeCommitLog.length === 0 && (
                    <div className="empty-history">No commit history found.</div>
                )}
            </div>
        </div>
    );
}
