import { ProductSortOrder, RemoteProductSortType } from "@/bindings/bindings";
import { RemoteItemCard } from "@/components/RemoteItemCard";
import { Skeleton } from "@/components/Skeleton";
import { rspc } from "@/pages/_state";
import { ArrowPathIcon } from "@heroicons/react/24/solid";
import { ActionIcon, Pagination } from "@mantine/core";

export function RemoteSearch(props: {
  limit: number;
  page: number;
  sortType: RemoteProductSortType;
  sortOrder: ProductSortOrder;
  query: string;
  setPage: (e: number) => void;
}) {
  const { data, refetch, isRefetching } = rspc.useQuery(["remote.search", {
    limit: props.limit,
    page: props.page,
    sort_type: props.sortType,
    sort_order: props.sortOrder,
    query: props.query ?? "",
  }]);
  const totalPage = data ? (data.count / props.limit + 1) : null;

  return data && totalPage && !isRefetching
    ? (
      <div className="flex flex-col gap-2 pt-2">
        <div className="flex flex-row gap-3 w-full items-center justify-center">
          <Pagination
            total={totalPage}
            value={props.page}
            onChange={props.setPage}
          />
          <div>{data.products.length} / {data.count} items</div>
          <ActionIcon
            onClick={() => {
              refetch();
            }}
          >
            <ArrowPathIcon />
          </ActionIcon>
        </div>
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
          {data.products.map((data) => {
            return <RemoteItemCard product={data} key={data.id} />;
          })}
        </div>

        <div className="flex flex-row gap-3 justify-center">
          <Pagination
            total={totalPage}
            value={props.page}
            onChange={props.setPage}
          />
          {data ? <div>{data.count} items</div> : null}
        </div>
      </div>
    )
    : <Skeleton />;
}
