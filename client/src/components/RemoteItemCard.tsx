import { RemoteSearchResponseItem } from "@/bindings/bindings";
import { Image } from "@mantine/core";
import { AgeBadge } from "./AgeBadge";

export function RemoteItemCard(
  { product }: { product: RemoteSearchResponseItem },
) {
  return (
    <div className="flex flex-col drop-shadow-lg rounded-b-md bg-white">
      <a
        href={`https://www.dlsite.com/maniax/work/=/product_id/${product.id}.html`}
        target="_blank"
        referrerPolicy="no-referrer"
      >
        <Image
          src={product.thumbnail_url}
          className="pb-1"
          h={200}
        />
        <div className="text-lg px-1">{product.title}</div>
      </a>

      <div className="flex flex-col pb-2 px-1">
        <p className="text-gray-500">{product.circle_name}</p>
        <div>販売数: {product.dl_count}</div>
        <AgeBadge age={product.age_category} />
      </div>
    </div>
  );
}
