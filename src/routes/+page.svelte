<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import Chart from "$lib/Chart.svelte";
  import {
    ChevronDown,
    Funnel,
    Plus,
    Settings,
    Target,
    Trash2,
    Trophy,
    X,
  } from "@lucide/svelte";
  import { path } from "@tauri-apps/api";
  import type { Template, TestRecord } from "$lib/types";

  type Subject = {
    name: string;
    total_q: number;
    attempted_q: number;
    correct_q: number;
  };

  let history = $state<TestRecord[]>([]);
  let templates = $state<Template[]>([]);
  let selectedFilter = $state("All");

  let showTestModal = $state(false);
  let showTemplateModal = $state(false);

  let testForm = $state({
    date: new Date().toISOString().split("T")[0],
    name: "",
    correct_points: 4,
    wrong_points: 1,
    is_negative: true,
    subjects: [] as Subject[],
  });
  let tmplForm = $state({
    name: "",
    correct_points: 4,
    wrong_points: 1,
    is_negative: true,
    subjects: [{ name: "Physics", default_total: 25 }],
  });

  let availableFilters = $derived.by(
    () => [
      "All",
      ...Array.from(new Set(history.map((h) => h.marking_config))),
    ],
  );
  let filteredHistory = $derived(
    selectedFilter === "All"
      ? history
      : history.filter((h) => h.marking_config === selectedFilter),
  );

  let chartData = $derived({
    labels: filteredHistory.map((h) => h.date).reverse(),
    datasets: [
      {
        label: "Score %",
        data: filteredHistory.map((h) => h.total_score_pct).reverse(),
        borderColor: "#3b82f6",
        backgroundColor: "rgba(59, 130, 246, 0.1)",
        fill: true,
        tension: 0.3,
      },
      {
        label: "Accuracy %",
        data: filteredHistory.map((h) => h.total_accuracy_pct).reverse(),
        borderColor: "#10b981",
        borderDash: [4, 4],
        borderWidth: 2,
        tension: 0.3,
      },
    ],
  });

  let stats = $derived({
    avgScore: filteredHistory.length
      ? (filteredHistory.reduce((a, b) => a + b.total_score_pct, 0) /
        filteredHistory.length).toFixed(1)
      : 0,
    count: filteredHistory.length,
  });

  async function loadData() {
    history = await invoke("get_history");
    templates = await invoke("get_templates");
  }

  function applyTemplate(id: string) {
    const t = templates.find((x) => x.id === parseInt(id));
    if (t) {
      testForm = {
        ...testForm,
        correct_points: t.correct_points,
        wrong_points: t.wrong_points,
        is_negative: t.is_negative,
        subjects: t.subjects.map((s) => ({
          name: s.name,
          total_q: s.default_total,
          attempted_q: 0,
          correct_q: 0,
        })),
      };
    }
  }

  async function saveTest(e?: Event) {
    if (e) e.preventDefault();
    await invoke("add_test", {
      data: {
        ...testForm,
        correct_points: Number(testForm.correct_points),
        wrong_points: Number(testForm.wrong_points),
        subjects: testForm.subjects.map((s) => ({
          ...s,
          total_q: Number(s.total_q),
          attempted_q: Number(s.attempted_q),
          correct_q: Number(s.correct_q),
        })),
      },
    });
    showTestModal = false;
    loadData();
  }
  async function saveTemplate(e?: Event) {
    if (e) e.preventDefault();
    await invoke("create_template", {
      data: {
        ...tmplForm,
        correct_points: Number(tmplForm.correct_points),
        wrong_points: Number(tmplForm.wrong_points),
        subjects: tmplForm.subjects.map((s) => ({
          name: s.name,
          default_total: Number(s.default_total),
        })),
      },
    });
    showTemplateModal = false;
    loadData();
  }
  async function deleteItem(type: "test" | "template", id: number) {
    if (confirm("Delete item?")) {
      await invoke(type === "test" ? "delete_test" : "delete_template", {
        id,
      });
      loadData();
    }
  }

  onMount(loadData);
  onMount(async () => {
    const appData = await path.appDataDir();
    console.log("Database is located at:", appData);
  });
</script>

