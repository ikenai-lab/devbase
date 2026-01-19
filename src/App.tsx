/**
 * DevBase - Main Application Component
 */

import { useEffect } from 'react';
import { MainLayout } from './components/layout';
import { Dashboard } from './components/dashboard';
import { useAppStore } from './stores/appStore';
import { getVersion } from './services/tauri';
import './App.css';

function App() {
  const { currentView, setVersion, setLoading } = useAppStore();

  useEffect(() => {
    // Initialize app
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

  // Render the current view
  const renderView = () => {
    switch (currentView) {
      case 'dashboard':
        return <Dashboard />;
      case 'search':
        return <PlaceholderView title="Global Search" description="Search across all repositories (Coming in Phase 4)" />;
      case 'analytics':
        return <PlaceholderView title="Local Analytics" description="Your coding statistics (Coming in Phase 4)" />;
      case 'hygiene':
        return <PlaceholderView title="Disk Hygiene" description="Clean up old repos and branches (Coming in Phase 5)" />;
      case 'settings':
        return <PlaceholderView title="Settings" description="Configure scan paths and preferences (Coming in Phase 2)" />;
      default:
        return <Dashboard />;
    }
  };

  return (
    <MainLayout>
      {renderView()}
    </MainLayout>
  );
}

// Placeholder component for unimplemented views
function PlaceholderView({ title, description }: { title: string; description: string }) {
  return (
    <div className="placeholder-view">
      <h2>{title}</h2>
      <p>{description}</p>
    </div>
  );
}

export default App;
