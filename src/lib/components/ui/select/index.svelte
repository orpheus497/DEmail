<script lang="ts">
  import { Select as SelectPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils';
  import { ChevronDown } from 'lucide-svelte';

  type $$Props = SelectPrimitive.Props;

  let className: $$Props['class'] = undefined;
  export { className as class };
</script>

<SelectPrimitive.Root {...$$restProps} onValueChange>
  <slot />
</SelectPrimitive.Root>
