import { useAtom } from "jotai";
import { playerDataAtom } from "./_state";
import { Player } from "./_components/Player";

import { AppShell } from "@mantine/core";

import { Header } from "./_components/Header";
import { Navbar } from "./_components/Navbar";

import { Outlet } from "react-router-dom";
import Protected from "./_components/Protected";
import { QueryParamProvider } from "use-query-params";
import { ReactRouter6Adapter } from "use-query-params/adapters/react-router-6";
import { useDisclosure } from "@mantine/hooks";

export default function Page() {
  const [opened, { toggle }] = useDisclosure();

  const [playerData] = useAtom(playerDataAtom);

  return (
    <Protected>
      <QueryParamProvider
        adapter={ReactRouter6Adapter}
      >
        <AppShell
          header={{ height: { base: 50 } }}
          navbar={{
            width: 150,
            breakpoint: "sm",
            collapsed: { mobile: !opened, desktop: !opened },
          }}
          footer={{ height: playerData ? 120 : 0 }}
        >
          <AppShell.Header>
            <Header opened={opened} toggle={toggle} />
          </AppShell.Header>

          <AppShell.Navbar
            style={{
              height: "100vh",
            }}
          >
            <Navbar />
          </AppShell.Navbar>

          <AppShell.Main>
            <div className="p-2 bg-gray-100">
              <Outlet />
            </div>
          </AppShell.Main>

          <AppShell.Footer>
            <div className="fixed bottom-0 w-full z-50">
              {playerData ? <Player playerData={playerData} /> : <></>}
            </div>
          </AppShell.Footer>
        </AppShell>
      </QueryParamProvider>
    </Protected>
  );
}
