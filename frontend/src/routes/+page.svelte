<script lang="ts">
	import { onMount } from 'svelte';

	let username = 'Human';

	let message = '';
	let messages: { name: string; message: string }[] = [];
	var thinking = false;
	const timeout = 3;

	let cooldownTimer = 0;
	let cooldown = false;

	// Focus the textview
	onMount(() => {
		let input = document.getElementById('messageInput');
		if (input !== null) {
			input.focus();
		}
	});

	function shakeButton() {
		let button = document.getElementById('sendButton');
		// To do this, I have to add a class to the button, and then remove it after 0.5 seconds
		if (button !== null) {
			button.classList.add('animate-shake');
			setTimeout(() => {
				button?.classList.remove('animate-shake');
			}, 1000);
		}
	}

	function saveMessages() {
		let text = '';
		messages.forEach((msg) => {
			text += msg.name + ': ' + msg.message + '\n';
		});
		let element = document.createElement('a');
		element.setAttribute('href', 'data:text/plain;charset=utf-8,' + encodeURIComponent(text));
		element.setAttribute('download', 'chat.txt');

		element.style.display = 'none';
		document.body.appendChild(element);

		element.click();

		document.body.removeChild(element);
		// I have actually no idea how this works, copilot did it for me lolol
	}

	// add messages to the array
	async function addMessage(name: string, message: string) {
		message = message.trim();
		messages = [...messages, { name, message }];
		if (name === 'ChatGPT') return;
		thinking = true;
		// send the message to the server
		let requestBody = {
			model: 'gpt-3.5-turbo',
			messages: messages.map((msg) => {
				return {
					role: msg.name === 'ChatGPT' ? 'assistant' : 'user',
					content: msg.message
				};
			})
		};

		thinking = true;
		let res = await fetch('https://gptapi.ducky.pics', {
			method: 'POST',
			headers: {
				// to set this up in svelte using vite
				'Content-Type': 'application/json'
			},

			body: JSON.stringify(requestBody)
		});
		// get the response from the server
		if (res.ok) {
			let response = (await res.json()).choices[0].message.content;
			addMessage('ChatGPT', response);
		} else {
			addMessage('ChatGPT', 'Something went wrong, please try again later.');
			console.error(res);
		}
		thinking = false;
		buttonCooldown(); // without async to not wait for it
		console.log(messages);
	}

	// Async function to add a cooldown to the button
	async function buttonCooldown() {
		cooldown = true;
		for (let i = timeout; i > 0; i--) {
			cooldownTimer = i;
			await new Promise((r) => setTimeout(r, 1000));
		}
		cooldown = false;
	}

	addMessage('ChatGPT', `Ask me anything!`);
</script>

<svelte:head>
	<link
		rel="stylesheet"
		href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/styles/dark.min.css"
	/>
	<script
		src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"
	></script>
</svelte:head>

