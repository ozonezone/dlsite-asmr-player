import { createBrowserRouter, RouterProvider } from "react-router-dom";
import "./index.css";
import Protected from "./components/Protected";
import { clientAtom, queryClient, rspc } from "./state";
import { useAtom } from "jotai";

import { Root } from "./Root";
import { Login } from "./login/Login";
import { router as appRouter } from "./app";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
  },
  {
    path: "/login",
    element: <Login />,
  },
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
