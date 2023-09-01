import { SortOrder, SortType } from "@/bindings/bindings";
import { rspc } from "@/state";
import { NativeSelect, Pagination } from "@mantine/core";
import { Skeleton } from "@/components/Skeleton";
import {
  createEnumParam,
  NumberParam,
  useQueryParam,
  withDefault,
} from "use-query-params";
import { ItemCard } from "@/components/ItemCard";

const PageParam = withDefault(NumberParam, 1);
const SortOrderParam = withDefault(createEnumParam(["Desc", "Asc"]), "Desc");
const SortTypeParam = withDefault(createEnumParam(["Date", "Name"]), "Date");

export default function Page() {
  const [page, setPage] = useQueryParam("page", PageParam);
  const [sortOrder, setSortOrder] = useQueryParam("order", SortOrderParam);
  const [sortType, setSortType] = useQueryParam("sortType", SortTypeParam);
  const limit = 50;

  const { data } = rspc.useQuery(["product.browse", {
    limit,
    page: page,
    sort_type: sortType as SortType,
    sort_order: sortOrder as SortOrder,
  }]);
  const totalPage = data ? (data[1] / limit + 1) : null;

  return (
    <div className="flex flex-col justify-center items-center gap-2">
      <div className="flex flex-row gap-3 justify-center items-center">
        <NativeSelect
          data={["Desc", "Asc"]}
          label="Sort order"
          value={sortOrder}
          onChange={(e) => {
            setSortOrder(e.currentTarget.value as "Desc" | "Asc");
          }}
        />
        <NativeSelect
          data={["Date", "Name"]}
          label="Sort type"
          value={sortType}
          onChange={(e) => {
            setSortType(e.currentTarget.value as "Date" | "Name");
          }}
        />
      </div>
      {totalPage
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
