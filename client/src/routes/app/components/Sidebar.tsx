import { Link } from "react-router-dom";
import { HomeIcon } from "@heroicons/react/24/solid";

export function Sidebar({ open }: { open: boolean }) {
  return (
    <aside
      className={`h-screen w-32 fixed md:relative mt-14 ${
        open ? "left-0" : "-left-32 md:hidden"
      } top-0 z-50 bg-white border-r border-gray-200 dark:bg-gray-800 dark:border-gray-700}`}
    >
      <div className="h-full px-3 pb-4 overflow-y-auto">
        <ul className="space-y-2 font-medium">
          <SidebarItem to="/app">
            <HomeIcon className="w-5 h-5" />
            <span className="ml-3">Home</span>
          </SidebarItem>
        </ul>
      </div>
    </aside>
  );
}

export function SidebarItem(
  { to, children }: { to: string; children: React.ReactNode },
) {
  return (
    <li>
      <Link
        to={to}
        className="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700"
      >
        {children}
      </Link>
    </li>
  );
}
