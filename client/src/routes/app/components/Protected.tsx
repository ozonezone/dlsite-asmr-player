import { rspc, signedInAtom } from "@/state";
import { Loader } from "@mantine/core";
import { useSetAtom } from "jotai";
import { Navigate } from "react-router-dom";
type ProtectedProps = {
  children: JSX.Element;
};
function Protected(props: ProtectedProps) {
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
    : props.children;
}
export default Protected;
