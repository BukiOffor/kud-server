'use client';

import { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { authApi } from '@/lib/api';
import { Lock, User, AlertCircle } from 'lucide-react';
import { getLastUser, saveLastUser, saveLastUserEmail } from '@/lib/auth';

const LoginPage = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [rememberedUser, setRememberedUser] = useState<any>(null);
  const router = useRouter();

  useEffect(() => {
    // Check if there's a remembered user
    const lastUser = getLastUser();
    if (lastUser) {
      setRememberedUser(lastUser);
      setEmail(lastUser.email);
    }
  }, []);

  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      const response = await authApi.login({ user: email, password });
      
      // Store user data and token
      localStorage.setItem('auth_token', 'mock_token_for_now'); 
      localStorage.setItem('user', JSON.stringify(response.data));
      
      // Save user data for next login
      saveLastUserEmail(response.data.email);
      saveLastUser(response.data);
      
      console.log('Login successful:', response.data);
      router.push('/dashboard');
    } catch (err) {
      setError('Invalid email or password');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-h-screen items-center justify-center bg-gray-50 dark:bg-gray-900">
      <div className="w-full max-w-md rounded-lg bg-white p-8 shadow-lg dark:bg-gray-800">
        <div className="mb-8 text-center">
          <h1 className="text-3xl font-bold text-blue-600 dark:text-blue-400">KUD Management System</h1>
          {rememberedUser ? (
            <p className="mt-2 text-gray-600 dark:text-gray-400">
              Welcome Back, <span className="font-bold text-gray-800 dark:text-white">{rememberedUser.first_name}</span>
            </p>
          ) : (
            <p className="mt-2 text-gray-600 dark:text-gray-400">Sign in to manage your server</p>
          )}
        </div>

        {error && (
          <div className="mb-4 flex items-center rounded-lg bg-red-100 p-4 text-red-700 dark:bg-red-900/30 dark:text-red-400">
            <AlertCircle className="me-2 h-5 w-5" />
            {error}
          </div>
        )}

        <form onSubmit={handleLogin} className="space-y-6">
          {!rememberedUser && (
            <div>
              <label className="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
                Email
              </label>
              <div className="relative">
                <User className="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
                <input
                  required
                  type="email"
                  className="w-full rounded-lg border border-gray-300 py-2 pl-10 pr-4 focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                />
              </div>
            </div>
          )}
          <div>
            <label className="mb-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
              Password
            </label>
            <div className="relative">
              <Lock className="absolute left-3 top-1/2 h-5 w-5 -translate-y-1/2 text-gray-400" />
              <input
                required
                type="password"
                className="w-full rounded-lg border border-gray-300 py-2 pl-10 pr-4 focus:border-blue-500 focus:outline-none dark:border-gray-600 dark:bg-gray-700 dark:text-white"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                autoFocus={!!rememberedUser}
              />
            </div>
          </div>
          <button
            type="submit"
            disabled={loading}
            className="w-full rounded-lg bg-blue-600 px-5 py-3 text-center text-base font-medium text-white hover:bg-blue-700 focus:outline-none disabled:opacity-50"
          >
            {loading ? 'Signing in...' : 'Sign In'}
          </button>
        </form>
      </div>
    </div>
  );
};

export default LoginPage;
