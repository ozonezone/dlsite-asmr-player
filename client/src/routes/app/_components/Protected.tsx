import { rspc, signedInAtom } from "@/state";
import { Loader } from "@mantine/core";
import { useSetAtom } from "jotai";
import { useEffect } from "react";
import { Navigate, useNavigate } from "react-router-dom";
type ProtectedProps = {
  children: JSX.Element;
};
function Protected(props: ProtectedProps) {
  const { data, isLoading, error } = rspc.useQuery(["ping_auth"]);
  const setSignedIn = useSetAtom(signedInAtom);
  const navigate = useNavigate();

  useEffect(() => {
    if (error) {
      localStorage.removeItem("auth");
      navigate("/login", { replace: true });
    }
    if (data && !error) {
      setSignedIn(true);
    }
  }, [data, error]);

  if (isLoading) {
    return (
      <div className="h-screen w-screen flex items-center justify-center flex-col gap-1">
        <Loader />
        <p>Loading app...</p>
      </div>
    );
  } else {
    return props.children;
  }
}
export default Protected;
