let input = await Bun.file("day04-input").text();

let part = 2;
let example = true;
example = false;

let part1Example = `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`;

const offsetsIncludingDiagonals = [
    [0, 1],
    [1, 0],
    [0, -1],
    [-1, 0],
    [1, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
] as const;

const onlyDiagonalOffsets = [
    [1, 1],
    [-1, -1],
    [1, -1],
    [-1, 1],
] as const;


class Grid {
    private grid: string[];

    constructor(str: string) {
        this.grid = str.split("\n").map(s => s.trim().split(""));
    }

    findAll(deltaOffset, needle: string, returnLocationIdx: number) {
        let count = 0;
        let locations = [];
        for (let y = 0; y < this.grid.length; y++) {
            for (let x = 0; x < this.grid[y].length; x++) {
                const coords = Array.from({length: needle.length}, (_, i) =>
                    [x + deltaOffset[0] * i, y + deltaOffset[1] * i],
                );

                const word = coords.map(([x, y]) => {
                    try {
                        return this.grid[y][x];
                    } catch (e) {

                    }
                }).join("");
                if (word === needle) {
                    count++;
                    locations.push(coords[returnLocationIdx]);
                }
            }
        }

        return [count, locations];
    }
}

function partOne() {
    if (example)
        input = part1Example;

    let grid = new Grid(input);
    let sum = 0;
    for (const offset of offsetsIncludingDiagonals) {
        let [count, _] = grid.findAll(offset, "XMAS");
        console.log(`found ${count} with direction ${offset}`);
        sum += count;
    }

    return sum;

}

function partTwo() {
    if (example)
        input = part1Example;

    let grid = new Grid(input);
    let sum = 0;
    let foundLocs = new Map();
    for (const offset of onlyDiagonalOffsets) {
        let [count, locs] = grid.findAll(offset, "MAS", 1); // A
        console.log(`found ${count} with direction ${offset} at ${JSON.stringify(locs)}`);
        for (const loc of locs) {
            let key = loc.join(",");

            foundLocs.set(key, (foundLocs.get(key)||0) + 1);
        }
    }
    // console.log(foundLocs);
    sum = foundLocs.values().reduce((acc, v) => acc+v-1, 0);
    return sum;
}

console.log(part === 1 ? partOne() : partTwo());
