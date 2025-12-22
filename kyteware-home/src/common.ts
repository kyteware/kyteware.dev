const md_files: Record<string, { default: string }> = import.meta.glob("../../facts/*.md", { query: "?raw", eager: true });

export enum Stage {
    LOADING = "LOADING",
    FILLING = "FILLING",
    READY = "READY",
    DROPPING = "DROPPING",
    FACT_DISPLAYED = "FACT_DISPLAYED",
    DONE = "DONE"
}

enum GumballCategory {
    PersonalProject = "PersonalProject",
    Experience = "Experience",
    Event = "Event",
    Tidbit = "Tidbit"
}

const cat_from_str_map = new Map();

cat_from_str_map.set("project", GumballCategory.PersonalProject);
cat_from_str_map.set("experience", GumballCategory.Experience);
cat_from_str_map.set("event", GumballCategory.Event);
cat_from_str_map.set("tidbit", GumballCategory.Tidbit);

interface Gumball {
    name: string,
    category: GumballCategory,
    content: string,
    id: number
}

export class Gumballs {
    gumballs: Array<Gumball>

    constructor() {
        let idCounter = 0;
        
        this.gumballs = Object.entries(md_files).map(([filename, content]) => {
            const truncated = filename.split('/').at(-1)!; // this must have at least one path ident!
            const [name, cat_str] = truncated.split('.').slice(0, 2);
            return {
                name: name,
                category: cat_from_str_map.get(cat_str),
                content: content.default,
                id: idCounter++
            }
        })
    }

    find(id: number) {
        return this.gumballs[id];
    }
}
