import { Burger, MediaQuery, Text, useMantineTheme } from "@mantine/core";

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
      <MediaQuery largerThan="sm" styles={{ display: "none" }}>
        <Burger
          opened={props.opened}
          onClick={() => props.setOpened((o) => !o)}
          size="sm"
          color={theme.colors.gray[6]}
          mr="xl"
        />
      </MediaQuery>

      <Text>dap</Text>
    </div>
  );
}
