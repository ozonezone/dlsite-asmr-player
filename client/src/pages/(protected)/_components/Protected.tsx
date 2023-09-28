import { rspc, tokenAtom } from "@/pages/_state";
import { Loader } from "@mantine/core";
import { useAtom } from "jotai";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
type ProtectedProps = {
  children: JSX.Element;
};
function Protected(props: ProtectedProps) {
  const { data, isLoading, error } = rspc.useQuery(["ping_auth"]);
  const [token] = useAtom(tokenAtom);
  const navigate = useNavigate();

  useEffect(() => {
    if (!token) {
      navigate("/login", { replace: true });
    }
    if (!isLoading && error) {
      localStorage.removeItem("token");
      navigate("/login", { replace: true });
    }
  }, [token, error]);

  if (isLoading) {
    return (
      <div className="h-screen w-screen flex items-center justify-center flex-col gap-1">
        <Loader />
        <p>Loading app...</p>
      </div>
    );
  } else if (data && !error) {
    return props.children;
  } else {
    return <div>Redirecting...</div>;
  }
}
export default Protected;
