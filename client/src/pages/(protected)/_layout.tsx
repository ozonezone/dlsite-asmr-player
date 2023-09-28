import { useAtom } from "jotai";
import { playerDataAtom } from "./_state";
import { Player } from "./_components/Player";

import { useState } from "react";
import {
  AppShell,
  Footer,
  Header,
  Navbar,
  useMantineTheme,
} from "@mantine/core";

import { Header as CustomHeader } from "./_components/Header";
import { Navbar as CustomNavbar } from "./_components/Navbar";

import { Outlet } from "react-router-dom";
import Protected from "./_components/Protected";
import { QueryParamProvider } from "use-query-params";
import { ReactRouter6Adapter } from "use-query-params/adapters/react-router-6";

export default function Page() {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);

  const [playerData] = useAtom(playerDataAtom);

  return (
    <Protected>
      <QueryParamProvider
        adapter={ReactRouter6Adapter}
      >
        <AppShell
          styles={{
            main: {
              background: theme.colorScheme === "dark"
                ? theme.colors.dark[8]
                : theme.colors.gray[0],
            },
          }}
          navbarOffsetBreakpoint="sm"
          asideOffsetBreakpoint="sm"
          navbar={
            <Navbar
              width={{ sm: opened ? 200 : 0 }}
              className={`${
                opened ? "" : "md:translate-x-[-200px] -translate-x-full"
              } transition-all duration-100`}
            >
              <CustomNavbar />
            </Navbar>
          }
          footer={
            <Footer height={playerData ? 120 : 0}>
              <div className="fixed bottom-0 w-full z-50">
                {playerData ? <Player playerData={playerData} /> : <></>}
              </div>
            </Footer>
          }
          header={
            <Header
              height={{ base: 50 }}
              p="md"
            >
              <CustomHeader opened={opened} setOpened={setOpened} />
            </Header>
          }
        >
          <Outlet />
        </AppShell>
      </QueryParamProvider>
    </Protected>
  );
}
