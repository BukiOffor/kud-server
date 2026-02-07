'use client';

import Link from 'next/link';
import { Home, Rocket, ArrowLeft } from 'lucide-react';

export default function NotFound() {
  return (
    <div className="flex min-h-[80vh] flex-col items-center justify-center text-center">
      <div className="relative mb-8">
        <div className="absolute -inset-4 rounded-full bg-blue-500/20 blur-2xl animate-pulse"></div>
        <div className="relative flex h-24 w-24 items-center justify-center rounded-2xl bg-blue-600 text-white shadow-xl shadow-blue-500/40">
          <Rocket className="h-12 w-12 animate-bounce" />
        </div>
      </div>
      
      <h1 className="mb-4 text-5xl font-extrabold tracking-tight text-gray-900 dark:text-white sm:text-6xl">
        Coming Soon
      </h1>
      
      <p className="mb-10 max-w-sm text-lg text-gray-500 dark:text-gray-400">
        We&apos;re working hard to bring this feature to life. Stay tuned for updates!
      </p>
      
      <div className="flex flex-col space-y-4 sm:flex-row sm:space-x-4 sm:space-y-0">
        <Link
          href="/"
          className="flex items-center justify-center rounded-xl bg-blue-600 px-8 py-3 text-sm font-bold text-white shadow-lg shadow-blue-500/30 transition-all hover:bg-blue-700 hover:scale-105 active:scale-95"
        >
          <Home className="mr-2 h-4 w-4" />
          Back to Dashboard
        </Link>
        
        <button
          onClick={() => window.history.back()}
          className="flex items-center justify-center rounded-xl border border-gray-200 bg-white px-8 py-3 text-sm font-bold text-gray-700 shadow-sm transition-all hover:bg-gray-50 hover:border-gray-300 active:scale-95 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
        >
          <ArrowLeft className="mr-2 h-4 w-4" />
          Go Back
        </button>
      </div>

      <div className="mt-16 grid grid-cols-3 gap-8 opacity-20 dark:opacity-10">
        <div className="h-1 w-24 rounded-full bg-blue-600"></div>
        <div className="h-1 w-24 rounded-full bg-blue-600"></div>
        <div className="h-1 w-24 rounded-full bg-blue-600"></div>
      </div>
    </div>
  );
}
