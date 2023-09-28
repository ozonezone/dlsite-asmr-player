import { Config } from "@/bindings/bindings";
import { Skeleton } from "@/components/Skeleton";
import { rspc } from "@/pages/_state";
import {
  Button,
  Divider,
  Loader,
  SimpleGrid,
  TextInput,
  Title,
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

  const passwordForm = useForm({
    initialValues: {
      password: "",
      newPassword: "",
      newPasswordConfirm: "",
    },
    validate: {
      newPassword: (v) => v == "" ? "Password cannot be empty" : null,
      newPasswordConfirm: (v, values) =>
        v != values.newPassword ? "Passwords do not match" : null,
    },
  });

  const { mutateAsync: setConfig, isLoading } = rspc.useMutation([
    "config.setConfig",
  ]);

  const {
    mutateAsync: setPassword,
    isLoading: isSetPasswordLoading,
  } = rspc.useMutation([
    "config.setPassword",
  ]);

  return (
    <div>
      <Title order={2} className="py-2">General config</Title>
      <form
        onSubmit={form.onSubmit(async (values) => {
          try {
            await setConfig(values);
          } catch (e) {
            notifications.show({
              title: "Error",
              color: "red",
              message: "Could not set password: " + e,
            });
            notifications.show({
              title: "Success",
              message: "Config updated",
            });
          }
        })}
      >
        <p className="text-xl mb-2">Scan</p>
        <SimpleGrid
          cols={{
            sm: 1,
            md: 2,
          }}
          spacing={{
            sm: "sm",
            md: "md",
          }}
        >
          <TextInput
            withAsterisk
            required
            label="Scan directory"
            {...form.getInputProps("scan_dir.0")}
          />
        </SimpleGrid>
        <Button
          variant="filled"
          disabled={isLoading}
          type="submit"
          className="mt-4"
        >
          {isLoading ? <Loader size="sm" /> : "Save"}
        </Button>
      </form>

      <Divider my="sm" />

      <Title order={2} className="py-2">Danger zone</Title>
      <form
        onSubmit={passwordForm.onSubmit(async (values) => {
          try {
            await setPassword({
              password: values.password,
              new_password: values.newPassword,
            });
            notifications.show({
              title: "Success",
              message: "Password updated",
            });
          } catch (e) {
            notifications.show({
              title: "Error",
              color: "red",
              message: "Could not change password: " + e,
            });
          }
        })}
      >
        <Title order={4}>Change password</Title>
        <SimpleGrid
          cols={1}
          spacing="lg"
        >
          <TextInput
            type="password"
            label="Current password"
            {...passwordForm.getInputProps("password")}
          />
          <TextInput
            type="password"
            label="New password"
            {...passwordForm.getInputProps("newPassword")}
          />
          <TextInput
            type="password"
            label="Confirm new password"
            {...passwordForm.getInputProps("newPasswordConfirm")}
          />
        </SimpleGrid>
        <Button
          variant="filled"
          disabled={isSetPasswordLoading}
          type="submit"
          className="mt-4"
        >
          {isSetPasswordLoading ? <Loader size="sm" /> : "Save"}
        </Button>
      </form>
    </div>
  );
}
