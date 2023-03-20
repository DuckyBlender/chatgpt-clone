<!-- It will show the login form and the anonymous "input token" form -->
<script lang="ts">
	import { onMount } from 'svelte';

	let token = '';
	let username = '';
	let password = '';

	onMount(() => {
		// Check if the user is logged in
		if (localStorage.getItem('token')) {
			// If the user is logged in, redirect to the chat page
			window.location.href = '/chat';
		}
	});

	async function loginPassword() {
		console.log('Logging in...');
		// Get the username and password from the form
		let username = (document.getElementById('username') as HTMLInputElement).value;
		let password = (document.getElementById('password') as HTMLInputElement).value;
		// Send a POST request to the backend to log in
		let res = await fetch('/api/login', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				username,
				password
			})
		});
		if (!res.ok) {
			// If the response is not successful, show an error message
			alert('Something went wrong');
			return;
		}
		// Get the response from the backend
		let res_json = await res.json();
		// If the response is successful, save the token to the local storage
		localStorage.setItem('userid', res_json.userid);
		// Redirect to the chat page
		window.location.href = '/chat';
	}

	async function loginToken() {
		// Get the token from the form
		let token = (document.getElementById('token') as HTMLInputElement).value;
		// Save the token to the local storage
		localStorage.setItem('token', token);
		// Redirect to the chat page
		window.location.href = '/chat';
	}

	function checkPassword() {
		// If the username and password are not empty, enable the submit button
		if (username && password) {
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = false;
		} else {
			(document.getElementById('submitPassword') as HTMLButtonElement).disabled = true;
		}
	}

	function checkToken() {
		// The token must be an openai token
		// Here is the regex to check if the token is valid
		let regex = /^sk-[a-zA-Z0-9]{48}$/;
		// If the token is valid, enable the submit button
		if (regex.test(token)) {
			(document.getElementById('submitToken') as HTMLButtonElement).disabled = false;
		} else {
			(document.getElementById('submitToken') as HTMLButtonElement).disabled = true;
		}
	}
</script>

<!-- This is the page when the user is not logged in -->
<p class="mt-4 mb-4 text-center text-2xl font-bold">
	You are not logged in. Please log in to continue.
</p>
<p class=" text-center text-sm">Why is this?</p>
<p class="text-center text-xs">
	We use OpenAI's API to generate responses. This is costly, so we need to limit the number of users
	who can use the API for free. If you want to use the API for free, you can get an anonymous token
	by going to
	<a href="https://platform.openai.com/account/api-keys" target="_blank" class="text-blue-400">
		OpenAI's beta program
	</a>
	and clicking on "Create API key". Then, you can log in with the token.
	<!-- This is the login form -->
</p>
<div class="flex items-center justify-center rounded-lg p-4 shadow-md ">
	<div
		class="m-4 flex cursor-not-allowed flex-col items-center justify-center rounded-lg border-2 border-gray-400 bg-gray-200 p-4 opacity-30 shadow-md dark:bg-gray-800"
	>
		<p>Log in with a username and password (coming soon!)</p>
		<label for="username">Username</label>
		<input
			type="text"
			name="username"
			id="username"
			class="bg-gray-200 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Username"
			bind:value={username}
			on:input={checkPassword}
			required
		/>
		<label for="password">Password</label>
		<input
			type="password"
			name="password"
			id="password"
			class="bg-gray-200 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Password"
			bind:value={password}
			on:input={checkPassword}
			required
		/>
		<button
			type="submit"
			id="submitPassword"
			on:click={loginPassword}
			class="mt-4 rounded-lg bg-blue-400 p-2 px-4 text-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
			disabled
		>
			Log in
		</button>
	</div>
	<p>Or</p>
	<div
		class="m-4 flex flex-col items-center justify-center rounded-lg border-2 border-gray-400 bg-gray-200 p-4 shadow-md dark:bg-gray-800 "
	>
		<p>Log in with an anonymous token</p>
		<!-- This is the anonymous "input token" form -->
		<label for="token">Token</label>
		<!-- To allow pasting, we need to use the on:change event like this: -->

		<input
			type="text"
			name="token"
			id="token"
			class="bg-gray-200 text-slate-800 dark:bg-gray-700 dark:text-gray-200"
			placeholder="Token"
			bind:value={token}
			on:input={checkToken}
			required
		/>
		<!-- Submit -->
		<button
			type="submit"
			id="submitToken"
			on:click={loginToken}
			class="mt-4 rounded-lg bg-blue-400 p-2 px-4 text-slate-800 disabled:cursor-not-allowed disabled:opacity-50"
			disabled
		>
			Log in
		</button>
	</div>
</div>
