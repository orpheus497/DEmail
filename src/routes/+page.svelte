'''<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { goto } from "$app/navigation";
</script>

<div class="flex flex-col items-center justify-center h-full">
    <Button on:click={() => goto("/Settings")}>Go to Settings</Button>
</div>
''