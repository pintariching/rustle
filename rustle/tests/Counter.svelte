<script>
    export let count;

    const increment = () => count++;
    const decrement = () => count--;
</script>

<button on:click={increment}>Increment</button>
<h1>{count}</h1>
<button on:click={decrement}>Decrement</button>
