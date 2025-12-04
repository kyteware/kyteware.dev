interface GumballData {
    name: string,
    description: string
}

interface GumballSourceData {
    personal_projects: Array<GumballData>,
    experiences: Array<GumballData>,
    events: Array<GumballData>,
    tidbits: Array<GumballData>,
}

enum GumballCategory {
    PersonalProject = "PersonalProject",
    Experience = "Experience",
    Event = "Event",
    Tidbit = "Tidbit"
}

interface Gumball {
    name: string,
    description: string,
    category: GumballCategory,
    id: number
}

export class Gumballs {
    gumballs: Array<Gumball>

    constructor(raw: GumballSourceData) {
        let idCounter = 0;

        const entries = [
            { cat: GumballCategory.PersonalProject, data: raw.personal_projects },
            { cat: GumballCategory.Experience, data: raw.experiences },
            { cat: GumballCategory.Event, data: raw.events },
            { cat: GumballCategory.Tidbit, data: raw.tidbits },
        ]
        
        this.gumballs = entries.flatMap(
            entry => entry.data.map(
                rawGumball => ({
                    ...rawGumball,
                    id: idCounter++,
                    category: entry.cat
                })
            )
        )
    }

    find(id: number) {
        return this.gumballs[id];
    }
}
