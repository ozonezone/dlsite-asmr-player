// Global state that is used in anywhere in the app (including unauthorized pages)

import { atomWithStorage, selectAtom } from "jotai/utils";
import { QueryCache, QueryClient } from "@tanstack/react-query";
import { createReactQueryHooks } from "@rspc/react";
import { createClient, WebsocketTransport } from "@rspc/client";
import type { Procedures } from "@/bindings/bindings";
import { atom } from "jotai";
import { SERVER_HOST } from "./const";

export const authAtom = atomWithStorage<null | string>(
  "auth",
  JSON.parse(localStorage.getItem("auth") ?? "null"),
);
export const signedInAtom = atom(false);

export const queryClient = new QueryClient();
export const rspc = createReactQueryHooks<Procedures>();

export const clientAtom = selectAtom(
  authAtom,
  (auth) => {
    const client = createClient<Procedures>({
      transport: new WebsocketTransport(
        `ws://${SERVER_HOST}/rspc/ws${auth ? "?token=" + auth : ""}`,
      ),
    });
    return client;
  },
);