<!-- If the user is logged in, show the chat -->
{#each messages as msg}
	{#if msg.name === 'ChatGPT'}
		<!-- ChatGPT -->

		<div
			class="my-2 whitespace-pre-line rounded-lg border-2 border-gray-600 bg-gray-700 p-2 text-white shadow-md"
		>
			<img src="/openai.svg" class="mr-1 inline-block h-6 w-6 align-top" alt="OpenAI Logo" />
			<!-- Code block support -->
			{#each msg.message.split('```') as text, i}
				{#if i % 2 === 0}
					<!-- Normal text -->
					<!-- Split by single backticks -->
					{#each text.split('`') as code, i2}
						{#if i2 % 2 === 0}
							<!-- Normal text -->
							{code}
						{:else}
							<!-- Code block -->
							<span class="rounded-lg bg-gray-800 font-mono shadow-md">
								{code.trim()}
							</span>
						{/if}
					{/each}
				{:else}
					<!-- Code block -->
					<!-- Remove everything until the first newline -->
					<!-- Todo -->

					<div class="code text-mono whitespace-pre rounded-lg bg-gray-800 p-2 shadow-md">
						{text.trim()}
					</div>

					<script>
						// Format the code block
						hljs.highlightElement(
							document.querySelectorAll('.code')[document.querySelectorAll('.code').length - 1]
						);
					</script>
				{/if}
			{/each}

			<!-- Copy icon -->
			<div class="float-right inline-block h-6 w-6 align-top text-gray-500 invert filter">
				<!-- Id is the index of the message -->
				<button
					id={'copyButton' + messages.indexOf(msg)}
					on:click={() => {
						navigator.clipboard.writeText(msg.message);
						// Change the icon to a checkmark for 1 second
						// we can't use the id because it is not unique
						// so let's use the index of the message
						let button = document.getElementById('copyButton' + messages.indexOf(msg));
						if (button !== null) {
							button.innerHTML = '<img src="/tick.svg" class="w-6 h-6 inline-block" alt="Check" />';
							setTimeout(() => {
								if (button !== null)
									button.innerHTML =
										'<img src="/copy.svg" class="w-6 h-6 inline-block" alt="Copy" />';
							}, 1000);
						}
					}}
				>
					<img src="/copy.svg" class="inline-block h-6 w-6" alt="Copy" />
				</button>
			</div>
			<!-- Check if this is the last message in the conversation. -->
			{#if msg === messages[messages.length - 1]}
				<!-- Regenerate icon -->
				<div class="float-right inline-block h-6 w-6 align-top text-gray-500 invert filter">
					<button
						on:click={() => {
							if (thinking || cooldown) {
								shakeButton();
								return;
							}
							// save the last human message
							let lastMessage = messages[messages.length - 2];
							// remove the last bot message
							messages.pop();
							// remove the last message with the user message
							messages.pop();
							// add a new message to generate a new response
							addMessage(lastMessage.name, lastMessage.message);
						}}
					>
						<img src="/regen.svg" class="inline-block h-6 w-6 px-1" alt="Regenerate" />
					</button>
				</div>
			{/if}
		</div>
	{:else}
		<!-- Human -->
		<div
			class="my-2 whitespace-pre-line rounded-lg border-2 border-blue-600 bg-blue-700 p-2 text-white shadow-md"
		>
			<img src="/default.svg" class="mr-1 inline-block h-6 w-6 align-top" alt="OpenAI Logo" />
			{msg.message}
		</div>
	{/if}
{/each}
{#if thinking}
	<div class="my-2 whitespace-pre-line rounded-lg bg-gray-700 p-2 text-gray-500 shadow-md">
		<svg
			class="mr-1 inline-block h-5 w-5 animate-spin align-middle text-white"
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
		>
			<circle
				class="inline-block opacity-25"
				cx="12"
				cy="12"
				r="10"
				stroke="currentColor"
				stroke-width="4"
			/>
			<path
				class="inline-block opacity-75"
				fill="currentColor"
				d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
			/>
		</svg>
		<!-- Got this SVG code from the tailwind website -->
		<span class="animate-pulse">Thinking...</span>
	</div>
{/if}
<!-- To fix shift+enter functionality in the above input, we need to use a textarea -->
<textarea
	bind:value={message}
	on:keydown={(e) => {
		if (e.key === 'Enter' && !e.shiftKey) {
			if (cooldown || thinking) {
				e.preventDefault();
				shakeButton();
				return;
			}
			if (message.trim() === '') return;
			e.preventDefault();
			addMessage(username, message);
			message = '';
		}
	}}
	placeholder="Your message"
	class=" w-full rounded-md border-2 border-gray-300 bg-gray-100 p-2 shadow-md focus:border-blue-600 focus:outline-none dark:border-gray-600 dark:bg-gray-700"
	rows="3"
	id="messageInput"
/>

<!-- Small reminder that the user can adjust the height by dragging it -->
<p class="text-xs text-gray-500">
	You can adjust the height of the input box by dragging the bottom of it
</p>

<div class="flex flex-row space-x-2">
	{#if thinking}
		<button
			disabled
			class="flex-grow rounded-md bg-gray-500 p-2 text-gray-600 shadow-md"
			id="sendButton"
		>
			Thinking...
		</button>
	{:else if cooldown}
		<button
			disabled
			class="flex-grow rounded-md bg-gray-500 p-2 text-gray-600 shadow-md"
			id="sendButton"
		>
			Cooldown... {cooldownTimer}s
		</button>
	{:else}
		<button
			on:click={() => {
				if (message.trim() === '') return;
				addMessage(username, message);
				message = '';
				// refocus the input
				let input = document.getElementById('messageInput');
				if (input !== null) {
					input.focus();
				}
			}}
			class="flex-grow rounded-md bg-blue-500 p-2 text-white shadow-md"
			id="sendButton"
		>
			Reply
		</button>
	{/if}

	<button
		on:click={() => {
			messages = [];
			addMessage('ChatGPT', 'Ask me anything!');
			message = '';
			// Also reset the textview
			let input = document.getElementById('messageInput');
			if (input !== null) {
				input.focus();
			}
		}}
		class="rounded-md bg-blue-500 p-2 text-white shadow-md"
	>
		<!-- Reset all of the history -->
		Reset
	</button>
	<button
		on:click={() => {
			saveMessages();
			// Also reset the textview
			let input = document.getElementById('messageInput');
			if (input !== null) {
				input.focus();
			}
		}}
		class="rounded-md bg-blue-500 p-2 text-white shadow-md"
	>
		<!-- On click get all of the messages and save them to a txt file -->
		Save
	</button>
</div>
