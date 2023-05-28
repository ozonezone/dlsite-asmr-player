import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import { clientAtom, queryClient, rspc } from "./state";
import { useAtom } from "jotai";

import { router as rootRouter } from "./routes/route";
import { router as loginRouter } from "./routes/login/route";
import { router as appRouter } from "./routes/app/route";

const router = createBrowserRouter([
  rootRouter,
  loginRouter,
  appRouter,
]);

function App() {
  const [client] = useAtom(clientAtom);

  return (
    <>
      <rspc.Provider client={client} queryClient={queryClient}>
        <RouterProvider router={router} />
      </rspc.Provider>
    </>
  );
}

export default App;
