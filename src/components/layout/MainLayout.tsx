/**
 * Main layout wrapper component.
 */

import { Sidebar } from './Sidebar';
import { Header } from './Header';
import './MainLayout.css';

interface MainLayoutProps {
    children: React.ReactNode;
}

export function MainLayout({ children }: MainLayoutProps) {
    return (
        <div className="app-layout">
            <Sidebar />
            <div className="main-content">
                <Header />
                <main className="content-area">
                    {children}
                </main>
            </div>
        </div>
    );
}
