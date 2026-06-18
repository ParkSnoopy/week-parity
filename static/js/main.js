import init, { get_week_status_wasm } from '../../pkg/week_parity_wasm.js';
import { CONFIG } from './config.js';

function parseConfigToml(toml) {
    const baseDateMatch = toml.match(/^base_date\s*=\s*\[([\s\S]*?)\]/m);
    const timezoneMatch = toml.match(/^timezone\s*=\s*"([^"]+)"/m);
    const baseDates = baseDateMatch
        ? [...baseDateMatch[1].matchAll(/"([^"]+)"/g)].map((match) => match[1])
        : [];

    return {
        baseDates,
        timezone: timezoneMatch?.[1],
    };
}

async function loadConfig() {
    try {
        const response = await fetch('./config.toml', { cache: 'no-store' });
        if (!response.ok) throw new Error(`config.toml ${response.status}`);

        const config = parseConfigToml(await response.text());
        return {
            baseDates: config.baseDates.length ? config.baseDates : CONFIG.baseDates,
            timezone: config.timezone || CONFIG.timezone,
        };
    } catch (error) {
        console.warn('Using static JS config fallback:', error);
        return CONFIG;
    }
}

function latestOldDate(dates, today) {
    const pastDates = dates
        .filter((date) => date <= today)
        .sort((a, b) => b.localeCompare(a));

    return pastDates[0] || [...dates].sort()[0];
}

function renderBaseDateSelect(dates, selectedDate) {
    const options = [...dates]
        .sort()
        .map((date) => `<option value="${date}"${date === selectedDate ? ' selected' : ''}>${date}</option>`)
        .join('');

    return `<select class="date-control" aria-label="Select semester start date">${options}</select>`;
}

function renderDatePicker(selectedDate, label) {
    return `<input class="date-control" type="date" value="${selectedDate}" aria-label="${label}">`;
}

function renderDateValue(element, date, control) {
    element.innerHTML = `
        <span class="date-current">${date}</span>
        ${control}
    `;
}

function renderStatus(baseDate, today) {
    const status = get_week_status_wasm(baseDate, today);
    const [week, parityEng] = status.split(': ');
    const parityMap = { odd: '单周', even: '双周' };
    const parity = parityMap[parityEng.toLowerCase()] || parityEng;
    const parityBadge = `<span class="badge badge-${parityEng.toLowerCase()}">${parity}</span>`;

    document.getElementById('res').innerHTML = `${week} ${parityBadge}`;
}

async function run() {
    await init();

    const { baseDates, timezone } = await loadConfig();

    const today = new Intl.DateTimeFormat('en-CA', {
        timeZone: timezone,
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
    }).format(new Date());

    let baseDate = latestOldDate(baseDates, today);
    let currentDate = today;
    const startValue = document.getElementById('start-val');
    const currentValue = document.getElementById('current-val');

    renderDateValue(startValue, baseDate, renderBaseDateSelect(baseDates, baseDate));
    renderDateValue(currentValue, currentDate, renderDatePicker(currentDate, 'Select current date'));

    renderStatus(baseDate, currentDate);

    startValue.querySelector('.date-control').addEventListener('change', (event) => {
        baseDate = event.target.value;
        startValue.querySelector('.date-current').innerText = baseDate;
        renderStatus(baseDate, currentDate);
    });

    currentValue.querySelector('.date-control').addEventListener('change', (event) => {
        currentDate = event.target.value;
        currentValue.querySelector('.date-current').innerText = currentDate;
        renderStatus(baseDate, currentDate);
    });
}

run().catch(console.error);