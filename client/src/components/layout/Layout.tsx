'use client';

import React from 'react';
import Sidebar from './Sidebar';
import { usePathname } from 'next/navigation';

const Layout = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname();
  const isLoginPage = pathname === '/login';

  if (isLoginPage) {
    return <div className="min-h-screen bg-gray-50 dark:bg-gray-900">{children}</div>;
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <Sidebar />
      <div className="p-4 sm:ml-64">
        <div className="rounded-lg p-4">
          {children}
        </div>
      </div>
    </div>
  );
};

export default Layout;
