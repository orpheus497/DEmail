<script lang="ts">
    import { Select as SelectPrimitive } from "bits-ui";
    import { cn } from "@/utils";
    import ChevronDown from "lucide-svelte/icons/chevron-down";

    type $$Props = SelectPrimitive.Props;
    type $$Events = SelectPrimitive.Events;

    let className: $$Props["class"] = undefined;
    export { className as class };
</script>

<SelectPrimitive.Root {...$$restProps} onValueChange>
    <slot />
</SelectPrimitive.Root>
