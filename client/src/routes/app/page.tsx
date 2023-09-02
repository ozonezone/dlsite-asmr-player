import { SortOrder, SortType } from "@/bindings/bindings";
import { rspc } from "@/state";
import { Button, NativeSelect, Pagination, TextInput } from "@mantine/core";
import { Skeleton } from "@/components/Skeleton";
import {
  createEnumParam,
  NumberParam,
  StringParam,
  useQueryParam,
  withDefault,
} from "use-query-params";
import { ItemCard } from "@/components/ItemCard";
import { useState } from "react";

const PageParam = withDefault(NumberParam, 1);
const SortOrderParam = withDefault(createEnumParam(["Desc", "Asc"]), "Desc");
const SortTypeParam = withDefault(createEnumParam(["Date", "Name"]), "Date");

export default function Page() {
  const [page, setPage] = useQueryParam("page", PageParam);
  const [sortOrder, setSortOrder] = useQueryParam("order", SortOrderParam);
  const [sortType, setSortType] = useQueryParam("sortType", SortTypeParam);
  const [query, setQuery] = useQueryParam("q", StringParam);
  const [input, setInput] = useState(query ?? "");

  const limit = 50;

  const { data } = rspc.useQuery(["product.browse", {
    limit,
    page: page,
    sort_type: sortType as SortType,
    sort_order: sortOrder as SortOrder,
    query: query ?? "",
  }]);
  const totalPage = data ? (data[1] / limit + 1) : null;

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
        <NativeSelect
          data={["Date", "Name"]}
          value={sortType}
          onChange={(e) => {
            setSortType(e.currentTarget.value as "Date" | "Name");
          }}
        />
        <NativeSelect
          data={["Desc", "Asc"]}
          value={sortOrder}
          onChange={(e) => {
            setSortOrder(e.currentTarget.value as "Desc" | "Asc");
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
      {totalPage
        ? (
          <div className="flex flex-row gap-3">
            <Pagination
              total={totalPage}
              value={page}
              onChange={(e) => setPage(e)}
            />
            {data ? <div>{data[0].length} / {data[1]} items</div> : null}
          </div>
        )
        : null}
      {data
        ? (
          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
            {data[0].map((data) => {
              return <ItemCard product={data} key={data.id} />;
            })}
          </div>
        )
        : <Skeleton />}
      {totalPage && data
        ? (
          <div className="flex flex-row gap-3">
            <Pagination
              total={totalPage}
              value={page}
              onChange={(e) => setPage(e)}
            />
            {data ? <div>{data[1]} items</div> : null}
          </div>
        )
        : null}
    </div>
  );
}
