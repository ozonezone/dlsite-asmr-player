import { Skeleton as MantineSkeleton } from "@mantine/core";

export function Skeleton() {
  return (
    <div>
      <MantineSkeleton height={8} mt={6} radius="xl" />
      <MantineSkeleton height={8} mt={6} radius="xl" />
      <MantineSkeleton height={8} mt={6} radius="xl" />
    </div>
  );
}
