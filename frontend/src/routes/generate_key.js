// Generate this key: 12.12.2020:asdh9f:12.00.00

// To import this function in svelte script tag, use this syntax:
// import { generateKey } from '../routes/generate_key.js';

export const generateKey = () => {
	const date = new Date();
	const dateKey = `${date.getDate()}.${date.getMonth()}.${date.getFullYear()}`;
	// Exactly 6 alphanumeric characters
	const randomKey = Math.random().toString(36).substring(2, 8);
	const timeKey = `${date.getHours()}.${date.getMinutes()}.${date.getSeconds()}`;
	return `${dateKey}:${randomKey}:${timeKey}`;
};
