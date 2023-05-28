import React from "react";
import { useAtom } from "jotai";
import { playerDataAtom } from "./state";
import { Player } from "./components/Player";

import { useState } from "react";
import {
  AppShell,
  Aside,
  Footer,
  Header,
  MediaQuery,
  Navbar,
  Text,
  useMantineTheme,
} from "@mantine/core";

import { Header as CustomHeader } from "./components/Header";
import { Navbar as CustomNavbar } from "./components/Navbar";

import { Outlet } from "react-router-dom";

export default function Page() {
  const theme = useMantineTheme();
  const [opened, setOpened] = useState(false);

  const [playerData] = useAtom(playerDataAtom);

  return (
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
          hiddenBreakpoint="sm"
          hidden={!opened}
          width={{ sm: 200, lg: 300 }}
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
          height={{ base: 50, md: 70 }}
          p="md"
        >
          <CustomHeader opened={opened} setOpened={setOpened} />
        </Header>
      }
    >
      <Outlet />
    </AppShell>
  );
}
