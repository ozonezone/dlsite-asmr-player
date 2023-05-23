"use client";
import { QueryClient } from "@tanstack/react-query";
import { createClient, FetchTransport, WebsocketTransport } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";

import type { Procedures } from "@/bindings/bindings"; // These were the bindings exported from your Rust code!

const client = createClient<Procedures>({
  transport: new WebsocketTransport(
    `ws://localhost:4000/rspc/ws?token=password`,
  ),
});

const queryClient = new QueryClient();
const rspc = createReactQueryHooks<Procedures>();

function SomeComponent() {
  const { data, isLoading, error } = rspc.useQuery(["ping"]);
  const { data: data2 } = rspc.useQuery(["ping_auth"]);

  return (
    <>
      <p>{data}</p>
      <p>{data2}</p>
    </>
  );
}

export default function App() {
  return (
    <rspc.Provider client={client} queryClient={queryClient}>
      <SomeComponent />
    </rspc.Provider>
  );
}
