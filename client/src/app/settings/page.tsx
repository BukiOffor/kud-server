'use client';

import { useRouter } from 'next/navigation';

const SettingsPage = () => {
  const router = useRouter();

  return (
    <div>
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Settings</h1>
        <p className="mt-1 text-gray-500 dark:text-gray-400">Manage your application settings</p>
      </div>

      <div className="rounded-lg border border-gray-200 bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <p className="text-gray-600 dark:text-gray-400">Settings page coming soon...</p>
      </div>
    </div>
  );
};

export default SettingsPage;
