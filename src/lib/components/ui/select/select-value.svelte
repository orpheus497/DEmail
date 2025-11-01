<script lang="ts">
    import { Select as SelectPrimitive } from "bits-ui";

    type $$Props = SelectPrimitive.ValueProps;

    let className: $$Props["class"] = undefined;
    export { className as class };
    export let placeholder: $$Props["placeholder"] = "";
</script>

<SelectPrimitive.Value class={className} {placeholder} {...$$restProps} />
