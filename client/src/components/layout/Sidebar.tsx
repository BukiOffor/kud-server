import Link from 'next/link';
import { useRouter, usePathname } from 'next/navigation';
import { Home, Users, Calendar, Settings, LogOut } from 'lucide-react';
import { useEffect, useState } from 'react';

interface SidebarProps {
  isOpen: boolean;
  setIsOpen: (isOpen: boolean) => void;
}

const Sidebar = ({ isOpen, setIsOpen }: SidebarProps) => {
  const router = useRouter();
  const pathname = usePathname();
  const [user, setUser] = useState<any>(null);

  useEffect(() => {
    const storedUser = localStorage.getItem('user');
    if (storedUser) {
      setUser(JSON.parse(storedUser));
    } else if (pathname !== '/login') {
      router.push('/login');
    }
  }, [pathname, router]);

  // Close sidebar on route change on mobile
  useEffect(() => {
    setIsOpen(false);
  }, [pathname, setIsOpen]);

  const handleLogout = () => {
    localStorage.removeItem('auth_token');
    localStorage.removeItem('user');
    router.push('/login');
  };

  const menuItems = [
    { name: 'Dashboard', icon: Home, href: '/' },
    { name: 'Users', icon: Users, href: '/users' },
    { name: 'Events', icon: Calendar, href: '/events' },
    { name: 'Settings', icon: Settings, href: '/settings' },
  ];

  return (
    <>
      {/* Backdrop for mobile */}
      {isOpen && (
        <div 
          className="fixed inset-0 z-30 bg-gray-900/50 backdrop-blur-sm sm:hidden"
          onClick={() => setIsOpen(false)}
        />
      )}

      <aside 
        className={`fixed left-0 top-0 z-40 h-screen w-64 border-r border-gray-200 bg-white transition-transform dark:border-gray-700 dark:bg-gray-800 ${
          isOpen ? 'translate-x-0' : '-translate-x-full'
        } sm:translate-x-0`}
      >
        <div className="flex h-full flex-col px-3 py-4">
          <div className="mb-8 flex items-center justify-between px-2">
            <span className="text-2xl font-bold text-blue-600 dark:text-blue-400">KUD Admin</span>
            <button 
              onClick={() => setIsOpen(false)}
              className="rounded-lg p-2 text-gray-500 hover:bg-gray-100 sm:hidden dark:text-gray-400 dark:hover:bg-gray-700"
            >
              <Users className="h-6 w-6 rotate-90" /> {/* Using Users as placeholder for close, or just use X if available */}
            </button>
          </div>
          
          {user && (
            <div className="mb-6 px-2">
              <p className="text-xs font-semibold uppercase text-gray-400">Current User</p>
              <p className="text-sm font-medium text-gray-900 dark:text-white">{user.first_name} {user.last_name}</p>
              <p className="text-xs text-gray-500">{user.role}</p>
            </div>
          )}

          <ul className="space-y-2 font-medium">
            {menuItems.map((item) => (
              <li key={item.name}>
                <Link
                  href={item.href}
                  className={`flex items-center rounded-lg p-2 transition-colors ${
                    pathname === item.href 
                      ? 'bg-blue-50 text-blue-600 dark:bg-blue-900/20 dark:text-blue-400' 
                      : 'text-gray-900 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700'
                  }`}
                >
                  <item.icon className={`h-5 w-5 ${pathname === item.href ? 'text-blue-600 dark:text-blue-400' : 'text-gray-500'}`} />
                  <span className="ms-3">{item.name}</span>
                </Link>
              </li>
            ))}
          </ul>
          <div className="mt-auto border-t border-gray-200 pt-4 dark:border-gray-700">
            <button 
              onClick={handleLogout}
              className="flex w-full items-center rounded-lg p-2 text-gray-900 hover:bg-gray-100 dark:text-white dark:hover:bg-gray-700"
            >
              <LogOut className="h-5 w-5 text-gray-500" />
              <span className="ms-3">Logout</span>
            </button>
          </div>
        </div>
      </aside>
    </>
  );
};

export default Sidebar;
