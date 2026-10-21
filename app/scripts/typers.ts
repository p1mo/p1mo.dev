export function typeIn(root: HTMLElement, speed = 20) {
    if (matchMedia('(prefers-reduced-motion: reduce)').matches) return;
    // Collect every non-empty text node, then blank them.
    const walker = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
    const nodes: { node: Text; text: string }[] = [];
    for (let n = walker.nextNode(); n; n = walker.nextNode()) {
        const t = n as Text;
        if (t.data.trim()) nodes.push({ node: t, text: t.data });
    }
    nodes.forEach(({ node }) => (node.data = ''));

    // Reveal one character at a time, across all text nodes in order.
    let ni = 0, ci = 0;
    const step = () => {
        if (ni >= nodes.length) return;
        const { node, text } = nodes[ni];
        node.data = text.slice(0, ++ci);
        if (ci >= text.length) { ni++; ci = 0; }
        if (ni < nodes.length) setTimeout(step, speed);
    };
    step();
}

export function typeText(el: HTMLElement, speed = 20) {
  const text = el.textContent ?? '';
  el.textContent = '';
  let i = 0;
  const step = () => {
    el.textContent = text.slice(0, ++i);
    if (i < text.length) setTimeout(step, speed);
  };
  step();
}