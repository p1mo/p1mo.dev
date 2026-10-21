import 'htmx.org';
import { typeIn } from "./typers.ts";


const skills = document.querySelector(`.skills .nav a:nth-child(${Math.floor(Math.random() * 8) + 1})`);


skills?.addEventListener('htmx:after:init', (evt) => {
    const element = evt.target as HTMLElement;
    if (element) {
        element.click();
    }
});


document.addEventListener('htmx:after:request', (e) => {
    if (e.detail.ctx.response?.status !== 200) return;
    const root = e.detail.ctx.sourceElement;
    const parrents = root.parentElement;
    if (!parrents?.matches('[data-select-group]'))  return;
    for (const element of parrents.children) {
        element.classList.remove('selected');
    }
    root.classList.add('selected');
});

document.addEventListener('htmx:after:swap', (e) => { // 1. Event name changed
    const event = e as CustomEvent;
    const target = event.detail.ctx.target as HTMLElement;
    if (target && target.id === 'skill_content') {
        typeIn(target, 8);
    }
});