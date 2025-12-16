function switchTab(os) {
    // Hide all commands
    document.getElementById('cmd-mac').style.display = 'none';
    document.getElementById('cmd-win').style.display = 'none';
    document.getElementById('cmd-cargo').style.display = 'none';

    // Show selected
    document.getElementById('cmd-' + os).style.display = 'block';

    // Update tabs
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    event.target.classList.add('active');
}

function copyCommand() {
    // Find visible command
    const mac = document.getElementById('cmd-mac');
    const win = document.getElementById('cmd-win');
    const cargo = document.getElementById('cmd-cargo');

    let text = "";
    if (mac.style.display !== 'none') text = mac.innerText;
    else if (win.style.display !== 'none') text = win.innerText;
    else text = cargo.innerText;

    navigator.clipboard.writeText(text).then(() => {
        const btn = document.querySelector('.copy-btn');
        const original = btn.innerHTML;
        btn.innerHTML = '<i data-lucide="check"></i>';
        lucide.createIcons();

        setTimeout(() => {
            btn.innerHTML = original;
            lucide.createIcons();
        }, 2000);
    });
}

// Theme Toggling
function toggleTheme() {
    const html = document.documentElement;
    const currentTheme = html.getAttribute('data-theme');
    const newTheme = currentTheme === 'dark' ? 'light' : 'dark';
    html.setAttribute('data-theme', newTheme);
    localStorage.setItem('theme', newTheme);
    updateThemeIcon(newTheme);
}

function updateThemeIcon(theme) {
    const moons = document.querySelectorAll('.moon-icon');
    const suns = document.querySelectorAll('.sun-icon');

    moons.forEach(icon => icon.style.display = theme === 'dark' ? 'none' : 'block');
    suns.forEach(icon => icon.style.display = theme === 'dark' ? 'block' : 'none');
}

// Mobile Menu Toggling
function toggleMobileMenu() {
    const navLinks = document.querySelector('.nav-links');
    const sidebar = document.querySelector('.docs-sidebar');

    if (navLinks) {
        navLinks.classList.toggle('mobile-open');
    }

    if (sidebar) {
        sidebar.classList.toggle('mobile-open');
    }
}

// Init Theme
(function initTheme() {
    const savedTheme = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    document.documentElement.setAttribute('data-theme', savedTheme);

    // Run immediately if DOM is ready, otherwise wait
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => updateThemeIcon(savedTheme));
    } else {
        updateThemeIcon(savedTheme);
    }
})();
