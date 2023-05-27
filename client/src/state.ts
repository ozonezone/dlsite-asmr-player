import { atomWithStorage, selectAtom } from "jotai/utils";
import { QueryClient } from "@tanstack/react-query";
import { createReactQueryHooks } from "@rspc/react";
import { createClient, WebsocketTransport } from "@rspc/client";
import type { Procedures } from "@/bindings/bindings";
import { atom } from "jotai";

export const authAtom = atomWithStorage<null | string>("auth", null);
export const signedInAtom = atom(false);

export const queryClient = new QueryClient();
export const rspc = createReactQueryHooks<Procedures>();

export const clientAtom = selectAtom(
  authAtom,
  (auth) =>
    createClient<Procedures>({
      transport: new WebsocketTransport(
        `ws://${location.hostname}:14567/rspc/ws${
          auth ? "?token=" + auth : ""
        }`,
      ),
    }),
);
