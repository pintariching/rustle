<script>
	let x = 0;
  let y = 0;

	const handleMousemove = (event) => {
		x = event.clientX;
		y = event.clientY;
	}
</script>

<div on:mousemove={handleMousemove} class="full">
  The mouse position is x:{x} y:{y}
</div>