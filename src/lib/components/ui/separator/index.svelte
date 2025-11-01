<script lang="ts">
	import { Separator as SeparatorPrimitive } from "bits-ui";
	import { cn } from "@/utils";

	type $$Props = SeparatorPrimitive.Props;

	let className: $$Props["class"] = undefined;
	export { className as class };
</script>

<SeparatorPrimitive
	class={cn("shrink-0 bg-border", className)}
	{...$$restProps}
/>
