import {
  ArrowPathIcon,
  Cog6ToothIcon,
  HomeIcon,
} from "@heroicons/react/24/solid";
import { NavLink } from "@mantine/core";
import {} from "@mantine/core";
import { useLocation, useNavigate } from "react-router-dom";
export function Navbar() {
  const navigate = useNavigate();
  const location = useLocation();

  return (
    <div>
      <NavLink
        label="Home"
        icon={<HomeIcon className="w-4 h-4" />}
        active={location.pathname === "/app"}
        onClick={() => navigate("/app")}
      />
      <NavLink
        label="Settings"
        icon={<Cog6ToothIcon className="w-4 h-4" />}
        active={location.pathname === "/app/settings"}
        onClick={() => navigate("/app/settings")}
      />
      <NavLink
        label="Scan"
        icon={<ArrowPathIcon className="w-4 h-4" />}
        active={location.pathname === "/app/scan"}
        onClick={() => navigate("/app/scan")}
      />
    </div>
  );
}
