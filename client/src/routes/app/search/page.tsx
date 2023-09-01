import { ItemCard } from "@/components/ItemCard";
import { Skeleton } from "@/components/Skeleton";
import { rspc } from "@/state";
import { Button, TextInput } from "@mantine/core";
import { useState } from "react";
import { StringParam, useQueryParam } from "use-query-params";

export default function Page() {
  const [query, setQuery] = useQueryParam("q", StringParam);
  const [input, setInput] = useState(query ?? "");

  return (
    <div className="flex flex-col gap-2">
      <form
        className="flex flex-row gap-2"
        onSubmit={(e) => e.preventDefault()}
      >
        <TextInput
          className="flex-grow"
          value={input}
          onChange={(e) => {
            setInput(e.target.value);
          }}
        />
        <Button
          type="submit"
          onClick={() => {
            setQuery(input);
          }}
        >
          Search
        </Button>
      </form>
      {query ? <SearchResult query={query} /> : "search"}
    </div>
  );
}

function SearchResult({ query }: { query: string }) {
  const { data } = rspc.useQuery(["product.search", query]);

  return data
    ? (
      <div className="flex flex-col">
        <span>{data.length} items</span>
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
          {data.map((product) => <ItemCard product={product} />)}
        </div>
      </div>
    )
    : <Skeleton />;
}
