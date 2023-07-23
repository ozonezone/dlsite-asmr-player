import { SERVER_HOST, SERVER_PROTOCOL } from "@/const";
import { authAtom } from "@/state";
import { useAtomValue } from "jotai";

export function useStreamUrl() {
  const token = useAtomValue(authAtom)!;

  return (productId: string, path: string[]) => {
    return `${SERVER_PROTOCOL}//${SERVER_HOST}/stream/${productId}/${
      path.map((path) => encodeURIComponent(path)).join("/")
    }?token=${token}`;
  };
}
