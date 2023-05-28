import { useSetAtom } from "jotai";
import { rspc, signedInAtom } from "@/state";
import { Navigate } from "react-router-dom";
import { Loader } from "@mantine/core";

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
    ? (
      <div className="h-screen w-screen flex items-center justify-center flex-col gap-1">
        <Loader />
        <p>Loading app...</p>
      </div>
    )
    : error
    ? <Navigate to="/login" replace />
    : <Navigate to="/app" replace />;
}
