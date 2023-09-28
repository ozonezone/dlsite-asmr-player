import { ProductSortOrder, ProductSortType } from "@/bindings/bindings";
import { ItemCard } from "@/components/ItemCard";
import { rspc } from "@/pages/_state";
import { ArrowPathIcon } from "@heroicons/react/24/solid";
import { ActionIcon, Pagination, Skeleton } from "@mantine/core";

export function LocalSearch(props: {
  limit: number;
  page: number;
  sortType: ProductSortType;
  sortOrder: ProductSortOrder;
  query: string;
  setPage: (e: number) => void;
}) {
  const { data, refetch, isRefetching } = rspc.useQuery(["product.browse", {
    limit: props.limit,
    page: props.page,
    sort_type: props.sortType as ProductSortType,
    sort_order: props.sortOrder as ProductSortOrder,
    query: props.query ?? "",
  }]);
  const totalPage = data ? (data[1] / props.limit + 1) : null;

  return data && totalPage && !isRefetching
    ? (
      <div className="flex flex-col gap-2 pt-2">
        <div className="flex flex-row gap-3 w-full items-center justify-center">
          <Pagination
            total={totalPage}
            value={props.page}
            onChange={props.setPage}
          />
          <div>{data[0].length} / {data[1]} items</div>
          <ActionIcon
            onClick={() => {
              refetch();
            }}
          >
            <ArrowPathIcon />
          </ActionIcon>
        </div>
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-2">
          {data[0].map((data) => {
            return <ItemCard product={data} key={data.id} />;
          })}
        </div>

        <div className="flex flex-row gap-3 justify-center">
          <Pagination
            total={totalPage}
            value={props.page}
            onChange={props.setPage}
          />
          {data ? <div>{data[1]} items</div> : null}
        </div>
      </div>
    )
    : <Skeleton />;
}
