class AppIconElement extends HTMLElement {  

    ui_name   = "home";
    ui_size   = "24px";
    ui_color  = "fill";
    ui_parser = new DOMParser();

    constructor() {
        super();
    }

    connectedCallback() {
        this.ui_name = this.getAttribute('name') || this.ui_name;
        this.ui_size = this.getAttribute('size') || this.ui_size;
        this.ui_color = this.getAttribute('color') || this.ui_color;
        this.classList.add("app-icon");
        this.icon(this.ui_name);
    }

    async icon(name: string) {        
        const icon = await this.fetchSVG(name);
        const svg = this.ui_parser.parseFromString(icon, "image/svg+xml");

        this.style.width = this.ui_size + " !important";
        this.style.height = this.ui_size + " !important";
        this.setAttribute("width", this.ui_size);
        this.setAttribute("height", this.ui_size);
        
        svg.documentElement.style.width = "100%";
        svg.documentElement.style.height = "100%";
        svg.documentElement.setAttribute("width", "100%");
        svg.documentElement.setAttribute("height", "100%");

        if (this.ui_color === "fill") {
            svg.documentElement.style.fill   = "currentColor";
            svg.documentElement.style.stroke = "transparent";
        } else if (this.ui_color === "stroke") {
            svg.documentElement.style.fill   = "transparent";
            svg.documentElement.style.stroke = "currentColor";
        }

        this.attachShadow({ mode: 'open' });
        if (this.shadowRoot !== null){
            this.shadowRoot.innerHTML  = svg.documentElement.outerHTML;
        }
        
    }

    async fetchSVG(name: string) {
        const req = await fetch(`/icons/${name}.svg`);
        return await req.text()
    }

}
  
customElements.define("app-icon", AppIconElement);