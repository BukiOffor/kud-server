export const isAuthenticated = (): boolean => {
  if (typeof window === 'undefined') return false;
  const token = localStorage.getItem('auth_token');
  const user = localStorage.getItem('user');
  return !!(token && user);
};

export const getStoredUser = () => {
  if (typeof window === 'undefined') return null;
  const userStr = localStorage.getItem('user');
  return userStr ? JSON.parse(userStr) : null;
};

export const getLastUserEmail = (): string | null => {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem('last_user_email');
};

export const getLastUser = () => {
  if (typeof window === 'undefined') return null;
  const userStr = localStorage.getItem('last_user');
  return userStr ? JSON.parse(userStr) : null;
};

export const saveLastUserEmail = (email: string) => {
  if (typeof window === 'undefined') return;
  localStorage.setItem('last_user_email', email);
};

export const saveLastUser = (user: any) => {
  if (typeof window === 'undefined') return;
  localStorage.setItem('last_user', JSON.stringify(user));
};

export const clearLastUserEmail = () => {
  if (typeof window === 'undefined') return;
  localStorage.removeItem('last_user_email');
  localStorage.removeItem('last_user');
};

export const logout = () => {
  if (typeof window === 'undefined') return;
  localStorage.removeItem('auth_token');
  localStorage.removeItem('user');
  // Note: We keep last_user_email for the enhanced login experience
};
