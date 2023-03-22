<script lang="ts">
	import { onMount } from 'svelte';
	import CopyButton from '../../components/CopyButton.svelte';
	import RegenerateButton from '../../components/RegenerateButton.svelte';
	import ThinkingIndicator from '../../components/ThinkingIndicator.svelte';

	// const SYSTEM_PREFIX = `You are a tutor that always responds in the Socratic style. You *never* give the student the answer, but always try to ask just the right question to help them learn to think for themselves. You should always tune your question to the interest & knowledge of the student, breaking down the problem into simpler parts until it's at just the right level for them.`;
	const SYSTEM_PREFIX = ``;

	let message = '';
	let messages: { name: string; message: string }[] = [];
	var thinking = false;
	const timeout = 0;

	let cooldownTimer = 0;
	let cooldown = false;

	let totalCost = 0;

	let isMobile = false;
	let textarea: HTMLTextAreaElement | null = null;

	let API_KEY = '';
	let MODEL = 'gpt-3.5-turbo';

	async function autoResize() {
		if (textarea === null) return;
		textarea.style.height = 'auto';
		textarea.style.height = textarea.scrollHeight + 'px';
	}

	// Focus the textarea
	onMount(() => {
		// Get the token if the user has one
		let token = localStorage.getItem('token');
		let userid = localStorage.getItem('userid');
		if (token === null && userid === null) {
			// Redirect to the login page if the user is not logged in
			window.location.href = '/';
		}
		API_KEY = token as string;

		// Define the textarea
		textarea = document.getElementById('messageInput') as HTMLTextAreaElement;
		// Set it to 64px
		textarea.style.height = '64px';

		// Check if the user is on mobile
		isMobile = /iPhone|iPad|iPod|Android/i.test(navigator.userAgent);

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

		// Create a Blob with the text content
		let blob = new Blob([text], { type: 'text/plain;charset=utf-8' });

		// Create a URL to represent the Blob
		let url = URL.createObjectURL(blob);

		let element = document.createElement('a');
		element.setAttribute('href', url);
		element.setAttribute('download', 'chat.txt');

		element.style.display = 'none';
		document.body.appendChild(element);

		// Trigger the click event to start the download
		element.click();

		// Remove the link from the document and revoke the URL to free up memory
		document.body.removeChild(element);
		URL.revokeObjectURL(url);
	}

	// add messages to the array
	async function addMessage(name: string, message: string, respond: boolean) {
		message = message.trim();
		messages = [...messages, { name, message }];
		if (!respond) {
			return;
		}
		thinking = true;
		// send the message to the server
		let requestBody = {
			model: MODEL,
			messages: messages.map((msg) => {
				return {
					role: msg.name,
					content: msg.message
				};
			})
		};

		thinking = true;
		let res = await fetch('https://api.openai.com/v1/chat/completions', {
			method: 'POST',
			headers: {
				Authorization: 'Bearer ' + API_KEY,
				'Content-Type': 'application/json'
			},

			body: JSON.stringify(requestBody)
		});
		// get the response from the server
		if (res.ok) {
			let response_json = await res.json();
			// check if this contains an "error" field
			if (response_json.error) {
				// if it does, add an error message
				addMessage('system', 'Something went wrong, please try again later.', false);
				thinking = false;
				console.error(res);
				return;
			}
			// if it doesn't, add the response
			let response = response_json.choices[0].message.content;
			addMessage('assistant', response, false);
			// update the cost counter
			let promptTokens = response_json.usage.prompt_tokens;
			let completionTokens = response_json.usage.completion_tokens;
			// calculate the cost (prompt is 0.03 per 1k, completion is 0.06 per 1k)
			let cost = (promptTokens / 1000) * 0.03 + (completionTokens / 1000) * 0.06;
			totalCost += cost;
		} else if (res.status === 404) {
			// If the response is 404, maybe the user doesnt have access to gpt-4
			let response_json = await res.json();
			if (response_json.error.message === 'The model: `gpt-4` does not exist.') {
				// If the model doesn't exist, change the model to gpt-3.5-turbo
				MODEL = 'gpt-3.5-turbo';
				addMessage('system', 'You do not have access to gpt-4, using gpt-3.5 instead.', false);
				// Disable the GPT-4 input
				let model_selection = document.getElementById('model') as HTMLInputElement;
				model_selection.disabled = true;

				addMessage('system', 'Please wait...', false);
				// Try again
				addMessage(name, message, respond);
				return;
			}
		} else {
			addMessage('system', 'Something went wrong, please try again later.', false);
			console.error(res);
			thinking = false;
			return;
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

	if (SYSTEM_PREFIX !== ``) {
		// If the prefix is not empty, add a message to the chat
		addMessage('system', SYSTEM_PREFIX, false);
	}
	// Get the bot to say something
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

<!-- ChatGPT model selection -->
<label for="model" class="block text-sm font-medium text-gray-700 dark:text-gray-300">Model</label>
<div class="mb-2">
	<select
		id="model"
		name="model"
		class="block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 dark:border-gray-700 dark:bg-gray-800 dark:text-gray-100"
		bind:value={MODEL}
	>
		<option value="gpt-3.5-turbo" selected>GPT-3 (10x cheaper)</option>
		<option value="gpt-4">GPT-4 (more advanced)</option>
		<option value="gpt-4-32k">GPT-4 (50 page context)</option>
	</select>
</div>

{#each messages as msg}
	{#if msg.name === 'assistant'}
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

					<div
						class="code text-mono overflow-auto whitespace-pre rounded-lg bg-gray-800 p-2 shadow-md"
					>
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
			<CopyButton {msg} {messages} />
			<!-- Check if this is the last message in the conversation. -->
			{#if msg === messages[messages.length - 1]}
				<!-- Regenerate icon -->
				<RegenerateButton {messages} {thinking} {cooldown} {shakeButton} {addMessage} />
			{/if}
		</div>
	{:else if msg.name === 'user'}
		<!-- Human -->
		<div
			class="my-2 overflow-auto whitespace-pre rounded-lg border-2 border-blue-600 bg-blue-700 p-2 text-white shadow-md"
		>
			<img src="/default.svg" class="mr-1 inline-block h-6 w-6 align-top" alt="OpenAI Logo" />
			{msg.message}
		</div>
	{:else if msg.name === 'system'}
		<!-- System -->
		<div
			class="my-2 whitespace-pre-line rounded-lg border-2 border-gray-800 bg-gray-900 p-2 text-slate-400 shadow-md"
		>
			<img src="/system.svg" class=" mr-1 inline-block h-6 w-6 align-top" alt="System Logo" />
			{msg.message}
			<!-- Copy icon -->
			<CopyButton {msg} {messages} />
		</div>
	{/if}
{/each}
{#if thinking}
	<ThinkingIndicator />
{/if}
<!-- To fix shift+enter functionality in the above input, we need to use a textarea -->
<textarea
	bind:value={message}
	on:keydown={(e) => {
		if (e.key === 'Enter' && !e.shiftKey && !isMobile) {
			if (cooldown || thinking) {
				e.preventDefault();
				shakeButton();
				return;
			}
			if (message.trim() === '') return;
			e.preventDefault();
			addMessage('user', message, true);
			message = '';
			// set the textarea to 64px (the default height)
			if (textarea !== null) textarea.style.height = '64px';
		}
		// To fix making a newline on mobile, we need to check if the user is on mobile
		if (e.key === 'Enter' && isMobile) {
			e.preventDefault();
			message += '\n';
			autoResize(); // because we prevent default behavior, we need to manually call this
		}
	}}
	on:input={autoResize}
	placeholder="Your message"
	class=" w-full resize-none overflow-hidden rounded-md border-2 border-gray-300 bg-gray-100 p-2 shadow-md focus:border-blue-600 focus:outline-none dark:border-gray-600 dark:bg-gray-700"
	id="messageInput"
/>
<!-- Small text to show total cost -->
<div class="text-xs text-gray-500 dark:text-gray-400">
	Total cost: ${totalCost.toFixed(2)}
</div>

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
				addMessage('user', message, true);
				message = '';
				// refocus the input
				let input = document.getElementById('messageInput');
				if (input !== null) {
					input.focus();
				}
				// set the textarea to 64px (the default height)
				if (textarea !== null) textarea.style.height = '64px';
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
			if (SYSTEM_PREFIX !== ``) {
				addMessage('system', SYSTEM_PREFIX, false);
			}
			message = '';
			// Also reset the textarea
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
			// Also reset the textarea
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
	<!-- Logout button -->
	<button
		on:click={() => {
			localStorage.removeItem('token');
			window.location.href = '/';
		}}
		class="rounded-md bg-blue-500 p-2 text-white shadow-md"
	>
		Logout
	</button>
</div>
