'use client';

import React, { useState, useEffect } from 'react';
import Sidebar from './Sidebar';
import { usePathname, useRouter } from 'next/navigation';
import { Menu, X } from 'lucide-react';
import { isAuthenticated } from '@/lib/auth';

const Layout = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  const router = useRouter();
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const [isChecking, setIsChecking] = useState(true);
  const isLoginPage = pathname === '/login';

  useEffect(() => {
    // Check authentication for protected routes
    if (!isLoginPage && pathname !== '/') {
      if (!isAuthenticated()) {
        router.replace('/login');
      } else {
        setIsChecking(false);
      }
    } else {
      setIsChecking(false);
    }
  }, [pathname, router, isLoginPage]);

  if (isLoginPage) {
    return <div className="min-h-screen bg-gray-50 dark:bg-gray-900">{children}</div>;
  }

  // Show loading state while checking auth
  if (isChecking) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-gray-50 dark:bg-gray-900">
        <div className="text-gray-500">Loading...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* Mobile Header */}
      <div className="flex items-center justify-between border-b border-gray-200 bg-white p-4 sm:hidden dark:border-gray-700 dark:bg-gray-800">
        <span className="text-xl font-bold text-blue-600 dark:text-blue-400">KUD Admin</span>
        <button 
          onClick={() => setIsSidebarOpen(!isSidebarOpen)}
          className="rounded-lg p-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-gray-700"
        >
          {isSidebarOpen ? <X className="h-6 w-6" /> : <Menu className="h-6 w-6" />}
        </button>
      </div>

      <Sidebar isOpen={isSidebarOpen} setIsOpen={setIsSidebarOpen} />
      
      <div className="p-4 sm:ml-64">
        <div className="rounded-lg p-4">
          {children}
        </div>
      </div>
    </div>
  );
};

export default Layout;
