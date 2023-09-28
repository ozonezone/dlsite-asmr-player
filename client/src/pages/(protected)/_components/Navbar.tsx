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
    <>
      <NavLink
        label="Home"
        leftSection={<HomeIcon className="w-4 h-4" />}
        active={location.pathname === "/"}
        onClick={() => navigate("/")}
      />
      <NavLink
        label="Settings"
        leftSection={<Cog6ToothIcon className="w-4 h-4" />}
        active={location.pathname === "/settings"}
        onClick={() => navigate("/settings")}
      />
      <NavLink
        label="Scan"
        leftSection={<ArrowPathIcon className="w-4 h-4" />}
        active={location.pathname === "//scan"}
        onClick={() => navigate("/scan")}
      />
    </>
  );
}
