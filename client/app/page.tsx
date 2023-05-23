"use client";
import { QueryClient } from "@tanstack/react-query";
import { createClient, FetchTransport, WebsocketTransport } from "@rspc/client";
import { createReactQueryHooks } from "@rspc/react";

import type { Procedures } from "@/bindings/bindings"; // These were the bindings exported from your Rust code!
import { useState } from "react";

const client = createClient<Procedures>({
  transport: new WebsocketTransport(
    `ws://localhost:4000/rspc/ws?token=password`,
  ),
});

const queryClient = new QueryClient();
const rspc = createReactQueryHooks<Procedures>();

function SomeComponent() {
  const { data: ping, isLoading, error } = rspc.useQuery(["ping"]);

  const { mutate: mutateScandir } = rspc.useMutation(["config.setScandir"]);
  const { mutate: startScan } = rspc.useMutation(["scan.start"]);

  const [pathForm, setPathForm] = useState("");

  return (
    <div className="flex flex-col">
      <p>ping: {ping}</p>
      <div className="flex flex-col">
        <input
          className="border border-gray-400"
          type="text"
          value={pathForm}
          onChange={(e) => setPathForm(e.target.value)}
        />
        <button
          onClick={() => {
            mutateScandir([pathForm]);
          }}
        >
          Set path
        </button>
      </div>
      <button
        onClick={() => {
          startScan(undefined);
        }}
      >
        Start scan
      </button>
    </div>
  );
}

export default function App() {
  return (
    <rspc.Provider client={client} queryClient={queryClient}>
      <SomeComponent />
    </rspc.Provider>
  );
}
