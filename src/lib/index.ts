import { writable } from 'svelte/store';

export const footerActionsStore = writable<Footer.Action[]>([]);
