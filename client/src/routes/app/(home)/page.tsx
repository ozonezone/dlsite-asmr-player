import { Button, NativeSelect, Tabs, TextInput } from "@mantine/core";
import {
  createEnumParam,
  NumberParam,
  StringParam,
  useQueryParam,
  withDefault,
} from "use-query-params";
import { useState } from "react";
import {
  ProductSortOrder,
  ProductSortType,
  RemoteProductSortType,
} from "@/bindings/bindings";
import { LocalSearch } from "./LocalSearch";
import { RemoteSearch } from "./RemoteSearch";

const sortTypeEnum = ["CreatedAt", "Name", "ReleasedAt"];
const remoteSortTypeEnum = ["ReleasedAt", "Trend", "Download"];

const PageParam = withDefault(NumberParam, 1);
const SortOrderParam = withDefault(createEnumParam(["Desc", "Asc"]), "Desc");
const SortTypeParam = withDefault(
  createEnumParam(sortTypeEnum),
  "ReleasedAt",
);
const RemoteSortTypeParam = withDefault(
  createEnumParam(remoteSortTypeEnum),
  "Trend",
);

export default function Page() {
  const [page, setPage] = useQueryParam("page", PageParam);
  const [sortOrder, setSortOrder] = useQueryParam("order", SortOrderParam);
  const [sortType, setSortType] = useQueryParam("sortType", SortTypeParam);
  const [remoteSortType, setRemoteSortType] = useQueryParam(
    "remoteSortType",
    RemoteSortTypeParam,
  );

  const [query, setQuery] = useQueryParam("q", StringParam);
  const [input, setInput] = useState(query ?? "");

  const [activeTab, setActiveTab] = useState<string | null>("local");

  const limit = 50;

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
        {activeTab === "remote"
          ? (
            <NativeSelect
              data={remoteSortTypeEnum}
              value={remoteSortType}
              onChange={(e) => {
                setRemoteSortType(e.currentTarget.value);
                setPage(1);
              }}
            />
          )
          : (
            <NativeSelect
              data={sortTypeEnum}
              value={sortType}
              onChange={(e) => {
                setSortType(e.currentTarget.value);
                setPage(1);
              }}
            />
          )}
        <NativeSelect
          data={["Desc", "Asc"]}
          value={sortOrder}
          onChange={(e) => {
            setSortOrder(e.currentTarget.value as "Desc" | "Asc");
            setPage(1);
          }}
        />
        <Button
          type="submit"
          onClick={() => {
            setQuery(input);
            setPage(1);
          }}
        >
          Search
        </Button>
      </form>
      <Tabs
        value={activeTab}
        onTabChange={setActiveTab}
        className="bg-white px-3"
      >
        <Tabs.List className="pb-2">
          <Tabs.Tab value="local">Local</Tabs.Tab>
          <Tabs.Tab value="remote">
            Remote
          </Tabs.Tab>
        </Tabs.List>

        <Tabs.Panel value="local">
          <LocalSearch
            limit={limit}
            page={page}
            sortType={sortType as ProductSortType}
            sortOrder={sortOrder as ProductSortOrder}
            query={query ?? ""}
            setPage={(e) => setPage(e)}
          />
        </Tabs.Panel>
        <Tabs.Panel value="remote">
          <RemoteSearch
            limit={limit}
            page={page}
            sortType={remoteSortType as RemoteProductSortType}
            sortOrder={sortOrder as ProductSortOrder}
            query={query ?? ""}
            setPage={(e) => setPage(e)}
          />
        </Tabs.Panel>
      </Tabs>
    </div>
  );
}
