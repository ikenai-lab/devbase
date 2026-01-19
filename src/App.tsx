/**
 * DevBase - Main Application Component
 */

import { useEffect } from 'react';
import { MainLayout } from './components/layout';
import { RepoGrid, RepoDetail } from './components/repos';
import { Settings } from './components/settings';
import { useAppStore } from './stores/appStore';
import { getVersion } from './services/tauri';
import './App.css';
import './styles/theme.css';
import './App.css';

function App() {
  const { currentView, setVersion, setLoading, theme } = useAppStore();

  useEffect(() => {
    // Apply theme
    document.documentElement.setAttribute('data-theme', theme);
  }, [theme]);

  useEffect(() => {
    const init = async () => {
      try {
        const version = await getVersion();
        setVersion(version);
      } catch (err) {
        console.error('Failed to get version:', err);
      } finally {
        setLoading(false);
      }
    };

    init();
  }, [setVersion, setLoading]);

  const renderView = () => {
    switch (currentView) {
      case 'dashboard':
        return <RepoGrid />;
      case 'search':
        return <PlaceholderView title="Global Search" description="Search across all repositories (Coming in Phase 4)" />;
      case 'analytics':
        return <PlaceholderView title="Local Analytics" description="Your coding statistics (Coming in Phase 4)" />;
      case 'hygiene':
        return <PlaceholderView title="Disk Hygiene" description="Clean up old repos and branches (Coming in Phase 5)" />;
      case 'settings':
        return <Settings />;
      case 'repo-detail':
        return <RepoDetail />;
      default:
        return <RepoGrid />;
    }
  };

  return (
    <MainLayout>
      {renderView()}
    </MainLayout>
  );
}

function PlaceholderView({ title, description }: { title: string; description: string }) {
  return (
    <div className="placeholder-view">
      <h2>{title}</h2>
      <p>{description}</p>
    </div>
  );
}

export default App;
