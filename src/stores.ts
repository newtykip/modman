import { writable } from 'svelte/store';

export const title = writable<string>();
export const back = writable<boolean>(false);