<div class="min-h-screen bg-background p-8 font-sans text-foreground">
  <div class="mx-auto max-w-5xl space-y-8">
    <header class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div
          class="h-8 w-3 rounded-sm bg-linear-to-b from-blue-500 to-blue-700"
        >
        </div>
        <h1 class="text-2xl font-bold tracking-tight">Akri</h1>
      </div>
      <div class="flex gap-3">
        <button
          type="button"
          onclick={() => showTemplateModal = true}
          class="inline-flex h-9 items-center justify-center rounded-md border border-input bg-transparent px-3 text-sm font-medium shadow-sm hover:bg-accent hover:text-accent-foreground transition-colors"
        >
          <Settings class="mr-2 h-4 w-4" /> Templates
        </button>
        <button
          type="button"
          onclick={() => showTestModal = true}
          class="inline-flex h-9 items-center justify-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 transition-colors"
        >
          <Plus class="mr-2 h-4 w-4" /> Log Test
        </button>
      </div>
    </header>
    <div class="rounded-xl border bg-card text-card-foreground shadow">
      <div class="flex flex-col space-y-1.5 p-6 pb-2">
        <div class="flex items-center justify-between">
          <div class="flex gap-6">
            <div class="flex items-center gap-2">
              <div class="rounded-md bg-blue-500/10 p-2 text-blue-500">
                <Trophy class="h-5 w-5" />
              </div>
              <div>
                <div class="text-2xl font-bold leading-none">
                  {stats.avgScore}%
                </div>
                <p
                  class="text-xs text-muted-foreground font-medium uppercase mt-1"
                >
                  Avg Score
                </p>
              </div>
            </div>
            <div class="flex items-center gap-2">
              <div class="rounded-md bg-emerald-500/10 p-2 text-emerald-500">
                <Target class="h-5 w-5" />
              </div>
              <div>
                <div class="text-2xl font-bold leading-none">{stats.count}</div>
                <p
                  class="text-xs text-muted-foreground font-medium uppercase mt-1"
                >
                  Tests
                </p>
              </div>
            </div>
          </div>
          <div class="relative w-[180px]">
            <Funnel
              class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground pointer-events-none"
            />
            <select
              aria-label="Filter tests by marking scheme"
              bind:value={selectedFilter}
              class="w-full appearance-none rounded-md border border-input bg-transparent py-2 pl-9 pr-8 text-sm shadow-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            >
              {#each availableFilters as f}<option value={f}>
                  {f === "All" ? "All Formats" : f}
                </option>{/each}
            </select>
            <ChevronDown
              class="absolute right-3 top-3 h-4 w-4 opacity-50 pointer-events-none"
            />
          </div>
        </div>
      </div>
      <div class="p-6 pt-4 h-[350px]">
        <Chart data={chartData} />
      </div>
    </div>
    <div class="space-y-4">
      <h3
        class="text-sm font-medium text-muted-foreground uppercase tracking-wider"
      >
        Recent Activity
      </h3>
      <div class="grid gap-3">
        {#each filteredHistory as item}
          <div
            class="flex items-center justify-between rounded-lg border bg-card p-4 transition-all hover:bg-accent/50 hover:border-blue-500/50 group"
          >
            <div class="flex items-center gap-4">
              <div
                class="flex h-12 w-12 flex-col items-center justify-center rounded-md border bg-muted/50 text-center"
              >
                <span class="text-sm font-bold">{item.date.split("-")[1]}/{
                    item.date.split("-")[2]
                  }</span>
                <span class="text-[10px] text-muted-foreground">{
                  item.date.split("-")[0]
                }</span>
              </div>
              <div>
                <p class="font-medium leading-none text-foreground">
                  {item.name || "Untitled Test"}
                </p>
                <p class="mt-1 text-xs text-muted-foreground">
                  {item.marking_config} • {item.subjects.length} Subjects
                </p>
              </div>
            </div>
            <div class="flex items-center gap-6">
              <div class="text-right">
                <p class="text-lg font-bold text-blue-500">
                  {item.total_score_pct.toFixed(1)}%
                </p>
                <p class="text-xs text-muted-foreground">
                  Accuracy: {item.total_accuracy_pct.toFixed(0)}%
                </p>
              </div>
              <button
                type="button"
                onclick={() => deleteItem("test", item.id)}
                class="text-muted-foreground hover:text-red-500 opacity-0 group-hover:opacity-100 transition-opacity"
                aria-label="Delete test"
              >
                <Trash2 class="h-4 w-4" />
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
  {#if showTestModal}
    <div
      class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4"
    >
      <form
        onsubmit={saveTest}
        class="w-full max-w-lg rounded-xl border bg-card shadow-lg animate-in fade-in zoom-in-95 duration-200"
      >
        <div class="flex items-center justify-between border-b p-4">
          <h2 class="font-semibold">Log New Test</h2>
          <button type="button" onclick={() => showTestModal = false}>
            <X class="h-4 w-4 opacity-70 hover:opacity-100" />
          </button>
        </div>
        <div class="p-4 space-y-4 max-h-[70vh] overflow-y-auto">
          <div class="grid gap-2">
            <label
              for="test-template"
              class="text-xs font-medium uppercase text-muted-foreground"
            >Load Template</label>
            <div class="relative">
              <select
                id="test-template"
                onchange={(e) => applyTemplate(e.currentTarget.value)}
                class="w-full appearance-none rounded-md border border-input bg-background py-2 px-3 text-sm focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              >
                <option value="">Select a template...</option>
                {#each templates as t}<option value={t.id}>
                    {t.name}
                  </option>{/each}
              </select>
              <ChevronDown
                class="absolute right-3 top-3 h-4 w-4 opacity-50 pointer-events-none"
              />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="grid gap-2">
              <label
                for="test-date"
                class="text-xs font-medium uppercase text-muted-foreground"
              >Date</label>
              <input
                id="test-date"
                type="date"
                bind:value={testForm.date}
                class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              />
            </div>
            <div class="grid gap-2">
              <label
                for="test-name"
                class="text-xs font-medium uppercase text-muted-foreground"
              >Name</label>
              <input
                id="test-name"
                type="text"
                bind:value={testForm.name}
                placeholder="Test Name"
                class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
              />
            </div>
          </div>
          <div
            class="rounded-md border bg-muted/40 p-3 grid grid-cols-3 gap-4 items-end"
          >
            <div>
              <label for="test-correct" class="text-xs font-medium"
              >Correct</label>
              <input
                id="test-correct"
                type="number"
                bind:value={testForm.correct_points}
                class="mt-1 flex h-8 w-full rounded-md border border-input bg-background px-3 text-sm"
              />
            </div>
            <div>
              <label for="test-wrong" class="text-xs font-medium">Wrong</label>
              <input
                id="test-wrong"
                type="number"
                bind:value={testForm.wrong_points}
                class="mt-1 flex h-8 w-full rounded-md border border-input bg-background px-3 text-sm"
              />
            </div>
            <label class="flex items-center gap-2 text-sm pb-2 cursor-pointer">
              <input
                type="checkbox"
                bind:checked={testForm.is_negative}
                class="rounded border-input text-primary shadow-sm focus-visible:ring-1 focus-visible:ring-ring"
              />
              Negative
            </label>
          </div>
          <div class="space-y-3">
            <div class="flex justify-between items-center">
              <h4 class="text-sm font-medium">Subjects</h4>
              <button
                type="button"
                onclick={() =>
                  testForm.subjects = [...testForm.subjects, {
                    name: "New",
                    total_q: 30,
                    attempted_q: 0,
                    correct_q: 0,
                  }]}
                class="text-xs text-primary hover:underline"
              >
                + Add Custom
              </button>
            </div>
            {#each testForm.subjects as sub}
              <div
                class="grid grid-cols-12 gap-2 items-center rounded-md border p-2 bg-card"
              >
                <div class="col-span-4 font-medium text-sm pl-1">
                  {sub.name} <span
                    class="text-muted-foreground text-xs font-normal"
                  >/ {sub.total_q}</span>
                </div>
                <input
                  type="number"
                  aria-label="Attempted questions"
                  placeholder="Att."
                  bind:value={sub.attempted_q}
                  class="col-span-4 flex h-8 rounded-md border border-input bg-background px-3 text-sm"
                />
                <input
                  type="number"
                  aria-label="Correct questions"
                  placeholder="Corr."
                  bind:value={sub.correct_q}
                  class="col-span-4 flex h-8 rounded-md border border-input bg-background px-3 text-sm"
                />
              </div>
            {/each}
          </div>
        </div>
        <div class="flex justify-end gap-2 border-t p-4">
          <button
            type="button"
            onclick={() => showTestModal = false}
            class="inline-flex h-9 items-center justify-center rounded-md border border-input bg-transparent px-4 text-sm font-medium hover:bg-accent hover:text-accent-foreground"
          >
            Cancel
          </button>
          <button
            type="submit"
            class="inline-flex h-9 items-center justify-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground hover:bg-primary/90"
          >
            Save Entry
          </button>
        </div>
      </form>
    </div>
  {/if}
  {#if showTemplateModal}
    <div
      class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm flex items-center justify-center p-4"
    >
      <form
        onsubmit={saveTemplate}
        class="w-full max-w-md rounded-xl border bg-card shadow-lg animate-in fade-in zoom-in-95 duration-200"
      >
        <div class="flex items-center justify-between border-b p-4">
          <h2 class="font-semibold">Templates</h2>
          <button type="button" onclick={() => showTemplateModal = false}>
            <X class="h-4 w-4 opacity-70 hover:opacity-100" />
          </button>
        </div>
        <div class="p-4 space-y-4">
          <div class="grid gap-2">
            <label for="tmpl-name" class="text-xs font-medium uppercase"
            >Template Name</label>
            <input
              id="tmpl-name"
              type="text"
              bind:value={tmplForm.name}
              class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 text-sm"
            />
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label for="tmpl-correct" class="text-xs font-medium"
              >Correct (+)</label>
              <input
                id="tmpl-correct"
                type="number"
                bind:value={tmplForm.correct_points}
                class="mt-1 flex h-9 w-full rounded-md border border-input bg-transparent px-3 text-sm"
              />
            </div>
            <div>
              <label for="tmpl-wrong" class="text-xs font-medium"
              >Wrong (-)</label>
              <input
                id="tmpl-wrong"
                type="number"
                bind:value={tmplForm.wrong_points}
                class="mt-1 flex h-9 w-full rounded-md border border-input bg-transparent px-3 text-sm"
              />
            </div>
          </div>
          <div class="space-y-2">
            <div class="flex justify-between">
              <p class="text-xs font-medium uppercase">Default Subjects</p>
              <button
                type="button"
                onclick={() =>
                  tmplForm.subjects = [...tmplForm.subjects, {
                    name: "",
                    default_total: 0,
                  }]}
                class="text-xs text-primary"
              >
                + Add
              </button>
            </div>
            {#each tmplForm.subjects as s, i}
              <div class="flex gap-2">
                <input
                  aria-label="Subject name"
                  bind:value={s.name}
                  placeholder="Subject Name"
                  class="flex-1 h-8 rounded-md border border-input bg-transparent px-2 text-sm"
                />
                <input
                  aria-label="Total questions"
                  type="number"
                  bind:value={s.default_total}
                  placeholder="Qty"
                  class="w-16 h-8 rounded-md border border-input bg-transparent px-2 text-sm"
                />
                <button
                  type="button"
                  onclick={() =>
                    tmplForm.subjects = tmplForm.subjects.filter((
                      _,
                      idx,
                    ) => idx !== i)}
                  class="text-muted-foreground hover:text-red-500"
                  aria-label="Remove subject"
                >
                  <Trash2 class="h-4 w-4" />
                </button>
              </div>
            {/each}
          </div>
          <button
            type="submit"
            class="w-full inline-flex h-9 items-center justify-center rounded-md bg-primary px-4 text-sm font-medium text-primary-foreground hover:bg-primary/90 mt-2"
          >
            Create Template
          </button>

          <div class="border-t pt-4 mt-4">
            <h4
              class="text-xs font-medium uppercase text-muted-foreground mb-2"
            >
              Existing Templates
            </h4>
            {#each templates as t}
              <div class="flex justify-between items-center py-1 text-sm">
                <span>{t.name}</span>
                <button
                  type="button"
                  onclick={() => deleteItem("template", t.id!)}
                  class="text-muted-foreground hover:text-red-500"
                  aria-label="Delete template"
                >
                  <Trash2 class="h-3 w-3" />
                </button>
              </div>
            {/each}
          </div>
        </div>
      </form>
    </div>
  {/if}
</div>
