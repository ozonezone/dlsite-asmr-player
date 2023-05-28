import { rspc } from "@/state";
import { Button, Loader } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { useEffect } from "react";

export default function Page() {
  const { mutateAsync: startScan, isLoading, error } = rspc.useMutation([
    "scan.start",
  ]);

  useEffect(() => {
    if (error) {
      notifications.show({
        title: "Error",
        color: "red",
        message: "Scan failed: " + error.message,
      });
    }
  }, [error]);

  return (
    <div className="flex flex-col gap-2">
      <Button
        variant="filled"
        disabled={isLoading}
        type="button"
        onClick={async () => {
          await startScan(false);
        }}
      >
        {isLoading ? <Loader size="sm" /> : "Start scan"}
      </Button>
      <Button
        variant="filled"
        disabled={isLoading}
        type="button"
        onClick={async () => {
          await startScan(true);
        }}
      >
        {isLoading ? <Loader size="sm" /> : "Force scan"}
      </Button>
    </div>
  );
}
