<script lang="ts">
  import { Pane as PaneforgePane } from "paneforge";
  import { cn } from "@/utils";

  type $$Props = PaneforgePane.Props;

  let className: $$Props["class"] = undefined;
  export { className as class };
</script>

<PaneforgePane
  class={cn("relative flex h-full w-full", className)}
  {...$$restProps}
  let:isCollapsed
>
  <slot {isCollapsed} />
</PaneforgePane>
