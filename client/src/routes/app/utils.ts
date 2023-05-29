import { SERVER_HOST } from "@/const";
import { authAtom } from "@/state";
import { useAtomValue } from "jotai";

export function useStreamUrl() {
  const token = useAtomValue(authAtom)!;

  return (productId: string, path: string[]) => {
    return `http://${SERVER_HOST}/stream/${productId}/${
      path.join("/")
    }?token=${token}`;
  };
}
