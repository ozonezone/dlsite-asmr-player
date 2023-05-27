import { Spinner } from "@/components/ui/Spinner";
import { rspc } from "@/state";

export function Root() {
  const { data, isLoading, error } = rspc.useQuery(["product.browse", {
    limit: 10,
    page: 1,
    sort_type: "Date",
    sort_order: "Asc",
  }]);

  return isLoading || !data ? <Spinner /> : (
    <>
      {data[0].map((data) => {
        return <div>{data.id}</div>;
      })}
    </>
  );
}
