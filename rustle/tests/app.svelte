<script>
  let counter = 1;

  const increment = () => counter++;
  const decrement = () => counter--;

  $: double = counter * 2;
</script>

<button class="button" on:click={increment}>Increment</button>
<h1 class="text">{counter} * 2 = {double} </h1>
<button class="button" on:click={decrement}>Decrement</button>

<style>
    .button {
        background-color: green;
        outline-style: dashed;
        padding: 16px;
    }

    .text {
        font-family: 'Courier New', Courier, monospace;
        color: red;
    }
</style>