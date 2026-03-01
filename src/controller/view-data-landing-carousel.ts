import { invoke } from '@tauri-apps/api/core' // v2 recommended import

interface AlertData {
    account_name: string
    delta: number
    total: number
}

export async function getAlerts() {
    // cast raw to the expected type
    const raw = await invoke<AlertData[]>('get_alert_data')

    return raw.map((item) => ({
        title: item.account_name,
        message: `Up $${item.delta}`,
        message2: `Total: $${item.total}`,
    }))
}