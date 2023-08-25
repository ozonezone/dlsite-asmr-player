import { SERVER_HOST, SERVER_PROTOCOL } from "@/const";
import { tokenAtom } from "@/state";
import { useAtomValue } from "jotai";

export function useStreamUrl() {
  const token = useAtomValue(tokenAtom)!;

  return (productId: string, path: string[]) => {
    return `${SERVER_PROTOCOL}//${SERVER_HOST}/stream/${productId}/${
      path.map((path) => encodeURIComponent(path)).join("/")
    }?token=${token}`;
  };
}
