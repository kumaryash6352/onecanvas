import { writable } from 'svelte/store'
export let xrSupported = writable<boolean | undefined>(undefined)