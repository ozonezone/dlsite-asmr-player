import { ProductResult, SortOrder, SortType } from "@/bindings/bindings";
import { rspc } from "@/state";
import {
  Card,
  Group,
  Image,
  NativeSelect,
  Pagination,
  Text,
} from "@mantine/core";
import { Link } from "react-router-dom";
import { AgeBadge } from "../[productId]/components/AgeBadge";
import { Skeleton } from "@/components/Skeleton";
import {
  createEnumParam,
  NumberParam,
  useQueryParam,
  withDefault,
} from "use-query-params";

const PageParam = withDefault(NumberParam, 1);
const SortOrderParam = withDefault(createEnumParam(["Desc", "Asc"]), "Desc");
const SortTypeParam = withDefault(createEnumParam(["Date", "Name"]), "Date");

export default function Page() {
  const [page, setPage] = useQueryParam("page", PageParam);
  const [sortOrder, setSortOrder] = useQueryParam("order", SortOrderParam);
  const [sortType, setSortType] = useQueryParam("sortType", SortTypeParam);
  const limit = 50;

  const { data, isLoading, error } = rspc.useQuery(["product.browse", {
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
        {data ? <div>{data[1]} items</div> : null}
      </div>
      {totalPage
        ? (
          <Pagination
            total={totalPage}
            value={page}
            onChange={(e) => setPage(e)}
          />
        )
        : null}
      {data
        ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-2">
            {data[0].map((data) => {
              return <ItemCard product={data} key={data.id} />;
            })}
          </div>
        )
        : <Skeleton />}
    </div>
  );
}

function ItemCard({ product }: { product: ProductResult }) {
  return (
    <Link to={`/app/product/${product.id}`}>
      <Card shadow="sm" padding="lg" radius="md" withBorder>
        <Card.Section>
          <Image
            src={product.remote_image[0]}
            height={160}
          />
        </Card.Section>

        <Group position="apart" mt="md" mb="xs">
          <Text weight={500}>{product.name}</Text>
          <AgeBadge age={product.age} />
          <Text size="sm">{product.released_at}</Text>
          <Text size="sm">{product.circle_name}</Text>
        </Group>
      </Card>
    </Link>
  );
}
