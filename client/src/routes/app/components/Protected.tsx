import { signedInAtom } from "@/state";
import { useAtom } from "jotai";
import { Navigate } from "react-router-dom";
type ProtectedProps = {
  children: JSX.Element;
};
function Protected(props: ProtectedProps) {
  const [isSignedIn] = useAtom(signedInAtom);

  if (!isSignedIn) {
    return <Navigate to="/login" replace />;
  }
  return props.children;
}
export default Protected;
