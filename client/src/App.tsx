import { MantineProvider } from "@mantine/core";
import { useAtom } from "jotai";
import { Notifications } from "@mantine/notifications";
import { Routes } from "@generouted/react-router";

import { clientAtom, queryClient, rspc } from "@/pages/_state";

import "./index.css";
import "@mantine/core/styles.css";

function App() {
  const [client] = useAtom(clientAtom);
  return (
    <>
      <MantineProvider>
        <Notifications />
        <rspc.Provider client={client} queryClient={queryClient}>
          <Routes />
        </rspc.Provider>
      </MantineProvider>
    </>
  );
}

export default App;
