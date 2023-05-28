import { rspc } from "@/state";
import { Skeleton } from "@mantine/core";
import { useState } from "react";
import { Link } from "react-router-dom";

export default function Page() {
  const { data, isLoading, error } = rspc.useQuery(["product.browse", {
    limit: 50,
    page: 1,
    sort_type: "Date",
    sort_order: "Asc",
  }]);

  return isLoading || !data
    ? <Skeleton />
    : (
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-2">
        {data[0].map((data) => {
          return (
            <div key={data.id}>
              <div className="max-w-sm bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
                <Link to={`/app/product/${data.id}`}>
                  {data.remote_image.length > 0
                    ? (
                      <img
                        className="rounded-t-lg"
                        src={data.remote_image[0]}
                        alt=""
                      />
                    )
                    : <></>}
                </Link>
                <div className="p-5">
                  <Link to={`/app/product/${data.id}`}>
                    <h6 className="mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white">
                      {data.name}
                    </h6>
                  </Link>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    );
}
