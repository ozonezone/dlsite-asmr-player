import { useAtom, useSetAtom } from "jotai";
import { authAtom, rspc, signedInAtom } from "./state";
import { Navigate } from "react-router-dom";
import { Spinner } from "./components/ui/Spinner";

export function Root() {
  const auth = localStorage.getItem("auth");

  if (!auth) {
    console.log(auth);
    return <Navigate to="/login" replace />;
  }

  return <CheckSignedIn />;
}

function CheckSignedIn() {
  const { data, isLoading, error } = rspc.useQuery(["ping_auth"]);
  const setSignedIn = useSetAtom(signedInAtom);

  if (data && !error) {
    setSignedIn(true);
  }

  return isLoading
    ? <Spinner />
    : error
    ? <Navigate to="/login" replace />
    : <Navigate to="/app" replace />;
}
