<script lang="ts">
	export let msg: any;
	export let messages: any;
</script>

<div class="float-right inline-block h-6 w-6 align-top text-gray-500 invert filter">
	<button
		id={'copyButton' + messages.indexOf(msg)}
		on:click={() => {
			navigator.clipboard.writeText(msg.message);
			let button = document.getElementById('copyButton' + messages.indexOf(msg));
			if (button !== null) {
				button.innerHTML = '<img src="/tick.svg" class="w-6 h-6 inline-block" alt="Check" />';
				setTimeout(() => {
					if (button !== null)
						button.innerHTML = '<img src="/copy.svg" class="w-6 h-6 inline-block" alt="Copy" />';
				}, 1000);
			}
		}}
	>
		<img src="/copy.svg" class="inline-block h-6 w-6" alt="Copy" />
	</button>
</div>
