<script lang="ts">
  import { onMount } from "svelte";
  import Chart from "chart.js/auto";
  import type { ChartData, Chart as ChartJS } from "chart.js";
  
  let { data }: { data: ChartData<"line" | "bar"> } = $props();
  let canvas: HTMLCanvasElement;
  let chartInstance = $state<ChartJS>();
  
  $effect(() => {
    if (chartInstance && data) {
      const plainData = JSON.parse(JSON.stringify(data));
      chartInstance.data = plainData;
      chartInstance.update();
    }
  });
  
  onMount(() => {
    Chart.defaults.color = "#94a3b8";
    Chart.defaults.borderColor = "#1e293b";
    Chart.defaults.font.family = "Inter, sans-serif";
    Chart.defaults.font.size = 11;
    
    const plainData = JSON.parse(JSON.stringify(data));
    
    chartInstance = new Chart(canvas, {
      type: "line",
      data: plainData,
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: { 
            display: true, 
            position: "top", 
            align: "end",
            labels: { 
              boxWidth: 12, 
              boxHeight: 3,
              usePointStyle: false,
              padding: 12,
              font: {
                size: 10
              }
            }
          },
          tooltip: {
            mode: 'index',
            intersect: false,
            backgroundColor: '#020617',
            titleColor: '#f8fafc',
            bodyColor: '#cbd5e1',
            borderColor: '#334155',
            borderWidth: 1,
            padding: 10,
            usePointStyle: true,
            callbacks: {
              label: (context) => {
                const label = context.dataset.label || '';
                const value = context.parsed.y;
                return `${label}: ${value !== null ? value.toFixed(1) : 'N/A'}%`;
              }
            }
          }
        },
        scales: {
          y: {
            beginAtZero: false,
            grid: { color: "#1e293b" },
            border: { display: false },
            ticks: { 
              callback: (v) => v + '%', 
              padding: 10 
            }
          },
          x: {
            type: 'category', 
            grid: { display: false },
            border: { display: false },
            ticks: {
              maxRotation: 45,
              minRotation: 0
            }
          },
        },
        elements: {
          point: { 
            radius: 4,
            hoverRadius: 6,
            borderWidth: 2,
            backgroundColor: '#020617'
          },
          line: { 
            tension: 0.3,
            borderWidth: 2 
          },
          bar: {
            borderRadius: 4
          }
        },
        interaction: {
          mode: 'index',
          intersect: false,
        },
      },
    });
    
    return () => chartInstance?.destroy();
  });
</script>

<div class="h-[350px] w-full relative">
  <canvas bind:this={canvas}></canvas>
</div>
