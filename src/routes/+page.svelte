<script lang="ts">
	let name = '';
	let loggedIn = false;

	let message = '';
	let messages: { name: string; message: string }[] = [];
	// todo: reply cooldown

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
		// I have actually no idea how this works, copilot did it for me lol
	}

	// add messages to the array
	function addMessage(name: string, message: string) {
		messages = [...messages, { name, message }];
		if (name !== 'ChatGPT') {
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
			fetch('https://api.openai.com/v1/chat/completions', {
				method: 'POST',
				headers: {
					Authorization: 'Bearer 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',
					'Content-Type': 'application/json'
				},

				body: JSON.stringify(requestBody)
			})
				.then((res) => res.json())
				.then((res) => {
					// get the response from the server
					console.log(res);
					let response = res.choices[0].message.content;
					addMessage('ChatGPT', response);
				});
		}
	}

	addMessage('ChatGPT', 'Ask me anything!');
	// If the user inputs a name, store it in localStorage
	// so that it can be used in the next session
	function startChatting() {
		if (name !== '') {
			if (name === 'ChatGPT') {
				alert('Please enter a name other than ChatGPT');
				return;
			}
			localStorage.setItem('name', name);
			loggedIn = true;
		} else {
			alert('Please enter a name');
		}
	}
</script>

{#if loggedIn === false}
	<div class="font-lexend text-center border-b-2 text-white mb-4">
		<h1 class="text-4xl pb-2">Welcome to ChatGPT</h1>
		<p>ChatGPT is a chatbot that uses GPT-3.5 to generate human-like responses to your messages.</p>
		<p>To start, enter your name and click "Start Chatting".</p>
	</div>
	<div
		class="font-lexend text-center text-black flex flex-row justify-center items-center space-x-2"
	>
		<input
			type="text"
			bind:value={name}
			on:keydown={(e) => {
				if (e.key === 'Enter' && name !== '') {
					startChatting();
				}
			}}
			placeholder="Your name"
			class="rounded-md p-2 shadow-md flex-grow"
		/>
		<button on:click={startChatting} class="text-white bg-blue-500 rounded-md p-2 shadow-md">
			<!-- On click start chatting -->
			Start Chatting
		</button>
	</div>
{:else}
	<!-- If the user is logged in, show the chat -->
	{#each messages as msg}
		{#if msg.name === name}
			<!-- <div class="bg-blue-700 text-white rounded-lg p-2 my-2"> -->
			<!-- To fix newlines -->
			<div class="bg-blue-700 text-white rounded-lg p-2 my-2 whitespace-pre-line shadow-md">
				{name}: {msg.message}
			</div>
		{:else}
			<div class="bg-gray-700 text-white rounded-lg p-2 my-2 whitespace-pre-line shadow-md">
				ChatGPT: {msg.message}
			</div>
		{/if}
	{/each}
	<!-- To fix shift+enter functionality in the above input, we need to use a textarea -->
	<textarea
		bind:value={message}
		on:keydown={(e) => {
			if (e.key === 'Enter' && !e.shiftKey) {
				e.preventDefault();
				addMessage(name, message);
				message = '';
			}
		}}
		placeholder="Your message"
		class="rounded-md p-2 w-full shadow-md"
		rows="5"
		id="messageInput"
	/>

	<!-- Small reminder that the user can adjust the height by dragging it -->
	<p class="text-xs text-gray-500">
		You can adjust the height of the input box by dragging the bottom of it
	</p>

	<div class="flex flex-row space-x-2">
		<button
			on:click={() => addMessage(name, message)}
			class="bg-blue-500 text-white rounded-md p-2 shadow-md flex-grow"
		>
			<!-- On click reset the message -->
			Reply
		</button>

		<button
			on:click={() => {
				messages = [];
				addMessage('ChatGPT', 'Ask me anything!');
				message = '';
			}}
			class="bg-blue-500 text-white rounded-md p-2 shadow-md"
		>
			<!-- On click reset the message -->
			Reset
		</button>
		<button on:click={saveMessages} class="bg-blue-500 text-white rounded-md p-2 shadow-md">
			<!-- On click get all of the messages and save them to a txt file -->
			Save
		</button>
		<!-- Button for resetting the username -->
		<button
			on:click={() => {
				localStorage.removeItem('name');
				loggedIn = false;
			}}
			class="bg-blue-500 text-white rounded-md p-2 shadow-md"
		>
			Reset Username
		</button>
	</div>
{/if}
