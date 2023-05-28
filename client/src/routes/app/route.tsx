import { RouteObject } from "react-router-dom";
import { default as AppPage } from "./page";
import { default as AppRootPage } from "./_root/page";
import { default as ProductPage } from "./[productId]/page";
import Protected from "./components/Protected";

export const router: RouteObject = {
  path: "/app",
  element: (
    <Protected>
      <AppPage />
    </Protected>
  ),
  children: [
    {
      path: "/app",
      element: <AppRootPage />,
    },
    {
      path: "/app/product/:productId",
      element: <ProductPage />,
    },
  ],
};
