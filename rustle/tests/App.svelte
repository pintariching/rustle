<script>
    import Counter from "./Counter.svelte";

    let counter = 5;

    const increment = () => counter++;
    const decrement = () => counter--;
</script>

<button on:click={increment}>Increment from parent</button>
<button on:click={decrement}>Decrement from parent</button>
<Counter count={counter} />
