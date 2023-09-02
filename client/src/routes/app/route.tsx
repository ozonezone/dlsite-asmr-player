import { RouteObject } from "react-router-dom";
import { default as AppPage } from "./(home)/page";
import { default as AppLayout } from "./layout";
import { default as ProductPage } from "./[productId]/page";
import { default as SettingsPage } from "./settings/page";
import { default as ScanPage } from "./scan/page";
import Protected from "./_components/Protected";
import { QueryParamProvider } from "use-query-params";
import { ReactRouter6Adapter } from "use-query-params/adapters/react-router-6";

export const router: RouteObject = {
  path: "/app",
  element: (
    <Protected>
      <QueryParamProvider
        adapter={ReactRouter6Adapter}
      >
        <AppLayout />
      </QueryParamProvider>
    </Protected>
  ),
  children: [
    {
      path: "/app",
      element: <AppPage />,
    },
    {
      path: "/app/settings",
      element: <SettingsPage />,
    },
    {
      path: "/app/scan",
      element: <ScanPage />,
    },
    {
      path: "/app/product/:productId",
      element: <ProductPage />,
    },
  ],
};
