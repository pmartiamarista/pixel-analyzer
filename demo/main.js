import init, { analyze, AnalysisOptions } from 'pixel-analyzer';

const tabBtns       = document.querySelectorAll('.tab-btn');
const panels        = document.querySelectorAll('.upload-zone');
const fileInput     = document.getElementById('file-input');
const urlInput      = document.getElementById('url-input');
const pasteZone     = document.getElementById('panel-paste');
const analyzeBtn    = document.getElementById('analyze-btn');
const btnLabel      = document.getElementById('btn-label');
const btnSpinner    = document.getElementById('btn-spinner');
const resultsArea   = document.getElementById('results-area');
const loader        = document.getElementById('analysis-loader');
const results       = document.getElementById('results');
const imagePreview  = document.getElementById('image-preview');
const dominantSwatch  = document.getElementById('dominant-swatch');
const accentSwatch    = document.getElementById('accent-swatch');
const vibrantGrid     = document.getElementById('vibrant-palette');
const mutedGrid       = document.getElementById('muted-palette');
const entropyValue    = document.getElementById('entropy-value');
const timingValue     = document.getElementById('timing-value');
const statsList       = document.getElementById('image-stats');
const accessibilityInfo = document.getElementById('accessibility-info');
const theoryInfo      = document.getElementById('theory-info');

const allInputs = () => [
    ...tabBtns,
    fileInput,
    urlInput,
    document.getElementById('max-colors'),
    document.getElementById('quality'),
    document.getElementById('convergence'),
];

let currentBytes  = null;
let isProcessing  = false;

async function bootstrap() {
    lockUI('Initializing…');
    try {
        await init();
        unlockUI();
        setupEventListeners();
    } catch (e) {
        console.error(e);
        setStatus(`WASM failed to load: ${e.message}. Try refreshing.`, 'error');
        setBtnLoading(false, 'Unavailable');
        analyzeBtn.disabled = true;
        analyzeBtn.setAttribute('aria-disabled', 'true');
    }
}

function setupEventListeners() {
    tabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            tabBtns.forEach(b => { b.classList.remove('active'); b.setAttribute('aria-selected', 'false'); });
            panels.forEach(p => p.classList.remove('active-panel'));
            btn.classList.add('active');
            btn.setAttribute('aria-selected', 'true');
            document.getElementById(`panel-${btn.dataset.tab}`).classList.add('active-panel');
        });
    });

    const filePanel = document.getElementById('panel-file');
    filePanel.addEventListener('click', () => fileInput.click());
    filePanel.addEventListener('dragover', e => { e.preventDefault(); filePanel.classList.add('drag-over'); });
    filePanel.addEventListener('dragleave', () => filePanel.classList.remove('drag-over'));
    filePanel.addEventListener('drop', e => {
        e.preventDefault();
        filePanel.classList.remove('drag-over');
        const file = e.dataTransfer.files[0];
        if (file && file.type.startsWith('image/')) handleFile(file);
    });

    fileInput.addEventListener('change', e => {
        if (e.target.files[0]) handleFile(e.target.files[0]);
    });

    urlInput.addEventListener('keydown', async e => {
        if (e.key === 'Enter') await handleUrl(urlInput.value.trim());
    });

    pasteZone.addEventListener('click', () => pasteZone.focus());
    window.addEventListener('paste', async e => {
        if (document.activeElement === pasteZone || document.activeElement === document.body) {
            for (const item of e.clipboardData.items) {
                if (item.type.startsWith('image/')) { handleFile(item.getAsFile()); break; }
            }
        }
    });

    analyzeBtn.addEventListener('click', () => {
        if (isProcessing) return;
        if (currentBytes) {
            analyzeBtn.classList.remove('highlight');
            runAnalysis(currentBytes);
        } else {
            setStatus('Load an image first.', 'error');
        }
    });
}

async function handleFile(file) {
    const objectUrl = URL.createObjectURL(file);
    imagePreview.src = objectUrl;
    imagePreview.onload = () => URL.revokeObjectURL(objectUrl);
    currentBytes = new Uint8Array(await file.arrayBuffer());
    setStatus('Image loaded — ready to analyze.', 'success');
    analyzeBtn.classList.add('highlight');
}

async function handleUrl(url) {
    if (!url) return;
    if (!isHttpsUrl(url)) {
        setStatus('Only HTTPS URLs are supported.', 'error');
        urlInput.setAttribute('aria-invalid', 'true');
        urlInput.setAttribute('aria-describedby', 'global-status');
        return;
    }
    urlInput.removeAttribute('aria-invalid');
    urlInput.removeAttribute('aria-describedby');
    setStatus('Fetching image…', '');
    try {
        const blob = await (await fetch(url)).blob();
        const objectUrl = URL.createObjectURL(blob);
        imagePreview.src = objectUrl;
        imagePreview.onload = () => URL.revokeObjectURL(objectUrl);
        currentBytes = new Uint8Array(await blob.arrayBuffer());
        setStatus('Image loaded — ready to analyze.', 'success');
        analyzeBtn.classList.add('highlight');
    } catch {
        setStatus('Fetch failed. CORS error or invalid URL.', 'error');
        urlInput.setAttribute('aria-invalid', 'true');
        urlInput.setAttribute('aria-describedby', 'global-status');
    }
}

async function runAnalysis(bytes) {
    isProcessing = true;
    lockUI('Analyzing…');
    showLoader();

    try {
        const maxColors  = parseInt(document.getElementById('max-colors').value, 10);
        const quality    = parseInt(document.getElementById('quality').value, 10);
        const convergence = parseFloat(document.getElementById('convergence').value);
        const report = await analyze(bytes, new AnalysisOptions(maxColors, quality, convergence));
        renderReport(report);
        setStatus('Analysis complete.', 'success');
    } catch (e) {
        console.error(e);
        setStatus('Analysis failed. See console for details.', 'error');
        hideLoader();
    } finally {
        unlockUI();
        isProcessing = false;
    }
}

