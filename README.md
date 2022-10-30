# Rustle

The Svelte compiler, rewritten in Rust.

## Description

This projects aims to make `Svelte` usable without `Node.js` and make the compiler _blazingly fast_.

## Work in progress

This is still a big work in progress. Only a few parts of Svelte are working now and the CLI tool still needs some work.

# Getting started

### Installation

To install with cargo, run `cargo install rustle_cli"` to install the alpha version of the CLI.

### Using rustle_cli

Run `rustle_cli app.rustle` to generate an `app.js` file. You can optionally specify a different output file with the `-o` flag.

You can also specify a directory for example `rustle_cli src` to parse all the files in that directory.

For debugging, you can print the generated AST with the `-a` or `--ast` flag and pretty print it with `-p`.

# Development

A lot of features still need to be implemented before `rustle` is ready for use. Check the examples folder or `rustle/tests` to get a better look at currently is supported:

- [x] on:click event handler
- [x] arrow function with single assignment
- [x] display variable (eg. {counter})

Feature roadmap:
- [ ] Dynamic attributes ({class} instead of class={class})
- [ ] Styling (&lt;style&gt;&lt;/style&gt;)
- [ ] Nested components
- [ ] HTML tags ({@html htmlString})
- [ ] Reactivity
	- [x] Reactive assignments (on:click={handleClick})
	- [x] Reactive declarations ($: doubled = count * 2)
	- [ ] Reactive statements
- [ ] Props
	- [ ] Declaring props
	- [ ] Default values
	- [ ] Spread props
- [ ] Logic
	- [ ] If blocks
	- [ ] Else blocks
	- [ ] Else-if blocks
	- [ ] Each blocks
	- [ ] Keyed each blocks
	- [ ] Await blocks
- [ ] Events
	- [ ] DOM events
	- [ ] Inline handlers
	- [ ] Event modifiers
	- [ ] Component events
	- [ ] Event forwarding
	- [ ] DOM event forwarding
- [ ] Bindings
	- [ ] Text inputs
	- [ ] Numeric inputs
	- [ ] Checkbox inputs
	- [ ] Group inputs
	- [ ] Textarea inputs
	- [ ] File inputs
	- [ ] Select bindings
	- [ ] Select multiple
	- [ ] Each block bindings
	- [ ] Media elements
	- [ ] Dimensions
	- [ ] bind:this={canvas}
	- [ ] Component bindings
- [ ] Lifecycle
	- [ ] onMount
	- [ ] onDestroy
	- [ ] beforeUpdate and afterUpdate
	- [ ] tick
- [ ] Stores
	- [ ] Writable stores
	- [ ] Auto-subscriptions
	- [ ] Readable stores
	- [ ] Derived stores
	- [ ] Custom stores
- [ ] Motion
	- [ ] Tweened
	- [ ] Spring
- [ ] Transitions
	- [ ] Adding parameters
	- [ ] In and out
	- [ ] Custom CSS transitions
	- [ ] Custom JS transitions
	- [ ] Transition events
	- [ ] Deferred transitions
- [ ] Animations
- [ ] Easing
- [ ] Actions
	- [ ] Use directive
	- [ ] Adding parameters
- [ ] Components
	- [ ] Slots
	- [ ] Slot fallbacks
	- [ ] Named slots
	- [ ] Slot props
	- [ ] Conditional slots
- [ ] Special elements
	- [ ] &lt;svelte:self&gt;
	- [ ] &lt;svelte:component&gt;
	- [ ] &lt;svelte:element&gt;
	- [ ] &lt;svelte:window&gt;
	- [ ] &lt;svelte:body&gt;
	- [ ] &lt;svelte:head&gt;
- [ ] Named exports
- [ ] @debug tag


## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Acknowledgments

* A big thank you to [lihautan](https://www.youtube.com/c/lihautan) on Youtube, for making the video [Build your own Svelte](https://www.youtube.com/watch?v=mwvyKGw2CzU) which helped me a lot in understanding how the Svelte compiler actually works!
