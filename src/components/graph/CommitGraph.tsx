import { CommitLogEntry } from '../../services/tauri';
import './CommitGraph.css';

interface CommitGraphProps {
    commits: CommitLogEntry[];
    onCommitSelect?: (oid: string) => void;
}

export function CommitGraph({ commits, onCommitSelect }: CommitGraphProps) {
    // Basic rendering for now: Just a list to verify data flow. 
    // We will implement SVG graph logic in the next step.
    return (
        <div className="commit-graph-container">
            <h3>Commit History</h3>
            <div className="commit-list">
                {commits.map(commit => (
                    <div
                        key={commit.oid}
                        className="commit-row"
                        onClick={() => onCommitSelect?.(commit.oid)}
                    >
                        <div className="commit-node">●</div>
                        <div className="commit-details">
                            <div className="commit-message">{commit.message}</div>
                            <div className="commit-meta">
                                <span className="commit-hash">{commit.short_oid}</span>
                                <span className="commit-author">{commit.author_name}</span>
                                <span className="commit-date">{new Date(commit.date * 1000).toLocaleString()}</span>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
}
