import { Burger, Title } from "@mantine/core";
import { Link } from "react-router-dom";

export function Header(
  props: {
    opened: boolean;
    toggle: () => void;
  },
) {
  return (
    <div className="flex items-center h-full">
      <Burger
        opened={props.opened}
        onClick={props.toggle}
        size="sm"
        className="mr-2 ml-2"
      />

      <Link to="/">
        <Title order={3}>DAPlayer</Title>
      </Link>
    </div>
  );
}
