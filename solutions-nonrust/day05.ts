const dayInput = await Bun.file("day05-input").text();

let part = 2;
let example = true;
example = false;

let part1Example = `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`;

// leave blank if same as part1
let part2Example = ``;

class Dependencies {
    public dependencies: Map<number, number[]>;

    constructor(input: string) {
        let lines = input.split("\n");
        let firstBlank = lines.findIndex(l => l.length === 0);
        if (firstBlank === -1) throw new Error("No blank line found");
        lines = lines.slice(0, firstBlank);

        let dependencies = new Map();
        lines.map(l => l.split("|").map(Number)).forEach(function ([a, b]) {
                dependencies.set(b, [...(dependencies.get(b) || []), a]);
            }
        );

        this.dependencies = dependencies;
        console.log(this.dependencies);

    }
}

class Section {
    constructor(line: string | number[]) {
        this.pages = Array.isArray(line) ? line : line.split(",").map(Number);
    }

    static fromInput(input: string) {
        let lines = input.split("\n");
        let firstBlank = lines.findIndex(l => l.length === 0);
        if (firstBlank === -1) throw new Error("No blank line found");
        lines = lines.slice(firstBlank + 1);

        return lines.map(l => new Section(l));
    }

    isCorrectOrder(deps: Dependencies) {
        return this.findPageOutOfOrder(deps) === undefined;
    }

    findPageOutOfOrder(deps: Dependencies) {
        const indices = new Map(this.pages.map((p, i) => [p, i]));
        for (const page of this.pages) {
            let idx = indices.get(page);

            let pageDeps = deps.dependencies.get(page) || [];

            function isDepCorrect(dep: number): boolean {
                let depIdx = indices.get(dep);
                if (depIdx === undefined) return true; // not in section
                return depIdx < idx; // before current
            }


            if (!pageDeps.every(isDepCorrect)) {
                // console.log(`page ${page} is out of order`);
                return idx;

            }

        }


        return undefined;
    }

    movePage(pageIdx: number, to: number): Section {
        let newPages = [...this.pages];
        let tmp = newPages[pageIdx];
        newPages[pageIdx] = newPages[to];
        newPages[to] = tmp;
        return new Section(newPages);
    }
}

function partOne(input: string) {
    let deps = new Dependencies(input);
    let sections = Section.fromInput(input);
    let sum = 0;

    for (const section of sections) {
        let correct = section.isCorrectOrder(deps);
        console.log("result", section.pages, correct);
        if (correct) {
            const middle = section.pages[Math.floor(section.pages.length / 2)];
            sum += middle;
        }
    }

    return sum;
}

const shuffle = <T>(array: T[]) => {
    for (let i = array.length - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1));
        [array[i], array[j]] = [array[j], array[i]];
    }
    return array;
}

function fixSection(section: Section, deps: Dependencies) {

    let i = 0;
    console.log("==== original", section.pages);
    while (true) {
        i += 1;

        let badPage = section.findPageOutOfOrder(deps);
        if (badPage === undefined) break;

        // console.log("current", section.pages);

        let tryIndices = Array.from({length: section.pages.length}, (_, i) => i);
        shuffle(tryIndices);
        for (let newIdx of tryIndices) {
            // if (newIdx === badPage) continue;
            let newSection = section.movePage(badPage, newIdx);
            // console.log("trying", newIdx, newSection.pages);
            let nowPageOutOfOrder = newSection.findPageOutOfOrder(deps);
            if (nowPageOutOfOrder !== badPage) {
                // console.log("found", newIdx, newSection.pages);
                section = newSection;
                break;
            }
        }
        // console.log("better order", section.pages);
    }


    return {
        section, looped: i > 1
    }


}

function partTwo(input: string) {
    let deps = new Dependencies(input);
    let sections = Section.fromInput(input);
    let sum = 0;

    for (const section of sections) {
        const res = fixSection(section, deps);
        if (res.looped) {
            const fixed = res.section.pages;
            console.log("===== fixed", fixed, "from original", section.pages);
            sum += fixed[Math.floor(fixed.length / 2)];
        }


    }

    return sum;
}


if (part === 1) console.log(partOne(example ? part1Example : dayInput));
else console.log(partTwo(example ? (part2Example.length === 0 ? part1Example : part2Example) : dayInput));
