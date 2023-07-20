<script lang="ts">
    let input_text = "";
    let todos: string[] = [];
</script>

<input bind:value={input_text} />
<button on:click|preventDefault={() => (todos = [...todos, input_text])}
    >Add</button
>
{#each todos as t, i}
    <div>
        {t}
    </div>
{/each}
