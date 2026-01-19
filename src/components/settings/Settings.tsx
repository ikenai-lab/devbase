import { useEffect, useState } from 'react';
import { FolderPlus, Trash2, ChevronDown, ChevronRight, RefreshCw, FolderSearch } from 'lucide-react';
import { open } from '@tauri-apps/plugin-dialog';
import { useRepoStore } from '../../stores/repoStore';
import { addScanPath, removeScanPath, updateScanPath } from '../../services/tauri';
import './Settings.css';

export function Settings() {
    const { scanPaths, fetchScanPaths, runScan, isScanning } = useRepoStore();
    const [newPath, setNewPath] = useState('');
    const [maxDepth, setMaxDepth] = useState(5);
    const [isAdding, setIsAdding] = useState(false);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        fetchScanPaths();
    }, [fetchScanPaths]);

    const handleBrowse = async () => {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                title: 'Select Folder to Scan'
            });

            if (selected && typeof selected === 'string') {
                setNewPath(selected);
                setError(null);
            }
        } catch (e) {
            console.error(e);
            setError('Failed to open directory picker');
        }
    };

    const handleAddPath = async () => {
        if (!newPath.trim()) return;

        setIsAdding(true);
        setError(null);

        try {
            await addScanPath(newPath.trim(), maxDepth);
            await fetchScanPaths();
            setNewPath('');
        } catch (e) {
            setError(String(e));
        } finally {
            setIsAdding(false);
        }
    };

    const handleRemovePath = async (id: number) => {
        try {
            await removeScanPath(id);
            await fetchScanPaths();
        } catch (e) {
            setError(String(e));
        }
    };

    const handleTogglePath = async (id: number, currentEnabled: boolean) => {
        try {
            await updateScanPath(id, !currentEnabled);
            await fetchScanPaths();
        } catch (e) {
            setError(String(e));
        }
    };

    return (
        <div className="settings-container">
            <section className="settings-section">
                <h2>Scan Paths</h2>
                <p className="section-description">
                    Configure which directories to scan for Git repositories.
                </p>

                {/* Add new path */}
                <div className="add-path-form">
                    <div className="input-group">
                        <input
                            type="text"
                            placeholder="Enter path (e.g., ~/Projects)"
                            value={newPath}
                            onChange={(e) => setNewPath(e.target.value)}
                            onKeyDown={(e) => e.key === 'Enter' && handleAddPath()}
                        />
                        <button
                            className="browse-button"
                            onClick={handleBrowse}
                            title="Browse Folders"
                        >
                            <FolderSearch size={18} />
                        </button>
                    </div>

                    <div className="form-actions">
                        <select
                            value={maxDepth}
                            onChange={(e) => setMaxDepth(Number(e.target.value))}
                        >
                            <option value={3}>Depth: 3</option>
                            <option value={5}>Depth: 5</option>
                            <option value={10}>Depth: 10</option>
                            <option value={15}>Depth: 15</option>
                        </select>
                        <button
                            className="add-button"
                            onClick={handleAddPath}
                            disabled={isAdding || !newPath.trim()}
                        >
                            <FolderPlus size={18} />
                            {isAdding ? 'Adding...' : 'Add Path'}
                        </button>
                    </div>
                </div>

                {error && (
                    <div className="error-message">{error}</div>
                )}

                {/* Path list */}
                <div className="path-list">
                    {scanPaths.length === 0 ? (
                        <div className="empty-paths">
                            <p>No scan paths configured. Add a path to start discovering repositories.</p>
                        </div>
                    ) : (
                        scanPaths.map((path) => (
                            <div key={path.id} className={`path-item ${!path.enabled ? 'disabled' : ''}`}>
                                <button
                                    className="toggle-button"
                                    onClick={() => handleTogglePath(path.id, path.enabled)}
                                    title={path.enabled ? 'Disable' : 'Enable'}
                                >
                                    {path.enabled ? <ChevronDown size={18} /> : <ChevronRight size={18} />}
                                </button>
                                <div className="path-info">
                                    <span className="path-text">{path.path}</span>
                                    <span className="path-depth">Max depth: {path.max_depth}</span>
                                </div>
                                <button
                                    className="remove-button"
                                    onClick={() => handleRemovePath(path.id)}
                                    title="Remove path"
                                >
                                    <Trash2 size={18} />
                                </button>
                            </div>
                        ))
                    )}
                </div>

                {/* Scan button */}
                <button
                    className="scan-button-large"
                    onClick={() => runScan()}
                    disabled={isScanning || scanPaths.filter(p => p.enabled).length === 0}
                >
                    <RefreshCw size={20} className={isScanning ? 'spinning' : ''} />
                    {isScanning ? 'Scanning...' : 'Scan All Paths'}
                </button>
            </section>
        </div>
    );
}
