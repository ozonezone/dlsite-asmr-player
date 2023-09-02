import clsx from "clsx";
import { Link as LinkRouter } from "react-router-dom";

export function Link(props: React.ComponentProps<typeof LinkRouter>) {
  return (
    <LinkRouter
      {...props}
      className={clsx(
        props.className,
        "text-blue-600 dark:text-blue-500 hover:underline",
      )}
    />
  );
}
