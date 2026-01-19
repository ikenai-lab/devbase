import { useEffect } from 'react';
import { FolderGit2, RefreshCw, Search, Filter, X, ChevronDown } from 'lucide-react';
import { useRepoStore, useFilteredRepos } from '../../stores/repoStore';
import { RepoCard } from './RepoCard';
import './RepoGrid.css';

export function RepoGrid() {
    const {
        isLoading,
        isScanning,
        error,
        searchQuery,
        selectedStatus,
        fetchRepositories,
        fetchScanPaths,
        runScan,
        setSearchQuery,
        setSelectedStatus,
        clearFilters
    } = useRepoStore();

    const filteredRepos = useFilteredRepos();
    const hasFilters = searchQuery || selectedStatus;

    useEffect(() => {
        fetchRepositories();
        fetchScanPaths();
    }, [fetchRepositories, fetchScanPaths]);

    const handleScan = async () => {
        await runScan();
    };

    const statusOptions = [
        { value: 'clean', label: 'Clean', color: 'var(--color-success)' },
        { value: 'dirty', label: 'Dirty', color: 'var(--color-warning)' },
        { value: 'ahead', label: 'Ahead', color: 'var(--color-info)' },
        { value: 'behind', label: 'Behind', color: 'var(--color-error)' },
        { value: 'diverged', label: 'Diverged', color: 'var(--color-error)' },
    ];

    return (
        <div className="repo-grid-container">
            {/* Toolbar */}
            <div className="repo-toolbar">
                <div className="search-bar">
                    <Search size={18} />
                    <input
                        type="text"
                        placeholder="Search repositories..."
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                    />
                    {searchQuery && (
                        <button
                            className="clear-search"
                            onClick={() => setSearchQuery('')}
                            aria-label="Clear search"
                        >
                            <X size={16} />
                        </button>
                    )}
                </div>

                <div className="filter-bar">
                    <Filter size={18} className="filter-icon" />
                    <div className="select-wrapper">
                        <select
                            value={selectedStatus || ''}
                            onChange={(e) => setSelectedStatus(e.target.value || null)}
                        >
                            <option value="">All Statuses</option>
                            {statusOptions.map(opt => (
                                <option key={opt.value} value={opt.value}>{opt.label}</option>
                            ))}
                        </select>
                    </div>
                    <ChevronDown size={16} className="dropdown-arrow" />
                </div>

                {hasFilters && (
                    <button className="clear-filters" onClick={clearFilters}>
                        <X size={16} />
                        Clear Filters
                    </button>
                )}

                <button
                    className="scan-button"
                    onClick={handleScan}
                    disabled={isScanning}
                >
                    <RefreshCw size={18} className={isScanning ? 'spinning' : ''} />
                    {isScanning ? 'Scanning...' : 'Scan'}
                </button>
            </div>

            {/* Error display */}
            {error && (
                <div className="error-banner">
                    <span>{error}</span>
                    <button onClick={() => useRepoStore.setState({ error: null })}>
                        <X size={16} />
                    </button>
                </div>
            )}

            {/* Loading state */}
            {isLoading && !filteredRepos.length && (
                <div className="loading-state">
                    <RefreshCw size={32} className="spinning" />
                    <p>Loading repositories...</p>
                </div>
            )}

            {/* Empty state */}
            {!isLoading && filteredRepos.length === 0 && (
                <div className="empty-state">
                    <FolderGit2 size={64} strokeWidth={1} />
                    <h3>No repositories found</h3>
                    {hasFilters ? (
                        <p>Try adjusting your filters or search query.</p>
                    ) : (
                        <p>Add scan paths in settings and run a scan to discover repositories.</p>
                    )}
                    {!hasFilters && (
                        <button className="primary-button" onClick={handleScan}>
                            <RefreshCw size={18} />
                            Scan for Repositories
                        </button>
                    )}
                </div>
            )}

            {/* Repository grid */}
            {filteredRepos.length > 0 && (
                <>
                    <div className="repo-count">
                        {filteredRepos.length} {filteredRepos.length === 1 ? 'repository' : 'repositories'}
                        {hasFilters && ' (filtered)'}
                    </div>
                    <div className="repo-grid">
                        {filteredRepos.map((repo) => (
                            <RepoCard key={repo.id} repo={repo} />
                        ))}
                    </div>
                </>
            )}
        </div>
    );
}
