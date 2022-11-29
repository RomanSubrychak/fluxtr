// import { invoke } from '@tauri-apps/api/tauri'
const invoke = window.__TAURI__.invoke;

export async function invokeHello(name) {
    return await invoke("hello", {name});
}