<script>
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";

  let { data } = $props();
  let canvas;
  let chartInstance;

  $effect(() => {
    if (chartInstance && data) {
      chartInstance.data = data;
      chartInstance.update();
    }
  });

  onMount(() => {
    // Global Chart Defaults for Dark Mode
    Chart.defaults.color = "#a1a1aa";
    Chart.defaults.borderColor = "#27272a";

    chartInstance = new Chart(canvas, {
      type: "line",
      data: data,
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { display: true, position: "top", align: "end" },
          title: { display: false },
        },
        scales: {
          y: {
            beginAtZero: true,
            max: 100,
            grid: { color: "#27272a" },
          },
          x: {
            grid: { display: false },
          },
        },
        interaction: {
          mode: "index",
          intersect: false,
        },
      },
    });
    return () => chartInstance.destroy();
  });
</script>

<div class="h-[300px] w-full">
  <canvas bind:this={canvas}></canvas>
</div>
