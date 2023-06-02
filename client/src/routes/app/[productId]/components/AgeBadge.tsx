import { Age } from "@/bindings/bindings";
import { Badge } from "@mantine/core";

export function AgeBadge(props: { age: Age }) {
  if (props.age === "All") {
    return (
      <Badge variant="filled" color="blue">
        全年齢
      </Badge>
    );
  } else if (props.age === "R") {
    return (
      <Badge variant="filled" color="orange">
        R指定
      </Badge>
    );
  } else {
    return (
      <Badge variant="filled" color="red">
        R-18
      </Badge>
    );
  }
}
