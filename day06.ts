const dayInput = await Bun.file("day06-input").text();

let part = 2;
let example = true;
example = false;

let part1Example = `....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...`;

// leave blank if same as part1
let part2Example = ``;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

function directionOffset(dir: Direction): [number, number] {
    switch (dir) {
        case Direction.Up:
            return [0, -1];
        case Direction.Down:
            return [0, 1];
        case Direction.Left:
            return [-1, 0];
        case Direction.Right:
            return [1, 0];
    }
}


class Map {
    constructor(input: string) {
        this.grid = input.split("\n").filter(l => l.length > 0).map(l => l.split(""));

        let guardPos;
        for (let y = 0; y < this.grid.length; y++) {
            for (let x = 0; x < this.grid[0].length; x++) {
                if (this.grid[y][x] === "^") {
                    this.grid[y][x] = ".";
                    guardPos = [x, y];
                }
            }
        }

        this.guard = guardPos;
        this.guardDir = Direction.Up;
    }

    getCell(pos: [number, number]): string {
        try {
            return this.grid[pos[1]][pos[0]];
        } catch (e) {
            return undefined;
        }
    }

    stepGuard(): boolean {
        let [x, y] = this.guard;
        let [dx, dy] = directionOffset(this.guardDir);

        let ahead = this.getCell([x + dx, y + dy]);

        const log = false;
        if (ahead === ".") {
            if (log) console.log(`stepping guard from ${this.guard} to ${[x + dx, y + dy]} (${ahead})`);
            this.guard = [x + dx, y + dy];
        } else if (ahead === "#" || ahead === "O") {
            if (log) console.log(`guard hit wall at ${[x + dx, y + dy]} (${ahead})`);
            this.guardDir = (this.guardDir + 1) % 4;

        } else if (ahead === undefined) {
            if (log) console.log(`guard left map at ${[x + dx, y + dy]} (${ahead})?`);
            return true;
        } else {
            throw new Error(`unexpected char ${ahead}`);
        }

        return false;
    }

}

function partOne(input: string) {
    let map = new Map(input);
    let distinctLocs = new Set<string>();
    for (; ;) {
        distinctLocs.add(map.guard.join(","));
        if (map.stepGuard()) break;

        if (false) {
            console.log("------------ step ------------");
            for (let y = 0; y < map.grid.length; y++) {
                let row = [];
                for (let x = 0; x < map.grid[0].length; x++) {
                    let char = map.getCell([x, y]);

                    let key = [x, y].join(",");
                    let seen = distinctLocs.has(key);
                    let print = char;
                    if (x == map.guard[0] && y == map.guard[1]) {
                        print = "x";
                    } else if (seen) {
                        print = 'X';
                    }

                    row.push(print);
                }
                console.log(row.join(","));
            }
        }


    }

    return distinctLocs.size;
}

function partTwo(input: string) {
    let map = new Map(input);

    let count = 0;
    let obstruction = null;

    let guardOrig = {
        pos: map.guard,
        dir: map.guardDir,
    }

    for (let y = 0; y < map.grid.length; y++) {
        for (let x = 0; x < map.grid[0].length; x++) {
            let char = map.getCell([x, y]);

            if (char === "#") {
                // already a wall
                continue;
            }

            if (obstruction != null) {
                // restore prev obstruction
                map.grid[obstruction.pos[1]][obstruction.pos[0]] = obstruction.char;
            }

            // place new obstruction
            obstruction = {
                pos: [x, y],
                char: char,
            };
            map.grid[y][x] = "O";

            // restore guard
            map.guard = guardOrig.pos;
            map.guardDir = guardOrig.dir;

            let distinctLocs = new Set<string>();

            let prevDistinct = distinctLocs.size;
            let sameFor = 0;
            for (; ;) {
                distinctLocs.add(map.guard.join(","));

                if (prevDistinct == distinctLocs.size) {
                    sameFor++;
                } else {
                    sameFor = 0;
                    prevDistinct = distinctLocs.size;
                }

                // console.log("same for", sameFor, " out of ", distinctLocs.size);
                if (sameFor > 1000) {
                    console.log(`loop found at ${[x,y].join(",")}`);
                    count += 1;
                    break;
                }

                if (map.stepGuard()) break;
            }


        }
    }

    return count;
}


if (part === 1) console.log(partOne(example ? part1Example : dayInput));
else console.log(partTwo(example ? (part2Example.length === 0 ? part1Example : part2Example) : dayInput));
