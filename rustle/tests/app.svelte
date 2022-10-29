<script>
  let counter = { count: 0 };

  const increment = () => {
	  counter.count += 3;
  }

  const decrement = () => counter.count--;

  function square(value) {
	  return value * value
  }
</script>

<button on:click={increment}>Increment</button>
<h1 disabled>{counter.count} ^ 2 = {square(counter.count)}</h1>
<button on:click={decrement}>Decrement</button>