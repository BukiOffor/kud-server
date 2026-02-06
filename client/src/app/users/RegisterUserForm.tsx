'use client';

import { useState } from 'react';
import { usersApi } from '@/lib/api';
import { NewUser, Role } from '@/lib/types';

interface RegisterUserFormProps {
  onSuccess: () => void;
  onCancel: () => void;
}

const RegisterUserForm = ({ onSuccess, onCancel }: RegisterUserFormProps) => {
  const [formData, setFormData] = useState<NewUser>({
    first_name: '',
    last_name: '',
    email: '',
    password: '',
    year_joined: new Date().getFullYear().toString(),
    is_active: true,
    role: 'User' as Role,
    gender: 'Other',
    phone: '',
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      await usersApi.register(formData);
      onSuccess();
    } catch (err) {
      alert('Failed to register user');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">First Name</label>
          <input
            required
            type="text"
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.first_name}
            onChange={(e) => setFormData({ ...formData, first_name: e.target.value })}
          />
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Last Name</label>
          <input
            required
            type="text"
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.last_name}
            onChange={(e) => setFormData({ ...formData, last_name: e.target.value })}
          />
        </div>
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Email</label>
        <input
          required
          type="email"
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={formData.email}
          onChange={(e) => setFormData({ ...formData, email: e.target.value })}
        />
      </div>
      <div>
        <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Password</label>
        <input
          required
          type="password"
          className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={formData.password}
          onChange={(e) => setFormData({ ...formData, password: e.target.value })}
        />
      </div>
      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Role</label>
          <select
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.role}
            onChange={(e) => setFormData({ ...formData, role: e.target.value as Role })}
          >
            <option value="User">User</option>
            <option value="Admin">Admin</option>
            <option value="Technical">Technical</option>
          </select>
        </div>
        <div>
          <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">Year Joined</label>
          <input
            type="text"
            className="w-full rounded-lg border border-gray-300 p-2 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            value={formData.year_joined}
            onChange={(e) => setFormData({ ...formData, year_joined: e.target.value })}
          />
        </div>
      </div>
      <div className="flex justify-end gap-2 pt-4">
        <button
          type="button"
          onClick={onCancel}
          className="rounded-lg border px-4 py-2 text-sm font-medium hover:bg-gray-100 dark:hover:bg-gray-700"
        >
          Cancel
        </button>
        <button
          type="submit"
          disabled={loading}
          className="rounded-lg bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? 'Registering...' : 'Register User'}
        </button>
      </div>
    </form>
  );
};

export default RegisterUserForm;
