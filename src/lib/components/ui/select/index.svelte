<script lang="ts">
  import { Select as SelectPrimitive } from 'bits-ui';
  import { cn } from '$lib/utils';
  import { ChevronDown } from 'lucide-svelte';

  type $$Props = SelectPrimitive.Props;
</script>

<SelectPrimitive.Root {...$$restProps}>
  <slot />
</SelectPrimitive.Root>
