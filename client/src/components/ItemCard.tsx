import { Link } from "react-router-dom";
import { ProductDetailed } from "@/bindings/bindings";
import { Image, Text } from "@mantine/core";
import { AgeBadge } from "@/routes/app/[productId]/_components/AgeBadge";

export function ItemCard(
  { product }: { product: ProductDetailed },
) {
  return (
    <div className="flex flex-col drop-shadow-md bg-white">
      <Link to={`/app/product/${product.id}`}>
        <Image
          src={product.images[0]}
          className="pb-1"
        />
        <Text weight={500}>{product.title}</Text>
      </Link>

      <div className="flex flex-wrap text-sm gap-2">
        <AgeBadge age={product.age} />
        <div>{product.released_at}</div>
        <div>{product.circle.name}</div>
      </div>
    </div>
  );
}
