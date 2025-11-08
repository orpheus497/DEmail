<script lang="ts">
  import { Pane } from "paneforge";
  import { cn } from "$lib/utils";

  let className: string | undefined = undefined;
  export { className as class };
</script>

<Pane
  class={cn("relative flex h-full w-full", className)}
  {...$$restProps}
  let:isCollapsed
>
  <slot {isCollapsed} />
</Pane>
