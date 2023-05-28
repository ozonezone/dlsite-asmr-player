import { useSetAtom } from "jotai";
import { Navigate } from "react-router-dom";

export function Root() {
  return <Navigate to="/app" />;
}
