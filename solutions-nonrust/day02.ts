let input = await Bun.file("day02-input").text();

class Report {
    constructor(line) {
        this.levels = line.split(" ").map(Number);
    }

    static isSafe(levels) {
        if (levels.length < 2) {
            return false;
        }

        let monotonic =
            levels.slice(1).every((level, i) => level >= levels[i]) ||
            levels.slice(1).every((level, i) => level <= levels[i]);

        let differs = levels.slice(1).every((level, i) => {
            const prev = levels[i];
            const diff = Math.abs(level - prev);
            return diff >= 1 && diff <= 3;
        });

        return monotonic && differs;


    }

    isSafe() {
        return Report.isSafe(this.levels);
    }

    isSafeWithDampening() {
        if (this.isSafe()) return true;

        for (let i = 0; i < this.levels.length; i++) {
            let newLevels = this.levels.filter((_, j) => j !== i);
            if (Report.isSafe(newLevels)) {
                return true;
            }

        }


        return false;

    }
}

let reports = [];

// input = "7 6 4 2 1\n" +
//     "1 2 7 8 9\n" +
//     "9 7 6 2 1\n" +
//     "1 3 2 4 5\n" +
//     "8 6 4 4 1\n" +
//     "1 3 6 7 9";

reports = input.split("\n").map(line => new Report(line));

console.log(reports);


function partOne() {
    let count = reports.filter(report => report.isSafe()).length;
    console.log(count);

}

function partTwo() {

    let count = reports.filter(report => report.isSafeWithDampening()).length;
    console.log(count);
}


partTwo();
