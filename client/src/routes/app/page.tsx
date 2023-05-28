import { Outlet } from "react-router-dom";
import { Navbar } from "./components/Navbar";
import { Sidebar } from "./components/Sidebar";
import React from "react";
import { useAtom } from "jotai";
import { playerDataAtom } from "./state";
import { Player } from "./components/Player";

export default function Page() {
  const [sidebarOpen, setSidebarOpen] = React.useState(false);
  const [playerData] = useAtom(playerDataAtom);

  return (
    <div>
      <Navbar
        toggleSidebarOpen={() => {
          setSidebarOpen(!sidebarOpen);
        }}
      />
      <div className="flex flex-row">
        <Sidebar open={sidebarOpen} />
        <div className={"flex flex-col"}>
          <div className="p-4 mt-14 ">
            <Outlet />
          </div>
          <div className="fixed bottom-0 w-full">
            {playerData ? <Player playerData={playerData} /> : <></>}
          </div>
        </div>
      </div>
    </div>
  );
}
