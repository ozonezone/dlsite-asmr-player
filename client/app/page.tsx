"use client";
import { QueryClient } from "@tanstack/react-query";
import { createClient, FetchTransport, WebsocketTransport } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";

import type { Procedures } from "@/bindings/bindings"; // These were the bindings exported from your Rust code!

// You must provide the generated types as a generic and create a transport (in this example we are using HTTP Fetch) so that the client knows how to communicate with your API.
const client = createClient<Procedures>({
  // Refer to the integration your using for the correct transport.
  transport: new WebsocketTransport(`ws://localhost:4000/rspc/ws`),
});

const queryClient = new QueryClient();
const rspc = createReactQueryHooks<Procedures>();

function SomeComponent() {
  const { data, isLoading, error } = rspc.useQuery(["hello"]);

  return (
    <>
      <p>{data}</p>
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