function lockUI(btnText = 'Processing…') {
    setBtnLoading(true, btnText);
    allInputs().forEach(el => { el.disabled = true; });
    panels.forEach(p => p.setAttribute('aria-disabled', 'true'));
    pasteZone.setAttribute('tabindex', '-1');
}

function unlockUI() {
    setBtnLoading(false, 'Analyze');
    allInputs().forEach(el => { el.disabled = false; });
    panels.forEach(p => p.removeAttribute('aria-disabled'));
    pasteZone.setAttribute('tabindex', '0');
}

function showLoader() {
    resultsArea.classList.remove('hidden');
    loader.classList.remove('hidden');
    results.classList.add('hidden');
    resultsArea.scrollIntoView({ behavior: 'smooth', block: 'start' });
    loader.focus();
}

function hideLoader() {
    loader.classList.add('hidden');
    results.classList.remove('hidden');
}

function renderReport(report) {
    entropyValue.textContent = report.image_stats.entropy.toFixed(2);
    timingValue.textContent  = `${report.analysis_time_ms.toFixed(1)}ms`;

    dominantSwatch.innerHTML = '';
    dominantSwatch.appendChild(createSwatchUnit(report.main.dominant.hex));

    accentSwatch.innerHTML = '';
    if (report.main.accent) {
        accentSwatch.appendChild(createSwatchUnit(report.main.accent.hex));
    } else {
        accentSwatch.innerHTML = '<p style="color:var(--text-muted);font-size:0.8rem">None</p>';
    }

    renderPalette(vibrantGrid, report.palettes.vibrant);
    renderPalette(mutedGrid, report.palettes.muted);

    statsList.innerHTML = `
        <p><strong>Brightness</strong>   <span>${report.image_stats.brightness.toFixed(1)}%</span></p>
        <p><strong>Colorfulness</strong> <span>${report.image_stats.colorfulness.toFixed(1)}</span></p>
        <p><strong>Hue Group</strong>    <span>${report.image_stats.dominant_hue_group}</span></p>
        <p><strong>Pixels</strong>       <span>${report.pixels_analyzed.toLocaleString()}</span></p>
    `;

    accessibilityInfo.innerHTML = `
        <p><strong>Contrast</strong>  <span>${report.accessibility.contrast_ratio.toFixed(2)}:1</span></p>
        <p><strong>WCAG AA</strong>   <span>${report.accessibility.is_aa_normal ? '✓ Pass' : '✗ Fail'}</span></p>
        <p><strong>Font Color</strong> ${colorChip(report.accessibility.recommended_font_color)}</p>
    `;

    theoryInfo.innerHTML = `
        <p><strong>Complement</strong> ${colorChip(report.color_theory.complementary)}</p>
        <p><strong>Triadic</strong>    <span class="theory-row">${report.color_theory.triadic.map(colorChip).join('')}</span></p>
        <p><strong>Analogous</strong>  <span class="theory-row">${report.color_theory.analogous.map(colorChip).join('')}</span></p>
    `;

    results.querySelectorAll('.inline-color-group').forEach(group => {
        const hex = group.dataset.hex;
        const box = group.querySelector('.mini-box');
        group.onclick = () => copyHex(hex, box);
        group.addEventListener('keydown', e => {
            if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); copyHex(hex, box); }
        });
    });

    hideLoader();
    results.scrollIntoView({ behavior: 'smooth', block: 'start' });
}

function renderPalette(container, colors) {
    container.innerHTML = '';
    if (!colors.length) {
        container.innerHTML = '<p style="color:var(--text-muted);font-size:0.8rem">None</p>';
        return;
    }
    colors.slice(0, 8).forEach(color => container.appendChild(createSwatchUnit(color.hex)));
}

function createSwatchUnit(hex) {
    const wrapper = document.createElement('div');
    wrapper.className = 'swatch-unit';

    const box = document.createElement('div');
    box.className = 'color-box';
    box.style.backgroundColor = hex;
    box.setAttribute('role', 'button');
    box.setAttribute('tabindex', '0');
    box.setAttribute('aria-label', `Copy color ${hex}`);
    box.onclick = () => copyHex(hex, box);
    box.addEventListener('keydown', e => {
        if (e.key === 'Enter' || e.key === ' ') { e.preventDefault(); copyHex(hex, box); }
    });

    const label = document.createElement('span');
    label.className = 'hex-text';
    label.setAttribute('aria-hidden', 'true');
    label.textContent = hex;

    wrapper.appendChild(box);
    wrapper.appendChild(label);
    return wrapper;
}

function colorChip(hex) {
    return `<span class="inline-color-group" role="button" tabindex="0" data-hex="${hex}" aria-label="Copy color ${hex}"><span class="mini-box" style="background-color:${hex}" aria-hidden="true"></span><code>${hex}</code></span>`;
}

function copyHex(hex, el) {
    navigator.clipboard.writeText(hex);
    el.classList.add('copied');
    setTimeout(() => el.classList.remove('copied'), 1200);
}

function setStatus(message, type = '') {
    const el = document.getElementById('global-status');
    el.textContent = message;
    el.className = `status-msg ${type}`.trim();
}

function setBtnLoading(loading, label) {
    btnLabel.textContent = label;
    btnSpinner.classList.toggle('hidden', !loading);
    analyzeBtn.disabled = loading;
    analyzeBtn.setAttribute('aria-busy', String(loading));
}

function isHttpsUrl(url) {
    try { return new URL(url).protocol === 'https:'; } catch { return false; }
}

bootstrap();
