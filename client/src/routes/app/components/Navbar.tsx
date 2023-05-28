import { Bars4Icon } from "@heroicons/react/24/solid";
import { Link } from "react-router-dom";

export function Navbar(
  { toggleSidebarOpen }: { toggleSidebarOpen: () => void },
) {
  return (
    <nav className="fixed top-0 z-50 w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700 h-14">
      <div className="px-3 py-2 lg:px-5 lg:pl-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center justify-start">
            <button
              type="button"
              onClick={() => toggleSidebarOpen()}
              className="inline-flex items-center p-2 text-sm text-gray-500 rounded-lg hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
            >
              <Bars4Icon className="w-6 h-6" />
            </button>
            <Link to="/app" className="flex ml-2 md:mr-24">
              <span className="self-center text-xl font-semibold sm:text-2xl whitespace-nowrap dark:text-white">
                dap
              </span>
            </Link>
          </div>
        </div>
      </div>
    </nav>
  );
}
