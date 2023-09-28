import { Burger, Title, useMantineTheme } from "@mantine/core";
import { Link } from "react-router-dom";

export function Header(
  props: {
    opened: boolean;
    setOpened: React.Dispatch<React.SetStateAction<boolean>>;
  },
) {
  const theme = useMantineTheme();

  return (
    <div
      style={{ display: "flex", alignItems: "center", height: "100%" }}
    >
      <Burger
        opened={props.opened}
        onClick={() => props.setOpened((o) => !o)}
        size="sm"
        color={theme.colors.gray[6]}
        mr="xl"
      />

      <Link to={`/`}>
        <Title order={3}>DAPlayer</Title>
      </Link>
    </div>
  );
}
