<script lang="ts">
  import "../app.css";

  let { children } = $props();
</script>

<div class="layout">{@render children()}</div>

<style>
  .layout {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
  }
</style>
