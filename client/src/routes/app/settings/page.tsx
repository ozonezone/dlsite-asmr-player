import { Config } from "@/bindings/bindings";
import { rspc } from "@/state";
import {
  Button,
  Divider,
  Loader,
  SimpleGrid,
  Skeleton,
  TextInput,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { notifications } from "@mantine/notifications";
import { useEffect } from "react";

export default function Page() {
  const { data: config, error } = rspc.useQuery(["config.getConfig"]);

  useEffect(() => {
    if (error) {
      notifications.show({
        title: "Error",
        message: "Could not get config: " + error,
        color: "red",
      });
    }
  }, [error]);

  return config ? <PageInner config={config} /> : <Skeleton />;
}

function PageInner(
  props: { config: Config },
) {
  const form = useForm({
    initialValues: props.config,
  });
  const { mutateAsync: setConfig, isLoading, error } = rspc.useMutation([
    "config.setConfig",
  ]);

  useEffect(() => {
    if (error) {
      notifications.show({
        title: "Error",
        color: "red",
        message: "Could not update config: " + error.message,
      });
    }
  }, [error]);

  return (
    <div>
      <form
        onSubmit={form.onSubmit(async (values) => {
          await setConfig(values);
        })}
      >
        <Divider my="sm" />
        <p className="text-xl mb-2">Scan</p>
        <SimpleGrid
          cols={3}
          spacing="lg"
          breakpoints={[
            { maxWidth: "md", cols: 2, spacing: "md" },
            { maxWidth: "sm", cols: 1, spacing: "sm" },
          ]}
        >
          <TextInput
            withAsterisk
            required
            label="Scan directory"
            {...form.getInputProps("scan_dir.0")}
          />
        </SimpleGrid>
        <Divider my="sm" />
        <Button variant="filled" disabled={isLoading} type="submit">
          {isLoading ? <Loader size="sm" /> : "Submit"}
        </Button>
      </form>
    </div>
  );
}
