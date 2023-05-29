import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import {
  ColorScheme,
  ColorSchemeProvider,
  createEmotionCache,
  MantineProvider,
} from "@mantine/core";
import { useAtom } from "jotai";
import { useState } from "react";
import { useColorScheme } from "@mantine/hooks";
import { Notifications } from "@mantine/notifications";
import { QueryParamProvider } from "use-query-params";
import { ReactRouter6Adapter } from "use-query-params/adapters/react-router-6";

import { router as rootRouter } from "./routes/route";
import { router as loginRouter } from "./routes/login/route";
import { router as appRouter } from "./routes/app/route";
import { clientAtom, queryClient, rspc } from "./state";

const router = createBrowserRouter([
  rootRouter,
  loginRouter,
  appRouter,
]);

const appendCache = createEmotionCache({ key: "mantine", prepend: false });

function App() {
  const [client] = useAtom(clientAtom);
  const [colorScheme, toggleColorScheme] = useColorSchemeCustom();

  return (
    <>
      <ColorSchemeProvider
        colorScheme={colorScheme}
        toggleColorScheme={toggleColorScheme}
      >
        <MantineProvider
          emotionCache={appendCache}
          withGlobalStyles
          withNormalizeCSS
          theme={{
            colorScheme: colorScheme,
          }}
        >
          <Notifications />
          <rspc.Provider client={client} queryClient={queryClient}>
            <RouterProvider router={router} />
          </rspc.Provider>
        </MantineProvider>
      </ColorSchemeProvider>
    </>
  );
}

export default App;

const reverseColorScheme = (colorScheme: ColorScheme): ColorScheme =>
  colorScheme === "dark" ? "light" : "dark";
function useColorSchemeCustom() {
  const [colorScheme, setColorScheme] = useState<ColorScheme | null>(null);
  const systemColorScheme = useColorScheme();
  const toggleColorScheme = (value?: ColorScheme) => {
    const newColorScheme = value ??
      (colorScheme
        ? reverseColorScheme(colorScheme)
        : reverseColorScheme(systemColorScheme));
    setColorScheme(newColorScheme);
  };

  return [colorScheme ?? systemColorScheme, toggleColorScheme] as const;
}
