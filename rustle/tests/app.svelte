<script>
  let counter = 1;
  let foo = 5;

  const increment = () => counter++;
  const decrement = () => counter--;
  const incrementFoo = () => foo++;

  $: quadruple = double * 2;
  $: double = counter * 2 + bar;
  
  $: bar = foo + 5;
</script>

<button on:click={increment}>Increment</button>
<h1>{counter} * 2 = {double} </h1>
<h1>{double} * 2 = {quadruple} </h1>
<button on:click={decrement}>Decrement</button>

<button on:click={incrementFoo}>Foo ++</button>
<h1>foo = {foo}</h1>