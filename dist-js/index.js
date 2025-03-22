import { invoke } from '@tauri-apps/api/core';

async function ping(value) {
    return await invoke('plugin:background-geolocation|ping', {
        payload: {
            value,
        },
    }).then((r) => (r.value ? r.value : null));
}

export { ping };
