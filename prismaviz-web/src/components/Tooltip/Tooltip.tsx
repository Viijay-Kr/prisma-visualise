import { Tooltip as MantineTooltip } from "@mantine/core";
export const Tooltip = (props: React.ComponentProps<typeof MantineTooltip>) => {
  return (
    <MantineTooltip
      {...props}
      arrowOffset={10}
      arrowSize={4}
      position="bottom-end"
      withArrow
      style={{ fontSize: "12px" }}
    ></MantineTooltip>
  );
};
