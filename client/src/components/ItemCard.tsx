import { Link } from "react-router-dom";
import { ProductDetailed } from "@/bindings/bindings";
import { Image, Text } from "@mantine/core";
import { AgeBadge } from "./AgeBadge";

export function ItemCard(
  { product }: { product: ProductDetailed },
) {
  return (
    <div className="flex flex-col drop-shadow-lg rounded-b-md bg-white">
      <Link to={`/product/${product.id}`}>
        <Image
          src={product.images[0]}
          className="pb-1"
          h={200}
        />
        <div className="text-lg px-1">{product.title}</div>
      </Link>

      <div className="flex flex-col gap-2 pb-2 px-1">
        <p className="text-gray-500">{product.circle.name}</p>
        <AgeBadge age={product.age} />
        <div>{(new Date(product.released_at)).toLocaleDateString()}</div>
      </div>
    </div>
  );
}
