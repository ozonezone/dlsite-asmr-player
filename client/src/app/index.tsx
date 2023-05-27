import Protected from "@/components/Protected";
import { ProtectedApp } from "./App";
import { Root } from "./Root";
import { RouteObject } from "react-router-dom";

export const router: RouteObject = {
  path: "/app",
  element: (
    <Protected>
      <ProtectedApp />
    </Protected>
  ),
  children: [
    {
      path: "/app",
      element: <Root />,
    },
  ],
};
