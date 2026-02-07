'use client';

import React, { useState } from 'react';
import Sidebar from './Sidebar';
import { usePathname } from 'next/navigation';
import { Menu, X } from 'lucide-react';

const Layout = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  const [isSidebarOpen, setIsSidebarOpen] = useState(false);
  const isLoginPage = pathname === '/login';

  if (isLoginPage) {
    return <div className="min-h-screen bg-gray-50 dark:bg-gray-900">{children}</div>;
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
