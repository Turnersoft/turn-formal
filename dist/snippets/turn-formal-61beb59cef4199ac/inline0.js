
export function initShoelace() {
    return true;
}

export function toggleAlert(alertId) {
    const alert = document.getElementById(alertId);
    if (alert) {
        alert.open = !alert.open;
    }
}
