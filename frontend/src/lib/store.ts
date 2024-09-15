import { writable } from 'svelte/store'

export let xrSupported = writable<boolean | undefined>(undefined)
export let enabled = writable<boolean>(false)
export let drawing: boolean = false