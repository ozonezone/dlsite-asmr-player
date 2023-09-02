import { Link } from "react-router-dom";
import { RemoteSearchResponseItem } from "@/bindings/bindings";
import { Image, Text } from "@mantine/core";
import { AgeBadge } from "@/routes/app/[productId]/_components/AgeBadge";

export function RemoteItemCard(
  { product }: { product: RemoteSearchResponseItem },
) {
  return (
    <div className="flex flex-col drop-shadow-md bg-white">
      <a
        href={`https://www.dlsite.com/maniax/work/=/product_id/${product.id}.html`}
        target="_blank"
        referrerPolicy="no-referrer"
      >
        <Image
          src={product.thumbnail_url}
          className="pb-1"
        />
        <Text weight={500}>{product.title}</Text>
      </a>

      <div className="flex flex-wrap text-sm gap-2">
        <AgeBadge age={product.age_category} />
        <div>{product.circle_name}</div>
      </div>
    </div>
  );
}
